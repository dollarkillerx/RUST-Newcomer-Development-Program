use::tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = TcpStream::connect("127.0.0.1:8181").await?;

    // Write some data.
    conn.write_all(b"hello world!").await?;

    let mut buf = [0; 1024];

    // Read the data
    let n = conn.read(&mut buf).await?;
    println!("GOT: {}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}
