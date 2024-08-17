mod route;
mod api;
mod entity;
mod models;
mod errors;
mod storage;
mod enums;
mod app_state;

use std::sync::Arc;
use salvo::prelude::*;
use crate::app_state::AppState;
use crate::storage::storage::Storage;

#[tokio::main]
async fn main() {
    // dotenvy::dotenv().unwrap(); // 读取 .env
    tracing_subscriber::fmt().init();
    let state = AppState { storage: Arc::new(Storage::default().await) };
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(route::route(state)).await;
}

