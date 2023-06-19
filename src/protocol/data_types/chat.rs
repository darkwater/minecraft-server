use super::{Chat, McString};
use crate::protocol::Encodable;

impl Encodable for Chat {
    fn encode(&self, dest: &mut bytes::BytesMut) {
        let s = serde_json::to_string(self).unwrap();
        McString::<262144>(s).encode(dest);
    }
}
