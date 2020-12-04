use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

use super::*;

pub struct Core {
    listen_addr: String,
    caesar_addr: String,
    token: Option<String>,
}

impl Core {
    pub fn new(listen_addr: String, caesar_addr: String) -> Core {
        Core {
            listen_addr,
            caesar_addr,
            token: None,
        }
    }

    fn discover(&self) -> Result<String, Box<dyn Error>> {
        let coc = Discover {
            server_type: "pc".to_string(),
        };
        let c = serde_json::to_string(&coc)?;
        let dis = MSG {
            msg_type: DISCOVER,
            data: c,
        };
        let req = serde_json::to_vec(&dis)?;

        let mut conn = TcpStream::connect(&self.caesar_addr)?;
        conn.write(&req)?;
        let mut buf = [0u8; 2048];
        let idx = conn.read(&mut buf)?;
        let buf = &buf[..idx];
        let resp: RegisterResp = serde_json::from_slice(buf)?;
        if !resp.success {
            return Err(Box::new(DiscoverError("Discover Error".to_string())));
        }

        let resp: DiscoverResp = serde_json::from_str(resp.data.unwrap().as_str())?;
        Ok(resp.server_src)
    }

    pub fn login(&mut self, account: String, password: String) -> Result<(), Box<dyn Error>> {
        let act = PCAccount {
            account,
            password,
            balance: None,
        };
        let req = serde_json::to_vec(&act)?;
        let msg = PCMsg {
            msg: req,
            typ: Login,
            token: None,
        };
        let req = serde_json::to_vec(&msg)?;
        let addr = self.discover()?;
        let mut conn = TcpStream::connect(addr)?;
        conn.write(&req)?;

        let mut buf = [0u8; 2048];
        let idx = conn.read(&mut buf)?;
        let buf = &buf[..idx];

        let resp: PCMsgResp = serde_json::from_slice(buf)?;
        if !resp.success {
            return Err(Box::new(PasswordErrorOrUserNofFound));
        }

        self.token = Some(resp.token.unwrap());
        Ok(())
    }

    pub fn create_account(&self, account: String, password: String) -> Result<(), Box<dyn Error>> {
        let act = PCAccount {
            account,
            password,
            balance: None,
        };
        let req = serde_json::to_vec(&act)?;
        let msg = PCMsg {
            msg: req,
            typ: CreateAccount,
            token: None,
        };
        let req = serde_json::to_vec(&msg)?;
        let addr = self.discover()?;
        let mut conn = TcpStream::connect(addr)?;
        conn.write(&req)?;

        let mut buf = [0u8; 2048];
        let idx = conn.read(&mut buf)?;
        let buf = &buf[..idx];

        let resp: PCMsgResp = serde_json::from_slice(buf)?;
        if !resp.success {
            return Err(Box::new(AccountNotFound));
        }

        Ok(())
    }

    pub fn balance_inquiry(&self) -> Result<(), Box<dyn Error>> {
        let token = match &self.token {
            None => {
                return Err(Box::new(Unauthorized));
            }
            Some(t) => t,
        };

        Ok(())
    }
}