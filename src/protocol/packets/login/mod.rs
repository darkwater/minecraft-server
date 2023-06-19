mod disconnect;
mod start;
mod success;

pub use disconnect::*;
pub use start::*;
pub use success::*;

use super::macros::{define_clientbound_packet, define_serverbound_packet};

define_serverbound_packet! { SbLoginPacket {
    0x00 => LoginStart(SbLoginStart),
}}

define_clientbound_packet! { CbLoginPacket {
    Disconnect(CbDisconnect) => 0x00,
    LoginSuccess(CbLoginSuccess) => 0x02,
}}
