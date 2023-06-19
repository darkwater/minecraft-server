use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::{io::BufReader, net::TcpStream};
use tokio_util::codec::Decoder;

use crate::protocol::{
    codec::PacketCodec,
    data_types::{Array, Chat},
    packets::login::{CbDisconnect, CbLoginPacket, CbLoginSuccess, SbLoginPacket},
};

pub async fn handle(stream: BufReader<TcpStream>) -> Result<()> {
    let mut framed = PacketCodec::<SbLoginPacket, CbLoginPacket>::new().framed(stream);

    while let Some(packet) = framed.next().await {
        tracing::info!("{packet:?}");
        let packet = packet?;

        match packet {
            SbLoginPacket::LoginStart(info) => {
                tracing::info!("{} tries to connect", info.name.0);

                if info.uuid.0.is_none() {
                    framed
                        .send(
                            CbDisconnect {
                                reason: Chat {
                                    text: "Your login packet didn't contain a UUID".to_string(),
                                },
                            }
                            .into(),
                        )
                        .await?;
                }

                framed
                    .send(
                        CbLoginSuccess {
                            username: info.name.truncate(),
                            uuid: info.uuid.0.unwrap(),
                            properties: Array(vec![]),
                        }
                        .into(),
                    )
                    .await?;
            }
        }
    }

    Ok(())
}
