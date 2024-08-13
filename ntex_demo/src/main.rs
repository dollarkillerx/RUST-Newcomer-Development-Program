mod errors;
mod models;
mod api;

use std::env;
use std::sync::Arc;
use ntex::web::{self, middleware, App, HttpServer};
use crate::errors::CustomError;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use log::info;
use crate::api::article::{delete_article, get_article, get_articles, new_article, search_article, update_article};

// ntex 中整个程序共享数据
#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap(); // 读取 .env

    env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    info!("db_url: {}", db_url);

    // 共享数据
    let app_state = Arc::new(AppState{
        db_pool: PgPoolOptions::new().
            max_connections(10).
            connect(&db_url).
            await.unwrap()
    });

    HttpServer::new(move || {
        App::new().wrap(middleware::Logger::default()).
            state(Arc::clone(&app_state)).
            service(index).service(err).
            configure(|cfg| route(cfg))

    }).bind("127.0.0.1:8741")?.run().await
}

#[web::get("/")]
async fn index() -> String {
    "hello world".into()
}

#[web::get("/error")]
async fn err() -> Result<String, errors::CustomError> {
    Err(CustomError::NotFound("not found".into()))
}


fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // .route("/{id}", web::get().to(article::view::get_article))
        web::scope("/api")
            .service(get_article)
            .service(get_articles)
            .service(new_article)
            .service(delete_article)
            .service(update_article)
            .service(search_article)
    );
}