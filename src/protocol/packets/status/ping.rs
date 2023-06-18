use crate::protocol::{data_types::Long, Decodable, Encodable};

#[derive(Debug)]
pub struct SbPingRequest {
    pub payload: Long,
}

impl Decodable for SbPingRequest {
    fn decode(src: impl bytes::Buf) -> Result<Self, crate::protocol::DecodeError> {
        Ok(Self {
            payload: Long::decode(src)?,
        })
    }
}

#[derive(Debug)]
pub struct CbPongResponse {
    pub payload: Long,
}

impl Encodable for CbPongResponse {
    fn encode(&self, mut dest: &mut bytes::BytesMut) {
        self.payload.encode(&mut dest);
    }
}
