use bytes::Buf;

use crate::protocol::{
    data_types::{ByteArray, VarInt},
    Decodable, DecodeError, Encodable,
};

pub struct UncompressedPacket {
    pub packet_id: VarInt,
    pub data: ByteArray,
}

impl Encodable for UncompressedPacket {
    fn encode(&self, dest: &mut bytes::BytesMut) {
        VarInt((self.packet_id.wire_length() + self.data.0.len()) as i32).encode(dest);
        self.packet_id.encode(dest);
        dest.extend_from_slice(&self.data.0);
    }
}

impl Decodable for UncompressedPacket {
    fn decode(mut src: impl Buf) -> Result<Self, DecodeError> {
        let length = VarInt::decode(&mut src)?;

        let remaining = src.remaining();
        if remaining < length.0 as usize {
            return Err(DecodeError::TooShort);
        }

        let packet_id = VarInt::decode(&mut src)?;
        let data_length = length.0 as usize - (remaining - src.remaining());

        let data = ByteArray(src.copy_to_bytes(data_length));

        Ok(UncompressedPacket { packet_id, data })
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use crate::protocol::{
        data_types::McString,
        packets::{
            packet::UncompressedPacket,
            status::{CbStatusPacket, CbStatusResponse},
        },
        Encodable,
    };

    #[test]
    fn encode_status_packet() {
        let status_response = CbStatusResponse {
            response: McString("foo bar".to_string()),
        };

        let packet = CbStatusPacket::StatusResponse(status_response);

        let packet = UncompressedPacket::from(packet);

        let mut bytes = BytesMut::new();
        packet.encode(&mut bytes);

        assert_eq!(&bytes[..], b"\x09\x00\x07foo bar");
    }
}
