use crate::protocol::{
    data_types::{Boolean, McString, Optional, Uuid},
    Decodable,
};

#[derive(Debug)]
pub struct SbLoginStart {
    pub name: McString<255>,
    pub uuid: Optional<Uuid>,
}

impl Decodable for SbLoginStart {
    fn decode(mut src: impl bytes::Buf) -> Result<Self, crate::protocol::DecodeError> {
        let name = McString::decode(&mut src)?;
        let has_uuid = Boolean::decode(&mut src)?;
        let uuid = Optional::decode_if(has_uuid.0, &mut src)?;

        Ok(Self { name, uuid })
    }
}
