#[tokio::main]
async fn main() {
    client::launch(&client::ClientArgs {  }).await.unwrap();
}
