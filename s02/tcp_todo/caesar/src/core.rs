use std::collections::HashMap;
use std::sync::Mutex;
use super::*;
use crate::define::*;
use uuid::Uuid;
use std::cell::RefCell;

lazy_static! {
    static ref DB: Mutex<HashMap<String, ServerNode>> = {
        let db = HashMap::new();
        Mutex::new(db)
    };
}

// 服务注册
pub fn register(msg: &define::MSG) -> Result<String, Box<dyn Error>> {
    let mut node: RegisterNode = serde_json::from_str(msg.data.as_str())?;

    if node.server_type == "" {
        return Err(Box::new(ServerTypeNone));
    }

    if node.server_id != "" {
        // 服务第一次进来进行服务注册
        node.server_id = Uuid::new_v4().to_string();
        let server_node = ServerNode::new(node.server_id.clone(), node.server_addr, 5);
        let mut p = DB.lock()?;
        p.insert(ServerNode::get_server_key(&node.server_type, &node.server_id), server_node);
        return Ok(node.server_id);
    }


    Ok(node.server_id)
}

// 服务发现
pub fn discover(msg: &define::MSG) -> Result<(), Box<dyn Error>> {
    Ok(())
}

impl ServerNode {
    // server_id, src, Heartbeat Time Seconds
    fn new(server_id: String, src: String, heartbeat: u32) -> Self {
        ServerNode {
            server_id,
            src,
            timestamp: RefCell::new(Utc::now().date().and_hms(0, 0, heartbeat).timestamp()),
        }
    }

    fn get_server_key(server_type: &String, server_id: &String) -> String {
        format!("{}/{}", server_type, server_id)
    }
}