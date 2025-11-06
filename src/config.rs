use dirs::data_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AppConfig {
    pub base_url: Option<String>,
    pub master_hash: Option<String>,
}

impl AppConfig {
    pub fn load() -> Self {
        let path = config_path();
        match fs::read(&path) {
            Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Self::default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_vec_pretty(self).map_err(|err| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to serialize config: {}", err),
            )
        })?;
        fs::write(path, data)
    }
}

fn config_path() -> PathBuf {
    let mut path = data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("x_cli");
    path.push("config.json");
    path
}
