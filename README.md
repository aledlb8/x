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
1. Start the host (once):
   ```bash
   x host
   ```
   - Set a master password when prompted.
   - The host remembers the hash so you won't be asked again unless you delete the data file.

2. Point a client at the host:
   ```bash
   x cloud http://your-host:4000
   ```
   - Enter the same master password the first time.
   - From now on the CLI keeps an encrypted hash so routine commands don't ask again.

3. Use the vault commands:
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

Helpful Extras
--------------
- `x cloud info` – quick health check that the host is reachable and how many entries are stored.
- `x cloud remove` – forget the current host and stored hash (use this before switching servers).
- Host data lives in `%APPDATA%/x_cli/cloud_host.db` (or the path you pass to `x host --data`).
- Client settings live in `%APPDATA%/x_cli/config.json`. Delete this file to force the CLI to re-prompt for the master password.

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
```
Folders you will see:
- `cloud/` – host API and HTTP client.
- `commands/` – all CLI subcommands.
- `vault.rs` – data model and encryption helpers.
- `config.rs` – minimal config loader/saver.

Install from Source
-------------------
```bash
cargo build --release
```
On Windows you can still use the original PowerShell helper:
```powershell
iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/aledlb8/x/main/install.ps1'))
```