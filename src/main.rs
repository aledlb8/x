mod cloud;
mod commands;
mod config;
mod security;
mod utils;
mod vault;

use crate::cloud::{CloudClientError, RemoteSession};
use crate::config::AppConfig;
use crate::security::master_password;
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;

#[derive(Parser)]
#[command(name = "x", version = "0.1.6", about = "X CLI")]
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
    Import,
    Export,
    Update,
    Cloud {
        #[arg(value_name = "TARGET")]
        target: Option<String>,
    },
    Host {
        #[arg(long, default_value = "0.0.0.0")]
        bind: String,
        #[arg(long, default_value_t = 4000)]
        port: u16,
        #[arg(long)]
        data: Option<std::path::PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut app_config = AppConfig::load();

    match cli.command {
        Commands::Host { bind, port, data } => {
            let mut config = cloud::ServerConfig::default();
            config.bind_address = bind;
            config.port = port;
            config.data_path = data;

            if let Err(e) = commands::host::host_server(config) {
                eprintln!("Failed to host cloud API server: {}", e);
            }
        }
        Commands::Cloud { target } => {
            if let Err(err) = commands::cloud::handle_cloud_command(&mut app_config, target) {
                eprintln!("{}", err.red());
            }
        }
        Commands::Passgen => commands::password_generator::generate_password(),
        Commands::Update => commands::update::update_program(),
        command => match build_session(&mut app_config) {
            Ok(session) => match command {
                Commands::Add => report(commands::add::add_item(&session)),
                Commands::List => report(commands::list::list_items(&session)),
                Commands::Get => report(commands::get::get_item(&session)),
                Commands::Delete => report(commands::delete::delete_item(&session)),
                Commands::Edit => report(commands::edit::edit_item(&session)),
                Commands::Import => report(commands::import::import_items(&session)),
                Commands::Export => report(commands::export::export_items(&session)),
                _ => unreachable!(),
            },
            Err(err) => eprintln!("{}", err.red()),
        },
    }
}

fn build_session(app_config: &mut AppConfig) -> Result<RemoteSession, String> {
    let base_url = app_config.base_url.clone().ok_or_else(|| {
        "No cloud endpoint configured. Run `x cloud <url>` to connect to a host.".to_string()
    })?;

    if let Some(hash) = app_config.master_hash.clone() {
        match RemoteSession::from_hash(base_url.clone(), hash.clone()) {
            Ok(session) => return Ok(session),
            Err(CloudClientError::AuthenticationFailed) => {
                eprintln!(
                    "{}",
                    "Stored master password hash no longer matches the host. Please re-enter the password."
                        .red()
                );
                app_config.master_hash = None;
            }
            Err(err) => return Err(err.to_string()),
        }
    }

    let password = master_password::prompt_master_password("Enter the host master password");
    let session = RemoteSession::new(base_url, password).map_err(|err| err.to_string())?;
    app_config.master_hash = Some(session.auth_hash().to_string());
    if let Err(err) = app_config.save() {
        eprintln!("Warning: failed to persist master password hash: {}", err);
    }
    Ok(session)
}

fn report(result: Result<(), String>) {
    if let Err(err) = result {
        eprintln!("{}", err.red());
    }
}
