mod server;
mod router;
mod handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let server = server::Server::new("0.0.0.0:8189");
    server.run().await;
    Ok(())
}
