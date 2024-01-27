use serde::{Deserialize, Serialize};

use super::header::Header;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Team {
    Unknown,
    Mercedes,
    Ferrari,
    RedBullRacing,
    Williams,
    AstonMartin,
    Alpine,
    AlphaTauri,
    Haas,
    McLaren,
    AlfaRomeo,
    RacingPoint,
    Renault,
    Konnersport,
    ArtGP,
    Campos,
    Carlin,
    SauberJuniorCharouz,
    Dams,
    UniVirtuosi,
    Virtuosi,
    MPMotorsport,
    Prema,
    Trident,
    Arden,
    Charouz,
    BWT,
    Hitech,
    VanAmersfoortRacing,
    Supercar,
    SafetyCar,
    CustomTeam,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Platform {
    Steam,
    PlayStation,
    Xbox,
    Origin,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Telemetry {
    Restricted, Public
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParticipantData {
    pub ai_controlled: bool,
    pub driver_id: u8,
    pub network_id: u8,
    pub team: Team,
    pub my_team: bool,
    pub race_number: u8,
    pub nationality: Option<celes::Country>,
    pub name: String,
    pub your_telemetry: Telemetry,
    pub show_online_names: bool,
    pub platform: Option<Platform>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParticipantsPacket {
    pub header: Header,
    pub num_active_cars: u8,
    pub participants: Vec<ParticipantData>,
}
