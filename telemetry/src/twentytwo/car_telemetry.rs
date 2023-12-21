use std::io::Cursor;

use bytes::{Buf, Bytes};

use crate::packet::car_telemetry::{
    CarTelemetryData, CarTelemetryPacket, MFDPanelIndex, SurfaceType,
};

use super::header::parse_header;

pub fn parse_car_telemetry(cursor: &mut Cursor<Bytes>) -> crate::Result<CarTelemetryPacket> {
    let header = parse_header(cursor)?;

    let car_telemetry_data: Vec<_> = (0..22).map(|_| parse_car_telemetry_data(cursor)).collect();

    let mfd_panel_index = parse_mfd_panel_index(cursor);
    let mfd_panel_index_secondary_player = parse_mfd_panel_index(cursor);

    let suggested_gear = cursor.get_i8();

    Ok(CarTelemetryPacket {
        header,
        car_telemetry_data,
        mfd_panel_index,
        mfd_panel_index_secondary_player,
        suggested_gear,
    })
}

fn parse_car_telemetry_data(cursor: &mut Cursor<Bytes>) -> CarTelemetryData {
    let speed = cursor.get_u16_le();
    let throttle = cursor.get_f32_le();
    let steer = cursor.get_f32_le();
    let brake = cursor.get_f32_le();
    let clutch = cursor.get_u8();
    let gear = cursor.get_i8();
    let engine_rpm = cursor.get_u16_le();
    let drs = cursor.get_u8() != 0;
    let rev_lights_percent = cursor.get_u8();
    let rev_lights_bit_value = cursor.get_u16_le();
    let brakes_temperature = [
        cursor.get_u16_le(),
        cursor.get_u16_le(),
        cursor.get_u16_le(),
        cursor.get_u16_le(),
    ];
    let tyres_surface_temperature = [
        cursor.get_u8(),
        cursor.get_u8(),
        cursor.get_u8(),
        cursor.get_u8(),
    ];
    let tyres_inner_temperature = [
        cursor.get_u8(),
        cursor.get_u8(),
        cursor.get_u8(),
        cursor.get_u8(),
    ];
    let engine_temperature = cursor.get_u16_le();
    let tyres_pressure = [
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
        cursor.get_f32_le(),
    ];
    let surface_type = [
        parse_surface_type(cursor),
        parse_surface_type(cursor),
        parse_surface_type(cursor),
        parse_surface_type(cursor),
    ];

    CarTelemetryData {
        speed,
        throttle,
        steer,
        brake,
        clutch,
        gear,
        engine_rpm,
        drs,
        rev_lights_percent,
        rev_lights_bit_value,
        brakes_temperature,
        tyres_surface_temperature,
        tyres_inner_temperature,
        engine_temperature,
        tyres_pressure,
        surface_type,
    }
}

fn parse_mfd_panel_index(cursor: &mut Cursor<Bytes>) -> MFDPanelIndex {
    match cursor.get_u8() {
        0 => MFDPanelIndex::CarSetup,
        1 => MFDPanelIndex::Pits,
        2 => MFDPanelIndex::Damage,
        3 => MFDPanelIndex::Engine,
        4 => MFDPanelIndex::Temperatures,
        _ => MFDPanelIndex::Closed,
    }
}

fn parse_surface_type(cursor: &mut Cursor<Bytes>) -> SurfaceType {
    match cursor.get_u8() {
        1 => SurfaceType::RumbleStrip,
        2 => SurfaceType::Concrete,
        3 => SurfaceType::Rock,
        4 => SurfaceType::Gravel,
        5 => SurfaceType::Mud,
        6 => SurfaceType::Sand,
        7 => SurfaceType::Grass,
        8 => SurfaceType::Water,
        9 => SurfaceType::Cobblestone,
        10 => SurfaceType::Metal,
        11 => SurfaceType::Ridged,
        _ => SurfaceType::Tarmac,
    }
}
