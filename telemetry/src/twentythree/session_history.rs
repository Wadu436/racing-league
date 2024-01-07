use std::io::Cursor;

use bytes::{Buf, Bytes};

use crate::packet::session_history::{LapHistoryData, SessionHistoryPacket, TyreStintHistoryData};

use super::{car_status::parse_tyre_compound, header::parse_header};

pub fn parse_session_history_packet(
    cursor: &mut Cursor<Bytes>,
) -> crate::Result<SessionHistoryPacket> {
    if cursor.remaining() < 1155 {
        return Err(crate::TelemetryError::InvalidPacket);
    }

    let header = parse_header(cursor)?;

    let car_index = cursor.get_u8();
    let num_laps = cursor.get_u8();
    let num_tyre_stints = cursor.get_u8();

    let best_lap_time_lap_num = cursor.get_u8();
    let best_sector_1_lap_num = cursor.get_u8();
    let best_sector_2_lap_num = cursor.get_u8();
    let best_sector_3_lap_num = cursor.get_u8();

    let mut lap_history_data = (0..100)
        .map(|_| parse_lap_history_data(cursor))
        .collect::<Vec<_>>();
    lap_history_data.truncate(num_laps as usize);

    let mut tyre_stint_history_data = (0..8)
        .map(|_| parse_tyre_stint_history_data(cursor))
        .collect::<Vec<_>>();
    tyre_stint_history_data.truncate(num_tyre_stints as usize);

    Ok(SessionHistoryPacket {
        header,
        car_index,
        best_lap_time_lap_num,
        best_sector_1_lap_num,
        best_sector_2_lap_num,
        best_sector_3_lap_num,
        lap_history_data,
        tyre_stint_history_data,
    })
}

fn parse_lap_history_data(cursor: &mut Cursor<Bytes>) -> LapHistoryData {
    let lap_time_in_ms = cursor.get_u32_le();
    let sector1_time_in_ms = cursor.get_u16_le();
    let sector2_time_in_ms = cursor.get_u16_le();
    let sector3_time_in_ms = cursor.get_u16_le();
    let lap_valid_bit_flags = cursor.get_u8();
    let lap_valid = (lap_valid_bit_flags & 0x01) != 0;
    let sector_1_valid = (lap_valid_bit_flags & 0x02) != 0;
    let sector_2_valid = (lap_valid_bit_flags & 0x04) != 0;
    let sector_3_valid = (lap_valid_bit_flags & 0x08) != 0;

    LapHistoryData {
        lap_time_in_ms,
        sector1_time_in_ms,
        sector2_time_in_ms,
        sector3_time_in_ms,
        lap_valid,
        sector_1_valid,
        sector_2_valid,
        sector_3_valid,
    }
}

fn parse_tyre_stint_history_data(cursor: &mut Cursor<Bytes>) -> TyreStintHistoryData {
    let end_lap = cursor.get_u8();
    let actual_tyre = parse_tyre_compound(cursor.get_u8());
    let visual_tyre = parse_tyre_compound(cursor.get_u8());

    TyreStintHistoryData {
        end_lap,
        actual_tyre,
        visual_tyre,
    }
}
