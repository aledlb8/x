use crate::security::encryption::decrypt_data;
use crate::security::master_password::initialize_master_password;
use crate::storage::database::DB;
use clipboard::{ClipboardContext, ClipboardProvider};
use dialoguer::Select;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
struct SecureData {
    data: String,
}

pub fn get_item(db: &DB) {
    let key = initialize_master_password(db);

    let keys: Vec<String> = db
        .iter()
        .keys()
        .filter_map(Result::ok)
        .map(|k| String::from_utf8(k.to_vec()).unwrap())
        .filter(|k| k != "master_password" && k != "session")
        .collect();

    if keys.is_empty() {
        println!("{}", "No items found in the vault.".red());
        return;
    }

    let selection = Select::new()
        .with_prompt("Select an item to view:")
        .items(&keys)
        .default(0)
        .interact()
        .unwrap();

    let item_key = &keys[selection];
    println!("Retrieving details for {}", item_key.bold().green());

    if let Some(data) = db.get(item_key).unwrap() {
        let stored: SecureData = serde_json::from_slice(&data).unwrap();
        let decrypted = decrypt_data(&key, &stored.data);
        let vault_item: Result<VaultItem, _> = serde_json::from_str(&decrypted);

        let mut clipboard_contents = String::new();

        match vault_item {
            Ok(VaultItem::Password {
                name,
                website,
                email,
                username,
                password,
            }) => {
                println!("{} {}", "Type:".cyan(), "Password".bold().green());
                println!("{} {}", "Name:".cyan(), name.bold());
                println!("{} {}", "Website:".cyan(), website.bold());
                println!("{} {}", "Email:".cyan(), email.bold());
                println!("{} {}", "Username:".cyan(), username.bold());
                println!("{} {}", "Password:".cyan(), "[hidden]".red());
                clipboard_contents = password;
            }
            Ok(VaultItem::CreditCard {
                name,
                number,
                expiration_date,
                cvv,
            }) => {
                println!("{} {}", "Type:".cyan(), "Credit Card".bold().blue());
                println!("{} {}", "Name:".cyan(), name.bold());
                println!("{} {}", "Number:".cyan(), number.bold());
                println!("{} {}", "Expiration Date:".cyan(), expiration_date.bold());
                println!("{} {}", "CVV:".cyan(), "[hidden]".red());
                clipboard_contents = cvv;
            }
            Ok(VaultItem::SecureNote { title, note }) => {
                println!("{} {}", "Type:".cyan(), "Secure Note".bold().magenta());
                println!("{} {}", "Title:".cyan(), title.bold());
                println!("{} {}", "Note:".cyan(), note.bold());
            }
            Err(e) => {
                println!("{} {}", "Error parsing item data:".red(), e);
            }
        }
        if !clipboard_contents.is_empty() {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(clipboard_contents).unwrap();
            println!("Sensitive data has been copied to the clipboard.");
        }
    } else {
        println!("{}", "Item not found!".red());
    }
}