use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;

fn main() {
    test1();
    println!("==============================================");
    test2();
}

fn test1() {
    println!("Hello, world!");

    let mut children = vec![];
    for i in 0..10 {
        let t = thread::spawn(move || {
            println!("=========== {}", i);
            for i in 0..99 {
                println!("i: {}", i);
            }
        });
        children.push(t);
    }

    for t in children {
        t.join().unwrap();
    }
}

fn test2() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    for i in 0..100 {
        let sed = tx.clone();
        thread::spawn(move || {
            sed.send(i).unwrap();
        });
    }

    let mut op = Vec::with_capacity(100);
    for _ in 0..100 {
        op.push(rx.recv().unwrap())
    }

    println!("{:?}", op);
}