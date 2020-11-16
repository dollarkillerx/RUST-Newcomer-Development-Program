use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    println!("Hello, world!");
    test_path();
    test_file();
    test_write();
    test_read_line();
}

fn test_path() {
    let path = Path::new(".");

    let display = path.display();
    println!("path: {}", display);

    let new_path = path.join("a").join("b");
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

fn test_file() {
    let path = Path::new("hello.txt");
    // let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("open file error: {}", why.to_string()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("could't read {}", why.to_string()),
        Ok(_) => println!("{}", s),
    }
}

static LOREM_IPSUM: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn test_write() {
    let path = Path::new("file.txt");
    let mut file = File::create(path).unwrap();
    file.write_all(LOREM_IPSUM.as_bytes()).unwrap();
}

fn test_read_line() {
    if let Ok(lines) = read_lines("file.txt") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}",ip);
            }
        }
    }
}

use std::io::{self,BufRead};
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}