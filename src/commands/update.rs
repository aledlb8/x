use owo_colors::OwoColorize;
use std::env::consts::{ARCH, OS};

const REPO_OWNER: &str = "aledlb8";
const REPO_NAME: &str = "x";

pub fn update_program() {
    println!("{}", "Checking for updates...".yellow());

    let current_version = env!("CARGO_PKG_VERSION");
    let (bin_name, target) = match platform_binaries() {
        Ok(values) => values,
        Err(err) => {
            eprintln!("{}", err.red());
            return;
        }
    };

    match self_update::backends::github::Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name(bin_name)
        .target(target)
        .show_download_progress(true)
        .current_version(current_version)
        .build()
    {
        Ok(update) => match update.update() {
            Ok(status) => {
                if status.version() == current_version {
                    println!(
                        "{}",
                        format!(
                            "No update available. You're already running version {}.",
                            current_version
                        )
                        .green()
                        .bold()
                    );
                } else {
                    println!(
                        "{} {}",
                        "Update successful!".green(),
                        format!("New version {} installed.", status.version()).bold()
                    );
                }
            }
            Err(e) => {
                println!("{} {}", "Update failed:".red(), e);
            }
        },
        Err(e) => {
            println!("{} {}", "Update configuration failed:".red(), e);
        }
    }
}

fn platform_binaries() -> Result<(&'static str, &'static str), &'static str> {
    let target = match (OS, ARCH) {
        ("windows", "x86_64") => "x86_64-pc-windows-msvc",
        ("linux", "x86_64") => "x86_64-unknown-linux-gnu",
        ("linux", "aarch64") => "aarch64-unknown-linux-gnu",
        ("macos", "x86_64") => "x86_64-apple-darwin",
        ("macos", "aarch64") => "aarch64-apple-darwin",
        _ => return Err("Automatic updates are not yet supported on this platform."),
    };

    let bin_name = if OS == "windows" { "x.exe" } else { "x" };
    Ok((bin_name, target))
}
