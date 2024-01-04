use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};

const PORT: u16 = 56743;
static IPV4: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);

pub async fn start_server_ws() -> Result<()> {
    let addr = SocketAddr::from((IPV4, PORT));
    let listener = TcpListener::bind(addr).await?;
    log::info!("Server listening on {:?}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    let (mut write, mut read) = ws_stream.split();

    while let Some(Ok(msg)) = read.next().await {
        match msg {
            Message::Text(text) => {
                println!("Received message: {}", text);
                write.send(Message::Text(text)).await?;
            }
            _ => (),
        }
    }

    Ok(())
}
