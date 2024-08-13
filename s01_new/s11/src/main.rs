use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // async
    println!("hello world");

    let h1 = tokio::spawn(async {
        let s = read_from_file1().await;
        println!("{}", s);
    });

    let h2 = tokio::spawn(async {
        let s = read_from_file2().await;
        println!("{}", s);
    });

    let _ = tokio::join!(h1, h2);
    println!("main end");
}

async fn read_from_file1() -> String {
    sleep(Duration::new(4,0));
    println!("Processing file1");
    String::from("file1")
}

async fn read_from_file2() -> String {
    sleep(Duration::new(4,0));
    println!("Processing file2");
    String::from("file2")
}