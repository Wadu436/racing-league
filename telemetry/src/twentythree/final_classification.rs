use std::io::Cursor;

use bytes::{Buf, Bytes};

use crate::{
    packet::final_classification::{FinalClassificationData, FinalClassificationPacket, TyreStint},
    Result,
};


use super::{header::parse_header, lap_data::parse_result_data, car_status::parse_tyre_compound};

pub fn parse_final_classification_packet(
    cursor: &mut Cursor<Bytes>,
) -> Result<FinalClassificationPacket> {
    let header = parse_header(cursor)?;
    let num_cars = cursor.get_u8();
    let classification_data = (0..num_cars)
        .map(|_| parse_final_classification_data(cursor))
        .collect::<Result<Vec<_>>>()?;

    Ok(FinalClassificationPacket {
        header,
        classification_data,
    })
}

fn parse_final_classification_data(cursor: &mut Cursor<Bytes>) -> Result<FinalClassificationData> {
    let position = cursor.get_u8();
    let num_laps = cursor.get_u8();
    let grid_position = cursor.get_u8();
    let points = cursor.get_u8();
    let num_pit_stops = cursor.get_u8();
    let status = parse_result_data(cursor);
    let best_laptime_in_ms = cursor.get_u32_le();
    let total_race_time_without_penalties_in_seconds = cursor.get_f64_le();
    let penalty_time_in_seconds = cursor.get_u8();
    let num_penalties = cursor.get_u8();
    let num_tyre_stints = cursor.get_u8();
    let tyre_data = cursor.copy_to_bytes(24);
    let tyre_stints = (0..num_tyre_stints as usize)
        .map(|i| {
            let actual_tyre = parse_tyre_compound(tyre_data[i]);
            let visual_tyre = parse_tyre_compound(tyre_data[8 + i]);
            let end_lap = tyre_data[16 + i];
            TyreStint {
                actual_tyre,
                visual_tyre,
                end_lap,
            }
        })
        .collect::<Vec<_>>();

    Ok(FinalClassificationData {
        position,
        num_laps,
        grid_position,
        points,
        num_pit_stops,
        status,
        best_laptime_in_ms,
        total_race_time_without_penalties_in_seconds,
        penalty_time_in_seconds,
        num_penalties,
        tyre_stints,
    })
}
