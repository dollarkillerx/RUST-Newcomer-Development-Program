use std::fs::{File, OpenOptions};
use std::io::{Read, ErrorKind, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() {
    // test4();
    // test5();
    // test4();

    test6();
}

fn test4() {
    // File::open is read-only mode
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("text.txt")
        .unwrap();

    f.write_all(b"some data").unwrap();
}

fn test5() {
    // File::open is read-only mode
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("text.txt")
        .unwrap();

    f.write_all(b"some data").unwrap();
}

fn test3() {
    let mut f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Error opening file: {:?}", error);
        }
    });

    f.write_all("hello world".to_string().as_bytes()).unwrap_or_else(|error| {
        let mut r = String::new();
        f.read_to_string(&mut r).unwrap();
        println!("err: {:?}, data: {}", error, r);
    });
}

fn test1() {
    let mut f = File::open("a.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("a.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error Creating File: {:?}", e),
            },
            other_error => panic!("Error opening the file: {:?}", other_error),
        }
    };


    f.write_all("a.txt".to_string().as_bytes()).unwrap();
}

fn test2() {
    let mut read = String::new();
    if let Ok(mut f) = File::open("src/main.rs") {
        f.read_to_string(&mut read).unwrap();
    }
    println!("main.rs: {}", read);
}

fn test6() {
    let test1 = vec![1, 2, 48, 59, 78, 1115, 15, 3, -1];
    println!("max: {:?}", max_int_list(&test1));

    let s_list = vec!["1".to_string(), "2".to_string(), "3".to_string(), "0009".to_string(), "sd".to_string()];
    println!("s max: {:?}", max_str(&s_list));
}

fn max_int_list(i_list: &Vec<i32>) -> i32 {
    let mut p = 0;
    for &i in i_list {
        if i > p {
            p = i;
            continue;
        }
    }
    p
}

fn max_str(s_list: &Vec<String>) -> String {
    let mut max = String::new();
    for s in s_list {
        if s > &max {
            max = s.clone();
        }
    }

    max
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<U> Point<i32, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T, U> Point<T, U> {
    fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

