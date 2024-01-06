use std::io::{self, Cursor};

use bytes::{Buf, Bytes};

use self::packet::Packet;

pub mod packet;
mod twentytwo;
mod twentythree;

#[derive(thiserror::Error, Debug)]
pub enum TelemetryError {
    #[error("invalid packet")]
    InvalidPacket,
    #[error("io error")]
    IoError(#[from] io::Error),
}

type Result<T> = std::result::Result<T, TelemetryError>;

pub fn decode_packet(bytes: Bytes) -> Result<Packet> {
    let mut cursor = Cursor::new(bytes);

    let format = cursor.get_u16_le();

    match format {
        2022 => twentytwo::decode_twentytwo(&mut cursor),
        2023 => twentythree::decode_twentythree(&mut cursor),
        _ => Err(TelemetryError::InvalidPacket),
    }
}
