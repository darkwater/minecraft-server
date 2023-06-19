pub use self::handshake::SbHandshake;
use super::macros::define_serverbound_packet;

pub mod handshake;

define_serverbound_packet! { SbHandshakingPacket {
    0x00 => Handshake(SbHandshake),
}}
