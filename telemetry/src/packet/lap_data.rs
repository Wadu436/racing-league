use super::header::Header;

#[derive(Copy, Clone, Debug)]
pub enum PitStatus {
    None,
    Pitting,
    InPitArea,
}

#[derive(Copy, Clone, Debug)]
pub enum Sector {
    Sector1,
    Sector2,
    Sector3,
}

#[derive(Copy, Clone, Debug)]
pub enum DriverStatus {
    InGarage,
    FlyingLap,
    InLap,
    OutLap,
    OnTrack,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub struct LapData {
    pub last_lap_time_in_ms: u32,
    pub current_lap_time_in_ms: u32,
    pub sector1_time_in_ms: u16,
    pub sector2_time_in_ms: u16,
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
    pub num_unserved_drive_through_pens: u8,
    pub num_unserved_stop_go_pens: u8,
    pub grid_position: u8,
    pub driver_status: DriverStatus,
    pub result_status: ResultStatus,
    pub pit_lane_timer_active: bool,
    pub pit_lane_time_in_lane_in_ms: u16,
    pub pit_stop_timer_in_ms: u16,
    pub pit_stop_should_serve_pen: bool,
}

#[derive(Clone, Debug)]
pub struct LapDataPacket {
    pub header: Header,
    pub lap_data: Vec<LapData>,
    pub time_trial_pb_car_idx: Option<u8>,
    pub time_trial_rival_car_idx: Option<u8>,
}
