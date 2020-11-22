extern crate chrono;

use caesar::*;

use chrono::Utc;

fn main() {
    test_data();
}

fn test_data() {
    let p = Utc::now().date();
    println!("p: {}", p.to_string());
    println!("Hello, world!");


    let now = Utc::now();
    println!("UTC now is: {}", now.timestamp());
    println!("UTC now is: {}", Utc::now().date().and_hms(1, 0, 0).timestamp_millis());
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!("UTC now in a custom format is: {}", now.format("%a %b %e %T %Y"));
}
