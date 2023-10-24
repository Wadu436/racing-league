use async_graphql::SimpleObject;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::debug;

#[derive(SimpleObject, Clone, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub sub: String,
    pub username: String,
    pub profile_picture_path: Option<String>,
    pub steam_id: Option<String>,
    pub ea_id: Option<String>,
}

#[derive(SimpleObject, Clone, Deserialize)]
pub struct NewUser {
    pub sub: String,
    pub username: String,
    pub profile_picture_path: Option<String>,
    pub steam_id: Option<String>,
    pub ea_id: Option<String>,
}

pub struct Client {
    pub db: PgPool,
}

impl Client {
    pub async fn get_user(&self, sub: &str) -> sqlx::Result<Option<User>> {
        match sqlx::query_as!(User, "select * from users where sub = $1", sub)
            .fetch_one(&self.db)
            .await
        {
            Ok(user) => Ok(Some(user)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub async fn create_user(&self, data: NewUser) -> sqlx::Result<User> {
        let id = uuid::Uuid::new_v4();
        sqlx::query!(
            "insert into users (id, sub, username, steam_id, ea_id, profile_picture_path) values ($1, $2, $3, $4, $5, $6)",
            id,
            data.sub,
            data.username,
            data.steam_id,
            data.ea_id,
            data.profile_picture_path
        )
        .execute(&self.db)
        .await?;

        Ok(User {
            id,
            sub: data.sub,
            username: data.username,
            profile_picture_path: data.profile_picture_path,
            steam_id: data.steam_id,
            ea_id: data.ea_id,
        })
    }

    pub async fn update_user(&self, data: &User) -> sqlx::Result<()> {
        debug!("profile picture path: {:?}", data.profile_picture_path);
        sqlx::query!(
            "update users set username=$2, steam_id=$3, ea_id=$4, profile_picture_path=$5 where id=$1",
            data.id,
            data.username,
            data.steam_id,
            data.ea_id,
            data.profile_picture_path
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
