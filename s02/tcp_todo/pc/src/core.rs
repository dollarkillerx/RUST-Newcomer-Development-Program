use super::*;
use std::sync::{Mutex, Arc};
use std::net::TcpStream;
use std::io::{Write, BufReader, Read};

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
        conn.set_ttl(100);
        let msg_data = serde_json::to_vec(&msg)?;
        conn.write(msg_data.as_slice());

        let mut reader = BufReader::new(&conn);
        let mut buf: [u8; 2048] = [0; 2048];
        let idx = reader.read(&mut buf)?;
        if idx == 0 {
            return Err(Box::new(RegisterError("reader idx is 0".to_string())));
        }

        let resp_data = &buf[..idx];
        let resp: RegisterResp = serde_json::from_slice(resp_data)?;

        let d = &mut self.server_id.lock().map_err(|_| MutexError)?;
        print!("{:?}", d);
        **d = Some(resp.server_id);  // what fuck? 这么恶心
        Ok(())
    }
}