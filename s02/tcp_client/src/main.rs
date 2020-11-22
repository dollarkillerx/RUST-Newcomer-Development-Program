use std::io;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut conn = TcpStream::connect("127.0.0.1:8081")?;
    conn.set_ttl(100).expect("ttl timeout");
    loop {
        let mut input = String::new();
        println!("Please input text: ");
        io::stdin().read_line(&mut input)?;

        if input.trim() == "exit" {
            break;
        }

        conn.write(input.trim().as_bytes())?;

        let mut reader = BufReader::new(&conn);
        // let mut buffer: Vec<u8> = Vec::new();
        // reader.read_until(b'\n',&mut buffer) // 读到定界符截至
        let mut buf: [u8; 512] = [0; 512];
        let p = reader.read(&mut buf)?;
        if p == 0 {
            continue;
        }
        println!("{}", String::from_utf8_lossy(&buf).trim());
    }

    Ok(())
}
