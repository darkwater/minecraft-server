use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::{io::BufReader, net::TcpStream};
use tokio_util::codec::Decoder;

use crate::protocol::{
    codec::PacketCodec,
    data_types::Chat, packets::login::{SbLoginPacket, CbLoginPacket, disconnect::CbDisconnect},
};

pub async fn handle(stream: BufReader<TcpStream>) -> Result<()> {
    let mut framed = PacketCodec::<SbLoginPacket, CbLoginPacket>::new().framed(stream);

    while let Some(packet) = framed.next().await {
        tracing::info!("{packet:?}");
        let packet = packet?;

        match packet {
            SbLoginPacket::LoginStart(info) => {
                tracing::info!("{} tries to connect", info.name.0);

                framed.send(CbLoginPacket::Disconnect(CbDisconnect {
                    reason: Chat { text: "Your client is not written in Rust.".to_string() },
                })).await?;
            }
        }
    }

    Ok(())
}
