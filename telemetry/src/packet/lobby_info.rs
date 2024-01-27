use serde::{Deserialize, Serialize};

use super::{header::Header, participants::{Platform, Team}};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Status {
    NotReady,
    Ready,
    Spectating,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LobbyInfoData {
    pub ai_controlled: bool,
    pub team: Team,
    pub nationality: u8,
    pub platform: Option<Platform>,
    pub name: String,
    pub car_number: u8,
    pub ready_status: Status,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LobbyInfoPacket {
    pub header: Header,

    pub lobby_players: Vec<LobbyInfoData>,
}
