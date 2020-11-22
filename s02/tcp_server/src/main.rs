use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut conn: TcpStream) -> Result<(), Error> {
    let mut buf = [0; 512];
    'ma:
    loop {
        // buf = [0; 512];  // clear
        let bytes_read = conn.read(&mut buf)?;
        if bytes_read == 0 {
            continue;
        }

        let s = String::from_utf8_lossy(&buf[..bytes_read]);
        match s.trim() {
            "exit" => {
                break 'ma;
            }
            _ => {
                println!("{:?}", s);
                conn.write_all(&buf[..bytes_read])?;
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8081")?;
    println!("Listener on: 0.0.0.0:8081");
    let mut thread_vec = vec![];

    for conn in listener.incoming() {
        let conn = match conn {
            Ok(t) => t,
            Err(e) => {
                println!("conn error: {}", e);
                continue;
            }
        };

        thread_vec.push(thread::spawn(move || {
            if let Err(e) = handle_client(conn) {
                println!("err: {}", e);
            }
        }));
    }

    for h in thread_vec {
        h.join().unwrap();
    }

    Ok(())
}


