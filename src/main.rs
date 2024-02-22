use std::fs;
use std::net::TcpStream;
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
    let target_file: String = load_target_file();
    let config: TargetConfig = serde_yaml::from_str(&*target_file).unwrap();

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
    let mut session = match ssh2::Session::new() {
        Ok(sess) => sess,
        Err(error) => {
            println!("Error while attempting to start ssh session:\n{}",error);
            return;
        }
    };

    session.set_tcp_stream(tcp);
    if let Err(error) = session.handshake() {
        println!("Error while attempting ssh handshake:\n{}",error);
        return;
    }
    if let Err(error) = session.userauth_agent(&*config.username) {
        println!("Error while attempting to authenticate with remote server:\n{}",error);
        return;
    }
}

fn load_target_file() -> String {
    match fs::read_to_string("target.yml") {
        Ok(contents) => return contents,
        Err(error) => {
            println!("Error while trying to read target file:\n{}",error);
            panic!()
        }
    }
}
