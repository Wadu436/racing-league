use std::time::Duration;

use serde::{Serialize, Deserialize};

use super::header::Header;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct FastestLap {
    pub vehicle_idx: u8,
    pub lap_time: Duration,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct VehicleIdx(pub u8);

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PenaltyType {
    Unknown,
    DriveThrough,
    StopGo,
    GridPenalty,
    PenaltyReminder,
    TimePenalty,
    Warning,
    Disqualified,
    RemovedFromFormationLap,
    ParkedTooLongTimer,
    TyreRegulations,
    ThisLapInvalidated,
    ThisAndNextLapInvalidated,
    ThisLapInvalidatedWithoutReason,
    ThisAndNextLapInvalidatedWithoutReason,
    ThisAndPreviousLapInvalidated,
    ThisAndPreviousLapInvalidatedWithoutReason,
    Retired,
    BlackFlagTimer,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum InfringementType {
    Unknown,
    BlockingBySlowDriving,
    BlockingByWrongWayDriving,
    ReversingOffTheStartLine,
    BigCollision,
    SmallCollision,
    CollisionFailedToHandBackPositionSingle,
    CollisionFailedToHandBackPositionMultiple,
    CornerCuttingGainedTime,
    CornerCuttingOvertakeSingle,
    CornerCuttingOvertakeMultiple,
    CrossedPitExitLane,
    IgnoringBlueFlags,
    IgnoringYellowFlags,
    IgnoringDriveThrough,
    TooManyDriveThroughs,
    DriveThroughReminderServeWithinNLaps,
    DriveThroughReminderServeThisLap,
    PitLaneSpeeding,
    ParkedForTooLong,
    IgnoringTyreRegulations,
    TooManyPenalties,
    MultipleWarnings,
    ApproachingDisqualification,
    TyreRegulationsSelectSingle,
    TyreRegulationsSelectMultiple,
    LapInvalidatedCornerCutting,
    LapInvalidatedRunningWide,
    CornerCuttingRanWideGainedTimeMinor,
    CornerCuttingRanWideGainedTimeSignificant,
    CornerCuttingRanWideGainedTimeExtreme,
    LapInvalidatedWallRiding,
    LapInvalidatedFlashbackUsed,
    LapInvalidatedResetToTrack,
    BlockingThePitlane,
    JumpStart,
    SafetyCarToCarCollision,
    SafetyCarIllegalOvertake,
    SafetyCarExceedingAllowedPace,
    VirtualSafetyCarExceedingAllowedPace,
    FormationlapBelowAllowedSpeed,
    FormationLapParking,
    RetiredMechanicalFailure,
    RetiredTerminallyDamaged,
    SafetyCarFallingTooFarBack,
    BlackFlagTimer,
    UnservedStopGoPenalty,
    UnservedDriveThroughPenalty,
    EngineComponentChange,
    GearboxChange,
    ParcFermeChange,
    LeagueGridPenalty,
    RetryPenalty,
    IllegalTimeGain,
    MandatoryPitstop,
    AttributeAssigned,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Penalty {
    pub penalty_type: PenaltyType,
    pub infringement_type: InfringementType,
    pub vehicle_idx: u8,
    pub other_vehicle_idx: u8,
    pub time: u8,
    pub lap_num: u8,
    pub places_gained: u8,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SpeedTrap {
    pub vehicle_idx: u8,
    pub speed: f32,
    pub is_overall_fastest_in_session: bool,
    pub is_driver_fastest_in_session: bool,
    pub fastest_vehicle_idx_in_session: u8,
    pub fastest_speed_in_session: f32,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct StartLights {
    pub num_lights: u8,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Flashback {
    pub flashback_frame_identifier: u32,
    pub flashback_session_time: f32,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Buttons {
    pub button_status: u32,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Overtake {
    pub overtaking_vehicle_idx: u8,
    pub being_overtaken_vehicle_idx: u8,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
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
    RedFlag,
    Overtake(Overtake),
    Unknown,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct EventPacket {
    pub header: Header,
    pub event: Event,
}
