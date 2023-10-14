use super::{Driver, Event, League, LeagueEntry, Session, Team, Track};

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
    pub drivers: Vec<Driver>,
    pub teams: Vec<Team>,
    pub leagues: Vec<super::League>,
    pub events: Vec<super::Event>,
    pub sessions: Vec<super::Session>,
    pub tracks: Vec<super::Track>,
    pub league_entries: Vec<LeagueEntry>,
}

impl Data {
    pub fn new() -> Self {
        let users = read_data!("users.json", Driver);
        let teams = read_data!("teams.json", Team);
        let leagues = read_data!("leagues.json", League);
        let events = read_data!("events.json", Event);
        let mut sessions = read_data!("sessions.json", Session);
        let tracks = read_data!("tracks.json", Track);
        let league_entries = read_data!("league_entries.json", LeagueEntry);

        for session in sessions.iter_mut() {
            session.entries.iter_mut().for_each(|e| {
                e.session_id = session.id.clone();
            })
        }

        Self {
            drivers: users,
            leagues,
            events,
            teams,
            sessions,
            tracks,
            league_entries,
        }
    }
}
