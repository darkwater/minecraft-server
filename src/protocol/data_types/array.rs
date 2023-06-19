use bytes::BytesMut;

use super::{Array, VarInt};
use crate::protocol::Encodable;

impl<T: Encodable> Array<T> {
    pub fn encode_with_length(&self, dest: &mut BytesMut) {
        VarInt(self.0.len() as i32).encode(dest);

        for element in &self.0 {
            element.encode(dest);
        }
    }
}
