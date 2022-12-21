use clap::Args;
use color_eyre::Result;
use futures_util::StreamExt;
use warp::{Filter};

use crate::server::ws_client::WSClient;
#[derive(Args, Debug)]
pub struct ServerArgs {}

pub async fn main(_args: &ServerArgs) -> Result<()> {
    let health_route = warp::path!("health").map(|| format!("I'm alive"));
    let ws_route = warp::path!("trowel")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(move |websocket| async {
                let (tx, rx) = websocket.split();
                let client = WSClient::new();
                client.handle(tx, rx).await;
            })
        });
    let routes = health_route.or(ws_route);
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
    Ok(())
}
