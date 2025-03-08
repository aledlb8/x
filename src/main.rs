mod commands;
mod security;
mod storage;
mod utils;

use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;

#[derive(Parser)]
#[command(name = "x", version = "0.1.3", about = "X CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    Add,
    Get,
    List,
    Delete,
    Edit,
    Passgen,
    Config,
    Import,
    Export,
    Update,
}

fn main() {
    let cli = Cli::parse();
    let db = storage::database::open_db();

    if let Commands::Import = cli.command {
        commands::import::import_items(&db);
        
    } else {
        if security::session::is_session_active(&db) {
            println!(
                "{}",
                "🔓 Session active, no need to enter password!".green()
            );
        } else {
            println!(
                "{}",
                "🔓 No active session found. Please enter your master password.".yellow()
            );
            security::master_password::initialize_master_password(&db);
            security::session::update_session(&db);
            println!("{}", "🔓 Session started!".green());
        }

        // Dispatch the selected command.
        match cli.command {
            Commands::Add => commands::add::add_item(&db),
            Commands::List => commands::list::list_items(&db),
            Commands::Get => commands::get::get_item(&db),
            Commands::Delete => commands::delete::delete_item(&db),
            Commands::Edit => commands::edit::edit_item(&db),
            Commands::Passgen => commands::password_generator::generate_password(),
            Commands::Config => commands::config::config(&db),
            Commands::Export => commands::export::export_items(&db),
            Commands::Update => commands::update::update_program(),
            Commands::Import => unreachable!(),
        }
    }
}