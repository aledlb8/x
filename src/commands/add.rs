use crate::security::encryption::encrypt_data;
use crate::security::master_password::initialize_master_password;
use crate::storage::database::DB;
use crate::utils::prompt_input;
use dialoguer::Select;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
struct SecureData {
    data: String,
    created_at: u64,
}

pub fn add_item(db: &DB) {
    let key = initialize_master_password(db);

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
            (name.clone(), VaultItem::Password { name, website, email, username, password })
        }
        1 => {
            let name = prompt_input("Enter the card name (or cardholder's name)");
            let number = prompt_input("Enter the credit card number");
            let expiration_date = prompt_input("Enter the expiration date (MM/YY)");
            let cvv = prompt_input("Enter the CVV");
            (name.clone(), VaultItem::CreditCard { name, number, expiration_date, cvv })
        }
        2 => {
            let title = prompt_input("Enter the title for the note");
            let note = prompt_input("Enter your secure note");
            (title.clone(), VaultItem::SecureNote { title, note })
        }
        _ => return,
    };

    let vault_item_json = serde_json::to_string(&vault_item).unwrap();

    let encrypted_data = encrypt_data(&key, &vault_item_json);

    let stored_item = SecureData {
        data: encrypted_data,
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    db.insert(
        unique_key.clone(),
        serde_json::to_string(&stored_item).unwrap().as_bytes(),
    )
    .unwrap();

    println!(" Successfully stored: {}", unique_key.green());
}