use bytes::{BytesMut, Buf};

pub mod data_types;
pub mod packets;
pub mod codec;
pub mod error;

pub use error::DecodeError;

#[derive(Clone, Copy, Debug)]
pub enum State {
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
