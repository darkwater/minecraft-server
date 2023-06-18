mod status;

use anyhow::{bail, Context, Result};
use futures::StreamExt;
use tokio::{io::BufReader, net::TcpStream};
use tokio_util::codec::Decoder;

use crate::protocol::{codec::PacketCodec, packets::handshaking::SbHandshakingPacket, State};

pub async fn handle(sock: TcpStream) -> Result<()> {
    let stream = BufReader::new(sock);
    let mut framed = PacketCodec::<SbHandshakingPacket, ()>::new().framed(stream);

    let packet = framed
        .next()
        .await
        .context("Connection closed before handshake")?
        .context("Handshake failed")?;

    tracing::debug!("handshake: {:?}", packet);

    let SbHandshakingPacket::Handshake(packet) = packet;

    let next_state = packet.next_state().context("Unknown next state")?;

    tracing::info!("Transitioning to {next_state:?}");

    match next_state {
        State::Handshaking => bail!("Can't go from handshake to handshake"),
        State::Status => status::handle(framed.into_inner()).await,
        State::Login => todo!(),
    }
}
