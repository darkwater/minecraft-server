use bytes::{Buf, BytesMut};

use super::{Optional, Boolean};
use crate::protocol::{Decodable, DecodeError, Encodable};

impl<T: Decodable> Optional<T> {
    pub fn decode_if(condition: bool, src: impl Buf) -> Result<Self, DecodeError> {
        if condition {
            Ok(Optional(Some(T::decode(src)?)))
        } else {
            Ok(Optional(None))
        }
    }
}


impl<T: Encodable> Optional<T> {
    pub fn encode_with_bool(&self, dest: &mut BytesMut) {
        Boolean(self.0.is_some()).encode(dest);
        if let Some(ref v) = self.0 {
            v.encode(dest)
        }
    }
}
