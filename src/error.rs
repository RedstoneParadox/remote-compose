use std::fmt::{Display, Formatter};
use WrappedError::Config;
use crate::error::WrappedError::{IO, SSH};

pub(crate) enum WrappedError {
    IO(std::io::Error),
    SSH(ssh2::Error),
    Config(serde_yaml::Error)
}

impl From<std::io::Error> for WrappedError {
    fn from(value: std::io::Error) -> Self {
        IO(value)
    }
}

impl From<ssh2::Error> for WrappedError {
    fn from(value: ssh2::Error) -> Self {
        SSH(value)
    }
}

impl From<serde_yaml::Error> for WrappedError {
    fn from(value: serde_yaml::Error) -> Self {
        Config(value)
    }
}

impl Display for WrappedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IO(value) => write!(f, "IO error, {}", value),
            SSH(value) => write!(f, "SSH error, {}", value),
            Config(value) => write!(f, "Config error, {}", value)
        }
    }
}