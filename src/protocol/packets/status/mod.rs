pub mod status;

use bytes::BytesMut;

use crate::protocol::{
    data_types::{ByteArray, VarInt},
    Decodable, DecodeError, Encodable,
};

use self::status::{CbStatusResponse, SbStatusRequest};

use super::UncompressedPacket;

#[derive(Debug)]
pub enum CbStatusPacket {
    StatusResponse(CbStatusResponse),
}

impl From<CbStatusPacket> for UncompressedPacket {
    fn from(value: CbStatusPacket) -> Self {
        let mut data = BytesMut::new();

        let packet_id = match value {
            CbStatusPacket::StatusResponse(r) => {
                r.encode(&mut data);
                0x00
            }
        };

        UncompressedPacket {
            packet_id: VarInt(packet_id),
            data: ByteArray(data.freeze()),
        }
    }
}

#[derive(Debug)]
pub enum SbStatusPacket {
    StatusRequest(SbStatusRequest),
}

impl TryFrom<UncompressedPacket> for SbStatusPacket {
    type Error = DecodeError;

    fn try_from(packet: UncompressedPacket) -> Result<Self, Self::Error> {
        match packet.packet_id.0 {
            0x00 => SbStatusRequest::decode(packet.data.0).map(Self::StatusRequest),
            _ => Err(DecodeError::Invalid),
        }
    }
}
