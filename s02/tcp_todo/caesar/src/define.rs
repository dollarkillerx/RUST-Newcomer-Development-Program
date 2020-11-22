use std::cell::RefCell;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug)]
pub struct ServerList {
    list: RefCell<Vec<ServerNode>>
}

#[derive(Debug)]
pub struct ServerNode {
    // 服务注册地址
    src: String,
    // 服务注册  淘汰时间
    timestamp: i64,
}

pub type MSGType = i32;

pub const REGISTER: MSGType = 1;
pub const DISCOVER: MSGType = 2;
pub const REGISTER_RESP: MSGType = 3;
pub const DISCOVER_RESP: MSGType = 4;

#[derive(Serialize, Deserialize, Debug)]
pub struct MSG {
    msg_type: MSGType,
    data: String, // JSON
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterNode {
    // ServerType
    server_type: String,
    // 服务ID
    server_id: String,
    // 服务地址
    server_addr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResp {
    server_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Discover {
    // 服务ID
    server_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscoverResp {
    server_id: String,
    server_addr: String,
}