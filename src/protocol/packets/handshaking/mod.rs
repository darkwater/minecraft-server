use crate::protocol::{Decodable, DecodeError};

pub use self::handshake::SbHandshake;

use super::UncompressedPacket;

pub mod handshake;

#[derive(Debug)]
pub enum SbHandshakingPacket {
    Handshake(SbHandshake),
}

impl TryFrom<UncompressedPacket> for SbHandshakingPacket {
    type Error = DecodeError;

    fn try_from(packet: UncompressedPacket) -> Result<Self, Self::Error> {
        match packet.packet_id.0 {
            0x00 => SbHandshake::decode(packet.data.0).map(Self::Handshake),
            _ => Err(DecodeError::Invalid),
        }
    }
}
