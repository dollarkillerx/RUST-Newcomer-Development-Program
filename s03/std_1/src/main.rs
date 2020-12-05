use std::time;

use async_std::sync;
use async_std::task;
use async_std::prelude::*;
use async_std::future;

fn main() {
    println!("Hello, world!");
    let main = task::spawn(async_main());
    task::block_on(main);
}

async fn async_main() {
    println!("async main start");

    test1().await;
    test3().await;
    test2().await;
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

async fn test1() {
    let f1 = async { add(15, 26) };
    let r = f1.await;
    println!("test1 r: {}", r);
    println!();
}

async fn test2() {
    let f1 = future::ready(add(56, 59));
    let r = f1.await;
    println!("test2 r: {}", r);
    println!();
}

async fn test3() {
    // delay 延迟执行
    let f1 = future::ready(add(56, 59)).delay(time::Duration::from_secs(1));
    let r = f1.await;
    println!("test3 r: {}", r);
    println!();
}

async fn test4() {
    // delay 延迟执行
    let f1 = future::ready(add(56, 59)).timeout(time::Duration::from_secs(2));
    let r = f1.await;
    println!("test3 r: {}", r);
    println!();
}


