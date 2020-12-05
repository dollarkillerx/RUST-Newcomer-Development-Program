use std::time;

use async_std::sync;
use async_std::task;
use async_std::prelude::*;
use async_std::future;
use std::ops::Deref;

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
    test4().await;
    test5().await;
    test6().await;
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
    println!("test3 r: {:?}", r); // Res
    println!();
}


// 合并操作
async fn test5() {
    let f1 = future::ready(add(19,56));
    let f2 = future::ready(add(898,5656));
    let f3 = future::ready(add(898,566));
    let f5 = future::ready(add(56, 48));
    let f6 = future::ready(add(12,34));
    let ff = f1.join(f2).join(f3).join(f5).join(f6);
    let rp = ff.await;
    println!("r: {:#?}",rp);
}

// 竞争操作
async fn test6() {
    // 先完成着成功
    let f1 = future::ready(add(1,2));
    let f2 = future::ready(add(2,3)).delay(time::Duration::from_secs(1));

    let f = f1.race(f2);// 那个先完成 按拉个来
    let val = f.await;
    println!("{}",val);
}