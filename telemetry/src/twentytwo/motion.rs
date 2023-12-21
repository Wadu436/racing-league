use crate::packet::motion::{CarMotionData, MotionPacket};
use bytes::{Buf, Bytes};
use std::io::Cursor;

use super::header::parse_header;

pub fn parse_motion_packet(cursor: &mut Cursor<Bytes>) -> crate::Result<MotionPacket> {
    let header = parse_header(cursor)?;

    let car_motion_data = (0..22).map(|_| parse_car_motion_data(cursor)).collect();

    Ok(MotionPacket {
        header,
        car_motion_data,
    })
}

fn parse_car_motion_data(cursor: &mut Cursor<Bytes>) -> CarMotionData {
    let world_position_x = cursor.get_f32_le();
    let world_position_y = cursor.get_f32_le();
    let world_position_z = cursor.get_f32_le();
    let world_velocity_x = cursor.get_f32_le();
    let world_velocity_y = cursor.get_f32_le();
    let world_velocity_z = cursor.get_f32_le();
    let world_forward_fir_x = cursor.get_i16_le();
    let world_forward_fir_y = cursor.get_i16_le();
    let world_forward_fir_z = cursor.get_i16_le();
    let world_right_dir_x = cursor.get_i16_le();
    let world_right_dir_y = cursor.get_i16_le();
    let world_right_dir_z = cursor.get_i16_le();
    let g_force_lateral = cursor.get_f32_le();
    let g_force_longitudinal = cursor.get_f32_le();
    let g_force_vertical = cursor.get_f32_le();
    let yaw = cursor.get_f32_le();
    let pitch = cursor.get_f32_le();
    let roll = cursor.get_f32_le();

    CarMotionData {
        world_position_x,
        world_position_y,
        world_position_z,
        world_velocity_x,
        world_velocity_y,
        world_velocity_z,
        world_forward_fir_x,
        world_forward_fir_y,
        world_forward_fir_z,
        world_right_dir_x,
        world_right_dir_y,
        world_right_dir_z,
        g_force_lateral,
        g_force_longitudinal,
        g_force_vertical,
        yaw,
        pitch,
        roll,
    }
}
