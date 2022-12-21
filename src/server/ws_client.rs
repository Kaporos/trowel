use std::sync::atomic::AtomicUsize;

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use log::*;
use deku::prelude::*;
use warp::ws::{WebSocket, Message, Ws};

static CLIENT_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub struct WSClient {
    pub id: usize
}

impl WSClient {
    pub fn new() -> WSClient {
        WSClient { 
            id: CLIENT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
         } 
    }
    pub async fn handle(&self, mut ws_out: SplitSink<WebSocket, Message>, mut ws_in: SplitStream<WebSocket>) {
        info!("Client {} connected !", self.id);
        while let Some(result) = ws_in.next().await {
            if result.is_err() {
                continue;
            } else {
                let msg = result.unwrap();
                let bytes = msg.as_bytes();
                if bytes.len() == 0 {
                    break;
                }
                let wsmessage = crate::shared::WSMessage::from_bytes((bytes, 0));
                if let Err(_) = wsmessage {
                    error!("Received invalid message from client {}.", self.id);
                    ws_out.send(Message::text("Invalid data received.")).await.unwrap();
                }
            }
        }
        info!("Client {} disconnected !", self.id)

    }
}