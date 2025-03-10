use crate::security::encryption::{encrypt_data, decrypt_data};
use crate::security::master_password::initialize_master_password;
use crate::storage::database::DB;
use dialoguer::Select;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum VaultItem {
    Password {
        name: String,
        website: String,
        email: String,
        username: String,
        password: String,
    },
    CreditCard {
        name: String,
        number: String,
        expiration_date: String,
        cvv: String,
    },
    SecureNote {
        title: String,
        note: String,
    },
}

#[derive(Deserialize, Serialize)]
struct SecureData {
    data: String,
    created_at: u64,
}

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

pub fn edit_item(db: &DB) {
    let key = initialize_master_password(db);

    let keys: Vec<String> = db
        .iter()
        .keys()
        .filter_map(Result::ok)
        .map(|k| String::from_utf8(k.to_vec()).unwrap())
        .filter(|k| k != "master_password" && k != "session" && k != "session_timeout" && k != "cloud_group")
        .collect();

    if keys.is_empty() {
        println!("{}", "No items found in the vault.".red());
        return;
    }

    let selection = Select::new()
        .with_prompt("Select item to update")
        .items(&keys)
        .interact()
        .unwrap();

    let item_key = &keys[selection];

    if let Some(data) = db.get(item_key).unwrap() {
        let stored: SecureData = serde_json::from_slice(&data).unwrap();
        let decrypted = decrypt_data(&key, &stored.data);
        let mut vault_item: VaultItem = serde_json::from_str(&decrypted).unwrap();

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
                *expiration_date = prompt_with_default("Enter the expiration date (MM/YY)", expiration_date);
                *cvv = prompt_with_default("Enter the CVV", cvv);
            }
            VaultItem::SecureNote { title, note } => {
                *title = prompt_with_default("Enter the title for the note", title);
                *note = prompt_with_default("Enter your secure note", note);
            }
        }

        let vault_item_json = serde_json::to_string(&vault_item).unwrap();
        let encrypted_data = encrypt_data(&key, &vault_item_json);
        let updated_item = SecureData {
            data: encrypted_data,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };

        db.insert(item_key, serde_json::to_string(&updated_item).unwrap().as_bytes())
            .unwrap();
        println!("Successfully updated: {}", item_key.green());
    } else {
        println!("{}", "Item not found!".red());
    }
}