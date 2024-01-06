use std::io::Cursor;

use bytes::{Buf, Bytes};

use crate::{
    packet::lobby_info::{LobbyInfoData, LobbyInfoPacket, Status},
    twentytwo::participants::parse_team,
    Result,
};

use super::header::parse_header;

pub fn parse_lobby_info(cursor: &mut Cursor<Bytes>) -> Result<LobbyInfoPacket> {
    let header = parse_header(cursor)?;
    let num_players = cursor.get_u8();
    let lobby_players = (0..num_players)
        .map(|_| parse_lobby_info_data(cursor))
        .collect::<Result<Vec<_>>>()?;

    Ok(LobbyInfoPacket {
        header,
        lobby_players,
    })
}

fn parse_lobby_info_data(cursor: &mut Cursor<Bytes>) -> Result<LobbyInfoData> {
    let ai_controlled: bool = cursor.get_u8() != 0;
    let team = parse_team(cursor);
    let nationality = cursor.get_u8();
    let name = std::ffi::CStr::from_bytes_until_nul(&cursor.copy_to_bytes(48)[..])
        .map_err(|_| crate::TelemetryError::InvalidPacket)?
        .to_str()
        .map_err(|_| crate::TelemetryError::InvalidPacket)?
        .to_owned();
    let car_number = cursor.get_u8();
    let ready_status = match cursor.get_u8() {
        0 => Status::NotReady,
        1 => Status::Ready,
        2 => Status::Spectating,
        _ => return Err(crate::TelemetryError::InvalidPacket),
    };

    Ok(LobbyInfoData {
        ai_controlled,
        team,
        nationality,
        name,
        car_number,
        ready_status,
    })
}
