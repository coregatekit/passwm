# passwm

A CLI-based password management tool built with Rust that lets you securely store, retrieve, and manage your passwords from the terminal.

## Features

- Add / List / Get / Update / Delete / Search password entries
- AES-256-GCM encryption at rest
- Argon2id key derivation from master password
- Default vault stored at `~/.passwm/vault.pwm` (auto-created on first run)
- Password copied to clipboard on `get`, auto-cleared after 30 seconds
- Master password prompt (hidden input — never echoed to terminal)
- Custom vault path via `--vault-path`

## Architecture

```text
User Input (terminal)
        │
        ▼
   helper.rs   ── resolve_vault_path() · prompt_password() · copy_to_clipboard_with_clear()
        │
        ▼
    main.rs    ── parse CLI → load/save vault → route command
        │
   ┌────┴────┐
   ▼         ▼
 cli.rs    cmd.rs   ── argument definitions · command handlers
              │
              ▼
           vault.rs  ── in-memory CRUD
              │
       ┌──────┴──────┐
       ▼             ▼
   crypto.rs     storage.rs  ── encrypt/decrypt · read/write disk
```

| File | Responsibility |
|---|---|
| `main.rs` | Entry point, wires all modules together |
| `cli.rs` | clap CLI argument and subcommand definitions |
| `cmd.rs` | Command handlers with terminal output |
| `vault.rs` | In-memory vault CRUD operations |
| `crypto.rs` | Argon2id KDF + AES-256-GCM encrypt/decrypt |
| `storage.rs` | Serialize vault → encrypt → persist to disk |
| `helper.rs` | Password prompt, vault path resolution, clipboard |
| `error.rs` | Custom `PasswmError` enum and `Result<T>` alias |

## Prerequisites

- **Rust toolchain (stable)** — install via <https://rustup.rs>
- **Linux only**: `libxcb` development libraries (required by the `arboard` clipboard crate)

## Installation

### Linux

```bash
# Install clipboard dependencies (required by arboard)
# Ubuntu / Debian:
sudo apt-get update
sudo apt-get install -y libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
# Fedora / RHEL:  sudo dnf install libxcb-devel
# Arch Linux:     sudo pacman -S libxcb

# Clone and build
git clone https://github.com/coregatekit/passwm.git
cd passwm
cargo build --release

# Install to PATH
cargo install --path .
```

### macOS (Intel)

```bash
git clone https://github.com/coregatekit/passwm.git
cd passwm
cargo build --release

# Install to PATH
cargo install --path .
```

### macOS (Apple Silicon — M1/M2/M3)

```bash
git clone https://github.com/coregatekit/passwm.git
cd passwm
cargo build --release --target aarch64-apple-darwin

# Install to PATH
cargo install --path . --target aarch64-apple-darwin
```

### Windows

```powershell
git clone https://github.com/coregatekit/passwm.git
cd passwm
cargo build --release

# Install to PATH
cargo install --path .
```

After `cargo install --path .`, the `passwm` binary will be available system-wide in `~/.cargo/bin/`.

## Usage

```bash
# Add a new password entry
passwm add --service github --username alice --password s3cr3t

# List all stored entries (shows service and username only, no passwords)
passwm list

# Get the password for a service (also copies to clipboard, clears after 30s)
passwm get --service github

# Update an existing entry (username and/or password)
passwm update --service github --password new_s3cr3t
passwm update --service github --username new_alice --password new_s3cr3t

# Delete an entry
passwm delete --service github

# Search entries by service name or username
passwm search --query git

# Use a custom vault path
passwm --vault-path /path/to/my.pwm list
```

## Vault File Location

- **Default**: `~/.passwm/vault.pwm`
- The directory `~/.passwm/` is created automatically on first run
- Can be overridden with `--vault-path <PATH>`

## Security Notes

- Master password is **never stored** — it is used only to derive the AES-256 encryption key via Argon2id
- Each save generates a **fresh random salt** → ciphertext differs on every save even for the same data
- AES-256-GCM provides both **confidentiality and integrity** — any tampering is detected on load
- Sensitive keys are held in `Zeroizing<T>` and wiped from memory on drop
- Clipboard is automatically cleared after 30 seconds

## Running Tests

```bash
cargo test
```

## License

This project is licensed under the [MIT License](LICENSE).
