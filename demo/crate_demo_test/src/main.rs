use std::env;
use std::fs;
use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::{Mutex, Arc};

fn main() {
    println!("Hello, world!");

    // test1()
    // test2();
    // test3()
    // test4()
    // test5()
    // test6()
    // test7()
    test8()
}


fn test1() {
    let p = env::current_dir().unwrap();
    let p = p.parent().unwrap();
    println!("{:?}", p);
}

fn test2() {
    let p = env::current_dir().unwrap();
    let mut path = p.as_path();
    let new_p = path.join("hello.toml");

    let m = fs::metadata(&new_p).unwrap();
    println!("path: {:?} metadata: {:#?}", new_p, m);
}

fn test3() {
    let p = String::from("hello world");
    if p != "" {
        if p == "hello world" {
            println!("success")
        }

        if let Some(idx) = p.find("world") {
            println!("{:?}", &p[idx..])
        }
    }
}

#[derive(Debug)]
struct User {
    name: Option<RefCell<String>>,
    age: u8,
}

fn test4() {
    let mut p = HashMap::new();

    for i in 0..5 {
        let dk = User {
            name: Some(RefCell::new(format!("dk_{}", i))),
            age: 18,
        };
        p.insert(format!("dk_{}", i), dk);
    }

    println!("{:#?}", p);

    println!("=====================================");
    for (k, mut v) in &p {
        if k == "dk_3" {
            let p = v.name.as_ref().take().unwrap();
            let c = p.borrow().clone();
            println!("{:?}", c);

            v.name.as_ref().unwrap().swap(&RefCell::new("sssspsp".to_string()));
        }
    }

    println!("{:#?}", p);
}

// #[derive(Debug)]
// struct User2 {
//     name: String,
//     age: u8,
// }
//
// fn test5() {
//     let mut p = HashMap::new();
//
//     for i in 0..5 {
//         let dk = User2 {
//             name: format!("dk_{}", i),
//             age: 18,
//         };
//         p.insert(format!("dk_{}", i), dk);
//     }
//
//     println!("{:#?}", p);
//
//     for (k, mut v) in &p {
//         if k == "dk_3" {
//             v.name = "hello world".to_string(),
//         }
//     }
//
//     println!("{:#?}", p);
// }

fn test6() {
    let p = Arc::new(Mutex::new(HashMap::new()));
    {
        let mut c = p.lock().unwrap();
        for i in 0..10 {
            c.insert(format!("hello_{}", i), format!("world_{}", i));
        }
    }

    {
        let c = p.lock().unwrap();
        let c = c.get("hello_1").unwrap();
        println!("c: {}", c);
    }

    {
        let c = p.lock().unwrap();
        for (k, v) in &*c {
            println!("key: {} val: {}", k, v);
        }
    }
}

fn test7() {
    let t1 = "Hello_ps".to_string();
    let c = t1.find("ps").unwrap();
    println!("{}", c);

    let px = "xxx".to_string();
    if let None = t1.find(px.as_str()) {
        println!("no")
    }

    println!("{}", px);
}

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Ping {
    ping: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Pang {
    pang: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Msg<T> {
    data: T,
}

#[derive(Serialize, Deserialize, Debug)]
enum MsgType {
    Login,
    Out,
}

fn test8() {
    let p = Msg { data: Ping { ping: "ping".to_string() } };
    let c = serde_json::to_string(&p).unwrap();
    println!("{}", c);

    // let a:Msg<Pang> = serde_json::from_str(c.as_str()).unwrap();
    // println!("{:?}",a);

    let cd = Msg{data: MsgType::Login};
    let c = serde_json::to_string(&cd).unwrap();
    println!("{}", c);
}