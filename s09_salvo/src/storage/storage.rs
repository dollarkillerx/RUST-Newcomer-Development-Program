use std::env;
use std::sync::Arc;
use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::info;

#[derive(Debug, Clone)]
pub struct Storage {
    pub(crate) db: DatabaseConnection,
    pub(crate) redis_conn: Arc<redis::Client>
}

impl Storage {
    pub async fn default() -> Self {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
        info!("db_url: {} redis_url: {}", db_url, redis_url);

        // database connection
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

        // redis connection
        let redis = Arc::new(redis::Client::open(redis_url).expect("Redis client creation failed"));

        Self {
            db,
            redis_conn: redis
        }
    }
}