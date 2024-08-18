mod route;
mod api;
mod entity;
mod models;
mod errors;
mod storage;
mod enums;
mod app_state;

use std::env;
use std::sync::Arc;
use salvo::prelude::*;
use crate::app_state::AppState;
use crate::storage::storage::Storage;

#[tokio::main]
async fn main() {
    // dotenvy::dotenv().unwrap(); // 读取 .env
    let address = env::var("ADDRESS").expect("DATABASE_URL must be set");

    tracing_subscriber::fmt().init();
    let state = AppState { storage: Arc::new(Storage::default().await) };
    let acceptor = TcpListener::new(address).bind().await;
    Server::new(acceptor).serve(route::route(state)).await;
}

