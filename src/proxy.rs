use anyhow::Result;
use async_std::{net::TcpStream, task};
use async_tungstenite::{async_std::connect_async, accept_async};
use futures::{SinkExt, StreamExt};
use log::info;

use crate::format::format_ws_message;

/// Proxies the traffic between the given client and the given server URL.
pub(crate) async fn proxy(client_stream: TcpStream, server_url: &str) -> Result<()> {
    let client_ws = accept_async(client_stream).await?;
    let (server_ws, _) = connect_async(server_url).await?;

    // Split connections so we can move them to their respective tasks
    let (mut client_sink, mut client_stream) = client_ws.split();
    let (mut server_sink, mut server_stream) = server_ws.split();

    // Relay messages from server to client
    let client_proxy = task::spawn::<_, Result<()>>(async move {
        while let Some(msg_result) = server_stream.next().await {
            let msg = msg_result?;
            info!("<- {}", format_ws_message(&msg));
            client_sink.send(msg).await?;
        }
        Ok(())
    });

    // Relay messages from client to server
    let server_proxy = task::spawn::<_, Result<()>>(async move {
        while let Some(msg_result) = client_stream.next().await {
            let msg = msg_result?;
            info!("-> {}", format_ws_message(&msg));
            server_sink.send(msg).await?;
        }
        Ok(())
    });

    // Join client and server
    client_proxy.await?;
    server_proxy.await?;

    Ok(())
}
