extern crate chrono;

use std::error::Error;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

use chrono::Utc;

use caesar::*;
use caesar::core;
use common::error::{CommonError::*};

fn main() -> Result<(), Box<dyn Error>> {
    // test_data();

    register_server()?;

    Ok(())
}

fn register_server() -> Result<(), Box<dyn Error>> {
    let listen = TcpListener::bind("0.0.0.0:8189")?;
    for conn in listen.incoming() {
        let con = conn?;
        thread::spawn(move || {
            if let Err(e) = handle_client(con) {
                println!("server error: {:?}", e);
            }
        });
    }
    Ok(())
}

fn handle_client(mut conn: TcpStream) -> Result<(), Box<dyn Error>> {
    // let mut buf:Vec<u0> = vec![0u8;4096];
    // let idx = conn.read(&mut buf)?;
    let mut buf = [0u8; 4096];
    let idx = conn.read(&mut buf)?;
    let buf = &buf[..idx];


    let msg: define::MSG = serde_json::from_slice(buf)?;
    match msg.msg_type {
        define::REGISTER => {  // 服务注册
            core::register(&msg);
        }
        define::DISCOVER => {  // 服务发现
            core::discover(&msg);
        }
        _ => {
            return Err(Box::new(UndefinedBehavior));
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
