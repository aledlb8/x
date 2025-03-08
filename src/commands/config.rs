use crate::security::encryption::{decrypt_data, encrypt_data};
use crate::security::master_password::MASTER_KEY_STORAGE;
use crate::storage::database::DB;
use dialoguer::{Input, Password, Select};
use owo_colors::OwoColorize;
use std::collections::HashSet;
use std::str;

pub fn config(db: &DB) {
    println!("🔧 Config Menu:");
    let options = vec!["Change Master Password", "Change Session Timeout", "Exit"];
    let selection = Select::new()
        .with_prompt("Select an option")
        .items(&options)
        .interact()
        .unwrap();

    match selection {
        0 => change_master_password(db),
        1 => change_session_timeout(db),
        _ => println!("Exiting config menu."),
    }
}

fn change_master_password(db: &DB) {
    println!("🔑 Change Master Password");

    let current_password: String = Password::new()
        .with_prompt("Enter your current master password")
        .interact()
        .unwrap();

    let current_hashed = blake3::hash(current_password.as_bytes())
        .as_bytes()
        .to_vec();

    let stored_hash = db
        .get(MASTER_KEY_STORAGE)
        .unwrap()
        .expect("No master password set.");

    if stored_hash.to_vec() != current_hashed {
        println!("{}", "❌ Incorrect master password!".red());
        return;
    }

    let new_password: String = Password::new()
        .with_prompt("Enter your new master password")
        .interact()
        .unwrap();
    let confirm_password: String = Password::new()
        .with_prompt("Confirm your new master password")
        .interact()
        .unwrap();
    if new_password != confirm_password {
        println!("{}", "❌ New passwords do not match!".red());
        return;
    }
    let new_hashed = blake3::hash(new_password.as_bytes()).as_bytes().to_vec();

    use aes_gcm::{Aes256Gcm, Key};
    let old_key = Key::<Aes256Gcm>::from_slice(&current_hashed);
    let new_key = Key::<Aes256Gcm>::from_slice(&new_hashed);

    let reserved_keys: HashSet<&str> = ["master_password", "session", "session_timeout"]
        .iter()
        .cloned()
        .collect();
    for item in db.iter() {
        let (key_bytes, value_bytes) = item.unwrap();
        let key_str = String::from_utf8(key_bytes.to_vec()).unwrap();
        if reserved_keys.contains(key_str.as_str()) {
            continue;
        }
        let mut secure_data: serde_json::Value = serde_json::from_slice(&value_bytes).unwrap();
        let encrypted_data = secure_data["data"].as_str().unwrap();
        let decrypted = decrypt_data(&old_key, encrypted_data);
        let re_encrypted = encrypt_data(&new_key, &decrypted);
        secure_data["data"] = serde_json::Value::String(re_encrypted);
        let new_value = serde_json::to_string(&secure_data).unwrap();
        db.insert(key_str, new_value.as_bytes()).unwrap();
    }

    db.insert(MASTER_KEY_STORAGE, &*new_hashed).unwrap();
    db.flush().expect("Failed to flush database");

    println!("{}", "✅ Master password updated successfully!".green());
}

fn change_session_timeout(db: &DB) {
    println!("⏱ Change Session Timeout");

    let new_timeout: i64 = Input::new()
        .with_prompt("Enter new session timeout (in minutes)")
        .default(15)
        .interact_text()
        .unwrap();

    db.insert("session_timeout", new_timeout.to_string().as_bytes())
        .unwrap();
    db.flush().expect("Failed to flush database");

    println!("✅ Session timeout updated to {} minutes.", new_timeout);
}
