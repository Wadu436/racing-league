use std::collections::HashMap;

use async_graphql::{ComplexObject, Context, EmptySubscription, Enum, Object, SimpleObject};
use async_graphql::{Error, ID};

use async_graphql::extensions::ApolloTracing;
use futures::future::join_all;
use serde::Deserialize;
use uuid::Uuid;

use crate::queries::Client;

pub mod data;
pub mod mutation;
pub mod query;

pub type Schema = async_graphql::Schema<query::Query, mutation::Mutation, EmptySubscription>;

#[derive(Enum, Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum LeagueStatus {
    Active,
    Finished,
    Planned,
}

#[derive(Deserialize)]
pub struct LeagueEntry {
    #[serde(rename = "league")]
    league_id: ID,
    #[serde(rename = "driver")]
    driver_id: ID,
    #[serde(rename = "team")]
    team_id: ID,
}

#[Object]
impl LeagueEntry {
    async fn league<'a>(&self, ctx: &Context<'a>) -> Result<&'a League, Error> {
        ctx.data_unchecked::<data::Data>()
            .leagues
            .iter()
            .find(|league| league.id == self.league_id)
            .ok_or(Error::new(format!("League {} not found", *self.league_id)))
    }

    async fn driver<'a>(&self, ctx: &Context<'a>) -> Result<&'a Driver, Error> {
        ctx.data_unchecked::<data::Data>()
            .drivers
            .iter()
            .find(|driver| driver.id == self.driver_id)
            .ok_or(Error::new(format!(
                "Driver `{:?}` not found",
                *self.driver_id
            )))
    }

    async fn team<'a>(&self, ctx: &Context<'a>) -> Result<&'a Team, Error> {
        ctx.data_unchecked::<data::Data>()
            .teams
            .iter()
            .find(|team| team.id == self.team_id)
            .ok_or(Error::new(format!("Team `{}` not found", *self.team_id)))
    }
}

#[derive(SimpleObject, Deserialize)]
#[graphql(complex)]
pub struct League {
    id: ID,
    name: String,
    status: LeagueStatus,
}

#[derive(SimpleObject, PartialEq, PartialOrd, Ord, Eq)]
#[graphql(complex)]
pub struct WdcLeaderboardEntry {
    points: u32,
    #[graphql(skip)]
    driver_id: ID,
    #[graphql(skip)]
    team_id: ID,
}

#[ComplexObject]
impl WdcLeaderboardEntry {
    async fn driver<'a>(&self, ctx: &Context<'a>) -> Result<&'a Driver, Error> {
        ctx.data_unchecked::<data::Data>()
            .drivers
            .iter()
            .find(|driver| driver.id == self.driver_id)
            .ok_or(Error::new(format!(
                "Driver `{:?}` not found",
                self.driver_id
            )))
    }

    async fn team<'a>(&self, ctx: &Context<'a>) -> Result<&'a Team, Error> {
        ctx.data_unchecked::<data::Data>()
            .teams
            .iter()
            .find(|team| team.id == self.team_id)
            .ok_or(Error::new("Team not found"))
    }
}

