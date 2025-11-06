use crate::cloud::RemoteSession;
use crate::vault::{load_vault, SecureEntry, VaultItem};
use clipboard::{ClipboardContext, ClipboardProvider};
use dialoguer::Select;
use owo_colors::OwoColorize;

pub fn get_item(session: &RemoteSession) -> Result<(), String> {
    let entries = load_vault(session)?;

    let keys: Vec<String> = entries.iter().map(|entry| entry.key.clone()).collect();

    if keys.is_empty() {
        println!("{}", "No items found in the vault.".red());
        return Ok(());
    }

    let selection = Select::new()
        .with_prompt("Select an item to view:")
        .items(&keys)
        .default(0)
        .interact()
        .unwrap();

    let item_key = &keys[selection];
    println!("Retrieving details for {}", item_key.bold().green());

    let entry = entries
        .into_iter()
        .find(|entry| &entry.key == item_key)
        .ok_or_else(|| "Selected item not found.".to_string())?;

    let vault_item = SecureEntry::decrypt(session.encryption_key(), &entry.value)?;
    let mut clipboard_contents = String::new();

    match vault_item {
        VaultItem::Password {
            name,
            website,
            email,
            username,
            password,
        } => {
            println!("{} {}", "Type:".cyan(), "Password".bold().green());
            println!("{} {}", "Name:".cyan(), name.bold());
            println!("{} {}", "Website:".cyan(), website.bold());
            println!("{} {}", "Email:".cyan(), email.bold());
            println!("{} {}", "Username:".cyan(), username.bold());
            println!("{} {}", "Password:".cyan(), "[hidden]".red());
            clipboard_contents = password;
        }
        VaultItem::CreditCard {
            name,
            number,
            expiration_date,
            cvv,
        } => {
            println!("{} {}", "Type:".cyan(), "Credit Card".bold().blue());
            println!("{} {}", "Name:".cyan(), name.bold());
            println!("{} {}", "Number:".cyan(), number.bold());
            println!("{} {}", "Expiration Date:".cyan(), expiration_date.bold());
            println!("{} {}", "CVV:".cyan(), "[hidden]".red());
            clipboard_contents = cvv;
        }
        VaultItem::SecureNote { title, note } => {
            println!("{} {}", "Type:".cyan(), "Secure Note".bold().magenta());
            println!("{} {}", "Title:".cyan(), title.bold());
            println!("{} {}", "Note:".cyan(), note.bold());
        }
    }

    if !clipboard_contents.is_empty() {
        let mut ctx: ClipboardContext = ClipboardProvider::new().map_err(|err| err.to_string())?;
        ctx.set_contents(clipboard_contents)
            .map_err(|err| err.to_string())?;
        println!("Sensitive data has been copied to the clipboard.");
    }

    Ok(())
}
