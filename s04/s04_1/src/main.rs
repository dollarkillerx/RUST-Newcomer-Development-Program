use std::error::Error;
use sqlx::postgres::PgPoolOptions;
use async_std::prelude::*;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[async_std::main]
async fn main() -> Result<()> {
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("").await?;

    // Make a simple query to return the given parameter
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    Ok(())
}
