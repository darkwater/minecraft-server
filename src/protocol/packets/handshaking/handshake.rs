use crate::protocol::{
    data_types::{Enum, McString, UnsignedShort, VarInt},
    Decodable, State,
};

#[derive(Debug)]
pub struct SbHandshake {
    protocol_version: VarInt,
    server_address: McString<255>,
    server_port: UnsignedShort,
    next_state: Enum<VarInt>,
}

impl Decodable for SbHandshake {
    fn decode(mut src: impl bytes::Buf) -> Result<Self, crate::protocol::DecodeError> {
        let protocol_version = VarInt::decode(&mut src)?;
        let server_address = McString::decode(&mut src)?;
        let server_port = UnsignedShort(src.get_u16());
        let next_state = Enum(VarInt::decode(&mut src)?);

        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}

impl SbHandshake {
    pub fn next_state(&self) -> Option<State> {
        match self.next_state.0 .0 {
            1 => Some(State::Status),
            2 => Some(State::Login),
            _ => None,
        }
    }
}
