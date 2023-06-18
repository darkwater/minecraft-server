use bytes::BufMut;

use crate::protocol::{Decodable, Encodable, DecodeError};

use super::*;

macro_rules! impl_for {
    ($ty:ty, $get:ident, $put:ident) => {
        impl Decodable for $ty {
            fn decode(mut src: impl bytes::Buf) -> Result<Self, crate::protocol::DecodeError> {
                Ok(Self(src.$get()))
            }
        }

        impl Encodable for $ty {
            fn encode(&self, dest: &mut bytes::BytesMut) {
                dest.$put(self.0);
            }
        }
    };
}

impl_for!(Byte, get_i8, put_i8);
impl_for!(UnsignedByte, get_u8, put_u8);
impl_for!(Short, get_i16, put_i16);
impl_for!(UnsignedShort, get_u16, put_u16);
impl_for!(Int, get_i32, put_i32);
impl_for!(Long, get_i64, put_i64);
impl_for!(Float, get_f32, put_f32);
impl_for!(Double, get_f64, put_f64);

impl Decodable for Boolean {
    fn decode(mut src: impl bytes::Buf) -> Result<Self, DecodeError> {
        match src.get_u8() {
            0x00 => Ok(Self(false)),
            0x01 => Ok(Self(true)),
            _ => Err(DecodeError::Invalid),
        }
    }
}

impl Encodable for Boolean {
    fn encode(&self, dest: &mut bytes::BytesMut) {
        dest.put_u8(self.0 as u8);
    }
}
