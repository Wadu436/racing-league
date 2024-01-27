use std::{
    io::{Read, Write},
    net::UdpSocket,
    path::Path,
    time::{Duration, Instant},
    vec,
};

use bytes::{Buf, Bytes};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use telemetry::{
    decode_header, decode_packet,
    packet::{
        event::Event, final_classification::TyreStint, header::PacketId, lap_data::ResultStatus,
        participants::Team, Packet,
    },
};
use tracing::{debug, level_filters::LevelFilter, warn};
use tracing_subscriber::FmtSubscriber;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RaceData {
    participants: Vec<RaceParticipant>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum RaceParticipantStatus {
    Finished,
    DNF,
    DSQ,
    DNS,
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RaceParticipant {
    id: u64,
    grid_position: u8,
    position: u8,
    num_laps: u8,
    status: RaceParticipantStatus,
    team: Team,
    race_number: u8,
    player: PlayerData,
    num_pitstops: u8,
    tyre_stints: Vec<TyreStint>,
    fastest_lap: f32,
    total_time_without_penalties: f32,
    penalty_time_in_s: u8,
    laps: Vec<RaceLap>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RaceLap {
    lap_number: u8,
    lap_time: f32,
    sector_1_time: f32,
    sector_2_time: f32,
    sector_3_time: f32,
    lap_valid: bool,
    position: u8, // Position at the end of the lap
    safety_car: bool,
    virtual_safety_car: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PlayerData {
    name: String,
    nationality: Option<celes::Country>,
}

// A packet that can be/was written to disk
#[derive(Debug, Serialize, Deserialize)]
struct DiskPacket {
    time: Duration,
    packet: Packet,
}

pub fn initialize(level: impl Into<LevelFilter>) -> eyre::Result<()> {
    color_eyre::install()?;

    FmtSubscriber::builder().with_max_level(level).init();

    Ok(())
}

pub fn record<P: AsRef<Path>>(file_path: P, address: &str) -> eyre::Result<()> {
    // open file
    let socket = UdpSocket::bind(address)?;
    debug!(address, "Opened socket");

    let mut file = std::fs::File::create(file_path.as_ref())?;
    debug!(
        path = file_path.as_ref().to_string_lossy().as_ref(),
        "Opened file"
    );

    let mut buf = [0; 2048]; // needs to be at least 1464, but we'll go for a nice, round, power of 2 instead

    let start_time = Instant::now();

    println!(
        "Listening on {}\nRecording to {}",
        address,
        file_path.as_ref().to_string_lossy()
    );

    while let Ok((size, _)) = socket.recv_from(&mut buf) {
        let b = Bytes::copy_from_slice(&buf[..size]);
        // let b = Bytes::copy_from_slice(&buf);
        let packet = decode_packet(b.clone());
        match packet {
            Ok(packet) => {
                debug!(
                    size,
                    packet_id = packet.header().packet_id.to_string(),
                    "Received packet"
                );
            }
            Err(e) => {
                warn!(
                    "Could not parse packet. (packet will still be saved to disk): {}",
                    e
                );
            }
        }
        file.write_all(&(size as u64).to_be_bytes())?;
        file.write_all(&(Instant::now() - start_time).as_secs_f64().to_be_bytes())?;
        file.write_all(&b)?;
        file.flush()?;
    }

    Ok(())
}

pub fn parse<P: AsRef<Path>>(
    file: P,
    out: Option<P>,
    filter: Option<Vec<PacketId>>,
    limit: Option<usize>,
) -> Result<(), eyre::Error> {
    let mut file = std::fs::File::open(file)?;
    let metadata = file.metadata()?;
    let mut buf = vec![0_u8; metadata.len() as usize];
    let mut packets: Vec<DiskPacket> = Vec::new();
    file.read_exact(&mut buf)?;
    let mut bytes = Bytes::copy_from_slice(&buf);
    while bytes.has_remaining() {
        let size = bytes.get_u64() as usize;
        let time = bytes.get_f64();
        let time = Duration::from_secs_f64(time);
        let packet_bytes = bytes.copy_to_bytes(size);

        if let Some(true) = limit.and_then(|l| Some(packets.len() > l)) {
            break;
        }

        // Only decode the packet if it is in our filter list
        match decode_header(packet_bytes.clone()) {
            Ok(header) => {
                if let Some(filter) = &filter {
                    if !filter.contains(&header.packet_id) {
                        continue;
                    }
                }
                match decode_packet(packet_bytes) {
                    Ok(packet) => {
                        if let Packet::Event(event_packet) = packet {
                            if let Event::Button { .. } = event_packet.event {
                                continue;
                            }
                        };

                        packets.push(DiskPacket { time, packet });
                    }
                    Err(e) => {
                        warn!("Could not parse packet: {}", e);
                    }
                }
            }
            Err(e) => {
                warn!("Could not parse packet: {}", e);
            }
        }
    }

    let packets_json = serde_json::to_string_pretty(&packets)?;

    if let Some(out) = out {
        std::fs::write(&out, packets_json)?;
        println!("Wrote packets to {:?}", out.as_ref());
    } else {
        println!("{}", packets_json);
    }
    Ok(())
}

pub fn race<P: AsRef<Path>>(file: P, out: Option<P>) -> Result<(), eyre::Error> {
    let mut file = std::fs::File::open(file)?;
    let metadata = file.metadata()?;
    let mut buf = vec![0_u8; metadata.len() as usize];
    let mut packets: Vec<DiskPacket> = Vec::new();
    file.read_exact(&mut buf)?;
    let mut bytes = Bytes::copy_from_slice(&buf);
    while bytes.has_remaining() {
        let size = bytes.get_u64() as usize;
        let time = bytes.get_f64();
        let time = Duration::from_secs_f64(time);
        let packet_bytes = bytes.copy_to_bytes(size);
        match decode_packet(packet_bytes) {
            Ok(packet) => {
                if ![
                    PacketId::FinalClassification,
                    PacketId::Participants,
                    PacketId::LapData,
                ]
                .contains(&packet.header().packet_id)
                {
                    continue;
                }
                packets.push(DiskPacket { time, packet });
            }
            Err(e) => {
                warn!("Could not parse packet: {}", e);
            }
        }
    }

    // Find the last participants packet
    let participants_packet = packets
        .iter()
        .filter_map(|p| {
            if let Packet::Participants(packet) = &p.packet {
                Some(packet)
            } else {
                None
            }
        })
        .last()
        .unwrap();

    let lap_data = {
        #[derive(Copy, Clone)]
        struct TempLap {
            lap_number: u8,
            sector_1_time: Duration,
            sector_2_time: Duration,
            lap_valid: bool,
            safety_car: bool,
            virtual_safety_car: bool,
        }

        let mut laps: [Vec<RaceLap>; 22] = Default::default();
        let mut current_lap_data: [TempLap; 22] = [TempLap {
            lap_number: 1,
            sector_1_time: Duration::default(),
            sector_2_time: Duration::default(),
            lap_valid: true,
            safety_car: false,
            virtual_safety_car: false,
        }; 22];

        packets
            .iter()
            .filter_map(|p| {
                if let Packet::LapData(packet) = &p.packet {
                    Some(packet)
                } else {
                    None
                }
            })
            .flat_map(|packet| packet.lap_data.iter().enumerate())
            .for_each(|(idx, lap_data)| {
                if let Some(lap_data) = lap_data {
                    if lap_data.current_lap_num > current_lap_data[idx].lap_number {
                        // Just crossed the line
                        laps[idx].push(RaceLap {
                            lap_number: current_lap_data[idx].lap_number,
                            lap_time: lap_data.last_lap_time.as_secs_f32(),
                            sector_1_time: current_lap_data[idx].sector_1_time.as_secs_f32(),
                            sector_2_time: current_lap_data[idx].sector_2_time.as_secs_f32(),
                            sector_3_time: (lap_data.last_lap_time.as_secs_f32()
                                - current_lap_data[idx].sector_2_time.as_secs_f32()
                                - current_lap_data[idx].sector_1_time.as_secs_f32()),

                            lap_valid: current_lap_data[idx].lap_valid,
                            position: lap_data.car_position,
                            safety_car: current_lap_data[idx].safety_car,
                            virtual_safety_car: current_lap_data[idx].virtual_safety_car,
                        });
                    }
                    // TODO: handle SC/VSC
                    current_lap_data[idx] = TempLap {
                        lap_number: lap_data.current_lap_num,
                        sector_1_time: lap_data.sector_1_time,
                        sector_2_time: lap_data.sector_2_time,
                        lap_valid: !lap_data.current_lap_invalid,
                        safety_car: false,
                        virtual_safety_car: false,
                    };
                }
            });

        laps
    };

    let final_classification = packets
        .iter()
        .find_map(|p| match &p.packet {
            Packet::FinalClassification(packet) => Some(packet),
            _ => None,
        })
        .unwrap();

    let race_participants = {
        let mut race_participants = final_classification
            .classification_data
            .iter()
            .enumerate()
            .map(|(index, data)| {
                let participant_data = &participants_packet.participants[index];

                RaceParticipant {
                    id: index as u64,
                    num_laps: data.num_laps,
                    grid_position: data.grid_position,
                    position: data.position,
                    num_pitstops: data.num_pit_stops,
                    status: match data.status {
                        ResultStatus::Finished => RaceParticipantStatus::Finished,
                        ResultStatus::Disqualified => RaceParticipantStatus::DSQ,
                        ResultStatus::DidNotFinish => RaceParticipantStatus::DNF,
                        ResultStatus::Retired => RaceParticipantStatus::DNF,
                        ResultStatus::NotClassified => RaceParticipantStatus::DNF,
                        _ => RaceParticipantStatus::Unknown,
                    },
                    race_number: participant_data.race_number,
                    team: participant_data.team.clone(),
                    player: PlayerData {
                        name: participant_data.name.clone(),
                        nationality: participant_data.nationality,
                    },
                    tyre_stints: data.tyre_stints.clone(),
                    fastest_lap: data.best_laptime.as_secs_f32(),
                    penalty_time_in_s: data.penalty_time_in_seconds,
                    total_time_without_penalties: data
                        .total_race_time_without_penalties
                        .as_secs_f32(),
                    laps: lap_data[index].clone(),
                }
            })
            .collect_vec();
        race_participants.sort_by_key(|rp| rp.position);
        race_participants
    };

    let race_data = RaceData {
        participants: race_participants,
    };

    let race_data_json = serde_json::to_string_pretty(&race_data)?;

    if let Some(out) = out {
        std::fs::write(&out, race_data_json)?;
        println!("Wrote race data to {:?}", out.as_ref());
    } else {
        println!("{}", race_data_json);
    }

    Ok(())
}
