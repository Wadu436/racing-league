use bytes::{Buf, Bytes};
use std::io::Cursor;

use crate::packet::header::{Format, GameVersion, Header, PacketId};

pub fn parse_header(cursor: &mut Cursor<Bytes>) -> crate::Result<Header> {
    cursor.get_u16();
    let game_version = GameVersion(cursor.get_u8(), cursor.get_u8());
    let packet_version = cursor.get_u8();
    let packet_id = parse_packet_id(cursor)?;
    let session_uid = cursor.get_u64_le();
    let session_time = cursor.get_f32_le();
    let frame_identifier = cursor.get_u32_le();
    let player_car_index = cursor.get_u8();
    let secondary_player_car_index = cursor.get_u8();
    let secondary_player_car_index = if secondary_player_car_index != 255 {
        Some(secondary_player_car_index)
    } else {
        None
    };
    let header = Header {
        format: Format::TwentyOne,
        game_version: game_version,
        packet_version,
        packet_id,
        session_uid,
        session_time,
        frame_identifier,
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
        _ => Err(crate::TelemetryError::InvalidPacket),
    }
}
