use clipboard::{ClipboardContext, ClipboardProvider};
use dialoguer::{Input, MultiSelect};
use owo_colors::OwoColorize;
use rand::Rng;

pub fn generate_password() {
    let lowercase: &str = "abcdefghijklmnopqrstuvwxyz";
    let uppercase: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers: &str = "0123456789";
    let symbols: &str = "!@#$%^&*()-_=+[]{};:,.<>?";

    let length: usize = Input::new()
        .with_prompt("Enter desired password length")
        .default(16)
        .interact_text()
        .unwrap();

    let selections: Vec<usize> = MultiSelect::new()
        .with_prompt("Select character types to include")
        .items(&["Lowercase", "Uppercase", "Numbers", "Symbols"])
        .defaults(&[true, true, true, false])
        .interact()
        .unwrap();

    let mut charset: String = String::new();
    if selections.is_empty() {
        charset.push_str(lowercase);
        charset.push_str(uppercase);
        charset.push_str(numbers);
        charset.push_str(symbols);
    } else {
        for i in selections {
            match i {
                0 => charset.push_str(lowercase),
                1 => charset.push_str(uppercase),
                2 => charset.push_str(numbers),
                3 => charset.push_str(symbols),
                _ => (),
            }
        }
    }

    let charset: Vec<char> = charset.chars().collect();
    if charset.is_empty() {
        println!("{}", "No character types selected. Aborting.".red());
        return;
    }

    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx: usize = rng.gen_range(0..charset.len());
            charset[idx]
        })
        .collect();

    println!("Generated Password: {}", password.green());

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(password.clone()).unwrap();
    println!("{}", "Password copied to clipboard.".green());
}