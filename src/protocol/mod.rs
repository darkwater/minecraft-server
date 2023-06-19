use bytes::{Buf, BytesMut};

pub mod codec;
pub mod data_types;
pub mod error;
pub mod packets;

pub use error::DecodeError;

#[derive(Clone, Copy, Debug)]
pub enum State {
    #[allow(dead_code)]
    Handshaking,
    Status,
    Login,
}

pub trait Encodable: Sized {
    fn encode(&self, dest: &mut BytesMut);
}

pub trait Decodable: Sized {
    fn decode(src: impl Buf) -> Result<Self, DecodeError>;
}
