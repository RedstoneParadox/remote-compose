use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetConfig {
    pub username: String,
    #[serde(flatten)]
    pub credentials: Credentials,
    pub ip: String,
    pub port: i32,
    pub remote_dir: String,
    pub stacks: HashMap<String, TargetStack>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Credentials {
    Password { password: String },
    KeyPath { key_path: String }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetStack {
    pub name: String,
    pub files: Option<Vec<String>>
}
