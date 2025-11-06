use crate::cloud::RemoteSession;
use crate::vault::{load_vault, save_vault, SecureEntry, VaultItem};
use dialoguer::Select;
use owo_colors::OwoColorize;
use std::io::{self, Write};

fn prompt_with_default(prompt: &str, default: &str) -> String {
    print!("{} [{}]: ", prompt, default);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    if input.is_empty() {
        default.to_string()
    } else {
        input.to_string()
    }
}

pub fn edit_item(session: &RemoteSession) -> Result<(), String> {
    let mut vault = load_vault(session)?;

    let keys: Vec<String> = vault.iter().map(|entry| entry.key.clone()).collect();

    if keys.is_empty() {
        println!("{}", "No items found in the vault.".red());
        return Ok(());
    }

    let selection = Select::new()
        .with_prompt("Select item to update")
        .items(&keys)
        .default(0)
        .interact()
        .unwrap();

    let item_key = &keys[selection];

    let entry = vault
        .iter_mut()
        .find(|entry| &entry.key == item_key)
        .ok_or_else(|| "Item not found in vault.".to_string())?;

    let mut vault_item = SecureEntry::decrypt(session.encryption_key(), &entry.value)?;

    match &mut vault_item {
        VaultItem::Password {
            name,
            website,
            email,
            username,
            password,
        } => {
            *name = prompt_with_default("Enter the name", name);
            let mut new_website = prompt_with_default("Enter the website", website);

            if !new_website.starts_with("http://") && !new_website.starts_with("https://") {
                new_website = format!("https://{}", new_website);
            }
            *website = new_website;
            *email = prompt_with_default("Enter the email", email);
            *username = prompt_with_default("Enter the username", username);
            *password = prompt_with_default("Enter the password", password);
        }
        VaultItem::CreditCard {
            name,
            number,
            expiration_date,
            cvv,
        } => {
            *name = prompt_with_default("Enter the card name (or cardholder's name)", name);
            *number = prompt_with_default("Enter the credit card number", number);
            *expiration_date =
                prompt_with_default("Enter the expiration date (MM/YY)", expiration_date);
            *cvv = prompt_with_default("Enter the CVV", cvv);
        }
        VaultItem::SecureNote { title, note } => {
            *title = prompt_with_default("Enter the title for the note", title);
            *note = prompt_with_default("Enter your secure note", note);
        }
    }

    let updated_item = SecureEntry::encrypt(session.encryption_key(), &vault_item)?;
    entry.value = updated_item.serialize()?;

    save_vault(session, &vault)?;

    println!("Successfully updated: {}", item_key.green());
    Ok(())
}
