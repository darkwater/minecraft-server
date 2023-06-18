use std::{fmt::Debug, io, marker::PhantomData};

use tokio_util::codec::{Decoder, Encoder};

use super::{packets::UncompressedPacket, Decodable, DecodeError, Encodable};

pub struct PacketCodec<S, C> {
    phantom: PhantomData<(S, C)>,
}

impl<S, C> PacketCodec<S, C> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<S, C: Into<UncompressedPacket> + Debug> Encoder<C> for PacketCodec<S, C> {
    type Error = io::Error;

    fn encode(&mut self, item: C, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        tracing::trace!("item: {item:?}");

        let packet = item.into();
        packet.encode(dst);
        Ok(())
    }
}

impl<S: TryFrom<UncompressedPacket, Error = DecodeError>, C> Decoder for PacketCodec<S, C> {
    type Item = S;
    type Error = DecodeError;

    fn decode(&mut self, mut src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        tracing::trace!("src: {src:?}~");

        let packet = UncompressedPacket::decode(&mut src);

        if matches!(packet, Err(DecodeError::TooShort)) {
            Ok(None)
        } else {
            S::try_from(packet?).map(Some)
        }
    }
}
