use crate::cloud::{CloudApi, CloudClientError};
use crate::config::AppConfig;
use crate::security::master_password;
use owo_colors::OwoColorize;
use reqwest::Url;

const DEFAULT_STATUS_MESSAGE: &str =
    "No cloud endpoint configured. Use `x cloud <url>` to link to a host.";

pub fn handle_cloud_command(config: &mut AppConfig, target: Option<String>) -> Result<(), String> {
    if let Some(argument) = target {
        match argument.trim() {
            "remove" => return remove_cloud_link(config),
            "info" => return cloud_info(config),
            "register" | "join" | "push" | "pull" | "sync" => {
                println!(
                    "{}",
                    "Registration codes and manual sync are no longer needed—every machine that knows the host URL and master password shares the same vault."
                        .yellow()
                );
                return Ok(());
            }
            other if !other.is_empty() => {
                return set_cloud_endpoint(config, other.to_string());
            }
            _ => {}
        }
    }

    show_status(config);
    Ok(())
}

fn set_cloud_endpoint(config: &mut AppConfig, input: String) -> Result<(), String> {
    let normalized = normalize_base_url(&input)?;
    let api = CloudApi::new(normalized.clone()).map_err(to_message)?;

    let password = master_password::prompt_master_password("Enter the host master password");
    let auth_hash = master_password::hash_password(&password);
    api.verify_master(&auth_hash).map_err(to_message)?;

    config.base_url = Some(normalized.clone());
    config.master_hash = Some(auth_hash);
    config
        .save()
        .map_err(|err| format!("Failed to save configuration: {}", err))?;

    println!(
        "{}",
        format!("Cloud API endpoint set to {}.", normalized).green()
    );
    println!(
        "{}",
        "Every CLI command will now operate directly against this host.".yellow()
    );
    Ok(())
}

fn cloud_info(config: &mut AppConfig) -> Result<(), String> {
    let base_url = config
        .base_url
        .clone()
        .ok_or_else(|| DEFAULT_STATUS_MESSAGE.to_string())?;

    let auth_hash = ensure_master_hash(config, &base_url)?;
    let api = CloudApi::new(base_url).map_err(to_message)?;

    let info = api.info(&auth_hash).map_err(to_message)?;
    if info.success {
        println!(
            "{}",
            format!(
                "Cloud vault reachable. Stored entries: {}",
                info.entry_count
            )
            .green()
            .bold()
        );
    } else {
        let message = info.message.unwrap_or_else(|| "Unknown error".into());
        println!("{}", format!("Cloud info error: {}", message).red());
    }
    Ok(())
}

fn remove_cloud_link(config: &mut AppConfig) -> Result<(), String> {
    let previous_endpoint = config.base_url.take();
    config.master_hash = None;
    config
        .save()
        .map_err(|err| format!("Failed to save configuration: {}", err))?;

    match previous_endpoint {
        Some(endpoint) => {
            println!(
                "{}",
                format!("Removed cloud endpoint {}.", endpoint).green()
            );
        }
        None => println!("{}", "No linked cloud endpoint to remove.".yellow()),
    }

    Ok(())
}

fn show_status(config: &AppConfig) {
    println!("{}", "Cloud status".green().bold());
    match &config.base_url {
        Some(url) => println!("Endpoint: {}", url),
        None => {
            println!("Endpoint: (none)");
            println!("{}", DEFAULT_STATUS_MESSAGE.yellow());
        }
    }

    let stored_msg = if config.master_hash.is_some() {
        "yes".green().to_string()
    } else {
        "no".red().to_string()
    };
    println!("Master password stored: {}", stored_msg);

    println!();
    println!("Commands:");
    println!("  x cloud <url>    Set or change the cloud endpoint");
    println!("  x cloud info     Show vault statistics");
    println!("  x cloud remove   Unlink from the cloud endpoint");
}

fn ensure_master_hash(config: &mut AppConfig, base_url: &str) -> Result<String, String> {
    if let Some(hash) = config.master_hash.clone() {
        return Ok(hash);
    }

    let password = master_password::prompt_master_password("Enter the host master password");
    let auth_hash = master_password::hash_password(&password);
    let api = CloudApi::new(base_url.to_string()).map_err(to_message)?;
    api.verify_master(&auth_hash).map_err(to_message)?;

    config.master_hash = Some(auth_hash.clone());
    config
        .save()
        .map_err(|err| format!("Failed to save configuration: {}", err))?;
    Ok(auth_hash)
}

fn normalize_base_url(input: &str) -> Result<String, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Cloud URL cannot be empty".to_string());
    }

    let parsed = Url::parse(trimmed)
        .or_else(|_| Url::parse(&format!("http://{}", trimmed)))
        .map_err(|err| format!("Invalid URL: {}", err))?;

    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err("Cloud URL must start with http:// or https://".to_string());
    }

    let mut normalized = parsed;
    if normalized.path() == "/" || normalized.path().is_empty() {
        normalized.set_path("api/cloudsync");
    }

    let mut final_url = normalized.to_string();
    while final_url.ends_with('/') {
        final_url.pop();
    }

    Ok(final_url)
}

fn to_message(error: CloudClientError) -> String {
    match error {
        CloudClientError::Http(err) => format!("Network error: {}", err),
        CloudClientError::Failure(msg) => msg,
        CloudClientError::AuthenticationFailed => AUTH_FAILURE.to_string(),
    }
}

const AUTH_FAILURE: &str =
    "Authentication failed. Verify the master password and host configuration.";
