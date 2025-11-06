use crate::cloud::server::MASTER_HASH_KEY;
use crate::cloud::{run_server, ServerConfig, ServerError};
use crate::security::master_password;
use owo_colors::OwoColorize;
use sled;
use std::path::Path;

pub fn host_server(mut config: ServerConfig) -> Result<(), ServerError> {
    let storage_path = config.storage_path();
    if let Some(existing_hash) = read_existing_master_hash(&storage_path) {
        println!("{}", "Reusing stored master password hash.".yellow());
        config.master_hash = Some(existing_hash);
    } else {
        let master_password =
            master_password::prompt_master_password("Set the cloud host master password");
        let master_hash = master_password::hash_password(&master_password);
        config.master_hash = Some(master_hash);
    }

    println!("{}", "Launching embedded cloud API server...".green());
    match run_server(config) {
        Err(ServerError::Unauthorized) => {
            eprintln!(
                "{}",
                "Master password rejected. Ensure you entered the correct host password.".red()
            );
            Err(ServerError::Unauthorized)
        }
        other => other,
    }
}

fn read_existing_master_hash(path: &Path) -> Option<String> {
    if !path.exists() {
        return None;
    }

    let db = sled::open(path).ok()?;
    let meta = db.open_tree("meta").ok()?;
    meta.get(MASTER_HASH_KEY)
        .ok()
        .flatten()
        .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
}
