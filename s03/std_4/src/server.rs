use super::*;
use futures::select; // 多路复用
use futures::{channel::mpsc};
use async_std::{
    io::BufReader,
    net::{TcpListener,TcpStream,ToSocketAddrs},
    prelude::*,
    task,
};
use async_std::sync::{Arc, Sender};
use std::collections::HashMap;


pub struct Server {}

impl Server {
    pub fn main() -> Result<()> {
        task::block_on(Self::accept_loop("0.0.0.0:8089"))
    }

    async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        let (broker_sender, broker_receiver) = mpsc::unbounded();

    }

    async fn broker_loop(mut events: mpsc::UnboundedReceiver<Event>) {
        let mut peer: HashMap<String, Arc<TcpStream>> = HashMap::new();

        loop {
            let event = select!{
                // event = events.next().f
            };
        }
    }
}

enum Event {
    NewPeer {
        name: String,
        stream: Arc<TcpStream>,
        shutdown: bool,
    },
    Message {
        from: String,
        msg: String,
    }
}