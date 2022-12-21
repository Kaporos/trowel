use std::sync::{Arc};
use futures_util::lock::Mutex;
use std::sync::atomic::AtomicUsize;

use color_eyre::Result;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use hyper::upgrade::Upgraded;
use hyper_tungstenite::{HyperWebsocket, WebSocketStream};
use hyper_tungstenite::tungstenite::Message;
use log::*;
use deku::prelude::*;

use crate::shared;
static CLIENT_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub struct WSClient {
    pub id: usize,
    pub tx: Arc<Mutex<SplitSink<WebSocketStream<Upgraded>, Message>>>,
    pub rx: Arc<Mutex<SplitStream<WebSocketStream<Upgraded>>>>
}

impl WSClient {
    pub async fn new(websocket: HyperWebsocket) -> WSClient {
        let websocket = websocket.await.unwrap();
        let (mut tx, mut rx) = websocket.split();
        WSClient { 
            id: CLIENT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            tx: Arc::new(Mutex::new(tx)),
            rx: Arc::new(Mutex::new(rx))
         } 
    }
    pub async fn handle_message(&self, message: crate::shared::WSMessage) -> Result<()> {
        match message {
            shared::WSMessage::Connect(data) => {
                
            }
            _ => {}
        }
        Ok(())
    }
    pub async fn handle(&self) -> Result<()> {
        info!("Client {} connected !", self.id);
        while let Some(result) = self.rx.lock().await.next().await {
            if result.is_err() {
                continue;
            } else {
                let msg = result.unwrap();
                println!("{:?}", msg);
                match msg {
                    Message::Close(_) => {
                        break;
                    },
                    Message::Binary(msg) => {
                        let ws_message = crate::shared::WSMessage::from_bytes((&msg, 0));
                        if let Ok(e) = ws_message {
                            self.handle_message(e.1).await?;
                        } else {
                            self.tx.lock().await.send(Message::Text(String::from("Invalid data."))).await?;
                        }
                    }
                    _ => {
                        self.tx.lock().await.send(Message::Text(String::from("Invalid data."))).await?;
                    }
                }
            }
        }
        info!("Client {} disconnected !", self.id);
        Ok(())

    }
}