use std::io::Cursor;

use bytes::Bytes;

use crate::packet::header::Header;

use super::packet::{header::PacketId, Packet};

mod car_status;
mod car_telemetry;
mod event;
mod final_classification;
mod header;
mod lap_data;
mod lobby_info;
mod motion;
mod participants;
mod session;
mod session_history;

pub fn decode_twentythree(cursor: &mut Cursor<Bytes>) -> crate::Result<Packet> {
    cursor.set_position(6);
    let packet_id = header::parse_packet_id(cursor)?;
    cursor.set_position(0);

    match packet_id {
        PacketId::Motion => Ok(Packet::Motion(motion::parse_motion_packet(cursor)?)),
        PacketId::Session => Ok(Packet::Session(session::parse_session_packet(cursor)?)),
        PacketId::LapData => Ok(Packet::LapData(lap_data::parse_lap_data_packet(cursor)?)),
        PacketId::Event => Ok(Packet::Event(event::parse_event_packet(cursor)?)),
        PacketId::Participants => Ok(Packet::Participants(
            participants::parse_participants_packet(cursor)?,
        )),
        PacketId::CarSetups => Ok(Packet::CarSetups(header::parse_header(cursor)?)),
        PacketId::CarTelemetry => Ok(Packet::CarTelemetry(car_telemetry::parse_car_telemetry(
            cursor,
        )?)),
        PacketId::CarStatus => Ok(Packet::CarStatus(car_status::parse_car_status(cursor)?)),
        PacketId::FinalClassification => Ok(Packet::FinalClassification(
            final_classification::parse_final_classification_packet(cursor)?,
        )),
        PacketId::LobbyInfo => Ok(Packet::LobbyInfo(lobby_info::parse_lobby_info(cursor)?)),
        PacketId::CarDamage => Ok(Packet::CarDamage(header::parse_header(cursor)?)),
        PacketId::SessionHistory => Ok(Packet::SessionHistory(
            session_history::parse_session_history_packet(cursor)?,
        )),
        PacketId::MotionEx => Ok(Packet::MotionEx(header::parse_header(cursor)?)),
        PacketId::TyreSets => Ok(Packet::TyreSets(header::parse_header(cursor)?)),
    }
}

pub fn decode_twentythree_header(cursor: &mut Cursor<Bytes>) -> crate::Result<Header> {
    header::parse_header(cursor)
}
