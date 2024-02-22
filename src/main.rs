use std::fmt::format;
use std::fs;
use std::net::{TcpStream, ToSocketAddrs};
use std::path::Path;
use serde::{Deserialize, Serialize};
use ssh2::Error;
use ssh2::ErrorCode::Session;
use ssh2::OpenType::File;

#[derive(Serialize, Deserialize, Debug)]
struct TargetConfig {
    username: String,
    ip: String,
    port: i32,
    key_path: String
}

fn main() {
    let config = match load_target_config() {
        Ok(c) => c,
        Err(error) => {
            println!("Error while loading config 'target.yml':\n{}",error);
            return;
        }
    };

    if config.port == 22 && config.ip != "127.0.0.1".to_string() {
        println!("Warning: It is recommended to change the SSH port from its default of 22 on the remote machine.")
    }

    let session = match connect(config.ip, config.port, config.username, &config.key_path) {
        Ok(s) => s,
        Err(error) => {
            println!("Error while attempting ssh connection to '{}:{}':\n{}", config.ip, config.port, error);
            return;
        }
    };
}

fn connect(addr: String, port: i32, username: String, key_path: &String) -> Result<ssh2::Session, Error> {
    let tcp = TcpStream::connect(format!("{}:{}",addr, port))?;
    let mut session = ssh2::Session::new()?;
    let key_path = Path::new(&key_path);

    session.set_tcp_stream(tcp);
    session.handshake()?;
    session.userauth_pubkey_file(&*username, None,&key_path, None)?;

    return Ok(session)
}

fn load_target_config() -> Result<TargetConfig, Error> {
    let target_file = fs::read_to_string("target.yml")?;
    let target_config = serde_yaml::from_str(&*target_file)?;
    return Ok(target_config)
}
