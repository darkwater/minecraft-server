use anyhow::Result;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

mod client_handler;
mod protocol;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let listener = TcpListener::bind("127.0.0.1:25565").await?;
    tracing::info!("Listening...");

    loop {
        let (socket, addr) = listener.accept().await?;

        tracing::info!("Accepting connection from {addr:?}");

        tokio::spawn(async move {
            match client_handler::handle(socket).await {
                Ok(()) => tracing::info!("{addr:?} connection closed"),
                Err(e) => tracing::error!("{addr:?} connection ended with error: {e:?}"),
            }
        });
    }
}
