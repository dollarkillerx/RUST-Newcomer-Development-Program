// RC 在单线程中 让一个值可以有多个所有者  ARC是他的多线程版本

use std::cell::{Cell, RefCell};
// 拥有内部可变性
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

struct User {
    name: String,
    balance: Cell<i32>,
}


impl User {
    fn new(name: String, balance: i32) -> User {
        User {
            name,
            balance: Cell::new(balance),
        }
    }
    fn work(&self) {
        self.balance.set(self.balance.get() + 18);
    }
    fn whoring(&self) -> Result<(), String> {
        if self.balance.get() < 500 {
            return Err(format!("not money {}", self.balance.get()));
        }
        self.balance.set(self.balance.get() - 500);
        Ok(())
    }
}

fn main() {
    test_rc();
    test_arc();
}

fn test_rc() {
    let d = Rc::new(User::new("dollarkiller".to_string(), 0));
    let d1 = d.clone();
    let p = d1.clone();
    match d.whoring() {
        Ok(()) => {}
        Err(e) => {
            println!("{}", e);
        }
    }


    for _ in 0..100 {
        p.work();
    }

    match p.whoring() {
        Ok(()) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
}


fn test_arc() {
    let mut work_map = vec![];
    let d = Arc::new(Mutex::new(User::new("dollarkiller".to_string(), 0)));
    // 并发去工作
    for _ in 0..100 {
        let p = d.clone();
        work_map.push(RefCell::new(thread::spawn(move || {
            p.lock().unwrap().work();
        })));
    }

    for i in work_map {
        i.join().unwrap();
    }

    match d.lock().unwrap().whoring() {
        Ok(()) => {}
        Err(e) => {
            println!("{}", e);
        }
    };
}