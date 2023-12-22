use serde::{Serialize, Deserialize};

use super::header::Header;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct CarMotionData {
    pub world_position_x: f32,
    pub world_position_y: f32,
    pub world_position_z: f32,
    pub world_velocity_x: f32,
    pub world_velocity_y: f32,
    pub world_velocity_z: f32,
    pub world_forward_fir_x: i16,
    pub world_forward_fir_y: i16,
    pub world_forward_fir_z: i16,
    pub world_right_dir_x: i16,
    pub world_right_dir_y: i16,
    pub world_right_dir_z: i16,
    pub g_force_lateral: f32,
    pub g_force_longitudinal: f32,
    pub g_force_vertical: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MotionPacket {
    pub header: Header,

    pub car_motion_data: Vec<CarMotionData>,
    // Additional things I don't care about
}
