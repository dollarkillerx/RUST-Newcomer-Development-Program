use async_std::prelude::*;
use async_std::future;
use async_std::task;
use std::time::Duration;

async fn async_main() {
    let a = future::ready(1).delay(Duration::from_millis(2000));
    dbg!(a.await);
}

fn main() {
    let main = task::spawn(async_main());
    task::block_on(main);
}