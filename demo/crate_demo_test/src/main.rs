use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");

    test2();
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