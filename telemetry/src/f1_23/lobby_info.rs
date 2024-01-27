use std::io::{Cursor, Read};

use bytes::{Buf, Bytes};

use crate::{
    packet::{lobby_info::{LobbyInfoData, LobbyInfoPacket, Status}, participants::Platform},
    f1_23::participants::parse_team,
    Result,
};

use super::header::parse_header;

pub fn parse_lobby_info(cursor: &mut Cursor<Bytes>) -> Result<LobbyInfoPacket> {
    if cursor.remaining() != 1218 {
        return Err(crate::TelemetryError::InvalidPacket(
            "invalid lobby info packet length".to_owned(),
        ));
    };

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
    let team = parse_team(cursor.get_u8());
    let nationality = cursor.get_u8();
    let platform  = match cursor.get_u8() {
        1 => Some(Platform::Steam),
        3 => Some(Platform::PlayStation),
        4 => Some(Platform::Xbox),
        6 => Some(Platform::Origin),
        _ => None,
    };
    let mut name = [0_u8; 48];
    let _ = cursor.read_exact(&mut name); // Shouldn't error if the packet is not malformed
    let name_end = name.iter().position(|&c| c == 0).unwrap_or(48);
    let name = String::from_utf8_lossy(&name[0..name_end]).to_string().replace('\u{00a0}', " ");
    let car_number = cursor.get_u8();
    let ready_status = match cursor.get_u8() {
        0 => Status::NotReady,
        1 => Status::Ready,
        2 => Status::Spectating,
        _ => return Err(crate::TelemetryError::InvalidPacket("invalid ready_status".to_owned())),
    };

    Ok(LobbyInfoData{
        ai_controlled,
        team,
        nationality,
        platform,
        name,
        car_number,
        ready_status,
    })
}
