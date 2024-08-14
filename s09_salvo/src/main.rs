mod route;
mod api;
mod entity;

use std::env;
use std::time::Duration;
use salvo::prelude::*;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::{info, log};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap(); // 读取 .env
    tracing_subscriber::fmt().init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    info!("db_url: {}", db_url);

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await.unwrap();
    let state = AppState { conn: db };

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(route::route(state)).await;
}

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}