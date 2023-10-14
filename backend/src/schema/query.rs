use tracing::error;

use super::data;
use super::Db;
use super::Driver;
use super::League;
use super::Session;

use crate::auth::Claims;
use crate::queries::User;

use async_graphql::Context;
use async_graphql::Error;
use async_graphql::Object;
use async_graphql::ID;

pub struct Query;

#[Object]
impl Query {
    pub(crate) async fn driver<'a>(&self, ctx: &Context<'a>, id: ID) -> Result<&'a Driver, Error> {
        ctx.data_unchecked::<data::Data>()
            .drivers
            .iter()
            .find(|driver| driver.id == id)
            .ok_or(Error::new(format!("Driver `{:?}` not found", id)))
    }

    pub(crate) async fn leagues<'a>(&self, ctx: &Context<'a>) -> &'a Vec<League> {
        &ctx.data_unchecked::<data::Data>().leagues
    }

    pub(crate) async fn session<'a>(
        &self,
        ctx: &Context<'a>,
        session_id: ID,
    ) -> Result<&'a Session, Error> {
        ctx.data_unchecked::<data::Data>()
            .sessions
            .iter()
            .find(|session| session.id == session_id)
            .ok_or(Error::new("Session not found"))
    }

    pub(crate) async fn league<'a>(&self, ctx: &Context<'a>, id: ID) -> Result<&'a League, Error> {
        ctx.data_unchecked::<data::Data>()
            .leagues
            .iter()
            .find(|league| league.id == id)
            .ok_or(Error::new("League not found"))
    }

    pub(crate) async fn me<'a>(&self, ctx: &Context<'a>) -> Result<Option<User>, Error> {
        let claims = ctx
            .data_opt::<Claims>()
            .ok_or(Error::new("401 Unauthorized"))?;

        let sub = claims.sub.clone();

        let Db(db) = ctx.data_unchecked::<Db>();

        match db.get_user(&sub).await {
            Ok(user) => Ok(user),
            Err(err) => {
                error!("error while accessing database {}", err);
                Err(Error::new("Database error"))
            }
        }
    }
}
