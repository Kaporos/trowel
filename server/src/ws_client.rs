use color_eyre::eyre::Context;
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

use shared;
static CLIENT_COUNTER: AtomicUsize = AtomicUsize::new(1);

enum State {
    Connecting,
    Connected
}

pub struct WSClient {
    pub id: usize,
    pub tx: Mutex<SplitSink<WebSocketStream<Upgraded>, Message>>,
    pub rx: Mutex<SplitStream<WebSocketStream<Upgraded>>>,
    state: State,
}

impl WSClient {
    pub async fn new(websocket: HyperWebsocket) -> WSClient {
        let websocket = websocket.await.unwrap();
        let (tx, rx) = websocket.split();
        WSClient { 
            id: CLIENT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            tx: Mutex::new(tx),
            rx: Mutex::new(rx),
            state: State::Connecting
         } 
    }
    pub async fn handle_message(&mut self, message: shared::WSMessage) -> Result<()> {
        match message {
            shared::WSMessage::Connect(_) => {
               let accept = shared::WSMessage::Accept;
               self.tx.lock().await.send(Message::Binary(accept.to_bytes().unwrap())).await.wrap_err("Error during salut send")?;
               self.state = State::Connected;
               Ok(())
            }
            _ => {
                Ok(())
            }

        }
    }
    pub async fn handle(&mut self) -> Result<()> {
        info!("Client {} connected !", self.id);
        while let Some(result) = self.rx.get_mut().next().await {
            if result.is_err() {
                continue;
            } else {
                let msg = result.unwrap();
                match msg {
                    Message::Close(_) => {
                        break;
                    },
                    Message::Binary(msg) => {
                        let ws_message = shared::WSMessage::from_bytes((&msg, 0));
                        if let Ok(e) = ws_message {
                            self.handle_message(e.1).await?;
                        } else {
                            self.tx.lock().await.send(Message::Text(String::from("Invalid binary data."))).await?;
                        }
                    },
                    Message::Text(msg) => {
                        let ws_message: color_eyre::Result<shared::WSMessage> = serde_json::from_str(&msg).wrap_err("");
                        if let Ok(e) = ws_message {
                            self.handle_message(e).await?;
                        } else {
                            self.tx.lock().await.send(Message::Text(String::from("Invalid text data."))).await?;
                        }
                    },
                    _ => {
                        self.tx.lock().await.send(Message::Text(String::from("Invalid data."))).await?;
                    }
                }
            }
        };
        info!("Client {} disconnected !", self.id);
        Ok(())

    }
}
