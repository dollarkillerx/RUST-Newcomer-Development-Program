extern crate chrono;

use caesar::*;
use chrono::Utc;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::io::Error;

fn main() -> std::io::Result<()>{
    // test_data();

    register_server()?;

    Ok(())
}

fn register_server() -> std::io::Result<()>{
    let listen = TcpListener::bind("0.0.0.0:8089")?;
    for conn in listen.incoming() {
        if conn.is_err() {
            continue;
        }

        thread::spawn(move || {
            handle_client(conn.unwrap());
        });
    }
    Ok(())
}

fn handle_client(mut conn: TcpStream) -> std::io::Result<()>{
    // let mut buf:Vec<u0> = vec![0u8;4096];
    // let idx = conn.read(&mut buf)?;
    let mut buf = [0u8;4096];
    let idx = conn.read(&mut buf)?;
    let buf = &buf[..idx];


    let msg: define::MSG = serde_json::from_slice(buf)?;
    match msg.msg_type {
        define::REGISTER => {

        },
        _ => {
            Err(Error::new())
        }
    }

    Ok(())
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
