use std::{
    io::{Read, Write},
    net::UdpSocket,
    path::Path,
    time::{Duration, Instant},
};

use bytes::{Buf, Bytes};
use serde::{Deserialize, Serialize};
use telemetry::{
    decode_packet,
    packet::{header::PacketId, Packet},
};
use tracing::{debug, level_filters::LevelFilter, warn};
use tracing_subscriber::FmtSubscriber;

// A packet that can be/was written to disk
#[derive(Debug, Serialize, Deserialize)]
struct DiskPacket {
    time: Duration,
    packet: Packet,
}

pub fn initialize(level: impl Into<LevelFilter>) -> eyre::Result<()> {
    color_eyre::install()?;

    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
    tracing::subscriber::set_global_default(subscriber)?;

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
        let packet = decode_packet(packet_bytes)?;

        if let Some(filter) = &filter {
            if !filter.contains(&packet.header().packet_id) {
                continue;
            }
        }

        packets.push(DiskPacket { time, packet });
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

pub fn finish<P: AsRef<Path>>(file: P, out: Option<P>) -> Result<(), eyre::Error> {
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
        let packet = decode_packet(packet_bytes)?;
        packets.push(DiskPacket { time, packet });
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