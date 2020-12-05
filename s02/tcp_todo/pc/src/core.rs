use std::cell::Cell;
use std::collections::HashMap;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

use lazy_static;
use uuid::Uuid;

use super::*;

lazy_static! {
    static ref DB: Mutex<HashMap<String, Account>> = {
        let mut db = HashMap::new();
        db.insert("dollarkiller@dollarkiller.com".to_string(), Account{
            account: String::from("dollarkiller@dollarkiller.com"),
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
    account: String,
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
        self.core()
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
        if !resp.success {
            panic!("server register error")
        }
        {
            let d = &mut self.server_id.lock().map_err(|_| MutexError)?;
            print!("server id {:?}", d);
            **d = Some(resp.server_id.clone());  // 注册服务ID  **好难看啊
        }

        // run heartbeat
        let addr = self.listen_addr.clone();
        let caesar = self.caesar_addr.clone();
        println!("register success id: {}", resp.server_id);
        thread::spawn(|| {
            Self::heartbeat(resp.server_id, addr, caesar);
        });

        Ok(())
    }

    // 这里设计的放送心跳 需要获取当前服务的负载信息, 他是存储在这个struct中   spawn回移交所有权 不能用Self
    fn heartbeat(server_id: String, server_addr: String, caesar_addr: String) {
        loop {
            sleep(Duration::from_secs(3));
            // 定时发送心跳确保存活

            let node = RegisterNode {
                server_id: server_id.clone(),
                server_addr: server_addr.clone(),
                server_type: "pc".to_string(),
            };

            let node_msg = serde_json::to_string(&node).expect("what fuck");

            let msg = MSG {
                msg_type: REGISTER,
                data: node_msg,
            };

            let mut conn = match TcpStream::connect(&caesar_addr) {
                Ok(r) => r,
                Err(e) => {
                    println!("conn errror {}", e);
                    continue;
                }
            };

            match conn.set_ttl(100) {
                Err(e) => {
                    println!("set ttl {}", e);
                    continue;
                }
                _ => {}
            }

            let msg_data = match serde_json::to_vec(&msg) {
                Ok(r) => r,
                Err(e) => {
                    println!("to vec {}", e);
                    continue;
                }
            };

            match conn.write(msg_data.as_slice()) {
                Err(e) => {
                    println!("write {}", e);
                    continue;
                }
                _ => {}
            }


            let mut reader = BufReader::new(&conn);
            let mut buf: [u8; 2048] = [0; 2048];
            let idx = match reader.read(&mut buf) {
                Ok(r) => r,
                Err(e) => {
                    println!("e: {}", e);
                    continue;
                }
            };
            if idx == 0 {
                println!("reader idx is 0");
                continue;
            }
            let resp_data = &buf[..idx];
            let resp: RegisterResp = match serde_json::from_slice(resp_data) {
                Ok(r) => r,
                Err(e) => {
                    println!("e: {}", e);
                    continue;
                }
            };
            if resp.success {
                println!("heartbeat success");
                continue
            }
            println!("heartbeat error: {:?}",resp.data);
        }
    }

    // 核心业务
    fn core(&mut self) -> Result<(), Box<dyn Error>> {
        let client = TcpListener::bind(&self.listen_addr)?;
        println!("listen on {}", &self.listen_addr);
        for conn in client.incoming() {
            let conn = conn?;
            conn.set_ttl(200)?;
            thread::spawn(move || {
                match Self::core_handle_client(conn) {
                    Err(e) => {
                        println!("err: {}", e);
                    }
                    _ => {}
                }
            });
        }
        Ok(())
    }

    fn core_handle_client(mut conn: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buf = [0u8; 2048];
        let idx = conn.read(&mut buf)?;
        let buf = &buf[..idx];
        let msg: PCMsg = serde_json::from_slice(buf)?;


        match msg.typ {
            CreateAccount => {
                match Self::create_account(&msg.msg.unwrap()) {
                    Err(e) => {
                        let msg = PCMsgResp {
                            success: false,
                            error_msg: Some(e.to_string()),
                            token: None,
                            balance: None,
                        };
                        let resp_data = serde_json::to_vec(&msg)?;
                        conn.write(resp_data.as_slice())?;
                    }
                    _ => {
                        let msg = PCMsgResp {
                            success: true,
                            error_msg: None,
                            token: None,
                            balance: None,
                        };
                        let resp_data = serde_json::to_vec(&msg)?;
                        conn.write(resp_data.as_slice())?;
                    }
                }
            }
            Login => {
                match Self::login(&msg.msg.unwrap()) {
                    Err(e) => {
                        let msg = PCMsgResp {
                            success: false,
                            error_msg: Some(e.to_string()),
                            token: None,
                            balance: None,
                        };
                        let resp_data = serde_json::to_vec(&msg)?;
                        conn.write(resp_data.as_slice())?;
                    }
                    Ok(token) => {
                        let msg = PCMsgResp {
                            success: true,
                            error_msg: None,
                            token: Some(token),
                            balance: None,
                        };
                        let resp_data = serde_json::to_vec(&msg)?;
                        conn.write(resp_data.as_slice())?;
                    }
                }
            }
            Deposits => {
                match Self::deposits(&msg.msg.unwrap(), msg.token.clone()) {
                    Err(e) => {
                        let msg = PCMsgResp {
                            success: false,
                            error_msg: Some(e.to_string()),
                            token: None,
                            balance: None,
                        };
                        let resp_data = serde_json::to_vec(&msg)?;
                        conn.write(resp_data.as_slice())?;
                    }
                    _ => {
                        let msg = PCMsgResp {
                            success: true,
                            error_msg: None,
                            token: None,
                            balance: None,
                        };
                        let resp_data = serde_json::to_vec(&msg)?;
                        conn.write(resp_data.as_slice())?;
                    }
                }
            }
            Withdrawal => {
                match Self::withdrawal(&msg.msg.unwrap(), msg.token.clone()) {
                    Err(e) => {
                        let msg = PCMsgResp {
                            success: false,
                            error_msg: Some(e.to_string()),
                            token: None,
                            balance: None,
                        };
                        let resp_data = serde_json::to_vec(&msg)?;
                        conn.write(resp_data.as_slice())?;
                    }
                    _ => {
                        let msg = PCMsgResp {
                            success: true,
                            error_msg: None,
                            token: None,
                            balance: None,
                        };
                        let resp_data = serde_json::to_vec(&msg)?;
                        conn.write(resp_data.as_slice())?;
                    }
                }
            }
            BalanceInquiry => {
                match Self::balance_inquiry(msg.token.clone()) {
                    Err(e) => {
                        let msg = PCMsgResp {
                            success: false,
                            error_msg: Some(e.to_string()),
                            token: None,
                            balance: None,
                        };
                        let resp_data = serde_json::to_vec(&msg)?;
                        conn.write(resp_data.as_slice())?;
                    }
                    Ok(balance) => {
                        let msg = PCMsgResp {
                            success: true,
                            error_msg: None,
                            token: None,
                            balance: Some(balance),
                        };
                        let resp_data = serde_json::to_vec(&msg)?;
                        conn.write(resp_data.as_slice())?;
                    }
                }
            }
        }
        Ok(())
    }

    fn create_account(data: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        let account: PCAccount = serde_json::from_slice(data)?;
        // 创建逻辑
        let mut p = DB.lock()?;
        p.insert(account.account.clone(), Account {
            account: account.account.clone(),
            password: account.password.clone(),
            balance: Cell::new(0.0),
        });
        Ok(())
    }

    fn login(data: &Vec<u8>) -> Result<String, Box<dyn Error>> {
        let account: PCAccount = serde_json::from_slice(data)?;

        let p = DB.lock()?;
        let pc = match p.get(&account.account) {
            None => {
                return Err(Box::new(PasswordErrorOrUserNofFound));
            }
            Some(data) => data,
        };
        if pc.password == account.password {
            let token = Self::generate_token(account.account.clone())?;
            return Ok(token);
        }
        Err(Box::new(PasswordErrorOrUserNofFound))
    }

    fn generate_token(account: String) -> Result<String, Box<dyn Error>> {
        let mut db = TOKEN.lock()?;
        let token = Uuid::new_v4().to_string();
        db.insert(token.clone(), account);
        Ok(token)
    }

    fn deposits(data: &Vec<u8>, toke: Option<String>) -> Result<(), Box<dyn Error>> {
        // check token
        let account_id = Self::check_token(toke)?;

        let msg: PCAccount = serde_json::from_slice(data)?;
        let money = match msg.balance {
            None => {
                return Err(Box::new(ParameterError));
            }
            Some(t) => t,
        };
        // get account
        let b = DB.lock()?;
        let account = match b.get(&account_id) {
            None => {
                return Err(Box::new(AccountNotFound));
            }
            Some(e) => e,
        };

        account.balance.set(account.balance.get() + money);
        Ok(())
    }

    fn withdrawal(data: &Vec<u8>, toke: Option<String>) -> Result<(), Box<dyn Error>> {
        // check token
        let account_id = Self::check_token(toke)?;

        let msg: PCAccount = serde_json::from_slice(data)?;
        let money = match msg.balance {
            None => {
                return Err(Box::new(ParameterError));
            }
            Some(t) => t,
        };
        // get account
        let b = DB.lock()?;
        let account = match b.get(&account_id) {
            None => {
                return Err(Box::new(AccountNotFound));
            }
            Some(e) => e,
        };

        if account.balance.get() < money {
            return Err(Box::new(InsufficientBalance(format!("InsufficientBalance balance: {}", account.balance.get()))));
        }
        account.balance.set(account.balance.get() - money);
        Ok(())
    }

    fn check_token(token: Option<String>) -> Result<String, Box<dyn Error>> {
        let token = match token {
            Some(p) => p,
            None => {
                return Err(Box::new(Unauthorized));
            }
        };
        let db = TOKEN.lock()?;
        let account = match db.get(&token) {
            Some(p) => p,
            None => {
                return Err(Box::new(Unauthorized));
            }
        };
        Ok(account.clone())
    }

    fn balance_inquiry(toke: Option<String>) -> Result<f32, Box<dyn Error>> {
        let account_id = Self::check_token(toke)?;
        let db = DB.lock()?;
        let ac = match db.get(&account_id) {
            None => {
                return Err(Box::new(PasswordErrorOrUserNofFound));
            }
            Some(data) => data,
        };

        Ok(ac.balance.get())
    }
}