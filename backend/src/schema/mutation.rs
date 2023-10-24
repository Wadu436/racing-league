use async_graphql::InputObject;
use async_graphql::SimpleObject;
use serde::Deserialize;
use tracing::debug;
use tracing::error;

use super::Db;

use crate::auth::Claims;
use crate::queries::NewUser;
use crate::queries::User;

use async_graphql::Context;
use async_graphql::Error;
use async_graphql::Object;

pub struct Mutation;

#[derive(InputObject, SimpleObject, Clone, Deserialize)]
pub struct SignupData {
    pub username: String,
    pub steam_id: Option<String>,
    pub ea_id: Option<String>,
    pub profile_picture_path: Option<String>,
}

#[derive(InputObject, SimpleObject, Clone, Deserialize)]
pub struct UpdateData {
    pub username: String,
    pub steam_id: Option<String>,
    pub ea_id: Option<String>,
    pub profile_picture_path: Option<String>,
    pub delete_profile_picture: bool,
}

#[Object]
impl Mutation {
    pub(crate) async fn signup(&self, ctx: &Context<'_>, data: SignupData) -> Result<User, Error> {
        let claims = ctx
            .data_opt::<Claims>()
            .ok_or(Error::new("401 Unauthorized"))?;

        let sub = &claims.sub;

        let Db(db) = ctx.data_unchecked::<Db>();

        if db.get_user(sub).await.unwrap().is_some() {
            error!("User already exists");
            return Err(Error::new("User already exists"));
        }

        debug!("profile picture path: {:?}", data.profile_picture_path);

        match db
            .create_user(NewUser {
                sub: sub.to_owned(),
                username: data.username,
                ea_id: data.ea_id,
                steam_id: data.steam_id,
                profile_picture_path: data.profile_picture_path,
            })
            .await
        {
            Ok(user) => Ok(user),
            Err(err) => {
                error!("error while accessing database {}", err);
                Err(Error::new("Database error"))
            }
        }
    }

    pub(crate) async fn update_user(
        &self,
        ctx: &Context<'_>,
        data: UpdateData,
    ) -> Result<User, Error> {
        let claims = ctx
            .data_opt::<Claims>()
            .ok_or(Error::new("401 Unauthorized"))?;

        let sub = &claims.sub;

        let Db(db) = ctx.data_unchecked::<Db>();

        if let Some(mut user) = db.get_user(sub).await.unwrap() {
            user.username = data.username;
            user.steam_id = data.steam_id;
            user.ea_id = data.ea_id;
            debug!("profile picture path: {:?}", data.profile_picture_path);
            debug!("delete profile picture: {:?}", data.delete_profile_picture);
            if let Some(path) = data.profile_picture_path {
                user.profile_picture_path = Some(path);
            } else if data.delete_profile_picture {
                user.profile_picture_path = None;
            }

            match db.update_user(&user).await {
                Ok(_) => Ok(user),
                Err(err) => {
                    error!("error while accessing database {}", err);
                    Err(Error::new("Database error"))
                }
            }
        } else {
            error!("User doesn't exist");
            Err(Error::new("User doesn't exist"))
        }
    }
}
