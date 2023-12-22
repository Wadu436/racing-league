use serde::{Deserialize, Serialize};

use super::header::Header;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Team {
    Unknown,
    Mercedes,
    Ferrari,
    #[serde(rename = "Red Bull Racing")]
    RedBullRacing,
    Williams,
    #[serde(rename = "Aston Martin")]
    AstonMartin,
    Alpine,
    #[serde(rename = "Alpha Tauri")]
    AlphaTauri,
    Haas,
    McLaren,
    #[serde(rename = "Alfa Romeo")]
    AlfaRomeo,
    #[serde(rename = "Racing Point")]
    RacingPoint,
    Renault,
    #[serde(rename = "Art GP")]
    ArtGP,
    Campos,
    Carlin,
    #[serde(rename = "Sauber Junior Charouz")]
    SauberJuniorCharouz,
    Dams,
    #[serde(rename = "Uni-Virtuosi")]
    UniVirtuosi,
    #[serde(rename = "MP Motorsport")]
    MPMotorsport,
    Prema,
    Trident,
    Arden,
    Charouz,
    BWT,
    Hitech,
    Supercar,
    SafetyCar,
    CustomTeam,
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
    pub your_telemetry: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParticipantsPacket {
    pub header: Header,
    pub num_active_cars: u8,
    pub participants: Vec<ParticipantData>,
}
