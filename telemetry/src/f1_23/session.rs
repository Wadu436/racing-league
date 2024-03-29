use bytes::{Buf, Bytes};

use std::io::Cursor;

use crate::packet::session::{
    BrakingAssist, DynamicRacingLine, DynamicRacingLineType, ForecastAccuracy, Formula, GameMode,
    GearboxAssist, MarshalFlag, MarshalZone, Ruleset, SafetyCarStatus, SessionLength,
    SessionPacket, SessionType, SpeedUnit, TempChange, TempUnit, Track, Weather,
    WeatherForecastSample,
};

use super::header::parse_header;

pub fn parse_session_packet(cursor: &mut Cursor<Bytes>) -> crate::Result<SessionPacket> {
    if cursor.remaining() != 644 {
        return Err(crate::TelemetryError::InvalidPacket(
            "invalid session packet length".to_owned(),
        ));
    }

    let header = parse_header(cursor)?;

    let weather = parse_weather(cursor);
    let track_temperature = cursor.get_i8();
    let air_temperature = cursor.get_i8();
    let total_laps = cursor.get_u8();
    let track_length = cursor.get_u16_le();
    let session_type = parse_session_type(cursor);
    let track = parse_track(cursor);
    let formula = match cursor.get_u8() {
        1 => Formula::F1Classic,
        2 => Formula::F2,
        3 => Formula::F1Generic,
        4 => Formula::Beta,
        5 => Formula::Supercars,
        6 => Formula::Esports,
        7 => Formula::F2_2021,
        _ => Formula::F1Modern,
    };
    let session_time_left = cursor.get_u16_le();
    let session_duration = cursor.get_u16_le();
    let pit_speed_limit = cursor.get_u8();
    let game_paused = cursor.get_u8() != 0;
    let is_spectating = cursor.get_u8() != 0;
    let spectator_car_index = cursor.get_u8();
    let sli_pro_native_support = cursor.get_u8() != 0;
    let num_marshal_zones = cursor.get_u8();
    let mut marshal_zones: Vec<MarshalZone> = (0..21).map(|_| parse_marshal_zone(cursor)).collect();
    marshal_zones.truncate(num_marshal_zones as usize);
    let marshal_zones = marshal_zones;
    let safety_car_status = match cursor.get_u8() {
        1 => SafetyCarStatus::Full,
        2 => SafetyCarStatus::Virtual,
        3 => SafetyCarStatus::Formation,
        _ => SafetyCarStatus::No,
    };
    let network_game = cursor.get_u8() != 0;
    let num_weather_forecast_samples = cursor.get_u8();
    let mut weather_forecast_samples: Vec<WeatherForecastSample> = (0..56)
        .map(|_| parse_weather_forecast_sample(cursor))
        .collect();
    weather_forecast_samples.truncate(num_weather_forecast_samples as usize);
    let weather_forecast_samples = weather_forecast_samples;
    let forecast_accuracy = if cursor.get_u8() == 0 {
        ForecastAccuracy::Perfect
    } else {
        ForecastAccuracy::Approximate
    };
    let ai_difficulty: u8 = cursor.get_u8();
    let season_link_identifier = cursor.get_u32_le();
    let weekend_link_identifier = cursor.get_u32_le();
    let session_link_identifier = cursor.get_u32_le();
    let pit_stop_window_ideal_lap = cursor.get_u8();
    let pit_stop_window_latest_lap = cursor.get_u8();
    let pit_stop_rejoin_position = cursor.get_u8();
    let steering_assist = cursor.get_u8() != 0;
    let braking_assist = match cursor.get_u8() {
        0 => BrakingAssist::Off,
        1 => BrakingAssist::Low,
        2 => BrakingAssist::Medium,
        _ => BrakingAssist::High,
    };
    let gearbox_assist = match cursor.get_u8() {
        1 => GearboxAssist::Manual,
        2 => GearboxAssist::ManualSuggested,
        _ => GearboxAssist::Auto,
    };
    let pit_assist = cursor.get_u8() != 0;
    let pit_release_assist = cursor.get_u8() != 0;
    let ers_assist = cursor.get_u8() != 0;
    let drs_assist = cursor.get_u8() != 0;
    let dynamic_racing_line = match cursor.get_u8() {
        0 => DynamicRacingLine::Off,
        1 => DynamicRacingLine::Corners,
        _ => DynamicRacingLine::Full,
    };
    let dynamic_racing_line_type = if cursor.get_u8() == 0 {
        DynamicRacingLineType::TwoD
    } else {
        DynamicRacingLineType::ThreeD
    };

    let game_mode = parse_game_mode(cursor);
    let ruleset = parse_ruleset(cursor);
    let time_of_day = cursor.get_u32_le();
    let session_length = parse_session_length(cursor);

    let speed_units_lead_player = if cursor.get_u8() == 0 {
        SpeedUnit::Mph
    } else {
        SpeedUnit::Kmh
    };
    let temperature_units_lead_player = if cursor.get_u8() == 0 {
        TempUnit::Celsius
    } else {
        TempUnit::Fahrenheit
    };
    let speed_units_secondary_player = if cursor.get_u8() == 0 {
        SpeedUnit::Mph
    } else {
        SpeedUnit::Kmh
    };
    let temperature_units_secondary_player = if cursor.get_u8() == 0 {
        TempUnit::Celsius
    } else {
        TempUnit::Fahrenheit
    };
    let num_safety_car_periods = cursor.get_u8();
    let num_virtual_safety_car_periods = cursor.get_u8();
    let num_red_flag_periods = cursor.get_u8();

    Ok(SessionPacket {
        header,
        weather,
        track_temperature,
        air_temperature,
        total_laps,
        track_length,
        session_type,
        track,
        formula,
        session_time_left,
        session_duration,
        pit_speed_limit,
        game_paused,
        is_spectating,
        spectator_car_index,
        sli_pro_native_support,
        marshal_zones,
        safety_car_status,
        network_game,
        weather_forecast_samples,
        forecast_accuracy,
        ai_difficulty,
        season_link_identifier,
        weekend_link_identifier,
        session_link_identifier,
        pit_stop_window_ideal_lap,
        pit_stop_window_latest_lap,
        pit_stop_rejoin_position,
        steering_assist,
        braking_assist,
        gearbox_assist,
        pit_assist,
        pit_release_assist,
        ers_assist,
        drs_assist,
        dynamic_racing_line,
        dynamic_racing_line_type,
        game_mode,
        ruleset,
        time_of_day,
        session_length,
        speed_units_lead_player,
        temperature_units_lead_player,
        speed_units_secondary_player,
        temperature_units_secondary_player,
        num_safety_car_periods,
        num_virtual_safety_car_periods,
        num_red_flag_periods,
    })
}

