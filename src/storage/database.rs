use std::path::PathBuf;
use dirs::data_dir;
use sled;

pub type DB = sled::Db;

pub fn open_db() -> DB {
    let db_path: PathBuf = if let Some(mut path) = data_dir() {
        path.push("x_cli");
        std::fs::create_dir_all(&path)
            .expect("Failed to create application data directory");
        path.push("x.db");
        path
    } else {
        PathBuf::from("x.db")
    };

    sled::open(db_path).expect("Failed to open database")
}