#[ComplexObject]
impl League {
    async fn events<'a>(&self, ctx: &Context<'a>) -> Vec<&'a Event> {
        let data = ctx.data_unchecked::<data::Data>();

        data.events
            .iter()
            .filter(|event| event.league_id == self.id)
            .collect()
    }

    async fn entries<'a>(&self, ctx: &Context<'a>) -> Vec<&'a LeagueEntry> {
        let data = ctx.data_unchecked::<data::Data>();

        data.league_entries
            .iter()
            .filter(|entry| entry.league_id == self.id)
            .collect()
    }

    async fn event<'a>(
        &self,
        ctx: &Context<'a>,
        championship_order: u32,
    ) -> Result<&'a Event, Error> {
        let data = ctx.data_unchecked::<data::Data>();

        data.events
            .iter()
            .find(|event| {
                event.league_id == self.id && event.championship_order == championship_order
            })
            .ok_or(Error::new("Event not found"))
    }

    // TODO implement countback
    async fn wdc_leaderboard(&self, ctx: &Context<'_>) -> Result<Vec<WdcLeaderboardEntry>, Error> {
        let events = self.events(ctx).await?;

        // Why do the results disappear when I do flatten() ?
        let sessions = join_all(
            events
                .iter()
                .map(|event| event.sessions(ctx))
                .collect::<Vec<_>>(),
        )
        .await
        .into_iter()
        .flatten()
        .flatten()
        .collect::<Vec<_>>();

        let mut leaderboard_hashmap: HashMap<ID, (u32, HashMap<ID, u32>)> = HashMap::new();

        for session in sessions.iter() {
            for entry in session.entries.iter() {
                let points = entry.points(ctx).await?;

                let entry = entry.entry(ctx).await?;

                leaderboard_hashmap
                    .entry(entry.driver_id.clone())
                    .and_modify(|entry| {
                        entry.0 += points;
                    })
                    .or_insert((points, HashMap::from([(entry.team_id.clone(), 1)])));
            }
        }

        let mut entries = leaderboard_hashmap
            .drain()
            .map(|(driver_id, (points, team_count))| {
                Ok(WdcLeaderboardEntry {
                    driver_id,
                    points,
                    team_id: team_count
                        .iter()
                        .max_by_key(|(_, v)| *v)
                        .ok_or(Error::new("Team not found"))?
                        .0
                        .clone(),
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        entries.sort_by(|a, b| b.cmp(a));

        Ok(entries)
    }
}

#[derive(SimpleObject, Deserialize)]
#[graphql(complex)]
pub struct Event {
    id: ID,
    name: String,
    championship_order: u32,
    date: chrono::DateTime<chrono::Utc>,
    #[graphql(skip)]
    track_id: ID,
    #[graphql(skip)]
    league_id: ID,
    entries: Vec<EventEntry>,
}

#[ComplexObject]
impl Event {
    async fn track<'a>(&self, ctx: &Context<'a>) -> Result<&'a Track, Error> {
        let data = ctx.data_unchecked::<data::Data>();

        data.tracks
            .iter()
            .find(|track| track.id == self.track_id)
            .ok_or(Error::new("Track not found"))
    }

    async fn league<'a>(&self, ctx: &Context<'a>) -> Result<&'a League, Error> {
        ctx.data_unchecked::<data::Data>()
            .leagues
            .iter()
            .find(|league| league.id == self.league_id)
            .ok_or(Error::new("League not found"))
    }

    async fn sessions<'a>(&self, ctx: &Context<'a>) -> Vec<&'a Session> {
        let data = ctx.data_unchecked::<data::Data>();

        data.sessions
            .iter()
            .filter(|item| self.id == item.event_id)
            .collect()
    }

    async fn points_rule(&self, _ctx: &Context<'_>, session_type: SessionType) -> PointsRule {
        match session_type {
            SessionType::Race => PointsRule {
                points_per_position: vec![25, 18, 15, 12, 10, 8, 6, 4, 2, 1],
                points_for_fastest_lap: 1,
            },
            SessionType::Sprint => PointsRule {
                points_per_position: vec![8, 7, 6, 5, 4, 3, 2, 1],
                points_for_fastest_lap: 1,
            },
            _ => PointsRule {
                points_per_position: vec![],
                points_for_fastest_lap: 0,
            },
        }
    }
}

