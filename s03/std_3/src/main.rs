use async_std::task;
use std::time::Duration;

fn main() {
    // test1();
    test2();
}

fn test1() {
    println!("Hello, world!");
    let main = task::spawn(async_main());
    task::block_on(main);

    let p = task::spawn(async move {
        panic!("panic test");
    });
    task::block_on(p);
    println!("panic test");
}

fn test2() {
    let t = task::spawn(pcr());
    task::block_on(t);
}

async fn pcr() {
    let _h = task::spawn(async move {
        loop {
            task::sleep(Duration::from_secs(1)).await;
            println!("abc");
        }
    });

    loop {
        task::sleep(Duration::from_secs(1)).await;
    }
}

async fn async_main() {
    let f1 = task::spawn(loop_print());
    let f2 = task::spawn(loop_print2());

    f1.await;
    println!("sp");
    f2.await;

    // let f = f1.join(f2);
    // f.await;
}

async fn loop_print() {
    loop {
        task::sleep(Duration::from_secs(3)).await;
        println!("Lp1");
    }
}

async fn loop_print2() {
    loop {
        task::sleep(Duration::from_secs(1)).await;
        println!("Lp2");
    }
}