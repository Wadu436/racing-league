use serde::{Serialize, Deserialize};

use super::header::Header;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum MFDPanelIndex {
    Closed,
    CarSetup,
    Pits,
    Damage,
    Engine,
    Temperatures,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum SurfaceType {
    #[default]
    Tarmac,
    RumbleStrip,
    Concrete,
    Rock,
    Gravel,
    Mud,
    Sand,
    Grass,
    Water,
    Cobblestone,
    Metal,
    Ridged,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct CarTelemetryData {
    pub speed: u16,
    pub throttle: f32,
    pub steer: f32,
    pub brake: f32,
    pub clutch: u8,
    pub gear: i8,
    pub engine_rpm: u16,
    pub drs: bool,
    pub rev_lights_percent: u8,
    pub rev_lights_bit_value: u16,
    pub brakes_temperature: [u16; 4],
    pub tyres_surface_temperature: [u8; 4],
    pub tyres_inner_temperature: [u8; 4],
    pub engine_temperature: u16,
    pub tyres_pressure: [f32; 4],
    pub surface_type: [SurfaceType; 4],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CarTelemetryPacket {
    pub header: Header,

    pub car_telemetry_data: Vec<CarTelemetryData>,

    pub mfd_panel_index: MFDPanelIndex,
    pub mfd_panel_index_secondary_player: MFDPanelIndex,
    pub suggested_gear: i8,
}
