use bytes::Buf;

use super::Optional;
use crate::protocol::{Decodable, DecodeError};

impl<T: Decodable> Optional<T> {
    pub fn decode_if(condition: bool, src: impl Buf) -> Result<Self, DecodeError> {
        if condition {
            Ok(Optional(Some(T::decode(src)?)))
        } else {
            Ok(Optional(None))
        }
    }
}
