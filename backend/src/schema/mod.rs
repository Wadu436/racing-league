use async_graphql::{
    scalar, ComplexObject, Context, EmptyMutation, EmptySubscription, Enum, Object, SimpleObject,
};
use async_graphql::{Error, ID};

use derive_more::{Add, Constructor, From, Into};
use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DurationMilliSeconds;
use uuid::Uuid;

pub mod data;

pub type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

// TODO: create country new-type

#[serde_as]
#[derive(
    serde::Serialize, Deserialize, Add, PartialEq, Eq, PartialOrd, Ord, From, Into, Constructor,
)]
struct Laptime(#[serde_as(as = "DurationMilliSeconds<i64>")] chrono::Duration);

impl From<Laptime> for i64 {
    fn from(value: Laptime) -> Self {
        value.0.num_milliseconds()
    }
}

impl From<i64> for Laptime {
    fn from(value: i64) -> Self {
        Laptime(chrono::Duration::milliseconds(value))
    }
}

scalar!(Laptime);

// TODO: add an alternative here too for leagues that havent started yet
#[derive(Enum, Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum Status {
    Active,
    Finished,
}

#[derive(SimpleObject, Deserialize)]
#[graphql(complex)]
pub struct League {
    id: ID,
    name: String,
    status: Status,
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
}

#[derive(SimpleObject, Deserialize)]
#[graphql(complex)]
pub struct Event {
    id: ID,
    name: String,
    championship_order: u32,
    date: chrono::DateTime<chrono::Utc>,
    track_id: ID,
    league_id: ID,
}

#[ComplexObject]
impl Event {
    async fn entries<'a>(&self, ctx: &Context<'a>) -> Vec<&'a Entry> {
        let data = ctx.data_unchecked::<data::Data>();

        data.entries
            .iter()
            .filter(|entry| entry.event_id == self.id)
            .collect()
    }

    async fn sessions<'a>(&self, ctx: &Context<'a>) -> Vec<&'a Session> {
        let data = ctx.data_unchecked::<data::Data>();

        data.sessions
            .iter()
            .filter(|item| self.id == item.event_id)
            .collect()
    }

    async fn track<'a>(&self, ctx: &Context<'a>) -> &'a Track {
        let data = ctx.data_unchecked::<data::Data>();

        data.tracks
            .iter()
            .find(|track| track.id == self.track_id)
            .unwrap()
    }

    async fn points_rule(&self, _ctx: &Context<'_>) -> PointsRule {
        PointsRule {
            points_per_position: vec![25, 18, 15, 12, 10, 8, 6, 4, 2, 1],
            points_for_pole: 0,
            points_for_fastest_lap: 1,
        }
    }
}

#[derive(SimpleObject, Deserialize)]
pub struct PointsRule {
    points_per_position: Vec<u32>,
    points_for_pole: u32,
    points_for_fastest_lap: u32,
}

#[derive(Deserialize)]
pub struct Entry {
    #[serde(rename = "event")]
    event_id: ID,
    #[serde(rename = "user")]
    user_id: ID,
    #[serde(rename = "team")]
    team_id: ID,
}

#[Object]
impl Entry {
    async fn user<'a>(&self, ctx: &Context<'a>) -> Result<&'a User, Error> {
        ctx.data_unchecked::<data::Data>()
            .users
            .iter()
            .find(|user| user.id == self.user_id)
            .ok_or(Error::new("User not found"))
    }

    async fn team<'a>(&self, ctx: &Context<'a>) -> Result<&'a Team, Error> {
        ctx.data_unchecked::<data::Data>()
            .teams
            .iter()
            .find(|team| team.id == self.team_id)
            .ok_or(Error::new("Team not found"))
    }
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum SessionType {
    Race,
    Sprint,
    Qualifying, // TODO! Figure out whether full Qualifying should be one session or three session Q1, Q2, Q3?
    SprintQualifying,
    Practice,
}

#[derive(SimpleObject, Deserialize)]
#[graphql(complex)]
pub struct Session {
    id: ID,
    event_id: ID,
    session_type: SessionType,
    classification: Vec<ClassificationEntry>,
}

#[derive(Deserialize)]
pub struct ClassificationEntry {
    user_id: ID,
    team_id: ID,
}

