mod error;
mod command;
mod config;
mod deployment;

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
use crate::config::Credentials;
use crate::error::WrappedError;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Deploy { config_path } => {
            let tuple = match connect_to_target(&*config_path) {
                Ok(v) => v,
                Err(error) => {
                    println!("Error:\n{}", error);
                    return;
                }
            };
            let config = tuple.0;
            let session = tuple.1;

            match deployment::deploy_stacks(session, config.remote_dir, config.stacks.values()) {
                Ok(_) => {}
                Err(error) => {
                    println!("Error while attempting to deploy stacks to '{}:{}':\n{}", config.ip, config.port, error)
                }
            }
        }
    }
}

fn connect_to_target(config_path: &str) -> Result<(TargetConfig, Session), WrappedError> {
    let config = load_target_config(config_path)?;
    let session = connect(&*config.ip, config.port, &*config.username, &config.credentials)?;
    Ok((config, session))
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

fn connect(addr: &str, port: i32, username: &str, credentials: &Credentials) -> Result<Session, WrappedError> {
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
