use owo_colors::OwoColorize;

pub fn update_program() {
    println!("{}", "🚀 Checking for updates...".yellow());

    let current_version = env!("CARGO_PKG_VERSION");
    let bin_name = "x.exe";

    let repo_owner = "aledlb8";
    let repo_name = "x";

    match self_update::backends::github::Update::configure()
        .repo_owner(repo_owner)
        .repo_name(repo_name)
        .bin_name(bin_name)
        .target(bin_name)
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
                            "✅ No update available. You're already running version {}.",
                            current_version
                        )
                        .green()
                        .bold()
                    );
                } else {
                    println!(
                        "{} {}",
                        "✅ Update successful!".green(),
                        format!("New version {} installed.", status.version()).bold()
                    );
                }
            }
            Err(e) => {
                println!("{} {}", "❌ Update failed:".red(), e);
            }
        },
        Err(e) => {
            println!("{} {}", "❌ Update configuration failed:".red(), e);
        }
    }
}