#[Object]
impl ClassificationEntry {
    async fn user<'a>(&self, ctx: &Context<'a>) -> Result<&'a User, Error> {
        ctx.data_unchecked::<data::Data>()
            .users
            .iter()
            .find(|user| user.id == self.user_id)
            .ok_or(Error::new("User not found"))
    }

    async fn team<'a>(&self, ctx: &Context<'a>) -> Result<&'a Team, Error> {
        ctx.data_unchecked::<data::Data>()
            .teams
            .iter()
            .find(|team| team.id == self.team_id)
            .ok_or(Error::new("Team not found"))
    }

    // TODO do this good
    async fn points(&self, _ctx: &Context<'_>) -> u32 {
        0
    }

    async fn fastest_lap(&self, _ctx: &Context<'_>) -> Option<Laptime> {
        None
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum LapType {
    In,
    Out,
    Hot,
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum Tyre {
    Soft,
    Medium,
    Hard,
    Inter,
    Wet,
}

// TODO: move data to data.rs
#[derive(SimpleObject, Deserialize)]
#[graphql(complex)]
pub struct Lap {
    id: ID,
    lap_number: u32,
    laptime_in_ms: Laptime,
    valid: bool,
    lap_type: LapType,
    tyres: Tyre,
    #[graphql(skip)]
    username: String,
}

// TODO: move data to data.rs
#[ComplexObject]
impl Lap {
    async fn driver(&self, _ctx: &Context<'_>) -> User {
        User {
            id: ID::from(Uuid::default()),
            name: self.username.clone(),
            nationality: "BE".to_owned(),
        }
    }
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

// TODO: move data to data.rs
#[ComplexObject]
impl Overtake {
    async fn overtaking_driver(&self, _ctx: &Context<'_>) -> User {
        User {
            id: ID::from(Uuid::default()),
            name: self.overtaking_driver_name.clone(),
            nationality: "BE".to_owned(),
        }
    }

    async fn overtaken_driver(&self, _ctx: &Context<'_>) -> User {
        User {
            id: ID::from(Uuid::default()),
            name: self.overtaken_driver_name.clone(),
            nationality: "BE".to_owned(),
        }
    }
}

// TODO: move data to data.rs
#[ComplexObject]
impl Session {
    async fn laps(&self, _ctx: &Context<'_>) -> Vec<Lap> {
        vec![
            Lap {
                id: ID::from(Uuid::default()),
                lap_number: 1,
                laptime_in_ms: (((60 + 14) * 1000) + 356).into(),
                username: "Nam".to_owned(),
                lap_type: LapType::Out,
                valid: true,
                tyres: Tyre::Soft,
            },
            Lap {
                id: ID::from(Uuid::default()),
                lap_number: 2,
                laptime_in_ms: (((60 + 14) * 1000) + 356).into(),
                username: "Nam".to_owned(),
                lap_type: LapType::Hot,
                valid: true,
                tyres: Tyre::Soft,
            },
            Lap {
                id: ID::from(Uuid::default()),
                lap_number: 3,
                laptime_in_ms: (((60 + 14) * 1000) + 356).into(),
                username: "Nam".to_owned(),
                lap_type: LapType::In,
                valid: true,
                tyres: Tyre::Soft,
            },
            Lap {
                id: ID::from(Uuid::default()),
                lap_number: 4,
                laptime_in_ms: (((60 + 14) * 1000) + 356).into(),
                username: "Nam".to_owned(),
                lap_type: LapType::Out,
                valid: true,
                tyres: Tyre::Medium,
            },
            Lap {
                id: ID::from(Uuid::default()),
                lap_number: 1,
                laptime_in_ms: (((60 + 14) * 1000) + 356).into(),
                username: "Warre".to_owned(),
                lap_type: LapType::Out,
                valid: true,
                tyres: Tyre::Hard,
            },
            Lap {
                id: ID::from(Uuid::default()),
                lap_number: 2,
                laptime_in_ms: (((60 + 14) * 1000) + 356).into(),
                username: "Warre".to_owned(),
                lap_type: LapType::In,
                valid: true,
                tyres: Tyre::Hard,
            },
            Lap {
                id: ID::from(Uuid::default()),
                lap_number: 3,
                laptime_in_ms: (((60 + 14) * 1000) + 356).into(),
                username: "Warre".to_owned(),
                lap_type: LapType::Out,
                valid: true,
                tyres: Tyre::Soft,
            },
            Lap {
                id: ID::from(Uuid::default()),
                lap_number: 4,
                laptime_in_ms: (((60 + 14) * 1000) + 356).into(),
                username: "Warre".to_owned(),
                lap_type: LapType::Hot,
                valid: true,
                tyres: Tyre::Soft,
            },
        ]
    }

    async fn overtakes(&self, _ctx: &Context<'_>) -> Option<Vec<Overtake>> {
        Some(vec![
            Overtake {
                id: ID::from(Uuid::default()),
                lap: 1,
                overtaken_driver_name: "Warre".to_owned(),
                overtaking_driver_name: "Nam".to_owned(),
            },
            Overtake {
                id: ID::from(Uuid::default()),
                lap: 3,
                overtaken_driver_name: "Nam".to_owned(),
                overtaking_driver_name: "Warre".to_owned(),
            },
        ])
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
pub struct User {
    id: ID,
    name: String,
    nationality: String, // Alpha 2 Code
}

#[Object]
impl Query {
    async fn user<'a>(&self, ctx: &Context<'a>, id: ID) -> Result<&'a User, Error> {
        ctx.data_unchecked::<data::Data>()
            .users
            .iter()
            .find(|user| user.id == id)
            .ok_or(Error::new("User not found"))
    }

    async fn leagues<'a>(&self, ctx: &Context<'a>) -> &'a Vec<League> {
        &ctx.data_unchecked::<data::Data>().leagues
    }

    async fn league<'a>(&self, ctx: &Context<'a>, id: ID) -> Result<&'a League, Error> {
        ctx.data_unchecked::<data::Data>()
            .leagues
            .iter()
            .find(|league| league.id == id)
            .ok_or(Error::new("League not found"))
    }
}

pub fn get_schema() -> Schema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(data::Data::new())
        .finish()
}
