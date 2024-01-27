use bytes::{Buf, Bytes};
use std::{
    io::{Cursor, Read},
    time::Duration,
};

use crate::packet::event::{
    Buttons, Event, EventPacket, FastestLap, Flashback, InfringementType, Overtake, Penalty, PenaltyType, SpeedTrap, StartLights, VehicleIdx
};

use super::header::parse_header;

pub fn parse_event_packet(cursor: &mut Cursor<Bytes>) -> crate::Result<EventPacket> {
    let header = parse_header(cursor)?;

    let mut event_string_code: [u8; 4] = [0; 4];
    cursor.read_exact(&mut event_string_code)?;
    let event_string_code = String::from_utf8_lossy(&event_string_code);
    let event = match event_string_code.as_ref() {
        "SSTA" => Event::SessionStarted,
        "SEND" => Event::SessionEnded,
        "FTLP" => Event::FastestLap(parse_fastest_lap(cursor)),
        "RTMT" => Event::Retirement(parse_vehicle_idx(cursor)),
        "DRSE" => Event::DRSEnabled,
        "DRSD" => Event::DRSDisabled,
        "TMPT" => Event::TeamMateInPits(parse_vehicle_idx(cursor)),
        "CHQF" => Event::ChequeredFlag,
        "RCWN" => Event::RaceWinner(parse_vehicle_idx(cursor)),
        "PENA" => Event::PenaltyIssued(parse_penalty(cursor)),
        "SPTP" => Event::SpeedTrapTriggered(parse_speed_trap(cursor)),
        "STLG" => Event::StartLights(parse_start_lights(cursor)),
        "LGOT" => Event::LightsOut,
        "DTSV" => Event::DriveThroughServed(parse_vehicle_idx(cursor)),
        "SGSV" => Event::StopGoServed,
        "FLBK" => Event::Flashback(parse_flashback(cursor)),
        "BUTN" => Event::Button(parse_buttons(cursor)),
        "RDFL" => Event::RedFlag,
        "OVTK" => Event::Overtake(parse_overtake(cursor)),
        _ => Event::Unknown,
    };

    Ok(EventPacket { header, event })
}

fn parse_overtake(cursor: &mut Cursor<Bytes>) -> Overtake {
    let overtaking_vehicle_idx = cursor.get_u8();
    let being_overtaken_vehicle_idx = cursor.get_u8();
    Overtake {
        overtaking_vehicle_idx,
        being_overtaken_vehicle_idx,
    }
}

fn parse_fastest_lap(cursor: &mut Cursor<Bytes>) -> FastestLap {
    let vehicle_idx = cursor.get_u8();
    let lap_time = Duration::from_secs_f32(cursor.get_f32_le());
    FastestLap {
        vehicle_idx,
        lap_time,
    }
}

fn parse_vehicle_idx(cursor: &mut Cursor<Bytes>) -> VehicleIdx {
    let vehicle_idx = cursor.get_u8();
    VehicleIdx(vehicle_idx)
}

fn parse_penalty_type(type_id: u8) -> PenaltyType {
    match type_id {
        0 => PenaltyType::DriveThrough,
        1 => PenaltyType::StopGo,
        2 => PenaltyType::GridPenalty,
        3 => PenaltyType::PenaltyReminder,
        4 => PenaltyType::TimePenalty,
        5 => PenaltyType::Warning,
        6 => PenaltyType::Disqualified,
        7 => PenaltyType::RemovedFromFormationLap,
        8 => PenaltyType::ParkedTooLongTimer,
        9 => PenaltyType::TyreRegulations,
        10 => PenaltyType::ThisLapInvalidated,
        11 => PenaltyType::ThisAndNextLapInvalidated,
        12 => PenaltyType::ThisLapInvalidatedWithoutReason,
        13 => PenaltyType::ThisAndNextLapInvalidatedWithoutReason,
        14 => PenaltyType::ThisAndPreviousLapInvalidated,
        15 => PenaltyType::ThisAndPreviousLapInvalidatedWithoutReason,
        16 => PenaltyType::Retired,
        17 => PenaltyType::BlackFlagTimer,
        _ => PenaltyType::Unknown,
    }
}

