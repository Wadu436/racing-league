use serde::{Serialize, Deserialize};

pub mod car_status;
pub mod car_telemetry;
pub mod event;
pub mod header;
pub mod lap_data;
pub mod motion;
pub mod participants;
pub mod session;

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
    FinalClassification(header::Header),
    LobbyInfo(header::Header),
    CarDamage(header::Header),
    SessionHistory(header::Header),
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
            Packet::FinalClassification(p) => *p,
            Packet::LobbyInfo(p) => *p,
            Packet::CarDamage(p) => *p,
            Packet::SessionHistory(p) => *p,
        }
    }
}
