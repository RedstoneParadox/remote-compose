mod error;

use std::fs;
use std::net::{TcpStream, ToSocketAddrs};
use std::path::Path;
use serde::{Deserialize, Serialize};
use ssh2::{Error, Session};
use crate::error::WrappedError;

#[derive(Serialize, Deserialize, Debug)]
struct TargetConfig {
    username: String,
    ip: String,
    port: i32,
    key_path: String
}

fn main() {
    let config = match load_target_config("target.yml") {
        Ok(c) => c,
        Err(error) => {
            println!("Error while loading config 'target.yml':\n{}",error);
            return;
        }
    };

    let session = match connect(&*config.ip, config.port, config.username, &config.key_path) {
        Ok(s) => s,
        Err(error) => {
            println!("Error while attempting ssh connection to '{}:{}':\n{}", config.ip, config.port, error);
            return;
        }
    };
}

fn connect(addr: &str, port: i32, username: String, key_path: &String) -> Result<Session, WrappedError> {
    println!("Connecting to '{}@{}:{}'", username, addr, port);
    let tcp = TcpStream::connect(format!("{}:{}",addr, port))?;
    let mut session = Session::new()?;
    let key_path = Path::new(&key_path);

    session.set_tcp_stream(tcp);
    session.handshake()?;
    session.userauth_pubkey_file(&*username, None,&key_path, None)?;

    return Ok(session)
}

fn load_target_config(config_file: &str) -> Result<TargetConfig, WrappedError> {
    println!("Loading target config '{}'", config_file);
    let target_file = fs::read_to_string(config_file)?;
    let config: TargetConfig = serde_yaml::from_str(&*target_file)?;

    if config.port == 22 && config.ip != "127.0.0.1".to_string() {
        println!("Warning: It is recommended to change the SSH port from its default of 22 on the remote machine.")
    }

    return Ok(config)
}