fn parse_infringement_type(type_id: u8) -> InfringementType {
    match type_id {
        0 => InfringementType::BlockingBySlowDriving,
        1 => InfringementType::BlockingByWrongWayDriving,
        2 => InfringementType::ReversingOffTheStartLine,
        3 => InfringementType::BigCollision,
        4 => InfringementType::SmallCollision,
        5 => InfringementType::CollisionFailedToHandBackPositionSingle,
        6 => InfringementType::CollisionFailedToHandBackPositionMultiple,
        7 => InfringementType::CornerCuttingGainedTime,
        8 => InfringementType::CornerCuttingOvertakeSingle,
        9 => InfringementType::CornerCuttingOvertakeMultiple,
        10 => InfringementType::CrossedPitExitLane,
        11 => InfringementType::IgnoringBlueFlags,
        12 => InfringementType::IgnoringYellowFlags,
        13 => InfringementType::IgnoringDriveThrough,
        14 => InfringementType::TooManyDriveThroughs,
        15 => InfringementType::DriveThroughReminderServeWithinNLaps,
        16 => InfringementType::DriveThroughReminderServeThisLap,
        17 => InfringementType::PitLaneSpeeding,
        18 => InfringementType::ParkedForTooLong,
        19 => InfringementType::IgnoringTyreRegulations,
        20 => InfringementType::TooManyPenalties,
        21 => InfringementType::MultipleWarnings,
        22 => InfringementType::ApproachingDisqualification,
        23 => InfringementType::TyreRegulationsSelectSingle,
        24 => InfringementType::TyreRegulationsSelectMultiple,
        25 => InfringementType::LapInvalidatedCornerCutting,
        26 => InfringementType::LapInvalidatedRunningWide,
        27 => InfringementType::CornerCuttingRanWideGainedTimeMinor,
        28 => InfringementType::CornerCuttingRanWideGainedTimeSignificant,
        29 => InfringementType::CornerCuttingRanWideGainedTimeExtreme,
        30 => InfringementType::LapInvalidatedWallRiding,
        31 => InfringementType::LapInvalidatedFlashbackUsed,
        32 => InfringementType::LapInvalidatedResetToTrack,
        33 => InfringementType::BlockingThePitlane,
        34 => InfringementType::JumpStart,
        35 => InfringementType::SafetyCarToCarCollision,
        36 => InfringementType::SafetyCarIllegalOvertake,
        37 => InfringementType::SafetyCarExceedingAllowedPace,
        38 => InfringementType::VirtualSafetyCarExceedingAllowedPace,
        39 => InfringementType::FormationlapBelowAllowedSpeed,
        40 => InfringementType::FormationLapParking,
        41 => InfringementType::RetiredMechanicalFailure,
        42 => InfringementType::RetiredTerminallyDamaged,
        43 => InfringementType::SafetyCarFallingTooFarBack,
        44 => InfringementType::BlackFlagTimer,
        45 => InfringementType::UnservedStopGoPenalty,
        46 => InfringementType::UnservedDriveThroughPenalty,
        47 => InfringementType::EngineComponentChange,
        48 => InfringementType::GearboxChange,
        49 => InfringementType::ParcFermeChange,
        50 => InfringementType::LeagueGridPenalty,
        51 => InfringementType::RetryPenalty,
        52 => InfringementType::IllegalTimeGain,
        53 => InfringementType::MandatoryPitstop,
        54 => InfringementType::AttributeAssigned,
        _ => InfringementType::Unknown,
    }
}

fn parse_penalty(cursor: &mut Cursor<Bytes>) -> Penalty {
    let penalty_type = parse_penalty_type(cursor.get_u8());
    let infringement_type = parse_infringement_type(cursor.get_u8());
    let vehicle_idx = cursor.get_u8();
    let other_vehicle_idx = cursor.get_u8();
    let time = cursor.get_u8();
    let lap_num = cursor.get_u8();
    let places_gained = cursor.get_u8();
    Penalty {
        penalty_type,
        infringement_type,
        vehicle_idx,
        other_vehicle_idx,
        time,
        lap_num,
        places_gained,
    }
}

fn parse_speed_trap(cursor: &mut Cursor<Bytes>) -> SpeedTrap {
    let vehicle_idx = cursor.get_u8();
    let speed = cursor.get_f32_le();
    let is_overall_fastest_in_session = cursor.get_u8() != 0;
    let is_driver_fastest_in_session = cursor.get_u8() != 0;

    let fastest_vehicle_idx_in_session = cursor.get_u8();
    let fastest_speed_in_session = cursor.get_f32_le();

    SpeedTrap {
        vehicle_idx,
        speed,
        is_overall_fastest_in_session,
        is_driver_fastest_in_session,
        fastest_vehicle_idx_in_session,
        fastest_speed_in_session,
    }
}

fn parse_start_lights(cursor: &mut Cursor<Bytes>) -> StartLights {
    let num_lights = cursor.get_u8();
    StartLights { num_lights }
}

fn parse_flashback(cursor: &mut Cursor<Bytes>) -> Flashback {
    let flashback_frame_identifier = cursor.get_u32_le();
    let flashback_session_time = cursor.get_f32_le();
    Flashback {
        flashback_frame_identifier,
        flashback_session_time,
    }
}

fn parse_buttons(cursor: &mut Cursor<Bytes>) -> Buttons {
    let button_status = cursor.get_u32_le();
    Buttons { button_status }
}
