use super::{Event, League, Session, Team, Track, User};

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
    pub sessions: Vec<super::Session>,
    pub tracks: Vec<super::Track>,
}

impl Data {
    pub fn new() -> Self {
        let users = read_data!("users.json", User);
        let teams = read_data!("teams.json", Team);
        let leagues = read_data!("leagues.json", League);
        let events = read_data!("events.json", Event);
        let sessions = read_data!("sessions.json", Session);
        let tracks = read_data!("tracks.json", Track);

        // for session in sessions.iter_mut() {
        //     session.participants.iter_mut().for_each(|participant| {
        //         participant.session_id = session.id.clone();
        //     })
        // }

        Self {
            users,
            leagues,
            events,
            teams,
            sessions,
            tracks,
        }
    }
}