#[derive(SimpleObject, Deserialize)]
pub struct PointsRule {
    points_per_position: Vec<u32>,
    points_for_fastest_lap: u32,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum SessionType {
    Race,
    Q1,
    Q2,
    Q3,
    ShortQualifying,
    OneShotQualifying,
    Sprint,
    SprintQualifying,
    Practice,
}

#[derive(SimpleObject, Deserialize)]
#[graphql(complex)]
pub struct Session {
    id: ID,
    #[graphql(skip)]
    event_id: ID,
    session_type: SessionType,
    // TODO these should be entries per Event, not per session (it shouldnt be possible to switch teams between sessions inside of an event)
    entries: Vec<SessionEntry>,
    // Entry fastest lap
    #[graphql(skip)]
    fastest_lap: ID,
}

#[ComplexObject]
impl Session {
    async fn event<'a>(&self, ctx: &Context<'a>) -> Result<&'a Event, Error> {
        ctx.data_unchecked::<data::Data>()
            .events
            .iter()
            .find(|event| event.id == self.event_id)
            .ok_or(Error::new("Event not found"))
    }

    async fn fastest_lap<'a>(&self, ctx: &Context<'a>) -> Result<&'a Driver, Error> {
        ctx.data_unchecked::<data::Data>()
            .drivers
            .iter()
            .find(|driver| driver.id == self.fastest_lap)
            .ok_or(Error::new(format!(
                "Driver `{:?}` not found",
                self.fastest_lap
            )))
    }
}

#[derive(Enum, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum FinishStatus {
    Classified,
    Dnf,
    Dns,
    Dsq,
    Dnq,
}

#[derive(SimpleObject, Deserialize)]
#[graphql(complex)]
pub struct SessionEntry {
    #[serde(rename = "driver")]
    #[graphql(skip)]
    driver_id: ID,
    // TODO: remove this Option
    #[serde(rename = "team")]
    #[graphql(skip)]
    team_id: Option<ID>,
    #[serde(skip)]
    #[graphql(skip)]
    session_id: ID,
    finish_status: FinishStatus,
    grid_position: Option<u32>,
    finish_position: u32,
    laps: Vec<Lap>,
}

#[ComplexObject]
impl SessionEntry {
    async fn driver<'a>(&self, ctx: &Context<'a>) -> Result<&'a Driver, Error> {
        ctx.data_unchecked::<data::Data>()
            .drivers
            .iter()
            .find(|d| d.id == self.driver_id)
            .ok_or(Error::new(format!(
                "Driver `{:?}` not found",
                self.driver_id
            )))
    }

    async fn team<'a>(&self, ctx: &Context<'a>) -> Result<&'a Team, Error> {
        let team_id = if let Some(team_id) = &self.team_id {
            team_id.clone()
        } else {
            ctx.data_unchecked::<data::Data>()
                .league_entries
                .iter()
                .find(|le| le.driver_id == self.driver_id)
                .map(|le| le.team_id.clone())
                .ok_or(Error::new(format!(
                    "League Entry `{:?}` not found",
                    self.driver_id
                )))?
        };

        ctx.data_unchecked::<data::Data>()
            .teams
            .iter()
            .find(|t| t.id == team_id)
            .ok_or(Error::new(format!("Team `{:?}` not found", team_id)))
    }

    // TODO do this good
    async fn points<'a>(&'a self, ctx: &Context<'_>) -> Result<u32, Error> {
        let session = self.session(ctx).await?;

        let points_rule = session
            .event(ctx)
            .await?
            .points_rule(ctx, session.session_type)
            .await?;

        let points = if self.finish_status != FinishStatus::Classified {
            0
        } else {
            let points = if points_rule.points_per_position.len() >= self.finish_position as usize
                && self.finish_position > 0
            {
                points_rule.points_per_position[(self.finish_position - 1) as usize]
            } else {
                0
            };

            let fastest_lap = if session.fastest_lap == self.driver_id {
                points_rule.points_for_fastest_lap
            } else {
                0
            };

            points + fastest_lap
        };

        Ok(points)
    }

    async fn session<'a>(&self, ctx: &Context<'a>) -> Result<&'a Session, Error> {
        ctx.data_unchecked::<data::Data>()
            .sessions
            .iter()
            .find(|session| session.id == self.session_id)
            .ok_or(Error::new(format!(
                "Session `{:?}` not found",
                self.session_id
            )))
    }

    async fn fastest_lap<'a>(&'a self, _ctx: &Context<'_>) -> Option<&'a Lap> {
        self.laps.iter().max_by_key(|lap| lap.laptime_in_ms)
    }

    // async fn laps(&self, _ctx: &Context<'_>) -> Vec<Lap> {
    //     vec![]
    // }

    async fn overtakes(&self, _ctx: &Context<'_>) -> Vec<Overtake> {
        vec![]
    }

    async fn entry<'a>(&self, ctx: &Context<'a>) -> Result<&'a EventEntry, Error> {
        let session = self.session(ctx).await?;

        let event = session.event(ctx).await?;

        event
            .entries
            .iter()
            .find(|entry| entry.driver_id == self.driver_id)
            .ok_or(Error::new("Entry not found"))
    }
}

