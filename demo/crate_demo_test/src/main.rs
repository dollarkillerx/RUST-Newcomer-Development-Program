use std::env;
use std::fs;
use std::collections::HashMap;
use std::cell::RefCell;

fn main() {
    println!("Hello, world!");

    // test1()
    // test2();
    // test3()
    test4()
    // test5()
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