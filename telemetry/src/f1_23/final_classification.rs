use std::{io::Cursor, time::Duration};

use bytes::{Buf, Bytes};

use crate::{
    packet::final_classification::{FinalClassificationData, FinalClassificationPacket, TyreStint},
    Result,
};


use super::{header::parse_header, lap_data::parse_result_data, car_status::{parse_tyre_compound_actual, parse_tyre_compound_visual}};

pub fn parse_final_classification_packet(
    cursor: &mut Cursor<Bytes>,
) -> Result<FinalClassificationPacket> {
    if cursor.remaining() != 1020 {
        return Err(crate::TelemetryError::InvalidPacket(
            "invalid final classification packet length".to_owned(),
        ));
    }

    let header = parse_header(cursor)?;
    let _num_cars = cursor.get_u8();
    let classification_data = (0..22)
        .map(|_| {
            let classification_data = parse_final_classification_data(cursor);
            classification_data.map(|cd| {
                if cd.status.valid_result() {
                    Some(cd)
                } else {
                    None
                }
            })
        })
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
    let best_laptime = Duration::from_millis(cursor.get_u32_le() as _);
    let total_race_time_without_penalties = Duration::from_secs_f64(cursor.get_f64_le());
    let penalty_time_in_seconds = cursor.get_u8();
    let num_penalties = cursor.get_u8();
    let num_tyre_stints = cursor.get_u8();
    let tyre_data = cursor.copy_to_bytes(24);
    let tyre_stints = (0..num_tyre_stints as usize)
        .map(|i| {
            let actual_tyre = parse_tyre_compound_actual(tyre_data[i]);
            let visual_tyre = parse_tyre_compound_visual(tyre_data[8 + i]);
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
        best_laptime,
        total_race_time_without_penalties,
        penalty_time_in_seconds,
        num_penalties,
        tyre_stints,
    })
}
