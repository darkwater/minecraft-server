use bytes::BufMut;

use crate::protocol::{Decodable, DecodeError, Encodable};

use super::{McString, VarInt};

impl<const N: usize> Encodable for McString<N> {
    fn encode(&self, dest: &mut bytes::BytesMut) {
        if self.0.len() > N {
            tracing::warn!("{} too long for McString<{}>, truncating...", self.0.len(), N);
        }

        let len = N.min(self.0.len());

        VarInt(len as i32).encode(dest);
        dest.put_slice(self.0.as_bytes());
    }
}

impl<const N: usize> Decodable for McString<N> {
    fn decode(mut src: impl bytes::Buf) -> Result<Self, DecodeError> {
        let len = VarInt::decode(&mut src)?.0 as usize;

        if len > N {
            return Err(DecodeError::TooLong(len));
        }

        if src.remaining() < len {
            return Err(DecodeError::TooShort);
        }

        let str = String::from_utf8(src.copy_to_bytes(len).to_vec())?;
        Ok(McString(str))
    }
}

impl<const N: usize> From<String> for McString<N> {
    fn from(value: String) -> Self {
        McString(value)
    }
}
