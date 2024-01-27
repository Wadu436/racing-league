use std::time::Duration;

use serde::{Serialize, Deserialize};

use super::header::Header;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PitStatus {
    None,
    Pitting,
    InPitArea,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Sector {
    Sector1,
    Sector2,
    Sector3,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DriverStatus {
    InGarage,
    FlyingLap,
    InLap,
    OutLap,
    OnTrack,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResultStatus {
    Invalid,
    Inactive,
    Active,
    Finished,
    DidNotFinish,
    Disqualified,
    NotClassified,
    Retired,
}

impl ResultStatus {
    pub fn valid_result(&self) -> bool {
        match self {
            Self::Invalid | Self::Inactive | Self::NotClassified => false,
            _ => true,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LapData {
    pub last_lap_time: Duration,
    pub current_lap_time: Duration,
    pub sector_1_time: Duration,
    pub sector_2_time: Duration,
    pub delta_to_car_in_front: Duration,
    pub delta_to_race_leader: Duration,
    pub lap_distance: f32,
    pub total_distance: f32,
    pub safety_car_delta: f32,
    pub car_position: u8,
    pub current_lap_num: u8,
    pub pit_status: PitStatus,
    pub num_pit_stops: u8,
    pub sector: Sector,
    pub current_lap_invalid: bool,
    pub penalties: u8,
    pub warnings: u8,
    pub corner_cutting_warnings: u8,
    pub num_unserved_drive_through_pens: u8,
    pub num_unserved_stop_go_pens: u8,
    pub grid_position: u8,
    pub driver_status: DriverStatus,
    pub result_status: ResultStatus,
    pub pit_lane_timer_active: bool,
    pub pit_lane_time_in_lane: Duration,
    pub pit_stop_timer: Duration,
    pub pit_stop_should_serve_pen: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LapDataPacket {
    pub header: Header,
    pub lap_data: Vec<Option<LapData>>,
    pub time_trial_pb_car_idx: Option<u8>,
    pub time_trial_rival_car_idx: Option<u8>,
}
