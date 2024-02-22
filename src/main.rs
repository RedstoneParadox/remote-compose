use std::net::{IpAddr, TcpStream};
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TargetConfig {
    username: String,
    ip: String,
    port: i32,
    key_path: String
}

fn main() {
    let config = TargetConfig {
        username: "".to_string(),
        ip: "127.0.0.1".to_string(),
        port: 22,
        key_path: "".to_string(),
    };

    if config.port == 22 && config.ip != "127.0.0.1".to_string() {
        println!("Warning: It is recommended to change the SSH port from its default of 22 on the remote machine.")
    }

    let tcp = match TcpStream::connect(format!("{}:{}", config.ip, config.port)) {
        Ok(stream) => stream,
        Err(err) => {
            println!("Error while opening TCP stream to '{}:{}':\n{}", config.ip, config.port,err);
            return;
        }
    };


    println!("Hello, world!");
}
