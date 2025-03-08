use crate::storage::database::DB;
use dialoguer::Input;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct ExportItem {
    key: String,
    value: String,
}

pub fn export_items(db: &DB) {
    println!("{}", "📤 Export Vault Items".yellow().bold());

    let file_path: String = Input::new()
        .with_prompt("Enter file path to export vault items to")
        .default("x_export.json".into())
        .interact_text()
        .unwrap();

    let mut export_items = Vec::new();
    for item in db.iter() {
        let (key_bytes, value_bytes) = match item {
            Ok(pair) => pair,
            Err(e) => {
                println!("{}", format!("Error reading a record: {}", e).red());
                continue;
            }
        };

        let key_str = String::from_utf8_lossy(&key_bytes).to_string();
        if key_str == "master_password" || key_str == "session" {
            continue;
        }
        let value_str = String::from_utf8_lossy(&value_bytes).to_string();
        export_items.push(ExportItem {
            key: key_str,
            value: value_str,
        });
    }

    let json =
        serde_json::to_string_pretty(&export_items).expect("Failed to serialize export items");

    let mut file = File::create(&file_path)
        .unwrap_or_else(|e| panic!("Failed to create file {}: {}", file_path, e));
    file.write_all(json.as_bytes())
        .unwrap_or_else(|e| panic!("Failed to write to file {}: {}", file_path, e));

    println!(
        "{}",
        format!("✅ Exported {} items to {}", export_items.len(), file_path).green()
    );
}