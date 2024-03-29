use serde::{Deserialize, Serialize};

pub mod car_status;
pub mod car_telemetry;
pub mod event;
pub mod final_classification;
pub mod header;
pub mod lap_data;
pub mod motion;
pub mod participants;
pub mod lobby_info;
pub mod session;
pub mod session_history;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Packet {
    Motion(motion::MotionPacket),
    Session(session::SessionPacket),
    LapData(lap_data::LapDataPacket),
    Event(event::EventPacket),
    Participants(participants::ParticipantsPacket),
    CarSetups(header::Header),
    CarTelemetry(car_telemetry::CarTelemetryPacket),
    CarStatus(car_status::CarStatusPacket),
    FinalClassification(final_classification::FinalClassificationPacket),
    LobbyInfo(lobby_info::LobbyInfoPacket),
    CarDamage(header::Header),
    SessionHistory(session_history::SessionHistoryPacket),
    TyreSets(header::Header),
    MotionEx(header::Header)
}

impl Packet {
    pub fn header(&self) -> header::Header {
        match self {
            Packet::Motion(p) => p.header,
            Packet::Session(p) => p.header,
            Packet::LapData(p) => p.header,
            Packet::Event(p) => p.header,
            Packet::Participants(p) => p.header,
            Packet::CarSetups(p) => *p,
            Packet::CarTelemetry(p) => p.header,
            Packet::CarStatus(p) => p.header,
            Packet::FinalClassification(p) => p.header,
            Packet::LobbyInfo(p) => p.header,
            Packet::CarDamage(p) => *p,
            Packet::SessionHistory(p) => p.header,
            Packet::TyreSets(p) => *p,
            Packet::MotionEx(p) => *p
        }
    }
}
