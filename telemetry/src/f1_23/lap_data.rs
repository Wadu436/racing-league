use bytes::{Buf, Bytes};
use std::{io::Cursor, time::Duration};

use crate::packet::lap_data::{
    DriverStatus, LapData, LapDataPacket, PitStatus, ResultStatus, Sector,
};

use super::header::parse_header;

pub fn parse_lap_data_packet(cursor: &mut Cursor<Bytes>) -> crate::Result<LapDataPacket> {
    let header = parse_header(cursor)?;
    let lap_data: Vec<_> = (0..22).map(|_| parse_lap_data(cursor)).collect();

    let time_trial_pb_car_idx = Some(cursor.get_u8());
    let time_trial_rival_car_idx = Some(cursor.get_u8());

    Ok(LapDataPacket {
        header,
        lap_data,
        time_trial_pb_car_idx,
        time_trial_rival_car_idx,
    })
}

pub fn parse_lap_data(cursor: &mut Cursor<Bytes>) -> Option<LapData> {
    let last_lap_time_in_ms = cursor.get_u32_le();
    let last_lap_time = Duration::from_millis(last_lap_time_in_ms.into());
    let current_lap_time_in_ms = cursor.get_u32_le();
    let current_lap_time = Duration::from_millis(current_lap_time_in_ms.into());
    let sector1_time_in_ms = cursor.get_u16_le();
    let sector1_time_in_minutes = cursor.get_u8();
    let sector_1_time =
        Duration::from_millis(sector1_time_in_minutes as u64 * 60000 + sector1_time_in_ms as u64);
    let sector2_time_in_ms = cursor.get_u16_le();
    let sector2_time_in_minutes = cursor.get_u8();
    let sector_2_time =
        Duration::from_millis(sector2_time_in_minutes as u64 * 60000 + sector2_time_in_ms as u64);
    let delta_to_car_in_front_in_ms = cursor.get_u16_le();
    let delta_to_car_in_front = Duration::from_millis(delta_to_car_in_front_in_ms.into());
    let delta_to_race_leader_in_ms = cursor.get_u16_le();
    let delta_to_race_leader = Duration::from_millis(delta_to_race_leader_in_ms.into());
    let lap_distance = cursor.get_f32_le();
    let total_distance = cursor.get_f32_le();
    let safety_car_delta_in_seconds = cursor.get_f32_le();
    let safety_car_delta = safety_car_delta_in_seconds;
    let car_position = cursor.get_u8();
    let current_lap_num = cursor.get_u8();
    let pit_status = match cursor.get_u8() {
        1 => PitStatus::Pitting,
        2 => PitStatus::InPitArea,
        _ => PitStatus::None,
    };
    let num_pit_stops = cursor.get_u8();
    let sector = match cursor.get_u8() {
        0 => Sector::Sector1,
        1 => Sector::Sector2,
        2 => Sector::Sector3,
        _ => Sector::Sector1,
    };
    let current_lap_invalid = cursor.get_u8() != 0;
    let penalties = cursor.get_u8();
    let warnings = cursor.get_u8();
    let corner_cutting_warnings = cursor.get_u8();
    let num_unserved_drive_through_pens = cursor.get_u8();
    let num_unserved_stop_go_pens = cursor.get_u8();
    let grid_position = cursor.get_u8();
    let driver_status = match cursor.get_u8() {
        0 => DriverStatus::InGarage,
        1 => DriverStatus::FlyingLap,
        2 => DriverStatus::InLap,
        3 => DriverStatus::OutLap,
        4 => DriverStatus::OnTrack,
        _ => DriverStatus::InGarage,
    };
    let result_status = parse_result_data(cursor);
    let pit_lane_timer_active = cursor.get_u8() != 0;
    let pit_lane_time_in_lane_in_ms = cursor.get_u16_le();
    let pit_lane_time_in_lane = Duration::from_millis(pit_lane_time_in_lane_in_ms.into());
    let pit_stop_timer_in_ms = cursor.get_u16_le();
    let pit_stop_timer = Duration::from_millis(pit_stop_timer_in_ms.into());
    let pit_stop_should_serve_pen = cursor.get_u8() != 0;

    if result_status == ResultStatus::Invalid {
        return None;
    }
    if result_status == ResultStatus::Inactive {
        return None;
    }

    Some(LapData {
        last_lap_time,
        current_lap_time,
        sector_1_time,
        sector_2_time,
        lap_distance,
        total_distance,
        safety_car_delta,
        car_position,
        current_lap_num,
        pit_status,
        num_pit_stops,
        sector,
        current_lap_invalid,
        penalties,
        warnings,
        num_unserved_drive_through_pens,
        num_unserved_stop_go_pens,
        grid_position,
        driver_status,
        result_status,
        pit_lane_timer_active,
        pit_stop_should_serve_pen,
        delta_to_car_in_front,
        delta_to_race_leader,
        corner_cutting_warnings,
        pit_lane_time_in_lane,
        pit_stop_timer,
    })
}

pub fn parse_result_data(cursor: &mut Cursor<Bytes>) -> ResultStatus {
    match cursor.get_u8() {
        1 => ResultStatus::Inactive,
        2 => ResultStatus::Active,
        3 => ResultStatus::Finished,
        4 => ResultStatus::DidNotFinish,
        5 => ResultStatus::Disqualified,
        6 => ResultStatus::NotClassified,
        7 => ResultStatus::Retired,
        _ => ResultStatus::Invalid,
    }
}
