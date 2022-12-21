use clap::Args;
use color_eyre::Result;
use hyper::body::{Body};
use hyper::{Request, Response, Method, StatusCode};

use super::ws_client::WSClient;
#[derive(Args, Debug)]
pub struct ServerArgs {}

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
async fn handle_request(mut req: Request<Body>) -> Result<Response<Body>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/health") => Ok(Response::new(Body::from("I'm alive !"))),
        (&Method::GET, "/trowel") => {
            println!("Hehe boy");
            if !(hyper_tungstenite::is_upgrade_request(&req)) {
                let mut bad_request = Response::new(Body::from("Bad request"));
                *bad_request.status_mut() = StatusCode::BAD_REQUEST;
                return Ok(bad_request);
            }
            let (response, websocket) = hyper_tungstenite::upgrade(&mut req, None).unwrap();
            // Spawn a task to handle the websocket connection.
            tokio::spawn(async move {
                let client = WSClient::new(websocket).await;
                client.handle().await.unwrap();
            });
            Ok(response)


        },
        // Return 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::new(Body::from("Not found"));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

pub async fn main(_args: &ServerArgs) -> Result<()> {
    let addr: std::net::SocketAddr = "0.0.0.0:3000".parse()?;
    println!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on {}", addr);

    let mut http = hyper::server::conn::Http::new();
    http.http1_only(true);
    http.http1_keep_alive(true);

    loop {
        let (stream, _) = listener.accept().await?;
        let connection = http
            .serve_connection(stream, hyper::service::service_fn(handle_request))
            .with_upgrades();
        tokio::spawn(async move {
            if let Err(err) = connection.await {
                println!("Error serving HTTP connection: {:?}", err);
            }
        });
    }
}
