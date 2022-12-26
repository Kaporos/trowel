use clap::Args;
use color_eyre::Result;
use deku::DekuContainerWrite;
use futures_util::{SinkExt, StreamExt};
use hyper_tungstenite::tungstenite::Message;
use shared;
use log::info;

const CONNECTION_URL: &str = "ws://127.0.0.1:3000/trowel";

#[derive(Args, Debug)]
pub struct ClientArgs {}
pub async fn launch(_args: &ClientArgs) -> Result<()> {
    info!("Running client");
    let (socket, _) = tokio_tungstenite::connect_async(CONNECTION_URL).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");
    let (mut tx, _) = socket.split();
    println!("Sending !");
    let connect = shared::WSMessage::Connect(shared::ConnectData::new(5000, "127.0.0.1"));
    tx.send(Message::Binary(connect.to_bytes().unwrap())).await?;
    Ok(())
}

