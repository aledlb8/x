use blake3;
use dialoguer::Password;

pub fn prompt_master_password(prompt: &str) -> String {
    Password::new().with_prompt(prompt).interact().unwrap()
}

pub fn hash_password(password: &str) -> String {
    blake3::hash(password.as_bytes()).to_hex().to_string()
}
