use serde::Serialize;

use crate::protocol::{
    data_types::{Chat, McString},
    Decodable, Encodable, 
};

#[derive(Debug)]
pub struct SbStatusRequest;

impl Decodable for SbStatusRequest {
    fn decode(_: impl bytes::Buf) -> Result<Self, crate::protocol::DecodeError> {
        Ok(Self)
    }
}

#[derive(Debug)]
pub struct CbStatusResponse {
    pub response: McString<16384>,
}

impl Encodable for CbStatusResponse {
    fn encode(&self, dest: &mut bytes::BytesMut) {
        self.response.encode(dest);
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    pub version: StatusResponseVersion,
    pub players: StatusResponsePlayers,
    pub description: Chat,
    pub favicon: Option<String>,
    pub enforces_secure_chat: bool,
    pub previews_chat: bool,
}

#[derive(Debug, Serialize)]
pub struct StatusResponseVersion {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, Serialize)]
pub struct StatusResponsePlayers {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<StatusResponsePlayersSample>,
}

#[derive(Debug, Serialize)]
pub struct StatusResponsePlayersSample {
    pub name: String,
    pub id: String,
}
