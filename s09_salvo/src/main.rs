mod route;
mod api;

use std::env;
use salvo::prelude::*;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap(); // 读取 .env
    tracing_subscriber::fmt().init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    info!("db_url: {}", db_url);

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(route::route()).await;
}