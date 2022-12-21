use clap::Args;
use color_eyre::Result;
use deku::DekuContainerWrite;
use futures_util::{SinkExt, StreamExt};
use hyper_tungstenite::tungstenite::Message;
use crate::shared;
use log::info;

const CONNECTION_URL: &str = "ws://127.0.0.1:3000/trowel";

#[derive(Args, Debug)]
pub struct ClientArgs {}
pub async fn main(_args: &ClientArgs) -> Result<()> {
    info!("Running client");
    let (socket, _) = tokio_tungstenite::connect_async(CONNECTION_URL).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");
    let (mut tx, mut rx) = socket.split();
    println!("Sending !");
    let connect = shared::WSMessage::Connect(shared::ConnectData::new(5000, "127.0.0.1")).to_bytes().unwrap();
    tx.send(Message::Binary(connect)).await?;
    Ok(())
}