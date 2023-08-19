use super::{Entry, Event, League, Session, Team, Track, User};

macro_rules! read_data {
    ($file:expr, $type:ty) => {
        serde_json::from_str::<Vec<$type>>(
            &std::fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/schema/data/",
                $file
            ))
            .expect(concat!("Failed to read data file '", $file, "'")),
        )
        .expect(concat!("Failed to parse data file '", $file, "'"))
    };
}

pub(crate) struct Data {
    pub users: Vec<User>,
    pub teams: Vec<Team>,
    pub leagues: Vec<super::League>,
    pub events: Vec<super::Event>,
    pub entries: Vec<super::Entry>,
    pub sessions: Vec<super::Session>,
    pub tracks: Vec<super::Track>,
}

impl Data {
    pub fn new() -> Self {
        let users = read_data!("users.json", User);
        let teams = read_data!("teams.json", Team);
        let leagues = read_data!("leagues.json", League);
        let events = read_data!("events.json", Event);
        let entries = read_data!("entries.json", Entry);
        let sessions = read_data!("sessions.json", Session);
        let tracks = read_data!("tracks.json", Track);

        Self {
            users,
            leagues,
            events,
            teams,
            entries,
            sessions,
            tracks,
        }
    }
}
