use bytes::{Buf, Bytes};
use std::io::{self, Cursor, Read};

use crate::packet::event::{
    Buttons, Event, EventPacket, FastestLap, Flashback, Penalty, SpeedTrap, StartLights, VehicleIdx,
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
        _ => Event::Unknown,
    };

    Ok(EventPacket { header, event })
}

fn parse_fastest_lap(cursor: &mut Cursor<Bytes>) -> FastestLap {
    let vehicle_idx = cursor.get_u8();
    let lap_time = cursor.get_f32_le();
    FastestLap {
        vehicle_idx,
        lap_time,
    }
}

fn parse_vehicle_idx(cursor: &mut Cursor<Bytes>) -> VehicleIdx {
    let vehicle_idx = cursor.get_u8();
    VehicleIdx { vehicle_idx }
}

fn parse_penalty(cursor: &mut Cursor<Bytes>) -> Penalty {
    let penalty_type = cursor.get_u8();
    let infringement_type = cursor.get_u8();
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
    let overall_fastest_in_session = cursor.get_u8();
    let driver_fastest_in_session = cursor.get_u8();

    let fastest_vehicle_idx_in_session = Some(cursor.get_u8());
    let fastest_speed_in_session = Some(cursor.get_f32_le());

    SpeedTrap {
        vehicle_idx,
        speed,
        overall_fastest_in_session,
        driver_fastest_in_session,
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
