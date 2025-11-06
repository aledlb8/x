use crate::cloud::{RemoteSession, VaultEntry};
use crate::utils::prompt_input;
use crate::vault::{load_vault, save_vault, SecureEntry, VaultItem};
use dialoguer::Select;
use owo_colors::OwoColorize;

pub fn add_item(session: &RemoteSession) -> Result<(), String> {
    let item_type = Select::new()
        .with_prompt("What would you like to store?")
        .items(&["Password", "Credit Card", "Secure Note"])
        .interact()
        .unwrap();

    let (unique_key, vault_item) = match item_type {
        0 => {
            let name = prompt_input("Enter the name");
            let mut website = prompt_input("Enter the website");

            if !website.starts_with("http://") && !website.starts_with("https://") {
                website = format!("https://{}", website);
            }
            let email = prompt_input("Enter the email");
            let username = prompt_input("Enter the username");
            let password = prompt_input("Enter the password");
            (
                name.clone(),
                VaultItem::Password {
                    name,
                    website,
                    email,
                    username,
                    password,
                },
            )
        }
        1 => {
            let name = prompt_input("Enter the card name (or cardholder's name)");
            let number = prompt_input("Enter the credit card number");
            let expiration_date = prompt_input("Enter the expiration date (MM/YY)");
            let cvv = prompt_input("Enter the CVV");
            (
                name.clone(),
                VaultItem::CreditCard {
                    name,
                    number,
                    expiration_date,
                    cvv,
                },
            )
        }
        2 => {
            let title = prompt_input("Enter the title for the note");
            let note = prompt_input("Enter your secure note");
            (title.clone(), VaultItem::SecureNote { title, note })
        }
        _ => return Ok(()),
    };

    let stored_item = SecureEntry::encrypt(session.encryption_key(), &vault_item)?;
    let mut vault = load_vault(session)?;

    vault.retain(|entry| entry.key != unique_key);
    let value_string = stored_item.serialize()?;
    vault.push(VaultEntry {
        key: unique_key.clone(),
        value: value_string,
    });

    save_vault(session, &vault)?;

    println!(" Successfully stored: {}", unique_key.green());
    Ok(())
}