#[derive(Deserialize)]
pub struct EventEntry {
    #[serde(skip)]
    event_id: ID,
    #[serde(rename = "driver")]
    driver_id: ID,
    #[serde(rename = "team")]
    team_id: ID,
}

#[Object]
impl EventEntry {
    async fn event<'a>(&self, ctx: &Context<'a>) -> Result<&'a Event, Error> {
        ctx.data_unchecked::<data::Data>()
            .events
            .iter()
            .find(|e| e.id == self.event_id)
            .ok_or(Error::new(format!("Event `{:?}` not found", self.event_id)))
    }

    async fn driver<'a>(&self, ctx: &Context<'a>) -> Result<&'a Driver, Error> {
        ctx.data_unchecked::<data::Data>()
            .drivers
            .iter()
            .find(|driver| driver.id == self.driver_id)
            .ok_or(Error::new(format!(
                "Driver `{:?}` not found",
                self.driver_id
            )))
    }

    async fn team<'a>(&self, ctx: &Context<'a>) -> Result<&'a Team, Error> {
        ctx.data_unchecked::<data::Data>()
            .teams
            .iter()
            .find(|team| team.id == self.team_id)
            .ok_or(Error::new("Team not found"))
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum LapType {
    In,
    Out,
    Hot,
    Sc,
    Vsc,
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum TyreType {
    Soft,
    Medium,
    Hard,
    Inter,
    Wet,
}

#[derive(SimpleObject, Deserialize)]
// #[graphql(complex)]
pub struct Lap {
    lap_number: u32,
    laptime_in_ms: u32,
    valid: bool,
    lap_type: LapType,
    tyres: TyreType,
}

#[derive(SimpleObject, Deserialize)]
#[graphql(complex)]
pub struct Overtake {
    id: ID,
    lap: u32,
    #[graphql(skip)]
    overtaking_driver_name: String,
    #[graphql(skip)]
    overtaken_driver_name: String,
}

#[ComplexObject]
impl Overtake {
    async fn overtaking_driver(&self, _ctx: &Context<'_>) -> Driver {
        Driver {
            id: ID::from(Uuid::default()),
            name: self.overtaking_driver_name.clone(),
            nationality: "BE".to_owned(),
        }
    }

    async fn overtaken_driver(&self, _ctx: &Context<'_>) -> Driver {
        Driver {
            id: ID::from(Uuid::default()),
            name: self.overtaken_driver_name.clone(),
            nationality: "BE".to_owned(),
        }
    }
}

#[derive(SimpleObject, Clone, Deserialize)]
pub struct Team {
    id: ID,
    name: String,
}

#[derive(SimpleObject, Clone, Deserialize)]
pub struct Track {
    id: ID,
    name: String,
    country: String, // Alpha 2 Code
}

// TODO Login Details, Account stuff
#[derive(SimpleObject, Clone, Deserialize)]
pub struct Driver {
    id: ID,
    name: String,
    nationality: String, // Alpha 2 Code
}

#[derive(SimpleObject, Clone, Deserialize)]
pub struct Me {
    pub sub: ID,
}

struct Db(Client);

pub fn get_schema(db_client: Client) -> Schema {
    Schema::build(query::Query, mutation::Mutation, EmptySubscription)
        .extension(ApolloTracing)
        .data(data::Data::new())
        .data(Db(db_client))
        .finish()
}
