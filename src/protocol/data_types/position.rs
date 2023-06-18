use bytes::{BufMut, BytesMut, Buf};

use crate::protocol::{Encodable, DecodeError, Decodable};

use super::Position;

fn bits(n: u64) -> u64 {
    (1 << n) - 1
}

impl Encodable for Position {
    fn encode(&self, w: &mut BytesMut) {
        let x = (self.x as u64) & bits(26);
        let z = (self.z as u64) & bits(26);
        let y = (self.y as u64) & bits(12);

        w.put_u64((x << 38) | (z << 12) | y);
    }
}

impl Decodable for Position {
    fn decode(mut r: impl Buf) -> Result<Position, DecodeError> {
        let n = r.get_u64();

        let mut x = (n >> 38) as i32;
        let mut z = ((n >> 12) & bits(26)) as i32;
        let mut y = (n & bits(12)) as i16;

        if x & (1 << 25) != 0 {
            x -= 1 << 26;
        }

        if z & (1 << 25) != 0 {
            z -= 1 << 26;
        }

        if y & (1 << 11) != 0 {
            y -= 1 << 12;
        }

        Ok(Position { x, z, y })
    }
}

#[cfg(test)]
mod tests {
    use bytes::Buf;

    use super::*;

    #[test]
    fn encode() {
        let mut buf = BytesMut::new();
        Position {
            x: 18357644,
            z: -20882616,
            y: 831,
        }
        .encode(&mut buf);
        let n = buf.get_u64();
        assert_eq!(n, 0b01000110000001110110001100_10110000010101101101001000_001100111111);
    }

    #[test]
    fn decode() {
        let mut buf = BytesMut::new();
        buf.put_u64(0b01000110000001110110001100_10110000010101101101001000_001100111111);

        let pos = Position::decode(&mut buf.freeze()).unwrap();
        assert_eq!(pos.x, 18357644);
        assert_eq!(pos.z, -20882616);
        assert_eq!(pos.y, 831);
    }
}
