use serde::{Deserialize, Serialize};

use super::header::Header;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Weather {
    Clear,
    #[serde(rename = "Light Clouds")]
    LightCloud,
    Overcast,
    #[serde(rename = "Light Rain")]
    LightRain,
    #[serde(rename = "Heavy Rain")]
    HeavyRain,
    Storm,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum SessionType {
    Unknown,
    #[serde(rename = "Practice")]
    P1,
    #[serde(rename = "Practice")]
    P2,
    #[serde(rename = "Practice")]
    P3,
    #[serde(rename = "Practice")]
    ShortP,
    #[serde(rename = "Qualifying Q1")]
    Q1,
    #[serde(rename = "Qualifying Q2")]
    Q2,
    #[serde(rename = "Qualifying Q3")]
    Q3,
    #[serde(rename = "Short Qualifying")]
    ShortQ,
    #[serde(rename = "One Shot Qualifying")]
    OSQ,
    #[serde(rename = "Race")]
    R,
    #[serde(rename = "Race 2")]
    R2,
    #[serde(rename = "Race 3")]
    R3,
    #[serde(rename = "Time Trial")]
    TimeTrial,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Track {
    Unknown,
    Melbourne,
    PaulRicard,
    Shanghai,
    Sakhir,
    Catalunya,
    Monaco,
    Montreal,
    Silverstone,
    Hockenheim,
    Hungaroring,
    Spa,
    Monza,
    Singapore,
    Suzuka,
    AbuDhabi,
    Texas,
    Brazil,
    Austria,
    Sochi,
    Mexico,
    Baku,
    SakhirShort,
    SilverstoneShort,
    TexasShort,
    SuzukaShort,
    Hanoi,
    Zandvoort,
    Imola,
    Portimao,
    Jeddah,
    Miami,
    Vegas,
    Losail,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Formula {
    #[serde(rename = "F1 Modern")]
    F1Modern,
    #[serde(rename = "F1 Classic")]
    F1Classic,
    F2,
    #[serde(rename = "F1")]
    F1Generic,
    Beta,
    Supercars,
    Esports,
    #[serde(rename = "F2 2021")]
    F2_2021,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SafetyCarStatus {
    No,
    Full,
    Virtual,
    Formation,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ForecastAccuracy {
    Perfect,
    Approximate,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum BrakingAssist {
    Off,
    Low,
    Medium,
    High,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub enum GearboxAssist {
    Manual,
    #[serde(rename = "Manual + Suggested Gear")]
    ManualSuggested,
    Auto,
    #[default]
    Unknown,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DynamicRacingLine {
    Off,
    Corners,
    Full,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DynamicRacingLineType {
    TwoD,
    ThreeD,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum MarshalFlag {
    Unknown,
    None,
    Green,
    Blue,
    Yellow,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MarshalZone {
    pub zone_start: f32,
    pub zone_flag: MarshalFlag,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TempChange {
    Decrease,
    NoChange,
    Increase,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct WeatherForecastSample {
    pub session_type: SessionType,
    pub time_offset: u8,
    pub weather: Weather,
    pub track_temperature: i8,
    pub track_temperature_change: TempChange,
    pub air_temperature: i8,
    pub air_temperature_change: TempChange,
    pub rain_percentage: u8,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum GameMode {
    Unknown,
    #[serde(rename = "Event Mode")]
    EventMode,
    #[serde(rename = "Grand Prix")]
    GrandPrix,
    #[serde(rename = "Grand Prix (23)")]
    GrandPrix23,
    #[serde(rename = "Time Trial")]
    TimeTrial,
    Splitscreen,
    #[serde(rename = "Online")]
    OnlineCustom,
    #[serde(rename = "Online League")]
    OnlineLeague,
    #[serde(rename = "Career Invitational")]
    CareerInvitational,
    #[serde(rename = "Championship Invitational")]
    ChampionshipInvitational,
    Championship,
    #[serde(rename = "Online Championship")]
    OnlineChampionship,
    #[serde(rename = "Online Weekly Event")]
    OnlineWeeklyEvent,
    StoryMode,
    #[serde(rename = "Career (2022)")]
    Career22,
    #[serde(rename = "Career Online (2022)")]
    Career22Online,
    #[serde(rename = "Career (23)")]
    Career23,
    #[serde(rename = "Career Online (23)")]
    Career23Online,
    Benchmark,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Ruleset {
    Unknown,
    PracticeAndQualifying,
    Race,
    TimeTrial,
    TimeAttack,
    CheckpointChallenge,
    Autocross,
    Drift,
    AverageSpeedZone,
    RivalDuel,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum SessionLength {
    None,
    VeryShort,
    Short,
    Medium,
    MediumLong,
    Long,
    Full,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum SpeedUnit {
    Kmh,
    Mph,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TempUnit {
    Celsius,
    Fahrenheit,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionPacket {
    pub header: Header,
    pub weather: Weather,
    pub track_temperature: i8,
    pub air_temperature: i8,
    pub total_laps: u8,
    pub track_length: u16,
    pub session_type: SessionType,
    pub track: Track,
    pub formula: Formula,
    pub session_time_left: u16,
    pub session_duration: u16,
    pub pit_speed_limit: u8,
    pub game_paused: bool,
    pub is_spectating: bool,
    pub spectator_car_index: u8,
    pub sli_pro_native_support: bool,
    pub marshal_zones: Vec<MarshalZone>,
    pub safety_car_status: SafetyCarStatus,
    pub network_game: bool,
    pub weather_forecast_samples: Vec<WeatherForecastSample>,
    pub forecast_accuracy: ForecastAccuracy,
    pub ai_difficulty: u8,
    pub season_link_identifier: u32,
    pub weekend_link_identifier: u32,
    pub session_link_identifier: u32,
    pub pit_stop_window_ideal_lap: u8,
    pub pit_stop_window_latest_lap: u8,
    pub pit_stop_rejoin_position: u8,
    pub steering_assist: bool,
    pub braking_assist: BrakingAssist,
    pub gearbox_assist: GearboxAssist,
    pub pit_assist: bool,
    pub pit_release_assist: bool,
    pub ers_assist: bool,
    pub drs_assist: bool,
    pub dynamic_racing_line: DynamicRacingLine,
    pub dynamic_racing_line_type: DynamicRacingLineType,
    pub game_mode: GameMode,
    pub ruleset: Ruleset,
    pub time_of_day: u32,
    pub session_length: SessionLength,
    pub speed_units_lead_player: SpeedUnit,
    pub temperature_units_lead_player: TempUnit,
    pub speed_units_secondary_player: SpeedUnit,
    pub temperature_units_secondary_player: TempUnit,
    pub num_safety_car_periods: u8,
    pub num_virtual_safety_car_periods: u8,
    pub num_red_flag_periods: u8,
}
