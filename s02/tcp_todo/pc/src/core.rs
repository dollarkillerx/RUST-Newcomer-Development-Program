use super::*;
use std::sync::{Mutex, Arc};
use std::net::TcpStream;
use std::io::{Write, BufReader, Read};
use std::thread::{self, sleep};
use std::time::Duration;
use std::cell::Cell;
use std::collections::HashMap;

lazy_static! {
    static ref DB: Mutex<HashMap<String, Account>> = {
        let mut db = HashMap::new();
        db.insert("dollarkiller@dollarkiller.com".to_string(), Account{
            email: String::from("dollarkiller@dollarkiller.com"),
            password: String::from("dollarkiller"),
            balance: Cell::new(12.0),
        });
        Mutex::new(db)
    };

    static ref TOKEN: Mutex<HashMap<String, String>> = {
        let db = HashMap::new();
        Mutex::new(db)
    };
}

#[derive(Debug)]
struct Account {
    email: String,
    password: String,
    balance: Cell<f32>,
}

pub struct Core {
    listen_addr: String,
    caesar_addr: String,
    server_id: Arc<Mutex<Option<String>>>,
}

impl Core {
    pub fn new(listen_addr: String, caesar_addr: String) -> Self {
        Core {
            listen_addr,
            caesar_addr,
            server_id: Arc::new(Mutex::new(None)),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // register server
        self.register()?;

        // run core
        Ok(())
    }

    fn register(&mut self) -> Result<(), Box<dyn Error>> {
        let node = RegisterNode {
            server_id: "".to_string(),
            server_addr: self.listen_addr.clone(),
            server_type: "pc".to_string(),
        };
        let node_msg = serde_json::to_string(&node)?;

        let msg = MSG {
            msg_type: REGISTER,
            data: node_msg,
        };

        let mut conn = TcpStream::connect(&self.caesar_addr)?;
        conn.set_ttl(100)?;
        let msg_data = serde_json::to_vec(&msg)?;
        conn.write(msg_data.as_slice())?;

        let mut reader = BufReader::new(&conn);
        let mut buf: [u8; 2048] = [0; 2048];
        let idx = reader.read(&mut buf)?;
        if idx == 0 {
            return Err(Box::new(RegisterError("reader idx is 0".to_string())));
        }

        let resp_data = &buf[..idx];
        let resp: RegisterResp = serde_json::from_slice(resp_data)?;

        {
            let d = &mut self.server_id.lock().map_err(|_| MutexError)?;
            print!("server id {:?}", d);
            **d = Some(resp.server_id.clone());  // 注册服务ID  **好难看啊
        }

        // run heartbeat
        let addr = self.listen_addr.clone();
        thread::spawn(|| {
            Self::heartbeat(resp.server_id, addr);
        });
        Ok(())
    }

    // 这里设计的放送心跳 需要获取当前服务的负载信息, 他是存储在这个struct中   spawn回移交所有权 不能用Self
    fn heartbeat(server_id: String, server_addr: String) {
        loop {
            sleep(Duration::from_secs(3));
            // 定时发送心跳确保存活

            let node = RegisterNode {
                server_id: server_id.clone(),
                server_addr: server_addr.clone(),
                server_type: "pc".to_string(),
            };
            let node_msg = serde_json::to_string(&node).unwrap();

            let msg = MSG {
                msg_type: REGISTER,
                data: node_msg,
            };

            let mut conn = match TcpStream::connect(&server_addr) {
                Ok(r) => r,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            match conn.set_ttl(100) {
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
                _ => {}
            }

            let msg_data = match serde_json::to_vec(&msg) {
                Ok(r) => r,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            match conn.write(msg_data.as_slice()) {
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
                _ => {}
            }
        }
    }
}