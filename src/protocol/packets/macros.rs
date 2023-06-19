/// # Example
/// ```
/// define_clientbound_packet! { CbStatusPacket {
///     StatusResponse(CbStatusResponse) => 0x00,
///     PongResponse(CbPongResponse) => 0x01,
/// }}
/// ```
macro_rules! define_clientbound_packet {
    ($packty:ident { $( $variant:ident($varty:ident) => $id:expr, )* }) => {
        #[derive(Debug)]
        pub enum $packty {
            $( $variant($varty), )*
        }

        impl From<$packty> for crate::protocol::packets::packet::UncompressedPacket {
            fn from(value: $packty) -> Self {
                use crate::protocol::Encodable;

                let mut data = bytes::BytesMut::new();

                let packet_id = match value {
                    $( $packty::$variant(r) => {
                        r.encode(&mut data);
                        $id
                    } )*
                };

                crate::protocol::packets::packet::UncompressedPacket {
                    packet_id: crate::protocol::data_types::VarInt(packet_id),
                    data: crate::protocol::data_types::ByteArray(data.freeze()),
                }
            }
        }
    };
}

/// # Example
/// ```
/// define_serverbound_packet! { SbStatusPacket {
///     0x00 => StatusRequest(SbStatusRequest),
///     0x01 => PingRequest(SbPingRequest),
/// }}
/// ```
macro_rules! define_serverbound_packet {
    ($packty:ident { $( $id:expr => $variant:ident($varty:ident), )* }) => {
        #[derive(Debug)]
        pub enum $packty {
            $( $variant($varty), )*
        }

        impl TryFrom<crate::protocol::packets::packet::UncompressedPacket> for $packty {
            type Error = crate::protocol::error::DecodeError;

            fn try_from(packet: crate::protocol::packets::packet::UncompressedPacket) -> Result<Self, Self::Error> {
                use crate::protocol::Decodable;

                match packet.packet_id.0 {
                    $( $id => $varty::decode(packet.data.0).map(Self::$variant), )*
                    _ => Err(crate::protocol::error::DecodeError::Invalid),
                }
            }
        }
    };
}

pub(crate) use define_clientbound_packet;
pub(crate) use define_serverbound_packet;
