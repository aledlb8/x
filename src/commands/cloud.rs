use dialoguer::{Input, Select};
use hostname::get;
use owo_colors::OwoColorize;
use rand::Rng;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::collections::HashMap;
use std::error::Error;

static API_URL: &str = "http://15.204.219.61:4000/api/cloudsync";

#[derive(Serialize, Deserialize)]
pub struct VaultEntry {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
struct CloudResponse {
    success: bool,
    message: String,
    vault: Option<Vec<VaultEntry>>,
    group_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CloudInfoResponse {
    success: bool,
    #[serde(default)]
    message: Option<String>,
    machines: Vec<MachineInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MachineInfo {
    #[serde(flatten)]
    info: HashMap<String, Vec<u8>>,
}

pub fn export_vault(db: &Db) -> Vec<VaultEntry> {
    let reserved = ["session", "master_password"];
    let mut vault = Vec::new();
    for item in db.iter() {
        if let Ok((key_bytes, value_bytes)) = item {
            let key = String::from_utf8(key_bytes.to_vec()).unwrap_or_default();
            if reserved.contains(&key.as_str()) {
                continue;
            }
            let value = String::from_utf8(value_bytes.to_vec()).unwrap_or_default();
            vault.push(VaultEntry { key, value });
        }
    }
    vault
}

pub fn cloud_sync(db: &Db) -> Result<(), reqwest::Error> {
    if let Ok(Some(group_id_bytes)) = db.get("cloud_group") {
        let group_id = String::from_utf8_lossy(&group_id_bytes).to_string();
        let choices = vec!["Upload vault to cloud", "Download vault from cloud"];
        let selection = Select::new()
            .with_prompt(format!(
                "Already linked to group {}. Select an action:",
                group_id
            ))
            .items(&choices)
            .default(0)
            .interact()
            .unwrap();

        let machine = get().unwrap_or_else(|_| "Unknown".into());
        let client = Client::new();

        if selection == 0 {
            println!("{}", "Uploading vault to cloud...".yellow());
            let vault = export_vault(db);
            let payload = serde_json::json!({
                "group_id": group_id,
                "machine": machine,
                "vault": vault,
            });
            let url = format!("{}/update", API_URL);
            let res = client.post(&url).json(&payload).send()?;
            if res.status().is_success() {
                let resp: CloudResponse = res.json()?;
                if resp.success {
                    println!("{}", "Vault updated with cloud sync.".green());
                } else {
                    println!("{}", format!("Update failed: {}", resp.message).red());
                }
            } else {
                println!("{}", "Error connecting to the cloud sync server.".red());
            }
        } else {
            println!("{}", "Downloading vault from cloud...".yellow());
            let payload = serde_json::json!({
                "code": group_id,
                "machine": machine,
            });
            let url = format!("{}/link", API_URL);
            let res = client.post(&url).json(&payload).send()?;
            if res.status().is_success() {
                let resp: CloudResponse = res.json()?;
                if resp.success {
                    println!("{}", "Cloud sync link established successfully.".green());
                    if let Some(vault) = resp.vault {
                        for entry in vault {
                            if entry.key == "master_password" {
                                continue;
                            }
                            db.insert(entry.key, entry.value.into_bytes()).unwrap();
                        }
                        db.flush().unwrap();
                        println!("{}", "Local vault updated with cloud data.".green());
                    } else {
                        println!("{}", "No vault data received from the server.".yellow());
                    }
                } else {
                    println!("{}", format!("Link failed: {}", resp.message).red());
                }
            } else {
                println!("{}", "Error connecting to the cloud sync server.".red());
            }
        }
    } else {
        // Not linked yet: perform registration.
        println!("{}", "Starting cloud sync registration...".yellow());
        let code: u32 = rand::thread_rng().gen_range(100000..1000000);
        let code_str = code.to_string();

        let machine = get().unwrap_or_else(|_| "Unknown".into());
        let vault = export_vault(db);
        let payload = serde_json::json!({
            "code": code_str,
            "machine": machine,
            "vault": vault,
        });

        let client = Client::new();
        let url = format!("{}/register", API_URL);
        let res = client.post(&url).json(&payload).send()?;

        if res.status().is_success() {
            let resp: CloudResponse = res.json()?;
            if resp.success {
                println!(
                    "{}",
                    format!(
                        "Registration successful. Cloud sync code: {}. Use this code on your other machine to link.",
                        code_str
                    )
                    .green()
                    .bold()
                );
                let group_id = resp.group_id.unwrap_or(code_str);
                db.insert("cloud_group", group_id.as_bytes()).unwrap();
                db.flush().unwrap();
            } else {
                println!("{}", format!("Registration failed: {}", resp.message).red());
            }
        } else {
            println!("{}", "Error connecting to the cloud sync server.".red());
        }
    }
    Ok(())
}

pub fn cloud_code(db: &Db) -> Result<(), reqwest::Error> {
    println!(
        "{}",
        "Enter the cloud sync code to link this machine:".yellow()
    );

    let code_input: String = Input::new()
        .with_prompt("Cloud Sync Code")
        .interact_text()
        .unwrap();

    let machine = get().unwrap_or_else(|_| "Unknown".into());
    let payload = serde_json::json!({
        "code": code_input,
        "machine": machine,
    });

    let client = Client::new();
    let url = format!("{}/link", API_URL);
    let res = client.post(&url).json(&payload).send()?;
    if res.status().is_success() {
        let resp: CloudResponse = res.json()?;
        if resp.success {
            println!("{}", "Cloud sync link established successfully.".green());
            if let Some(vault) = resp.vault {
                for entry in vault {
                    if entry.key == "master_password" {
                        continue;
                    }
                    db.insert(entry.key, entry.value.into_bytes()).unwrap();
                }
                db.flush().unwrap();
                println!("{}", "Local vault updated with cloud data.".green());
            } else {
                println!("{}", "No vault data received from the server.".yellow());
            }
            if let Some(group_id) = resp.group_id {
                db.insert("cloud_group", group_id.as_bytes()).unwrap();
                db.flush().unwrap();
            }
        } else {
            println!("{}", format!("Link failed: {}", resp.message).red());
        }
    } else {
        println!("{}", "Error connecting to the cloud sync server.".red());
    }

    Ok(())
}

pub fn cloud_info(db: &Db) -> Result<(), Box<dyn Error>> {
    println!("{}", "Fetching cloud sync info...".yellow());

    let group_code_opt = db.get("cloud_group")?;
    let group_code = if let Some(group_bytes) = group_code_opt {
        String::from_utf8_lossy(&group_bytes).to_string()
    } else {
        println!(
            "{}",
            "This machine is not linked to any cloud group. Please register or link first.".red()
        );
        return Ok(());
    };

    let client = Client::new();
    let url = format!("{}/info?code={}", API_URL, group_code);
    let res = client.get(&url).send()?;

    if res.status().is_success() {
        let raw_body = res.text()?;

        let resp: CloudInfoResponse = serde_json::from_str(&raw_body).map_err(|e| {
            eprintln!("Error decoding response body: {}", e);
            e
        })?;

        if resp.success {
            println!("{}", "Connected machines:".green().bold());
            for machine in resp.machines {
                for (platform, data) in machine.info {
                    let decoded =
                        String::from_utf8(data).unwrap_or_else(|_| "Invalid UTF-8".to_string());
                    println!("  - {}: {}", platform, decoded);
                }
            }
        } else {
            println!(
                "{}",
                format!(
                    "Cloud info error: {}",
                    resp.message.unwrap_or("Unknown error".into())
                )
                .red()
            );
        }
    } else {
        let status = res.status();
        let body = res
            .text()
            .unwrap_or_else(|_| "Unable to read response body".to_string());
        println!(
            "{}",
            format!(
                "Error fetching cloud sync info. HTTP Status: {}. Details: {}",
                status, body
            )
            .red()
        );
    }

    Ok(())
}
