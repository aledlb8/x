use crate::storage::database::DB;
use owo_colors::OwoColorize;
use dialoguer::Select;

pub fn delete_item(db: &DB) {
    let keys: Vec<String> = db
        .iter()
        .keys()
        .filter_map(Result::ok)
        .map(|k| String::from_utf8(k.to_vec()).unwrap())
        .filter(|k| k != "master_password" && k != "session" && k != "session_timeout")
        .collect();

    if keys.is_empty() {
        println!("{}", "No items found in the vault.".red());
        return;
    }

    let selection = Select::new()
        .with_prompt("Select item to delete")
        .items(&keys)
        .interact()
        .unwrap();

    let item_name = &keys[selection];

    if db.remove(item_name).is_ok() {
        println!("Deleted: {}", item_name.red());
    } else {
        println!("{}", "Item not found!".red());
    }
}