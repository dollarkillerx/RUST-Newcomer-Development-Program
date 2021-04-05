use std::fs::File;
use std::io::Read;

fn main() {
    let mut read = String::new();
    if let Ok(mut f) = File::open("src/main.rs") {
        f.read_to_string(&mut read).unwrap();
    }
    println!("main.rs: {}", read);
}
