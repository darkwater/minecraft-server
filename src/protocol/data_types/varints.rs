use bytes::{Buf, BufMut, BytesMut};

use super::{VarInt, VarLong};
use crate::protocol::{Decodable, DecodeError, Encodable};

impl VarInt {
    /// Returns how long this varint would be if encoded.
    pub fn wire_length(&self) -> usize {
        let mut buf = BytesMut::new();
        self.encode(&mut buf);
        buf.len()
    }
}

impl Encodable for VarInt {
    fn encode(&self, w: &mut BytesMut) {
        let mut n = self.0 as u32;

        loop {
            if n <= 0x7f {
                w.put_u8(n as u8);
                return;
            } else {
                w.put_u8((n & 0x7f) as u8 | 0x80);
                n >>= 7;
            }
        }
    }
}

impl Decodable for VarInt {
    fn decode(mut r: impl Buf) -> Result<Self, DecodeError> {
        let mut n = 0u32;
        let mut offset = 0;

        loop {
            if !r.has_remaining() {
                return Err(DecodeError::TooShort);
            }

            let b = r.get_u8();
            n += ((b & 0x7f) as u32) << offset;
            offset += 7;

            if b & 0x80 == 0 {
                return Ok(VarInt(n as i32));
            } else if offset >= 32 {
                return Err(DecodeError::Invalid);
            }
        }
    }
}

impl Encodable for VarLong {
    fn encode(&self, w: &mut BytesMut) {
        let mut n = self.0 as u64;

        loop {
            if n <= 0x7f {
                w.put_u8(n as u8);
                return;
            } else {
                w.put_u8((n & 0x7f) as u8 | 0x80);
                n >>= 7;
            }
        }
    }
}

impl Decodable for VarLong {
    fn decode(mut r: impl Buf) -> Result<VarLong, DecodeError> {
        let mut n = 0u64;
        let mut offset = 0;

        loop {
            let b = r.get_u8();
            n += ((b & 0x7f) as u64) << offset;
            offset += 7;

            if b & 0x80 == 0 {
                return Ok(VarLong(n as i64));
            } else if offset >= 64 {
                return Err(DecodeError::Invalid);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::*;

    fn encode_int(n: i32) -> Vec<u8> {
        let mut buf = BytesMut::new();
        VarInt(n).encode(&mut buf);
        buf.to_vec()
    }

    fn decode_int(v: &'static [u8]) -> i32 {
        let mut bytes = Bytes::from_static(v);
        VarInt::decode(&mut bytes).unwrap().0
    }

    fn encode_long(n: i64) -> Vec<u8> {
        let mut buf = BytesMut::new();
        VarLong(n).encode(&mut buf);
        buf.to_vec()
    }

    fn decode_long(v: &'static [u8]) -> i64 {
        let mut bytes = Bytes::from_static(v);
        VarLong::decode(&mut bytes).unwrap().0
    }

    #[test]
    fn encode_ints() {
        assert_eq!(encode_int(0), &[0x00]);
        assert_eq!(encode_int(1), &[0x01]);
        assert_eq!(encode_int(2), &[0x02]);
        assert_eq!(encode_int(127), &[0x7f]);
        assert_eq!(encode_int(128), &[0x80, 0x01]);
        assert_eq!(encode_int(255), &[0xff, 0x01]);
        assert_eq!(encode_int(25565), &[0xdd, 0xc7, 0x01]);
        assert_eq!(encode_int(2147483647), &[0xff, 0xff, 0xff, 0xff, 0x07]);
        assert_eq!(encode_int(-1), &[0xff, 0xff, 0xff, 0xff, 0x0f]);
        assert_eq!(encode_int(-2147483648), &[0x80, 0x80, 0x80, 0x80, 0x08]);
    }

    #[test]
    fn decode_ints() {
        assert_eq!(decode_int(&[0x00]), 0);
        assert_eq!(decode_int(&[0x01]), 1);
        assert_eq!(decode_int(&[0x02]), 2);
        assert_eq!(decode_int(&[0x7f]), 127);
        assert_eq!(decode_int(&[0x80, 0x01]), 128);
        assert_eq!(decode_int(&[0xff, 0x01]), 255);
        assert_eq!(decode_int(&[0xdd, 0xc7, 0x01]), 25565);
        assert_eq!(decode_int(&[0xff, 0xff, 0xff, 0xff, 0x07]), 2147483647);
        assert_eq!(decode_int(&[0xff, 0xff, 0xff, 0xff, 0x0f]), -1);
        assert_eq!(decode_int(&[0x80, 0x80, 0x80, 0x80, 0x08]), -2147483648);
    }

    #[test]
    fn encode_longs() {
        assert_eq!(encode_long(0), &[0x00]);
        assert_eq!(encode_long(1), &[0x01]);
        assert_eq!(encode_long(2), &[0x02]);
        assert_eq!(encode_long(127), &[0x7f]);
        assert_eq!(encode_long(128), &[0x80, 0x01]);
        assert_eq!(encode_long(255), &[0xff, 0x01]);
        assert_eq!(encode_long(2147483647), &[0xff, 0xff, 0xff, 0xff, 0x07]);
        assert_eq!(encode_long(9223372036854775807), &[
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f
        ]);
        assert_eq!(encode_long(-1), &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]);
        assert_eq!(encode_long(-2147483648), &[
            0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01
        ]);
        assert_eq!(encode_long(-9223372036854775808), &[
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01
        ]);
    }

    #[test]
    fn decode_longs() {
        assert_eq!(decode_long(&[0x00]), 0);
        assert_eq!(decode_long(&[0x01]), 1);
        assert_eq!(decode_long(&[0x02]), 2);
        assert_eq!(decode_long(&[0x7f]), 127);
        assert_eq!(decode_long(&[0x80, 0x01]), 128);
        assert_eq!(decode_long(&[0xff, 0x01]), 255);
        assert_eq!(decode_long(&[0xff, 0xff, 0xff, 0xff, 0x07]), 2147483647);
        assert_eq!(
            decode_long(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]),
            9223372036854775807
        );
        assert_eq!(decode_long(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]), -1);
        assert_eq!(
            decode_long(&[0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01]),
            -2147483648
        );
        assert_eq!(
            decode_long(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]),
            -9223372036854775808
        );
    }
}
