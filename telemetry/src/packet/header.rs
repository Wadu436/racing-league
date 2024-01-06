use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "F1 2021")]
    TwentyOne,
    #[serde(rename = "F1 2022")]
    TwentyTwo,
    #[serde(rename = "F1 2023")]
    TwentyThree,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GameVersion(pub u8, pub u8);

#[derive(Copy, Clone, Debug, Display, Serialize, Deserialize)]
pub enum PacketId {
    Motion,
    Session,
    LapData,
    Event,
    Participants,
    CarSetups,
    CarTelemetry,
    CarStatus,
    FinalClassification,
    LobbyInfo,
    CarDamage,
    SessionHistory,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    pub format: Format,
    pub game_version: GameVersion,
    pub packet_version: u8,
    pub packet_id: PacketId,
    pub session_uid: u64,
    pub session_time: f32,
    pub frame_identifier: u32,
    pub player_car_index: u8,
    pub secondary_player_car_index: Option<u8>,
}
