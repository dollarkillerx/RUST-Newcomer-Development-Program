use std::cell::{RefCell, Cell};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ServerList {
    list: RefCell<Vec<ServerNode>>
}

#[derive(Debug)]
pub struct ServerNode {
    // server id
    pub server_id: String,
    // 服务注册地址
    pub src: String,
    // 服务注册  淘汰时间
    pub timestamp: Cell<i64>,
}

pub type MSGType = i32;

pub const REGISTER: MSGType = 1;
pub const DISCOVER: MSGType = 2;
pub const REGISTER_RESP: MSGType = 3;
pub const DISCOVER_RESP: MSGType = 4;

#[derive(Serialize, Deserialize, Debug)]
pub struct MSG {
    pub msg_type: MSGType,
    pub data: String, // JSON
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterNode {
    // ServerType
    pub server_type: String,
    // 服务ID
    pub server_id: String,
    // 服务地址
    pub server_addr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResp {
    pub server_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Discover {
    // 服务ID
    pub server_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscoverResp {
    pub server_id: String,
    pub server_src: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PCAccount {
    pub account: String,
    pub password: String,
    pub balance: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PCMsg {
    pub msg: String,
    pub typ: PCType,
}

#[derive(Serialize, Deserialize, Debug)]
enum PCType{
    Login,
    CreateAccount,

}