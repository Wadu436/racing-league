use async_graphql::{InputObject, SimpleObject, ID};
use serde::Deserialize;
use sqlx::PgPool;

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

    pub async fn create_user(&self, data: NewUser) -> sqlx::Result<()> {
        // TODO: profile picture
        sqlx::query!(
            "insert into users (id, sub, username, steam_id, ea_id) values ($1, $2, $3, $4, $5)",
            uuid::Uuid::new_v4(),
            data.sub,
            data.username,
            data.steam_id,
            data.ea_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn update_user(&self, data: User) -> sqlx::Result<()> {
        // TODO: profile picture
        sqlx::query!(
            "update users set username=$2, steam_id=$3, ea_id=$4 where id=$1",
            data.id,
            data.username,
            data.steam_id,
            data.ea_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
