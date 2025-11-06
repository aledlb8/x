X Password Manager
==================

`x` is a simple command-line vault. You run a small host process, point your machines at it, and every secret stays in sync.

What You Can Do
---------------
- Save passwords, secure notes, or card details (all encrypted before they leave your machine).
- Generate strong passwords on demand.
- Export or import your data when you need a backup.
- Run the host anywhere: your laptop, a Raspberry Pi, or a remote VM.

Getting Started
---------------
1. Install
   - Windows (PowerShell)
     ```powershell
     iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/aledlb8/x/main/install.ps1'))
     ```
     Set `X_VERSION`, `X_INSTALL_ROOT`, `X_INSTALL_BIN_DIR`, or `X_TARGET` before running if you need to pin a release or customize the install location/target triple.
   - Linux or macOS (Bash)
     ```bash
     curl -fsSL https://raw.githubusercontent.com/aledlb8/x/main/install.sh | bash
     ```
     The script installs to `~/.local/bin` by default. Set `X_INSTALL_ROOT`, `X_INSTALL_BIN_DIR`, or `X_TARGET` to customize the destination or target triple.

2. Start the host (once):
   ```bash
   x host
   ```
   - Set a master password when prompted.
   - The host remembers the hash so you won't be asked again unless you delete the data file.

3. Point a client at the host:
   ```bash
   x cloud http://your-host:4000
   ```
   - Enter the same master password the first time.
   - From now on the CLI keeps an encrypted hash so routine commands don't ask again.

4. Use the vault commands:
   ```bash
   x add        # create a new entry
   x list       # show stored names
   x get        # view details (and copy sensitive fields)
   x edit       # update an item
   x delete     # remove an item
   x passgen    # generate a password
   x export     # write an encrypted JSON export
   x import     # load an encrypted JSON export
   ```
   Every command fetches the latest data from the host, applies your change, and saves it back immediately.

Images
--------------
<img width="922" height="562" alt="image" src="https://github.com/user-attachments/assets/b1320b4d-52f9-44f7-bba0-26ca224532c6" />
<img width="922" height="562" alt="image" src="https://github.com/user-attachments/assets/768165a7-dbb5-431c-a600-6299fd88f20a" />
<img width="922" height="562" alt="image" src="https://github.com/user-attachments/assets/97c5608c-df25-457c-9d15-39eda8d142a8" />
<img width="922" height="562" alt="image" src="https://github.com/user-attachments/assets/08bc0604-9465-41b8-ac75-29860f92d4de" />

Helpful Extras
--------------
- `x cloud info` – quick health check that the host is reachable and how many entries are stored.
- `x cloud remove` – forget the current host and stored hash (use this before switching servers).
- Host data lives in `%APPDATA%/x_cli/cloud_host.db` on Windows, `~/.local/share/x_cli/cloud_host.db` on Linux, or `~/Library/Application Support/x_cli/cloud_host.db` on macOS (or the path you pass to `x host --data`).
- Client settings live in `%APPDATA%/x_cli/config.json` on Windows, `~/.local/share/x_cli/config.json` on Linux, or `~/Library/Application Support/x_cli/config.json` on macOS. Delete this file to force the CLI to re-prompt for the master password.

Security Notes
--------------
- The master password never leaves your machine; only a Blake3 hash is sent for authentication.
- Secrets are encrypted with AES-256-GCM before they travel to the host.
- Exports stay encrypted. If you lose the master password, you cannot decrypt them.

Working on the Project
----------------------
```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo check
cargo test
cargo build
```
Folders you will see:
- `cloud/` – host API and HTTP client.
- `commands/` – all CLI subcommands.
- `vault.rs` – data model and encryption helpers.
- `config.rs` – minimal config loader/saver.

Build for release:
-------------------
```bash
cargo build --release
```

Releasing
---------
1. Update the version in `Cargo.toml` and create a matching tag (e.g., `v0.2.0`).
2. Push the tag to GitHub (or trigger the `release` workflow manually). The workflow builds signed archives for:
   - `x86_64-pc-windows-msvc`
   - `x86_64-unknown-linux-gnu`
   - `x86_64-apple-darwin`
   - `aarch64-apple-darwin`
3. The workflow uploads assets named `x-<tag>-<target>.(zip|tar.gz)` to the release. The installers and `x update` use these names, so keep the format intact.
