use crate::cloud::RemoteSession;
use crate::vault::load_vault;
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

pub fn export_items(session: &RemoteSession) -> Result<(), String> {
    println!("{}", "Export Vault Items".yellow().bold());

    let file_path: String = Input::new()
        .with_prompt("Enter file path to export vault items to")
        .default("x_export.json".into())
        .interact_text()
        .unwrap();

    let export_items: Vec<ExportItem> = load_vault(session)?
        .into_iter()
        .map(|entry| ExportItem {
            key: entry.key,
            value: entry.value,
        })
        .collect();

    let json = serde_json::to_string_pretty(&export_items)
        .map_err(|err| format!("Failed to serialize export items: {}", err))?;

    let mut file = File::create(&file_path)
        .map_err(|err| format!("Failed to create file {}: {}", file_path, err))?;
    file.write_all(json.as_bytes())
        .map_err(|err| format!("Failed to write to file {}: {}", file_path, err))?;

    println!(
        "{}",
        format!("Exported {} items to {}", export_items.len(), file_path).green()
    );
    Ok(())
}
