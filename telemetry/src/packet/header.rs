use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "F1 2023")]
    TwentyThree,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GameVersion(pub u8, pub u8);

#[derive(Copy, Clone, Debug, Display, Serialize, Deserialize, PartialEq, Eq)]
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
    TyreSets,
    MotionEx,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    pub format: Format,
    pub game_year: u8,
    pub game_version: GameVersion,
    pub packet_version: u8,
    pub packet_id: PacketId,
    pub session_uid: u64,
    pub session_time: f32,
    /// Identifier for what frame the data was retrieved on
    pub frame_identifier: u32,
    /// Overal identifier for the frame the data was retrieved on, doesn't go back after flashbacks
    pub overall_frame_identifier: u32,
    pub player_car_index: u8,
    pub secondary_player_car_index: Option<u8>,
}
