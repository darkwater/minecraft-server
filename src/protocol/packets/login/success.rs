use crate::protocol::{
    data_types::{Array, McString, Optional, Uuid},
    Encodable,
};

#[derive(Debug)]
pub struct CbLoginSuccess {
    pub uuid: Uuid,
    pub username: McString<16>,
    pub properties: Array<LoginProperty>,
}

#[derive(Debug)]
pub struct LoginProperty {
    pub name: McString<32767>,
    pub value: McString<32767>,
    pub signature: Optional<McString<32767>>,
}

impl Encodable for CbLoginSuccess {
    fn encode(&self, dest: &mut bytes::BytesMut) {
        self.uuid.encode(dest);
        self.username.encode(dest);
        self.properties.encode_with_length(dest);
    }
}

impl Encodable for LoginProperty {
    fn encode(&self, dest: &mut bytes::BytesMut) {
        self.name.encode(dest);
        self.value.encode(dest);
        self.signature.encode_with_bool(dest);
    }
}
