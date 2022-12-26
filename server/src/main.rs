#[tokio::main]
async fn main() {
    server::launch(&server::ServerArgs {}).await.unwrap();
}
