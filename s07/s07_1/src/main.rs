use tide::Request;
use tide::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[async_std::main]
async fn main() ->tide::Result<()> {
    let mut app = tide::new();

    // middlewares 做的不太好 不好玩
    app.at("/hello").get(hello); // 当有多个get 最后一个get会覆盖 ， 这个不想Gin

    println!("run in: 127.0.0.1:8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn logs(mut req: Request<()>) ->tide::Result {
    println!("logs: {} , {}",req.url().to_string(), req.peer_addr().unwrap().to_string());

    Ok("".into())
}

async fn hello(mut req: Request<()>) ->tide::Result {
    println!("hello: {} , {}",req.url().to_string(), req.peer_addr().unwrap().to_string());

    Ok("hello world".into())
}
