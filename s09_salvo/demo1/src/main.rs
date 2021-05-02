use salvo::prelude::*;
use demo1::*;

#[tokio::main]
async fn main() {
    let router = router::router();
    Server::new(router).bind(([0, 0, 0, 0], 7878)).await;
}

