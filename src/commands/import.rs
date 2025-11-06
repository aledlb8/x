use crate::cloud::{RemoteSession, VaultEntry};
use crate::vault::{load_vault, save_vault};
use dialoguer::Input;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
struct ImportItem {
    key: String,
    value: String,
}

pub fn import_items(session: &RemoteSession) -> Result<(), String> {
    println!("{}", "Import Vault Items".yellow().bold());

    let file_path: String = Input::new()
        .with_prompt("Enter file path to import vault items from")
        .default("x_export.json".into())
        .interact_text()
        .unwrap();

    let mut file = File::open(&file_path)
        .map_err(|err| format!("Failed to open file {}: {}", file_path, err))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|err| format!("Failed to read file {}: {}", file_path, err))?;

    let items: Vec<ImportItem> =
        serde_json::from_str(&contents).map_err(|err| format!("Failed to parse JSON: {}", err))?;

    if items.is_empty() {
        println!("{}", "No items found in the import file.".yellow());
        return Ok(());
    }

    let mut vault = load_vault(session)?;

    let mut imported = 0usize;
    for item in items {
        vault.retain(|entry| entry.key != item.key);
        vault.push(VaultEntry {
            key: item.key,
            value: item.value,
        });
        imported += 1;
    }

    save_vault(session, &vault)?;

    println!(
        "{}",
        format!("Imported {} items from {}", imported, file_path).green()
    );
    Ok(())
}
