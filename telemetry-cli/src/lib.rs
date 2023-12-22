use std::{
    io::{Read, Write},
    net::UdpSocket,
    path::Path,
    time::{Duration, Instant},
};

use bytes::{Buf, Bytes};
use serde::{Deserialize, Serialize};
use telemetry::{decode_packet, packet::Packet};

// A packet that can be/was written to disk
#[derive(Debug, Serialize, Deserialize)]
struct DiskPacket {
    time: Duration,
    packet: Packet,
}

pub fn record<P: AsRef<Path>>(file: P) -> eyre::Result<()> {
    // open file
    println!("Recording to {:?}", file.as_ref());
    let mut file = std::fs::File::create(file)?;
    let socket = UdpSocket::bind("0.0.0.0:20777")?;
    let mut buf = [0; 1464];

    let start_time = Instant::now();

    while let Ok((size, _)) = socket.recv_from(&mut buf) {
        println!("Packet of size: {}", size);
        let b = Bytes::copy_from_slice(&buf[..size]);
        // let b = Bytes::copy_from_slice(&buf);
        let packet = decode_packet(b.clone())?;
        println!("Packet: {:?}", packet);
        file.write_all(&(size as u64).to_be_bytes())?;
        file.write_all(&(Instant::now() - start_time).as_secs_f64().to_be_bytes())?;
        file.write_all(&b)?;
        file.flush()?;
    }

    Ok(())
}

pub fn parse<P: AsRef<Path>>(file: P, out: Option<P>) -> Result<(), eyre::Error> {
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
