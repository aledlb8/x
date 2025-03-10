use crate::storage::database::DB;
use dialoguer::{Input, Password};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
struct ExportItem {
    key: String,
    value: String,
}

pub fn import_items(db: &DB) {
    println!("{}", "📥 Import Vault Items".yellow().bold());

    let file_path: String = Input::new()
        .with_prompt("Enter file path to import vault items from")
        .default("x_export.json".into())
        .interact_text()
        .unwrap();

    let mut file = File::open(&file_path)
        .unwrap_or_else(|e| panic!("Failed to open file {}: {}", file_path, e));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|e| panic!("Failed to read file {}: {}", file_path, e));

    let export_items: Vec<ExportItem> =
        serde_json::from_str(&contents).unwrap_or_else(|e| panic!("Failed to parse JSON: {}", e));

    let master_item_opt = export_items
        .iter()
        .find(|item| item.key == "master_password");
    if let Some(master_item) = master_item_opt {
        let input_password = Password::new()
            .with_prompt("Enter master password to confirm import")
            .interact()
            .unwrap();

        let hashed_input = blake3::hash(input_password.as_bytes());
        let hashed_input_hex = hex::encode(hashed_input.as_bytes());

        if master_item.value != hashed_input_hex {
            println!("{}", "Incorrect master password. Import aborted.".red());
            return;
        }
    } else {
        println!(
            "{}",
            "No master password found in export file. Import aborted.".red()
        );
        return;
    }

    for item in &export_items {
        db.insert(item.key.clone(), item.value.clone().into_bytes())
            .unwrap();
    }
    db.flush().unwrap();

    println!(
        "{}",
        format!(
            "Imported {} items from {}",
            export_items.len(),
            file_path
        )
        .green()
    );
}