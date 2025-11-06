use crate::cloud::RemoteSession;
use crate::vault::load_vault;
use owo_colors::OwoColorize;

pub fn list_items(session: &RemoteSession) -> Result<(), String> {
    println!("{}", "Stored Items:".yellow().bold());

    let mut keys: Vec<String> = load_vault(session)?
        .into_iter()
        .map(|entry| entry.key)
        .collect();

    keys.sort();

    if keys.is_empty() {
        println!("{}", "No items found in your vault.".red());
    } else {
        for key in keys {
            println!("  - {}", key.bold().green());
        }
    }

    Ok(())
}
