use super::macros::{define_clientbound_packet, define_serverbound_packet};

mod ping;
#[allow(clippy::module_inception)]
mod status;

pub use ping::*;
pub use status::*;

define_clientbound_packet! { CbStatusPacket {
    StatusResponse(CbStatusResponse) => 0x00,
    PongResponse(CbPongResponse) => 0x01,
}}

define_serverbound_packet! { SbStatusPacket {
    0x00 => StatusRequest(SbStatusRequest),
    0x01 => PingRequest(SbPingRequest),
}}
