mod commands;
mod security;
mod storage;
mod utils;

use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;

#[derive(Parser)]
#[command(name = "x", version = "1.0", about = "X CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add,
    Get,
    List,
    Delete,
    Edit,
    PassGen,
    Config,
}

fn main() {
    let db = storage::database::open_db();

    if security::session::is_session_active(&db) {
        println!("{}", "🔓 Session active, no need to enter password!".green());
    } else {
        println!("{}", "🔓 No active session found. Please enter your master password.".yellow());
        security::master_password::initialize_master_password(&db);
        security::session::update_session(&db);
        println!("{}", "🔓 Session started!".green());
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Add => commands::add::add_item(&db),
        Commands::List => commands::list::list_items(&db),
        Commands::Get => commands::get::get_item(&db),
        Commands::Delete => commands::delete::delete_item(&db),
        Commands::Edit => commands::edit::edit_item(&db),
        Commands::PassGen => commands::password_generator::generate_password(),
        Commands::Config => commands::config::config(&db),
    }
}