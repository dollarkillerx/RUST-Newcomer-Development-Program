use std::{io, str};
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let conn = UdpSocket::bind("0.0.0.0:0")?;  // :0  随机端口
    conn.connect("0.0.0.0:8082")?;

    loop {
        let mut input = String::new();
        println!("please you input: ");
        if io::stdin().read_line(&mut input).is_err() {
            println!("what are you fuck input?");
        }

        conn.send(input.as_bytes())?;

        let mut buf = [0; 512];
        conn.recv_from(&mut buf)?;
        let p = str::from_utf8(&buf).unwrap();
        println!("ser: {}", p);
    }

    // Ok(())
}
