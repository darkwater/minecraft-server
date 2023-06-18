use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::{io::BufReader, net::TcpStream};
use tokio_util::codec::Decoder;

use crate::protocol::{
    data_types::Chat,
    codec::PacketCodec,
    packets::status::{
        status::{
            CbStatusResponse, SbStatusRequest, StatusResponse, StatusResponsePlayers,
            StatusResponseVersion,
        },
        CbStatusPacket, SbStatusPacket,
    },
};

pub async fn handle(stream: BufReader<TcpStream>) -> Result<()> {
    let mut framed = PacketCodec::<SbStatusPacket, CbStatusPacket>::new().framed(stream);

    while let Some(packet) = framed.next().await {
        tracing::info!("{packet:?}");
        let packet = packet?;

        match packet {
            SbStatusPacket::StatusRequest(SbStatusRequest) => {
                let response = StatusResponse {
                    version: StatusResponseVersion {
                        name: "1.20.1".to_string(),
                        protocol: 763,
                    },
                    players: StatusResponsePlayers {
                        max: 20,
                        online: 0,
                        sample: vec![],
                    },
                    description: Chat {
                        text: "Hello, Rust!".to_string(),
                    },
                    favicon: Some(String::new()),
                    enforces_secure_chat: false,
                    previews_chat: false,
                };

                let response = serde_json::to_string(&response).unwrap().into();

                framed.send(CbStatusPacket::StatusResponse(CbStatusResponse {
                    response,
                })).await?;
            }
        }
    }

    Ok(())
}
