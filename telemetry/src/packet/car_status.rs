use serde::{Deserialize, Serialize};

use super::{header::Header, session::MarshalFlag};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub enum TractionControl {
    #[default]
    Off,
    Medium,
    Full,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum FuelMix {
    Lean,
    Standard,
    Rich,
    Max,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub enum TyreCompound {
    C5,
    C4,
    C3,
    C2,
    C1,
    C0,
    Inter,
    Wet,
    Dry,
    #[serde(rename = "Super Soft")]
    SuperSoft,
    #[default]
    Soft,
    Medium,
    Hard,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub enum ERSDeployMode {
    #[default]
    None,
    Medium,
    Hotlap,
    Overtake,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct CarStatusData {
    pub traction_control: TractionControl,
    pub anti_lock_brakes: bool,
    pub fuel_mix: FuelMix,
    pub front_brake_bias: u8,
    pub pit_limiter_status: bool,
    pub fuel_in_tank: f32,
    pub fuel_capacity: f32,
    pub fuel_remaining_laps: f32,
    pub max_rpm: u16,
    pub idle_rpm: u16,
    pub max_gears: u8,
    pub drs_allowed: u8,
    pub drs_activation_distance: u16,
    pub actual_tyre_compound: TyreCompound,
    pub visual_tyre_compound: TyreCompound,
    pub tyres_age_laps: u8,
    pub vehicle_fia_flags: MarshalFlag,
    pub engine_power_ice: f32,
    pub engine_power_mguk: f32,
    pub ers_store_energy: f32,
    pub ers_deploy_mode: ERSDeployMode,
    pub ers_harvested_this_lap_mguk: f32,
    pub ers_harvested_this_lap_mguh: f32,
    pub ers_deployed_this_lap: f32,
    pub network_paused: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CarStatusPacket {
    pub header: Header,

    pub car_status_data: Vec<CarStatusData>,
}
