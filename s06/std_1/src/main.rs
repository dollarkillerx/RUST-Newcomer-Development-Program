use std::error::Error;

extern crate redis;

use redis::{AsyncCommands, RedisResult};
use futures::prelude::*;
pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[async_std::main]
async fn main() -> Result<()> {
    hello_world().await?;
    fetch_an_integer().await?;
    Ok(())
}

async fn hello_world() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}

async fn fetch_an_integer() -> Result<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection().await?;

    con.set("key1", b"foo").await?;
    redis::cmd("SET").arg(&["key2", "bar"]).query_async(&mut con).await?;

    let result:RedisResult<(String,String)> = redis::cmd("MGET").
        arg(&["key1", "key2"]).
        query_async(&mut con).
        await;

    println!("resp: {:#?}", result);

    Ok(())

    // let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    // let mut con = client.get_async_connection().await?;
    //
    // con.set("key1", b"foo").await?;
    //
    // redis::cmd("SET").arg(&["key2", "bar"]).query_async(&mut con).await?;
    //
    // let result = redis::cmd("MGET")
    //     .arg(&["key1", "key2"])
    //     .query_async(&mut con)
    //     .await;
    // assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));
    // println!("rp: {:#?}",result);
    // Ok(())
}


