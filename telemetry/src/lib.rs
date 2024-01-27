use std::io::{self, Cursor};

use bytes::{Buf, Bytes};

use self::packet::Packet;

pub mod packet;
mod f1_23;

#[derive(thiserror::Error, Debug)]
pub enum TelemetryError {
    #[error("invalid packet: {0}")]
    InvalidPacket(String),
    #[error("io error")]
    IoError(#[from] io::Error),
}

type Result<T> = std::result::Result<T, TelemetryError>;

pub fn decode_packet(bytes: Bytes) -> Result<Packet> {
    let mut cursor = Cursor::new(bytes);

    let format = cursor.get_u16_le();

    match format {
        2023 => f1_23::decode_twentythree(&mut cursor),
        _ => Err(TelemetryError::InvalidPacket("Unsupported format".to_owned())),
    }
}

pub fn decode_header(bytes: Bytes) -> Result<packet::header::Header> {
    let mut cursor = Cursor::new(bytes);

    let format = cursor.get_u16_le();
    
    cursor.set_position(0);

    match format {
        2023 => f1_23::decode_twentythree_header(&mut cursor),
        _ => Err(TelemetryError::InvalidPacket("Unsupported format".to_owned())),
    }
}