use serde::{Serialize, Deserialize};

use super::{header::Header, lap_data::ResultStatus, car_status::TyreCompound};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct TyreStint {
    pub actual_tyre: TyreCompound,
    pub visual_tyre: TyreCompound,
    pub end_lap: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FinalClassificationData {
    pub position: u8,
    pub num_laps: u8,
    pub grid_position: u8,
    pub points: u8,
    pub num_pit_stops: u8,
    pub status: ResultStatus,
    pub best_laptime_in_ms: u32,
    pub total_race_time_without_penalties_in_seconds: f64,
    pub penalty_time_in_seconds: u8,
    pub num_penalties: u8,
    pub tyre_stints: Vec<TyreStint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FinalClassificationPacket {
    pub header: Header,

    pub classification_data: Vec<FinalClassificationData>,
}
