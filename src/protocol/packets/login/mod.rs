pub mod start;
pub mod disconnect;

use self::{start::SbLoginStart, disconnect::CbDisconnect};

use super::macros::{define_serverbound_packet, define_clientbound_packet};

define_serverbound_packet! { SbLoginPacket {
    0x00 => LoginStart(SbLoginStart),
}}

define_clientbound_packet! { CbLoginPacket {
    Disconnect(CbDisconnect) => 0x00,
}}
