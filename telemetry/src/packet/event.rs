use super::header::Header;

#[derive(Copy, Clone, Debug)]
pub struct FastestLap {
    pub vehicle_idx: u8,
    pub lap_time: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct VehicleIdx {
    pub vehicle_idx: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct Penalty {
    pub penalty_type: u8,
    pub infringement_type: u8,
    pub vehicle_idx: u8,
    pub other_vehicle_idx: u8,
    pub time: u8,
    pub lap_num: u8,
    pub places_gained: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct SpeedTrap {
    pub vehicle_idx: u8,
    pub speed: f32,
    pub overall_fastest_in_session: u8,
    pub driver_fastest_in_session: u8,
    pub fastest_vehicle_idx_in_session: Option<u8>,
    pub fastest_speed_in_session: Option<f32>,
}

#[derive(Copy, Clone, Debug)]
pub struct StartLights {
    pub num_lights: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct Flashback {
    pub flashback_frame_identifier: u32,
    pub flashback_session_time: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Buttons {
    pub button_status: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    SessionStarted,
    SessionEnded,
    FastestLap(FastestLap),
    Retirement(VehicleIdx),
    DRSEnabled,
    DRSDisabled,
    TeamMateInPits(VehicleIdx),
    ChequeredFlag,
    RaceWinner(VehicleIdx),
    PenaltyIssued(Penalty),
    SpeedTrapTriggered(SpeedTrap),
    StartLights(StartLights),
    LightsOut,
    DriveThroughServed(VehicleIdx),
    StopGoServed,
    Flashback(Flashback),
    Button(Buttons),
    Unknown,
}

#[derive(Copy, Clone, Debug)]
pub struct EventPacket {
    pub header: Header,
    pub event: Event,
}
