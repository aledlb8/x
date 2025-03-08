use dialoguer::Input;
use owo_colors::OwoColorize;
use rand::Rng;

pub fn generate_password() {
    // Prompt the user for the desired password length.
    let length: usize = Input::new()
        .with_prompt("Enter desired password length")
        .default(16)
        .interact_text()
        .unwrap();

    // Define a character set including letters, numbers, and symbols.
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{};:,.<>?"
        .chars()
        .collect();

    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx]
        })
        .collect();

    println!("🔑 Generated Password: {}", password.green());
}