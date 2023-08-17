use std::collections::HashMap;

use async_graphql::ID;
use chrono::{TimeZone, Utc};

pub(crate) struct Data {
    pub users: HashMap<ID, super::User>,
    pub leagues: HashMap<ID, super::League>,
    // League -> Events
    pub events: HashMap<ID, Vec<super::Event>>,
    // Event -> Entries
    pub entries: HashMap<ID, Vec<super::Entry>>,
    // Event -> Sessions
    pub sessions: HashMap<ID, Vec<super::Session>>,
    pub tracks: HashMap<ID, super::Track>,
}

impl Data {
    pub fn new() -> Self {
        let users: HashMap<_, _> = [
            super::User {
                id: ID::from("nam"),
                name: "Nam".to_owned(),
                nationality: "NL".to_owned(),
            },
            super::User {
                id: ID::from("warre"),
                name: "Warre".to_owned(),
                nationality: "BE".to_owned(),
            },
            super::User {
                id: ID::from("aleks"),
                name: "Aleks".to_owned(),
                nationality: "BG".to_owned(),
            },
            super::User {
                id: ID::from("charles"),
                name: "Charles".to_owned(),
                nationality: "NL".to_owned(),
            },
        ]
        .into_iter()
        .map(|item| (item.id.clone(), item))
        .collect();

        let teams: HashMap<_, _> = [
            super::Team {
                id: ID::from("Mercedes"),
                name: "Mercedes AMG Petronas".to_owned(),
            },
            super::Team {
                id: ID::from("Ferrari"),
                name: "Scuderia Ferrari".to_owned(),
            },
            super::Team {
                id: ID::from("Red Bull Racing"),
                name: "Oracle Red Bull Racing".to_owned(),
            },
        ]
        .into_iter()
        .map(|item| (item.id.clone(), item))
        .collect();

        let leagues: HashMap<_, _> = [
            super::League {
                id: ID::from("league-1"),
                name: "NAMR1 Season 4".to_owned(),
                status: super::Status::Active,
            },
            super::League {
                id: ID::from("league-0"),
                name: "NAMR1 Season 3".to_owned(),
                status: super::Status::Finished,
            },
        ]
        .into_iter()
        .map(|item| (item.id.clone(), item))
        .collect();

        let events: HashMap<_, _> = [(
            ID::from("league-1"),
            vec![
                super::Event {
                    id: ID::from("event-1"),
                    name: "Eifel Grand Prix".to_owned(),
                    championship_order: 1,
                    date: Utc
                        .with_ymd_and_hms(2023, 8, 12, 18, 00, 00)
                        .single()
                        .unwrap(),
                    track_id: ID::from("hockenheim"),
                },
                super::Event {
                    id: ID::from("event-2"),
                    name: "Belgian Grand Prix".to_owned(),
                    championship_order: 2,
                    date: Utc
                        .with_ymd_and_hms(2023, 8, 19, 18, 00, 00)
                        .single()
                        .unwrap(),
                    track_id: ID::from("spa"),
                },
                super::Event {
                    id: ID::from("event-3"),
                    name: "Monaco Grand Prix".to_owned(),
                    championship_order: 3,
                    date: Utc
                        .with_ymd_and_hms(2023, 8, 26, 18, 00, 00)
                        .single()
                        .unwrap(),
                    track_id: ID::from("monaco"),
                },
            ],
        )]
        .into_iter()
        .collect();

        let entries: HashMap<_, _> = [
            (
                ID::from("event-1"),
                vec![
                    super::Entry {
                        team: teams.get(&ID::from("Mercedes")).unwrap().clone(),
                        user: users.get(&ID::from("aleks")).unwrap().clone(),
                    },
                    super::Entry {
                        team: teams.get(&ID::from("Red Bull Racing")).unwrap().clone(),
                        user: users.get(&ID::from("nam")).unwrap().clone(),
                    },
                    super::Entry {
                        team: teams.get(&ID::from("Ferrari")).unwrap().clone(),
                        user: users.get(&ID::from("warre")).unwrap().clone(),
                    },
                    super::Entry {
                        team: teams.get(&ID::from("Red Bull Racing")).unwrap().clone(),
                        user: users.get(&ID::from("charles")).unwrap().clone(),
                    },
                ],
            ),
            (
                ID::from("event-2"),
                vec![
                    super::Entry {
                        team: teams.get(&ID::from("Mercedes")).unwrap().clone(),
                        user: users.get(&ID::from("aleks")).unwrap().clone(),
                    },
                    super::Entry {
                        team: teams.get(&ID::from("Red Bull Racing")).unwrap().clone(),
                        user: users.get(&ID::from("nam")).unwrap().clone(),
                    },
                    super::Entry {
                        team: teams.get(&ID::from("Ferrari")).unwrap().clone(),
                        user: users.get(&ID::from("warre")).unwrap().clone(),
                    },
                    super::Entry {
                        team: teams.get(&ID::from("Red Bull Racing")).unwrap().clone(),
                        user: users.get(&ID::from("charles")).unwrap().clone(),
                    },
                ],
            ),
            (
                ID::from("event-3"),
                vec![
                    super::Entry {
                        team: teams.get(&ID::from("Mercedes")).unwrap().clone(),
                        user: users.get(&ID::from("aleks")).unwrap().clone(),
                    },
                    super::Entry {
                        team: teams.get(&ID::from("Red Bull Racing")).unwrap().clone(),
                        user: users.get(&ID::from("nam")).unwrap().clone(),
                    },
                    super::Entry {
                        team: teams.get(&ID::from("Ferrari")).unwrap().clone(),
                        user: users.get(&ID::from("warre")).unwrap().clone(),
                    },
                    super::Entry {
                        team: teams.get(&ID::from("Red Bull Racing")).unwrap().clone(),
                        user: users.get(&ID::from("charles")).unwrap().clone(),
                    },
                ],
            ),
        ]
        .into_iter()
        .collect();

        let sessions: HashMap<_, _> = [
            (
                ID::from("event-1"),
                vec![
                    super::Session {
                        id: ID::from("session-0"),
                        session_type: super::SessionType::Practice,
                        classification: vec![
                            users.get(&ID::from("charles")).unwrap().clone(),
                            users.get(&ID::from("aleks")).unwrap().clone(),
                            users.get(&ID::from("nam")).unwrap().clone(),
                            users.get(&ID::from("warre")).unwrap().clone(),
                        ],
                    },
                    super::Session {
                        id: ID::from("session-1"),
                        session_type: super::SessionType::Sprint,
                        classification: vec![
                            users.get(&ID::from("aleks")).unwrap().clone(),
                            users.get(&ID::from("warre")).unwrap().clone(),
                            users.get(&ID::from("nam")).unwrap().clone(),
                            users.get(&ID::from("charles")).unwrap().clone(),
                        ],
                    },
                    super::Session {
                        id: ID::from("session-2"),
                        session_type: super::SessionType::Qualifying,
                        classification: vec![
                            users.get(&ID::from("aleks")).unwrap().clone(),
                            users.get(&ID::from("nam")).unwrap().clone(),
                            users.get(&ID::from("charles")).unwrap().clone(),
                            users.get(&ID::from("warre")).unwrap().clone(),
                        ],
                    },
                    super::Session {
                        id: ID::from("session-3"),
                        session_type: super::SessionType::Race,
                        classification: vec![
                            users.get(&ID::from("nam")).unwrap().clone(),
                            users.get(&ID::from("charles")).unwrap().clone(),
                            users.get(&ID::from("aleks")).unwrap().clone(),
                            users.get(&ID::from("warre")).unwrap().clone(),
                        ],
                    },
                ],
            ),
            (
                ID::from("event-2"),
                vec![
                    super::Session {
                        id: ID::from("session-0"),
                        session_type: super::SessionType::Practice,
                        classification: vec![
                            users.get(&ID::from("charles")).unwrap().clone(),
                            users.get(&ID::from("aleks")).unwrap().clone(),
                            users.get(&ID::from("nam")).unwrap().clone(),
                            users.get(&ID::from("warre")).unwrap().clone(),
                        ],
                    },
                    super::Session {
                        id: ID::from("session-1"),
                        session_type: super::SessionType::Sprint,
                        classification: vec![
                            users.get(&ID::from("aleks")).unwrap().clone(),
                            users.get(&ID::from("warre")).unwrap().clone(),
                            users.get(&ID::from("nam")).unwrap().clone(),
                            users.get(&ID::from("charles")).unwrap().clone(),
                        ],
                    },
                    super::Session {
                        id: ID::from("session-2"),
                        session_type: super::SessionType::Qualifying,
                        classification: vec![
                            users.get(&ID::from("aleks")).unwrap().clone(),
                            users.get(&ID::from("nam")).unwrap().clone(),
                            users.get(&ID::from("charles")).unwrap().clone(),
                            users.get(&ID::from("warre")).unwrap().clone(),
                        ],
                    },
                    super::Session {
                        id: ID::from("session-3"),
                        session_type: super::SessionType::Race,
                        classification: vec![
                            users.get(&ID::from("nam")).unwrap().clone(),
                            users.get(&ID::from("charles")).unwrap().clone(),
                            users.get(&ID::from("aleks")).unwrap().clone(),
                            users.get(&ID::from("warre")).unwrap().clone(),
                        ],
                    },
                ],
            ),
            (
                ID::from("event-3"),
                vec![
                    super::Session {
                        id: ID::from("session-0"),
                        session_type: super::SessionType::Practice,
                        classification: vec![
                            users.get(&ID::from("charles")).unwrap().clone(),
                            users.get(&ID::from("aleks")).unwrap().clone(),
                            users.get(&ID::from("nam")).unwrap().clone(),
                            users.get(&ID::from("warre")).unwrap().clone(),
                        ],
                    },
                    super::Session {
                        id: ID::from("session-1"),
                        session_type: super::SessionType::Sprint,
                        classification: vec![
                            users.get(&ID::from("aleks")).unwrap().clone(),
                            users.get(&ID::from("warre")).unwrap().clone(),
                            users.get(&ID::from("nam")).unwrap().clone(),
                            users.get(&ID::from("charles")).unwrap().clone(),
                        ],
                    },
                    super::Session {
                        id: ID::from("session-2"),
                        session_type: super::SessionType::Qualifying,
                        classification: vec![
                            users.get(&ID::from("aleks")).unwrap().clone(),
                            users.get(&ID::from("nam")).unwrap().clone(),
                            users.get(&ID::from("charles")).unwrap().clone(),
                            users.get(&ID::from("warre")).unwrap().clone(),
                        ],
                    },
                    super::Session {
                        id: ID::from("session-3"),
                        session_type: super::SessionType::Race,
                        classification: vec![
                            users.get(&ID::from("nam")).unwrap().clone(),
                            users.get(&ID::from("charles")).unwrap().clone(),
                            users.get(&ID::from("aleks")).unwrap().clone(),
                            users.get(&ID::from("warre")).unwrap().clone(),
                        ],
                    },
                ],
            ),
        ]
        .into_iter()
        .collect();

        let tracks = [super::Track {
            id: ID::from("hockenheim"),
            name: "Hockenheimring".to_owned(),
            country: "DE".to_owned(),
        },
        super::Track {
            id: ID::from("spa"),
            name: "Spa-Francorchamps".to_owned(),
            country: "BE".to_owned(),
        },
        super::Track {
            id: ID::from("monaco"),
            name: "Monaco".to_owned(),
            country: "MC".to_owned(),
        }]
        .into_iter()
        .map(|item| (item.id.clone(), item))
        .collect();

        Self {
            users,
            leagues,
            events,
            entries,
            sessions,
            tracks,
        }
    }
}
