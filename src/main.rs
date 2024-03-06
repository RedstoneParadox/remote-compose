mod error;
mod command;
mod config;

use std::collections::hash_map::Values;
use std::fs;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::ops::Deref;
use std::path::Path;
use clap::Parser;
use serde::{Deserialize, Serialize};
use ssh2::Session;
use config::TargetConfig;
use crate::command::{Cli, Commands};
use crate::config::{Credentials, TargetStack};
use crate::error::WrappedError;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Deploy { config_path } => {
            let config = match load_target_config(&*config_path) {
                Ok(c) => c,
                Err(error) => {
                    println!("Error while loading config '{}':\n{}", config_path, error);
                    return;
                }
            };
            let session = match connect(&*config.ip, config.port, config.username, config.credentials) {
                Ok(s) => s,
                Err(error) => {
                    println!("Error while attempting ssh connection to '{}:{}':\n{}", config.ip, config.port, error);
                    return;
                }
            };

            match deploy_stacks(session, config.remote_dir, config.stacks.values()) {
                Ok(_) => {}
                Err(error) => {
                    println!("Error while attempting to deploy stacks to '{}:{}':\n{}", config.ip, config.port, error)
                }
            }
        }
    }
}

fn load_target_config(config_path: &str) -> Result<TargetConfig, WrappedError> {
    println!("Loading target config '{}'", config_path);
    let target_file = fs::read_to_string(config_path)?;
    let config: TargetConfig = serde_yaml::from_str(&*target_file)?;

    if config.port == 22 && config.ip != "127.0.0.1".to_string() {
        println!("Warning: It is recommended to change the SSH port from its default of 22 on the remote machine.")
    }

    return Ok(config)
}

fn connect(addr: &str, port: i32, username: String, credentials: Credentials) -> Result<Session, WrappedError> {
    println!("Connecting to '{}@{}:{}'", username, addr, port);
    let tcp = TcpStream::connect(format!("{}:{}",addr, port))?;
    let mut session = Session::new()?;

    session.set_tcp_stream(tcp);
    session.handshake()?;

    match credentials {
        Credentials::Password { password} => {
            session.userauth_password(&*username, &*password)
        }
        Credentials::KeyPath { key_path} => {
            let key_path = Path::new(&key_path);
            session.userauth_pubkey_file(&*username, None, &key_path, None)
        }
    }?;

    return Ok(session);
}

fn deploy_stacks(session: Session, remote_dir: String, stacks: Values<String, TargetStack>) -> Result<(), WrappedError> {
    for stack in stacks {
        println!("Deploying stack {}", stack.name);
        // If yaml only had one valid file extension, I wouldn't have to do this.
        let yaml_path_string = format!("{}/compose.yaml", stack.name);
        let yaml_path = Path::new(&yaml_path_string);
        let yml_path_string = format!("{}/compose.yml", stack.name);
        let yml_path = Path::new(&yml_path_string);
        let compose_file = if yaml_path.exists() {
            fs::read_to_string(yaml_path)?
        } else {
            fs::read_to_string(yml_path)?
        };
        let remote_path_string = format!("{}/{}/compose.yaml", remote_dir, stack.name);
        let remote_path = Path::new(&remote_path_string);
        let mut remote_file = session.scp_send(remote_path, 0o644, compose_file.len() as u64, None)?;

        remote_file.write(compose_file.as_bytes())?;
        remote_file.send_eof()?;
        remote_file.wait_eof()?;
        remote_file.close()?;
        remote_file.wait_close()?;
        println!("Successfully deployed stack {}", stack.name)
    }

    return Ok(());
}
