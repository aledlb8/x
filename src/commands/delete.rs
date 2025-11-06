use crate::cloud::RemoteSession;
use crate::vault::{load_vault, save_vault};
use dialoguer::Select;
use owo_colors::OwoColorize;

pub fn delete_item(session: &RemoteSession) -> Result<(), String> {
    let mut vault = load_vault(session)?;

    let keys: Vec<String> = vault.iter().map(|entry| entry.key.clone()).collect();

    if keys.is_empty() {
        println!("{}", "No items found in the vault.".red());
        return Ok(());
    }

    let selection = Select::new()
        .with_prompt("Select item to delete")
        .items(&keys)
        .default(0)
        .interact()
        .unwrap();

    let item_name = &keys[selection];
    let initial_len = vault.len();
    vault.retain(|entry| &entry.key != item_name);

    if vault.len() == initial_len {
        println!("{}", "Item not found!".red());
        return Ok(());
    }

    save_vault(session, &vault)?;

    println!("Deleted: {}", item_name.red());
    Ok(())
}
