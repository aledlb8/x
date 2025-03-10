use crate::storage::database::DB;
use owo_colors::OwoColorize;

pub fn list_items(db: &DB) {
    println!("{}", "Stored Items:".yellow().bold());

    let mut keys: Vec<String> = db
        .iter()
        .keys()
        .filter_map(Result::ok)
        .map(|k| String::from_utf8_lossy(&k).to_string())
        .filter(|k| k != "master_password" && k != "session")
        .collect();

    keys.sort();

    if keys.is_empty() {
        println!("{}", "No items found in your vault.".red());
    } else {
        for key in keys {
            println!("  - {}", key.bold().green());
        }
    }
}