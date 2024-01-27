use std::{
    collections::HashMap,
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
        event::{Event, Penalty},
        final_classification::TyreStint,
        header::PacketId,
        lap_data::{PitStatus, ResultStatus},
        participants::Team,
        session::{SafetyCarStatus, SessionLength, SessionType, Track},
        session_history::{LapHistoryData, SessionHistoryPacket},
        Packet,
    },
};
use tracing::{debug, level_filters::LevelFilter, warn};
use tracing_subscriber::FmtSubscriber;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ParsedSessions {
    sessions: Vec<ParsedSessionData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ParsedSessionData {
    session_id: u64,
    session_link_id: u32,
    session_type: SessionType,
    track: Track,
    participants: Vec<SessionParticipant>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
enum SessionParticipantStatus {
    Finished,
    DNF,
    DSQ,
    DNS,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct SessionParticipant {
    id: u64,
    ai_controlled: bool,
    grid_position: u8,
    position: u8,
    num_laps: u8,
    status: SessionParticipantStatus,
    team: Team,
    race_number: u8,
    player: PlayerData,
    num_pitstops: u8,
    tyre_stints: Vec<TyreStint>,
    fastest_lap: u64,
    total_time_without_penalties: u64,
    penalty_time_in_s: u8,
    laps: Vec<LapData>,
    #[serde(skip)]
    session_history: Vec<LapHistoryData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LapData {
    lap_number: u8,
    lap_time: u64,
    sector_1_time: u64,
    sector_2_time: u64,
    sector_3_time: u64,
    lap_valid: bool,
    position: u8, // Position at the end of the lap
    safety_car: bool,
    virtual_safety_car: bool,
    formation: bool,
    in_lap: bool,
    out_lap: bool,
    infringements: Vec<Penalty>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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
    session_id: Option<u64>,
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
                if let Some(session_id) = session_id {
                    if header.session_uid != session_id {
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

struct SessionState {
    packets: Vec<DiskPacket>,
}

impl SessionState {
    fn new() -> Self {
        Self {
            packets: Vec::new(),
        }
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}

// pub fn race<P: AsRef<Path>>(file: P, out: Option<P>) -> Result<(), eyre::Error> {
//     let mut file = std::fs::File::open(file)?;
//     let metadata = file.metadata()?;
//     let mut buf = vec![0_u8; metadata.len() as usize];
//     file.read_exact(&mut buf)?;
//     let mut bytes = Bytes::copy_from_slice(&buf);

//     let mut sessions: HashMap<u64, SessionState> = HashMap::new();

//     while bytes.has_remaining() {
//         let size = bytes.get_u64() as usize;
//         let time = bytes.get_f64();
//         let time = Duration::from_secs_f64(time);
//         let packet_bytes = bytes.copy_to_bytes(size);
//         match decode_header(packet_bytes.clone()) {
//             Ok(header) => {
//                 if ![
//                     PacketId::FinalClassification,
//                     PacketId::Participants,
//                     PacketId::LapData,
//                     PacketId::Session,
//                 ]
//                 .contains(&header.packet_id)
//                 {
//                     continue;
//                 }
//                 match decode_packet(packet_bytes.clone()) {
//                     Ok(packet) => {
//                         let session = sessions
//                             .entry(packet.header().session_uid)
//                             .or_insert(SessionState::default());
//                         session.packets.push(DiskPacket { time, packet });
//                     }
//                     Err(e) => {
//                         warn!("Could not parse packet: {}", e);
//                     }
//                 }
//             }
//             Err(e) => {
//                 warn!("Could not parse header: {}", e);
//             }
//         }
//     }

//     let mut parsed_sessions = ParsedSessions {
//         sessions: Vec::new(),
//     };

//     for (session_id, session) in sessions {
//         println!("Parsing session {}", session_id);
//         // Find the last participants packet
//         let participants_packet = session
//             .packets
//             .iter()
//             .filter_map(|p| {
//                 if let Packet::Participants(packet) = &p.packet {
//                     Some(packet)
//                 } else {
//                     None
//                 }
//             })
//             .last()
//             .unwrap();

//         let (session_type, track, session_link_id) = session
//             .packets
//             .iter()
//             .find_map(|p| {
//                 if let Packet::Session(packet) = &p.packet {
//                     Some(packet)
//                 } else {
//                     None
//                 }
//             })
//             .map(|s| (s.session_type, s.track, s.session_link_identifier))
//             .unwrap_or((SessionType::Unknown, Track::Unknown, 0));

//         let lap_data = {
//             #[derive(Copy, Clone)]
//             struct TempLap {
//                 lap_number: u8,
//                 sector_1_time: Duration,
//                 sector_2_time: Duration,
//                 lap_valid: bool,
//                 safety_car: bool,
//                 virtual_safety_car: bool,
//             }

//             let mut laps: [Vec<LapData>; 22] = Default::default();
//             let mut current_lap_data: [TempLap; 22] = [TempLap {
//                 lap_number: 1,
//                 sector_1_time: Duration::default(),
//                 sector_2_time: Duration::default(),
//                 lap_valid: true,
//                 safety_car: false,
//                 virtual_safety_car: false,
//             }; 22];

//             session
//                 .packets
//                 .iter()
//                 .filter_map(|p| {
//                     if let Packet::LapData(packet) = &p.packet {
//                         Some(packet)
//                     } else {
//                         None
//                     }
//                 })
//                 .flat_map(|packet| packet.lap_data.iter().enumerate())
//                 .for_each(|(idx, lap_data)| {
//                     if let Some(lap_data) = lap_data {
//                         if lap_data.current_lap_num > current_lap_data[idx].lap_number {
//                             // Just crossed the line
//                             let lap_time = lap_data.last_lap_time.as_millis() as u64;
//                             let sector_1_time =
//                                 current_lap_data[idx].sector_1_time.as_millis() as u64;
//                             let sector_2_time =
//                                 current_lap_data[idx].sector_2_time.as_millis() as u64;

//                             laps[idx].push(LapData {
//                                 lap_number: current_lap_data[idx].lap_number,
//                                 lap_time,
//                                 sector_1_time,
//                                 sector_2_time,
//                                 sector_3_time: (lap_time - sector_1_time - sector_2_time),

//                                 lap_valid: current_lap_data[idx].lap_valid,
//                                 position: lap_data.car_position,
//                                 safety_car: current_lap_data[idx].safety_car,
//                                 virtual_safety_car: current_lap_data[idx].virtual_safety_car,
//                             });
//                         }
//                         // TODO: handle SC/VSC
//                         current_lap_data[idx] = TempLap {
//                             lap_number: lap_data.current_lap_num,
//                             sector_1_time: lap_data.sector_1_time,
//                             sector_2_time: lap_data.sector_2_time,
//                             lap_valid: !lap_data.current_lap_invalid,
//                             safety_car: false,
//                             virtual_safety_car: false,
//                         };
//                     }
//                 });

//             laps
//         };

//         let final_classification = session
//             .packets
//             .iter()
//             .find_map(|p| match &p.packet {
//                 Packet::FinalClassification(packet) => Some(packet),
//                 _ => None,
//             })
//             .unwrap();

//         let race_participants = {
//             let mut race_participants = final_classification
//                 .classification_data
//                 .iter()
//                 .enumerate()
//                 .map(|(index, data)| {
//                     let participant_data = &participants_packet.participants[index];

//                     SessionParticipant {
//                         id: index as u64,
//                         num_laps: data.num_laps,
//                         grid_position: data.grid_position,
//                         position: data.position,
//                         num_pitstops: data.num_pit_stops,
//                         status: match data.status {
//                             ResultStatus::Finished => SessionParticipantStatus::Finished,
//                             ResultStatus::Disqualified => SessionParticipantStatus::DSQ,
//                             ResultStatus::DidNotFinish => SessionParticipantStatus::DNF,
//                             ResultStatus::Retired => SessionParticipantStatus::DNF,
//                             ResultStatus::NotClassified => SessionParticipantStatus::DNF,
//                             _ => SessionParticipantStatus::Unknown,
//                         },
//                         race_number: participant_data.race_number,
//                         team: participant_data.team.clone(),
//                         player: PlayerData {
//                             name: participant_data.name.clone(),
//                             nationality: participant_data.nationality,
//                         },
//                         tyre_stints: data.tyre_stints.clone(),
//                         fastest_lap: data.best_laptime.as_millis() as u64,
//                         penalty_time_in_s: data.penalty_time_in_seconds,
//                         total_time_without_penalties: data
//                             .total_race_time_without_penalties
//                             .as_millis()
//                             as u64,
//                         laps: lap_data[index].clone(),
//                     }
//                 })
//                 .collect_vec();
//             race_participants.sort_by_key(|rp| rp.position);
//             race_participants
//         };

//         let session_data = ParsedSessionData {
//             session_id,
//             session_link_id,
//             session_type,
//             track,
//             participants: race_participants,
//         };

//         parsed_sessions.sessions.push(session_data);
//     }

//     let race_data_json = serde_json::to_string_pretty(&parsed_sessions)?;

//     if let Some(out) = out {
//         std::fs::write(&out, race_data_json)?;
//         println!("Wrote race data to {:?}", out.as_ref());
//     } else {
//         println!("{}", race_data_json);
//     }

//     Ok(())
// }

pub fn race2<P: AsRef<Path>, P2: AsRef<Path>>(file: P, out: Option<P2>) -> Result<(), eyre::Error> {
    let mut file = std::fs::File::open(file)?;
    let metadata = file.metadata()?;
    let mut buf = vec![0_u8; metadata.len() as usize];
    file.read_exact(&mut buf)?;
    let mut bytes = Bytes::copy_from_slice(&buf);

    let mut sessions: HashMap<u64, SessionState> = HashMap::new();

    while bytes.has_remaining() {
        let size = bytes.get_u64() as usize;
        let time = bytes.get_f64();
        let time = Duration::from_secs_f64(time);
        let packet_bytes = bytes.copy_to_bytes(size);
        match decode_header(packet_bytes.clone()) {
            Ok(header) => {
                if ![
                    PacketId::FinalClassification,
                    PacketId::Participants,
                    PacketId::LapData,
                    PacketId::Session,
                    PacketId::SessionHistory,
                    PacketId::Event,
                ]
                .contains(&header.packet_id)
                    || header.session_uid == 0
                {
                    continue;
                }
                match decode_packet(packet_bytes.clone()) {
                    Ok(packet) => {
                        if let Packet::Event(event_packet) = packet {
                            if let Event::Button { .. } = event_packet.event {
                                continue;
                            }
                        };

                        let session = sessions
                            .entry(packet.header().session_uid)
                            .or_insert(SessionState::default());
                        session.packets.push(DiskPacket { time, packet });
                    }
                    Err(e) => {
                        warn!("Could not parse packet: {}", e);
                    }
                }
            }
            Err(e) => {
                warn!("Could not parse header: {}", e);
            }
        }
    }

    let parsed_sessions = ParsedSessions {
        sessions: sessions
            .into_iter()
            .map(|(session_id, session)| {
                #[derive(Clone, Debug, Default)]
                struct CurrentLapData {
                    lap_num: u8,
                    sector_1_time: Duration,
                    sector_2_time: Duration,
                    safety_car: bool,
                    virtual_safety_car: bool,
                    formation: bool,
                    in_lap: bool,
                    out_lap: bool,
                    lap_valid: bool,
                    infringements: Vec<Penalty>,
                }

                #[derive(Clone, Debug, Default)]
                struct DriverState {
                    valid: bool,
                    participant: SessionParticipant,
                    session_history: Option<SessionHistoryPacket>,
                    current_lap: Option<CurrentLapData>,
                    pitting: bool,
                }

                struct State {
                    session_type: SessionType,
                    track: Track,
                    length: SessionLength,
                    session_link_id: u32,
                    drivers: [DriverState; 22],
                    safety_car_status: SafetyCarStatus,
                }

                let mut state = State {
                    session_type: SessionType::Unknown,
                    session_link_id: 0,
                    track: Track::Unknown,
                    length: SessionLength::None,
                    drivers: Default::default(),
                    safety_car_status: SafetyCarStatus::No,
                };

                state
                    .drivers
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, driver)| driver.participant.id = i as u64);

                session.packets.iter().for_each(|p| match &p.packet {
                    Packet::Session(s) => {
                        state.session_type = s.session_type;
                        state.track = s.track;
                        state.length = s.session_length;
                        state.session_link_id = s.session_link_identifier;
                        state.safety_car_status = s.safety_car_status;
                    }
                    Packet::LapData(ld) => {
                        for (i, ld) in ld
                            .lap_data
                            .iter()
                            .enumerate()
                            .filter_map(|(i, ld)| ld.map(|ld| (i, ld)))
                        {
                            let driver_entry = &mut state.drivers[i];
                            let current_lap = if let Some(mut current_lap) =
                                driver_entry.current_lap.take()
                            {
                                if current_lap.lap_num < ld.current_lap_num {
                                    // save current lap and start a new one
                                    // TODO: inlap/outlap
                                    let lap_time = ld.last_lap_time.as_millis() as u64;
                                    let sector_1_time =
                                        current_lap.sector_1_time.as_millis() as u64;
                                    let sector_2_time =
                                        current_lap.sector_2_time.as_millis() as u64;
                                    let sector_3_time = lap_time
                                        .saturating_sub(sector_1_time)
                                        .saturating_sub(sector_2_time);

                                    let lap = LapData {
                                        lap_number: current_lap.lap_num,
                                        lap_time,
                                        sector_1_time,
                                        sector_2_time,
                                        sector_3_time,
                                        lap_valid: current_lap.lap_valid,
                                        position: ld.car_position,
                                        safety_car: current_lap.safety_car,
                                        virtual_safety_car: current_lap.virtual_safety_car,
                                        formation: current_lap.formation,
                                        in_lap: current_lap.in_lap,
                                        out_lap: current_lap.out_lap,
                                        infringements: current_lap.infringements,
                                    };

                                    driver_entry.participant.laps.push(lap);

                                    // Start new lap
                                    Some(CurrentLapData {
                                        lap_num: ld.current_lap_num,
                                        safety_car: state.safety_car_status
                                            == SafetyCarStatus::Full,
                                        virtual_safety_car: state.safety_car_status
                                            == SafetyCarStatus::Virtual,
                                        formation: state.safety_car_status
                                            == SafetyCarStatus::Formation,
                                        sector_1_time: ld.sector_1_time,
                                        sector_2_time: ld.sector_2_time,
                                        lap_valid: !ld.current_lap_invalid,
                                        in_lap: false,
                                        out_lap: current_lap.in_lap,
                                        infringements: Vec::new(),
                                    })
                                } else if current_lap.lap_num == ld.current_lap_num {
                                    // update current lap
                                    current_lap.formation = current_lap.formation
                                        || state.safety_car_status == SafetyCarStatus::Formation;
                                    current_lap.safety_car = current_lap.safety_car
                                        || state.safety_car_status == SafetyCarStatus::Full;
                                    current_lap.virtual_safety_car = current_lap.virtual_safety_car
                                        || state.safety_car_status == SafetyCarStatus::Virtual;
                                    current_lap.sector_1_time = ld.sector_1_time;
                                    current_lap.sector_2_time = ld.sector_2_time;
                                    current_lap.lap_valid = !ld.current_lap_invalid;

                                    if !driver_entry.pitting && ld.pit_status == PitStatus::Pitting
                                    {
                                        current_lap.in_lap = true;
                                        driver_entry.pitting = true;
                                    } else if driver_entry.pitting
                                        && ld.pit_status == PitStatus::None
                                    {
                                        driver_entry.pitting = false;
                                    }

                                    Some(current_lap)
                                } else {
                                    // println!("What the fuck is this packet ??");
                                    // println!("{:?}", ld);
                                    Some(current_lap)
                                }
                            } else {
                                Some(CurrentLapData {
                                    lap_num: ld.current_lap_num,
                                    safety_car: state.safety_car_status == SafetyCarStatus::Full,
                                    virtual_safety_car: state.safety_car_status
                                        == SafetyCarStatus::Virtual,
                                    formation: state.safety_car_status
                                        == SafetyCarStatus::Formation,
                                    sector_1_time: ld.sector_1_time,
                                    sector_2_time: ld.sector_2_time,
                                    lap_valid: true,
                                    in_lap: false,
                                    out_lap: false,
                                    infringements: Vec::new(),
                                })
                            };
                            driver_entry.current_lap = current_lap;
                        }
                    }
                    Packet::Event(e) => match e.event {
                        Event::PenaltyIssued(p) => {
                            // println!("penalty issued: {:?}", p);
                            let driver_entry = &mut state.drivers[p.vehicle_idx as usize];
                            if let Some(current_lap) = driver_entry.current_lap.as_mut() {
                                current_lap.infringements.push(p);
                                // println!("infringements: {:?}", current_lap.infringements);
                            }
                        }
                        _ => {}
                    },
                    Packet::Participants(p) => {
                        for (i, p) in p.participants.iter().enumerate() {
                            let driver_entry = &mut state.drivers[i].participant;
                            driver_entry.ai_controlled = p.driver_id != 255;
                            driver_entry.player.name = p.name.clone();
                            if p.name == "HULKENBERG" {
                                println!("Hello, HULKENBERG! Session ID: {}", session_id);
                            
                            }
                            
                            driver_entry.player.nationality = p.nationality;
                            driver_entry.race_number = p.race_number;
                            driver_entry.team = p.team;
                        }
                    }
                    Packet::FinalClassification(fc) => {
                        for (i, cd) in fc
                            .classification_data
                            .iter()
                            .enumerate()
                            .filter_map(|(i, cd)| cd.as_ref().map(|cd| (i, cd)))
                        {
                            println!("{}: {:?}", i, cd);
                            let driver_entry = &mut state.drivers[i].participant;
                            driver_entry.grid_position = cd.grid_position;
                            driver_entry.position = cd.position;
                            driver_entry.fastest_lap = cd.best_laptime.as_millis() as u64;
                            driver_entry.num_laps = cd.num_laps;
                            driver_entry.num_pitstops = cd.num_pit_stops;
                            driver_entry.penalty_time_in_s = cd.penalty_time_in_seconds;
                            driver_entry.status = match cd.status {
                                ResultStatus::Finished => SessionParticipantStatus::Finished,
                                ResultStatus::Disqualified => SessionParticipantStatus::DSQ,
                                ResultStatus::DidNotFinish => SessionParticipantStatus::DNF,
                                ResultStatus::Retired => SessionParticipantStatus::DNF,
                                ResultStatus::NotClassified => SessionParticipantStatus::DNF,
                                _ => SessionParticipantStatus::Unknown,
                            };
                            driver_entry.total_time_without_penalties =
                                cd.total_race_time_without_penalties.as_millis() as u64;
                            driver_entry.tyre_stints = cd.tyre_stints.clone();
                            state.drivers[i].valid = true;
                        }
                    }
                    Packet::SessionHistory(sh) => {
                        state.drivers[sh.car_index as usize].session_history = Some(sh.clone());
                        state.drivers[sh.car_index as usize].participant.session_history = sh.lap_history_data.clone();
                    }
                    _ => {}
                });

                state.drivers.sort_by_key(|d| d.participant.position); //TODO: not strictly needed I guess, but looks better when inspecting the json manually
                let participants: Vec<SessionParticipant> = state
                    .drivers
                    .into_iter()
                    .filter_map(|ds| if ds.valid { Some(ds.participant) } else { None })
                    .collect_vec();

                ParsedSessionData {
                    session_id,
                    session_link_id: state.session_link_id,
                    session_type: state.session_type,
                    track: state.track,
                    participants,
                }
            })
            .collect_vec(),
    };

    let race_data_json = serde_json::to_string_pretty(&parsed_sessions)?;

    if let Some(out) = out {
        std::fs::write(&out, race_data_json)?;
        println!("Wrote race data to {:?}", out.as_ref());
    } else {
        println!("{}", race_data_json);
    }

    Ok(())
}
