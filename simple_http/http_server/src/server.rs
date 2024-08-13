use std::sync::Arc;
use http::http_request::{HttpRequest,};
use tokio::io::{AsyncReadExt};
use tokio::net::TcpListener;
use crate::router::Router;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl <'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server {
            socket_addr
        }
    }

    pub async fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).await.unwrap();
        println!("Listening on http://{}", self.socket_addr);

        loop {
            let (mut socket, _) = connection_listener.accept().await.unwrap();
            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                loop {
                    let bytes_read = socket.read(&mut buffer).await.unwrap();
                    if bytes_read == 0 {
                        break;
                    }
                    let req: HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();
                    let socket = Arc::new(&socket);
                    Router::router(req,socket);
                }
            });
        }
    }
}