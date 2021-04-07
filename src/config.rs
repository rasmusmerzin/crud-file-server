use async_std::path::PathBuf;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub dir_path: PathBuf,
    pub srv_addr: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            dir_path: PathBuf::from(env::var("DIRECTORY").unwrap_or("content".into())),
            srv_addr: env::var("SERVER_ADDR").unwrap_or("0.0.0.0:8000".into()),
        }
    }
}
