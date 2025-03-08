use crate::security::session::is_session_active;
use crate::storage::database::DB;
use aes_gcm::{Aes256Gcm, Key};
use blake3;
use dialoguer::Password;
use owo_colors::OwoColorize;

pub(crate) const MASTER_KEY_STORAGE: &str = "master_password";

pub fn initialize_master_password(db: &DB) -> Key<Aes256Gcm> {
    if is_session_active(db) {
        if let Some(master_password) = db.get(MASTER_KEY_STORAGE).unwrap() {
            return Key::<Aes256Gcm>::from_slice(&master_password[..]).clone();
        }
    }

    if let Some(stored_password) = db.get(MASTER_KEY_STORAGE).unwrap() {
        let password: String = Password::new()
            .with_prompt("Enter Master Password")
            .interact()
            .unwrap();

        let hashed_password = blake3::hash(password.as_bytes()).as_bytes().to_vec();

        if stored_password.to_vec() != hashed_password {
            println!("{}", "❌ Incorrect password!".red());
            std::process::exit(1);
        }

        println!("{}", "🔓 Password verified!".green());
        return Key::<Aes256Gcm>::from_slice(&hashed_password[..]).clone();
    }

    let password: String = Password::new()
        .with_prompt("Set a Master Password")
        .interact()
        .unwrap();

    let hashed_password = blake3::hash(password.as_bytes()).as_bytes().to_vec();
    db.insert(MASTER_KEY_STORAGE, &*hashed_password).unwrap();
    db.flush().expect("Failed to flush master password update");

    println!("{}", "✅ Master Password Set!".green());

    Key::<Aes256Gcm>::from_slice(&hashed_password[..]).clone()
}