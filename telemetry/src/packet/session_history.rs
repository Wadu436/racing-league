use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::{car_status::TyreCompound, header::Header};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LapHistoryData {
    pub lap_time_in_ms: u32,
    pub sector_1_time: Duration,
    pub sector_2_time: Duration,
    pub sector_3_time: Duration,
    pub lap_valid: bool,
    pub sector_1_valid: bool,
    pub sector_2_valid: bool,
    pub sector_3_valid: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct TyreStintHistoryData {
    pub end_lap: u8,
    pub actual_tyre: TyreCompound,
    pub visual_tyre: TyreCompound,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionHistoryPacket {
    pub header: Header,

    pub car_index: u8,

    pub best_lap_time_lap_num: u8,
    pub best_sector_1_lap_num: u8,
    pub best_sector_2_lap_num: u8,
    pub best_sector_3_lap_num: u8,

    pub lap_history_data: Vec<LapHistoryData>,
    pub tyre_stint_history_data: Vec<TyreStintHistoryData>,
}
