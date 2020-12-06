use std_4::*;

fn main() -> Result<()>{
    let mut args = std::env::args();
    match (args.nth(1).as_ref().map(String::as_str),args.next()) {
        (Some("client"), None) => Client::main(),
        (Some("server"), None) => Server::main(),
        _ => Err("Usage: a-chat [client|server]".into()),
    }
}
