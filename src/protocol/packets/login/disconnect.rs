use crate::protocol::{data_types::Chat, Encodable};

#[derive(Debug)]
pub struct CbDisconnect {
    pub reason: Chat,
}

impl Encodable for CbDisconnect {
    fn encode(&self, dest: &mut bytes::BytesMut) {
        self.reason.encode(dest);
    }
}