fn parse_track(cursor: &mut Cursor<Bytes>) -> Track {
    match cursor.get_i8() {
        -1 => Track::Unknown,
        0 => Track::Melbourne,
        1 => Track::PaulRicard,
        2 => Track::Shanghai,
        3 => Track::Sakhir,
        4 => Track::Catalunya,
        5 => Track::Monaco,
        6 => Track::Montreal,
        7 => Track::Silverstone,
        8 => Track::Hockenheim,
        9 => Track::Hungaroring,
        10 => Track::Spa,
        11 => Track::Monza,
        12 => Track::Singapore,
        13 => Track::Suzuka,
        14 => Track::AbuDhabi,
        15 => Track::Texas,
        16 => Track::Brazil,
        17 => Track::Austria,
        18 => Track::Sochi,
        19 => Track::Mexico,
        20 => Track::Baku,
        21 => Track::SakhirShort,
        22 => Track::SilverstoneShort,
        23 => Track::TexasShort,
        24 => Track::SuzukaShort,
        25 => Track::Hanoi,
        26 => Track::Zandvoort,
        27 => Track::Imola,
        28 => Track::Portimao,
        29 => Track::Jeddah,
        30 => Track::Miami,
        31 => Track::Vegas,
        32 => Track::Losail,
        _ => Track::Unknown,
    }
}

pub fn parse_marshal_flag(cursor: &mut Cursor<Bytes>) -> MarshalFlag {
    match cursor.get_i8() {
        0 => MarshalFlag::None,
        1 => MarshalFlag::Green,
        2 => MarshalFlag::Blue,
        3 => MarshalFlag::Yellow,
        _ => MarshalFlag::Unknown,
    }
}

