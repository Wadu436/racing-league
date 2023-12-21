use std::io::Cursor;

use bytes::{Buf, Bytes};

use crate::packet::car_status::{
    CarStatusData, CarStatusPacket, ERSDeployMode, FuelMix, TractionControl, TyreCompound,
};

use super::{header::parse_header, session::parse_marshal_flag};

pub fn parse_car_status(cursor: &mut Cursor<Bytes>) -> crate::Result<CarStatusPacket> {
    let header = parse_header(cursor)?;
    let car_status_data: Vec<_> = (0..22).map(|_| parse_car_status_data(cursor)).collect();

    Ok(CarStatusPacket {
        header,
        car_status_data,
    })
}

fn parse_car_status_data(cursor: &mut Cursor<Bytes>) -> CarStatusData {
    let traction_control = match cursor.get_u8() {
        1 => TractionControl::Medium,
        2 => TractionControl::Full,
        _ => TractionControl::Off,
    };
    let anti_lock_brakes = cursor.get_u8() != 0;
    let fuel_mix = match cursor.get_u8() {
        0 => FuelMix::Lean,
        2 => FuelMix::Rich,
        3 => FuelMix::Max,
        _ => FuelMix::Standard,
    };
    let front_brake_bias = cursor.get_u8();
    let pit_limiter_status = cursor.get_u8() != 0;
    let fuel_in_tank = cursor.get_f32_le();
    let fuel_capacity = cursor.get_f32_le();
    let fuel_remaining_laps = cursor.get_f32_le();
    let max_rpm = cursor.get_u16_le();
    let idle_rpm = cursor.get_u16_le();
    let max_gears = cursor.get_u8();
    let drs_allowed = cursor.get_u8();
    let drs_activation_distance = cursor.get_u16_le();
    let actual_tyre_compound = match cursor.get_u8() {
        16 => TyreCompound::C5,
        17 => TyreCompound::C4,
        18 => TyreCompound::C3,
        19 => TyreCompound::C2,
        20 => TyreCompound::C1,
        7 => TyreCompound::Inter,
        8 | 10 | 15 => TyreCompound::Wet,
        9 => TyreCompound::Dry,
        11 => TyreCompound::SuperSoft,
        12 => TyreCompound::Soft,
        13 => TyreCompound::Medium,
        14 => TyreCompound::Hard,
        _ => TyreCompound::Hard,
    };
    let visual_tyre_compound = match cursor.get_u8() {
        7 => TyreCompound::Inter,
        8 | 10 | 15 => TyreCompound::Wet,
        9 => TyreCompound::Dry,
        19 => TyreCompound::SuperSoft,
        16 | 20 => TyreCompound::Soft,
        17 | 21 => TyreCompound::Medium,
        18 | 22 => TyreCompound::Hard,
        _ => TyreCompound::Hard,
    };
    let tyres_age_laps = cursor.get_u8();
    let vehicle_fia_flags = parse_marshal_flag(cursor);
    let ers_store_energy = cursor.get_f32_le();
    let ers_deploy_mode = match cursor.get_u8() {
        1 => ERSDeployMode::Medium,
        2 => ERSDeployMode::Hotlap,
        3 => ERSDeployMode::Overtake,
        _ => ERSDeployMode::None,
    };
    let ers_harvested_this_lap_mguk = cursor.get_f32_le();
    let ers_harvested_this_lap_mguh = cursor.get_f32_le();
    let ers_deployed_this_lap = cursor.get_f32_le();
    let network_paused = cursor.get_u8() != 0;

    CarStatusData {
        traction_control,
        anti_lock_brakes,
        fuel_mix,
        front_brake_bias,
        pit_limiter_status,
        fuel_in_tank,
        fuel_capacity,
        fuel_remaining_laps,
        max_rpm,
        idle_rpm,
        max_gears,
        drs_allowed,
        drs_activation_distance,
        actual_tyre_compound,
        visual_tyre_compound,
        tyres_age_laps,
        vehicle_fia_flags,
        ers_store_energy,
        ers_deploy_mode,
        ers_harvested_this_lap_mguk,
        ers_harvested_this_lap_mguh,
        ers_deployed_this_lap,
        network_paused,
    }
}
