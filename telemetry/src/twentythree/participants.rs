use std::io::{Cursor, Read};

use bytes::{Buf, Bytes};
use serde::__private::from_utf8_lossy;

use crate::packet::participants::{ParticipantData, ParticipantsPacket, Team};

use super::header::parse_header;

pub fn parse_participants_packet(cursor: &mut Cursor<Bytes>) -> crate::Result<ParticipantsPacket> {
    let header = parse_header(cursor)?;

    let num_active_cars = cursor.get_u8();
    let participants = (0..22).map(|_| parse_participants_data(cursor)).collect();

    Ok(ParticipantsPacket {
        header,
        num_active_cars,
        participants,
    })
}

fn parse_participants_data(cursor: &mut Cursor<Bytes>) -> ParticipantData {
    let ai_controlled = cursor.get_u8() != 0;
    let driver_id = cursor.get_u8();
    let network_id = cursor.get_u8();
    let team_id = match cursor.get_u8() {
        0 | 85 => Team::Mercedes,
        1 | 86 => Team::Ferrari,
        2 | 87 => Team::RedBullRacing,
        3 | 88 => Team::Williams,
        4 => Team::AstonMartin,
        5 => Team::Alpine,
        6 | 91 => Team::AlphaTauri,
        7 | 92 => Team::Haas,
        8 | 93 => Team::McLaren,
        9 | 94 => Team::AlfaRomeo,

        106 => Team::Prema,
        107 => Team::UniVirtuosi,
        108 => Team::Carlin,
        109 => Team::Hitech,
        110 => Team::ArtGP,
        111 => Team::MPMotorsport,
        112 => Team::Charouz,
        113 => Team::Dams,
        114 => Team::Campos,
        115 => Team::BWT,
        116 => Team::Trident,

        95..=96 | 98..=101 | 103 | 117 => Team::Supercar,
        97 | 102 => Team::SafetyCar,
        104 => Team::CustomTeam,

        89 => Team::RacingPoint,
        90 => Team::Renault,
        _ => Team::Unknown,
    };
    let my_team = cursor.get_u8() != 0;
    let race_number = cursor.get_u8();
    let nationality = match cursor.get_u8() {
        1 => Some(celes::Country::the_united_states_of_america()),
        2 => Some(celes::Country::argentina()),
        3 => Some(celes::Country::australia()),
        4 => Some(celes::Country::austria()),
        5 => Some(celes::Country::azerbaijan()),
        6 => Some(celes::Country::bahrain()),
        7 => Some(celes::Country::belgium()),
        8 => Some(celes::Country::bolivia()),
        9 => Some(celes::Country::brazil()),
        10 => Some(celes::Country::the_united_kingdom_of_great_britain_and_northern_ireland()),
        11 => Some(celes::Country::bulgaria()),
        12 => Some(celes::Country::cameroon()),
        13 => Some(celes::Country::canada()),
        14 => Some(celes::Country::chile()),
        15 => Some(celes::Country::china()),
        16 => Some(celes::Country::colombia()),
        17 => Some(celes::Country::costa_rica()),
        18 => Some(celes::Country::croatia()),
        19 => Some(celes::Country::cyprus()),
        20 => Some(celes::Country::czechia()),
        21 => Some(celes::Country::denmark()),
        22 => Some(celes::Country::the_netherlands()),
        23 => Some(celes::Country::ecuador()),
        24 => Some(celes::Country::the_united_kingdom_of_great_britain_and_northern_ireland()),
        25 => Some(celes::Country::the_united_arab_emirates()),
        26 => Some(celes::Country::estonia()),
        27 => Some(celes::Country::finland()),
        28 => Some(celes::Country::france()),
        29 => Some(celes::Country::germany()),
        30 => Some(celes::Country::ghana()),
        31 => Some(celes::Country::greece()),
        32 => Some(celes::Country::guatemala()),
        33 => Some(celes::Country::honduras()),
        34 => Some(celes::Country::hong_kong()),
        35 => Some(celes::Country::hungary()),
        36 => Some(celes::Country::iceland()),
        37 => Some(celes::Country::india()),
        38 => Some(celes::Country::indonesia()),
        39 => Some(celes::Country::ireland()),
        40 => Some(celes::Country::israel()),
        41 => Some(celes::Country::italy()),
        42 => Some(celes::Country::jamaica()),
        43 => Some(celes::Country::japan()),
        44 => Some(celes::Country::jordan()),
        45 => Some(celes::Country::kuwait()),
        46 => Some(celes::Country::latvia()),
        47 => Some(celes::Country::lebanon()),
        48 => Some(celes::Country::lithuania()),
        49 => Some(celes::Country::luxembourg()),
        50 => Some(celes::Country::malaysia()),
        51 => Some(celes::Country::malta()),
        52 => Some(celes::Country::mexico()),
        53 => Some(celes::Country::monaco()),
        54 => Some(celes::Country::new_zealand()),
        55 => Some(celes::Country::nicaragua()),
        56 => Some(celes::Country::the_united_kingdom_of_great_britain_and_northern_ireland()),
        57 => Some(celes::Country::norway()),
        58 => Some(celes::Country::oman()),
        59 => Some(celes::Country::pakistan()),
        60 => Some(celes::Country::panama()),
        61 => Some(celes::Country::paraguay()),
        62 => Some(celes::Country::peru()),
        63 => Some(celes::Country::poland()),
        64 => Some(celes::Country::portugal()),
        65 => Some(celes::Country::qatar()),
        66 => Some(celes::Country::romania()),
        67 => Some(celes::Country::the_russian_federation()),
        68 => Some(celes::Country::el_salvador()),
        69 => Some(celes::Country::saudi_arabia()),
        70 => Some(celes::Country::the_united_kingdom_of_great_britain_and_northern_ireland()),
        71 => Some(celes::Country::serbia()),
        72 => Some(celes::Country::singapore()),
        73 => Some(celes::Country::slovakia()),
        74 => Some(celes::Country::slovenia()),
        75 => Some(celes::Country::the_republic_of_korea()),
        76 => Some(celes::Country::south_africa()),
        77 => Some(celes::Country::spain()),
        78 => Some(celes::Country::sweden()),
        79 => Some(celes::Country::switzerland()),
        80 => Some(celes::Country::thailand()),
        81 => Some(celes::Country::turkey()),
        82 => Some(celes::Country::uruguay()),
        83 => Some(celes::Country::ukraine()),
        84 => Some(celes::Country::bolivarian_republic_of_venezuela()),
        85 => Some(celes::Country::barbados()),
        86 => Some(celes::Country::the_united_kingdom_of_great_britain_and_northern_ireland()),
        87 => Some(celes::Country::vietnam()),
        _ => None,
    };
    let mut name = [0_u8; 48];
    let _ = cursor.read_exact(&mut name); // Shouldn't error if the packet is not malformed
    let name_end = name.iter().position(|&c| c == 0).unwrap_or(48);
    let name = from_utf8_lossy(&name[0..name_end]).to_string();
    let your_telemetry = cursor.get_u8() != 0;

    ParticipantData {
        ai_controlled,
        driver_id,
        network_id,
        team: team_id,
        my_team,
        race_number,
        nationality,
        name,
        your_telemetry,
    }
}

pub fn parse_team(cursor: &mut Cursor<Bytes>) -> Team {
    match cursor.get_u8() {
        0 | 85 => Team::Mercedes,
        1 | 86 => Team::Ferrari,
        2 | 87 => Team::RedBullRacing,
        3 | 88 => Team::Williams,
        4 => Team::AstonMartin,
        5 => Team::Alpine,
        6 | 91 => Team::AlphaTauri,
        7 | 92 => Team::Haas,
        8 | 93 => Team::McLaren,
        9 | 94 => Team::AlfaRomeo,

        106 => Team::Prema,
        107 => Team::UniVirtuosi,
        108 => Team::Carlin,
        109 => Team::Hitech,
        110 => Team::ArtGP,
        111 => Team::MPMotorsport,
        112 => Team::Charouz,
        113 => Team::Dams,
        114 => Team::Campos,
        115 => Team::BWT,
        116 => Team::Trident,

        95..=96 | 98..=101 | 103 | 117 => Team::Supercar,
        97 | 102 => Team::SafetyCar,
        104 => Team::CustomTeam,

        89 => Team::RacingPoint,
        90 => Team::Renault,
        _ => Team::Unknown,
    }
}
