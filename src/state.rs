use crate::Config;
use async_std::path::PathBuf;
use async_std::{fs, io};

pub struct State {
    pub dir_path: PathBuf,
}

impl State {
    pub async fn from_config(config: Config) -> io::Result<Self> {
        let Config { dir_path, .. } = config;
        if !dir_path.is_dir().await {
            fs::create_dir_all(&dir_path).await?;
        }
        Ok(Self { dir_path })
    }
}
