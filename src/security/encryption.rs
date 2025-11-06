use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;

pub fn encrypt_data(key: &Key<Aes256Gcm>, plaintext: &str) -> String {
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .expect("Encryption failure");

    let mut combined = nonce_bytes.to_vec();
    combined.extend(ciphertext);

    general_purpose::STANDARD.encode(&combined)
}

pub fn decrypt_data(key: &Key<Aes256Gcm>, ciphertext: &str) -> String {
    let cipher = Aes256Gcm::new(key);

    let decoded = general_purpose::STANDARD
        .decode(ciphertext)
        .expect("Invalid base64");

    let (nonce_bytes, ciphertext) = decoded.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .expect("Decryption failure");
    String::from_utf8(plaintext).expect("Invalid UTF-8")
}
