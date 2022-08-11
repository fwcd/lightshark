mod format;
mod proxy;

use anyhow::Result;
use async_std::net::TcpListener;
use clap::Parser;
use log::{Level, info};

#[derive(Parser, Debug)]
struct Args {
    /// The WebSocket URL to proxy to.
    url: String,
    /// The host to listen on.
    #[clap(short, long, default_value = "localhost")]
    host: String,
    /// The port to listen on.
    #[clap(short, long, default_value_t = 9094)]
    port: u16,
}

#[async_std::main]
async fn main() -> Result<()> {
    simple_logger::init_with_level(Level::Info).unwrap();
    
    let args = Args::parse();

    let listener = TcpListener::bind((args.host.as_str(), args.port)).await?;
    info!("Listening on ws://{}:{}...", args.host, args.port);

    let (client_stream, client_addr) = listener.accept().await?;
    info!("Connected to {}", client_addr);

    proxy::proxy(client_stream, args.url.as_str()).await?;

    Ok(())
}