fn parse_marshal_zone(cursor: &mut Cursor<Bytes>) -> MarshalZone {
    let zone_start = cursor.get_f32_le();
    let zone_flag = parse_marshal_flag(cursor);

    MarshalZone {
        zone_start,
        zone_flag,
    }
}

fn parse_session_type(cursor: &mut Cursor<Bytes>) -> SessionType {
    match cursor.get_u8() {
        1 => SessionType::P1,
        2 => SessionType::P2,
        3 => SessionType::P3,
        4 => SessionType::ShortP,
        5 => SessionType::Q1,
        6 => SessionType::Q2,
        7 => SessionType::Q3,
        8 => SessionType::ShortQ,
        9 => SessionType::OSQ,
        10 => SessionType::R,
        11 => SessionType::R2,
        12 => SessionType::R3,
        13 => SessionType::TimeTrial,
        _ => SessionType::Unknown,
    }
}

fn parse_weather(cursor: &mut Cursor<Bytes>) -> Weather {
    match cursor.get_u8() {
        1 => Weather::LightCloud,
        2 => Weather::Overcast,
        3 => Weather::LightRain,
        4 => Weather::HeavyRain,
        5 => Weather::Storm,
        _ => Weather::Clear,
    }
}

fn parse_weather_forecast_sample(cursor: &mut Cursor<Bytes>) -> WeatherForecastSample {
    let session_type = parse_session_type(cursor);
    let time_offset = cursor.get_u8();
    let weather = parse_weather(cursor);

    let track_temperature = cursor.get_i8();
    let track_temperature_change = match cursor.get_i8() {
        0 => TempChange::Increase,
        1 => TempChange::Decrease,
        _ => TempChange::NoChange,
    };
    let air_temperature = cursor.get_i8();
    let air_temperature_change = match cursor.get_i8() {
        0 => TempChange::Increase,
        1 => TempChange::Decrease,
        _ => TempChange::NoChange,
    };
    let rain_percentage = cursor.get_u8();

    WeatherForecastSample {
        session_type,
        time_offset,
        weather,
        track_temperature,
        track_temperature_change,
        air_temperature,
        air_temperature_change,
        rain_percentage,
    }
}

fn parse_game_mode(cursor: &mut Cursor<Bytes>) -> GameMode {
    match cursor.get_u8() {
        0 => GameMode::EventMode,
        3 => GameMode::GrandPrix,
        4 => GameMode::GrandPrix23,
        5 => GameMode::TimeTrial,
        6 => GameMode::Splitscreen,
        7 => GameMode::OnlineCustom,
        8 => GameMode::OnlineLeague,
        11 => GameMode::CareerInvitational,
        12 => GameMode::ChampionshipInvitational,
        13 => GameMode::Championship,
        14 => GameMode::OnlineChampionship,
        15 => GameMode::OnlineWeeklyEvent,
        17 => GameMode::StoryMode,
        19 => GameMode::Career22,
        20 => GameMode::Career22Online,
        21 => GameMode::Career23,
        22 => GameMode::Career23Online,
        127 => GameMode::Benchmark,
        _ => GameMode::Unknown,
    }
}

fn parse_ruleset(cursor: &mut Cursor<Bytes>) -> Ruleset {
    match cursor.get_u8() {
        0 => Ruleset::PracticeAndQualifying,
        1 => Ruleset::Race,
        2 => Ruleset::TimeTrial,
        4 => Ruleset::TimeAttack,
        6 => Ruleset::CheckpointChallenge,
        8 => Ruleset::Autocross,
        9 => Ruleset::Drift,
        10 => Ruleset::AverageSpeedZone,
        11 => Ruleset::RivalDuel,
        _ => Ruleset::Unknown,
    }
}

fn parse_session_length(cursor: &mut Cursor<Bytes>) -> SessionLength {
    match cursor.get_u8() {
        0 => SessionLength::None,
        2 => SessionLength::VeryShort,
        3 => SessionLength::Short,
        4 => SessionLength::Medium,
        5 => SessionLength::MediumLong,
        6 => SessionLength::Long,
        7 => SessionLength::Full,
        _ => SessionLength::None,
    }
}
