use async_std::prelude::*;
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[async_std::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}