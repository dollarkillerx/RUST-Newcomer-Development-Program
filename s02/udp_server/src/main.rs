use std::net::{SocketAddr, UdpSocket};
use std::thread;

fn main() -> std::io::Result<()> {
    let listen = UdpSocket::bind("0.0.0.0:8082")?;
    println!("Listen on 0.0.0.0:8082");

    loop {
        let socket = listen.try_clone()?;
        let mut buf = [0u8; 512];
        let (idx, src) = socket.recv_from(&mut buf)?;
        if idx == 0 {
            continue;
        }
        thread::spawn(move || {
            if udp_core(buf, idx, socket, src).is_err() {
                println!("udp core error");
            };
        });
    }
}

fn udp_core(buf: [u8; 512], idx: usize, conn: UdpSocket, ser: SocketAddr) -> std::io::Result<usize> {
    let buf = &buf[..idx];
    let p = String::from_utf8_lossy(buf);
    let mut p = p.trim().to_string();
    p.push_str("   reps");
    println!("msg: {}", p);
    conn.send_to(p.as_bytes(), ser)
}

