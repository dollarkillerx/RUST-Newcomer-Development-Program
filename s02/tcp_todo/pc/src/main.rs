// 此服务拥有状态 只能起一个实例
use pc::core::{Core};
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    let listen_addr = "0.0.0.0:8095".to_string();
    let caesar_addr = "0.0.0.0:8189".to_string();
    println!("Run PC on: ",);

    let mut core = Core::new(listen_addr,caesar_addr);
    core.run()
}
