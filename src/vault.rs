use crate::cloud::types::VaultEntry;
use crate::cloud::RemoteSession;
use crate::security::encryption::{decrypt_data, encrypt_data};
use aes_gcm::{Aes256Gcm, Key};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VaultItem {
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
pub struct SecureEntry {
    pub data: String,
    pub created_at: u64,
}

impl SecureEntry {
    pub fn encrypt(key: &Key<Aes256Gcm>, item: &VaultItem) -> Result<Self, String> {
        let plaintext =
            serde_json::to_string(item).map_err(|err| format!("Encode error: {}", err))?;
        let encrypted = encrypt_data(key, &plaintext);
        Ok(Self {
            data: encrypted,
            created_at: timestamp_now(),
        })
    }

    pub fn decrypt(key: &Key<Aes256Gcm>, value: &str) -> Result<VaultItem, String> {
        let stored: SecureEntry =
            serde_json::from_str(value).map_err(|err| format!("Decode error: {}", err))?;
        let plaintext = decrypt_data(key, &stored.data);
        serde_json::from_str(&plaintext).map_err(|err| format!("Decode error: {}", err))
    }

    pub fn serialize(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|err| format!("Encode error: {}", err))
    }
}

fn timestamp_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn load_vault(session: &RemoteSession) -> Result<Vec<VaultEntry>, String> {
    session.fetch_vault().map_err(|err| err.to_string())
}

pub fn save_vault(session: &RemoteSession, vault: &[VaultEntry]) -> Result<(), String> {
    session.save_vault(vault).map_err(|err| err.to_string())
}
