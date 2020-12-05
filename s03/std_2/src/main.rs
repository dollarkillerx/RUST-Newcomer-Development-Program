use async_std::prelude::*;
use async_std::future;
use async_std::task;
use std::time::Duration;
use async_std::sync;

fn main() {
    let main = task::spawn(async_main());
    task::block_on(main);
}

async fn async_main() {
    test1().await;
    test2().await;
}

async fn test1() {
    let a = future::ready(1).delay(Duration::from_millis(2000));
    dbg!(a.await);
}

async fn test2() {
    let (sed, rev) = sync::channel(1);

    let sed1 = sed.clone();
    let f1 = task::spawn(async move {
        println!("send start");
        sed1.send(String::from("hello")).await;
        println!("send end")
    });

    let sed2 = sed.clone();
    let f2 = task::spawn(async move {
        println!("send start");
        sed2.send(String::from("hello")).await;
        println!("send end")
    });

    let rev1 = rev.clone();
    let f3 = task::spawn(async move {
        println!("rev start");
        let data = rev1.recv().await;
        println!("data: {:?}",data);
    });

    f1.await;
    f2.await;
    f3.await;
    println!("over");
    // let p = f1.join(f2).join(f3);
    // p.await;

    let r = rev.clone();
    let f5 = task::spawn(async move {
        println!("start");
        'a1:
        loop {
            let c = r.recv().await;
            match c {
                Err(e) => {
                    println!("e: {}",e.to_string());
                    break 'a1
                }
                Ok(a) => {
                    println!("msg: {}",a);
                }
            }

            // task::sleep(Duration::from_secs(1)).await;
        }
    });

    for i in 1..10 {
        let s = sed.clone();
        let f = task::spawn(async move {
            s.send(format!("f data: {}",i)).await;
        });
        println!("in: {}",i);
        f.await;
    }

    println!("p start");
    f5.await;
}