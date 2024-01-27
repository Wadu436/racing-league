use bytes::{Buf, Bytes};
use std::io::Cursor;

use crate::packet::header::{Format, GameVersion, Header, PacketId};

const HEADER_SIZE: usize = 29;

pub fn parse_header(cursor: &mut Cursor<Bytes>) -> crate::Result<Header> {
    if cursor.remaining() < HEADER_SIZE {
        return Err(crate::TelemetryError::InvalidPacket("packet too small".to_owned()));
    }

    let packet_format = cursor.get_u16_le();
    if packet_format != 2023 {
        return Err(crate::TelemetryError::InvalidPacket("expected packet format to be 2023".to_owned()));
    }
    let game_year = cursor.get_u8();
    let game_version = GameVersion(cursor.get_u8(), cursor.get_u8());
    let packet_version = cursor.get_u8();
    let packet_id = parse_packet_id(cursor)?;
    let session_uid = cursor.get_u64_le();
    let session_time = cursor.get_f32_le();
    let frame_identifier = cursor.get_u32_le();
    let overall_frame_identifier = cursor.get_u32_le();
    let player_car_index = cursor.get_u8();
    let secondary_player_car_index = match cursor.get_u8() {
        255 => None,
        x => Some(x),
    };

    let header = Header {
        format: Format::TwentyThree,
        game_year,
        game_version,
        packet_version,
        packet_id,
        session_uid,
        session_time,
        frame_identifier,
        overall_frame_identifier,
        player_car_index,
        secondary_player_car_index,
    };

    Ok(header)
}

pub fn parse_packet_id(cursor: &mut Cursor<Bytes>) -> crate::Result<PacketId> {
    match cursor.get_u8() {
        0 => Ok(PacketId::Motion),
        1 => Ok(PacketId::Session),
        2 => Ok(PacketId::LapData),
        3 => Ok(PacketId::Event),
        4 => Ok(PacketId::Participants),
        5 => Ok(PacketId::CarSetups),
        6 => Ok(PacketId::CarTelemetry),
        7 => Ok(PacketId::CarStatus),
        8 => Ok(PacketId::FinalClassification),
        9 => Ok(PacketId::LobbyInfo),
        10 => Ok(PacketId::CarDamage),
        11 => Ok(PacketId::SessionHistory),
        12 => Ok(PacketId::TyreSets),
        13 => Ok(PacketId::MotionEx),
        i => Err(crate::TelemetryError::InvalidPacket(format!("invalid packet id: {}", i))),
    }
}
