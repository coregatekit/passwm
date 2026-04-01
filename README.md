# passwm

A CLI-based password management tool built with Rust that lets you securely store, retrieve, and manage your passwords from the terminal.

## Features

- **Add Password** – Store a new password entry with a service name, username, and password.
- **List Passwords** – Display all stored password entries (service names and usernames only).
- **Get Password** – Retrieve the password for a specific service.
- **Update Password** – Update the username or password for an existing entry.
- **Delete Password** – Remove a stored password entry.
- **Search** – Search for entries by service name or username.
- **Encryption** – All passwords are encrypted at rest using strong symmetric encryption.
- **Master Password** – Protect your vault with a single master password.

## Software Design

```
passwm/
├── src/
│   ├── main.rs          # CLI entry point and command routing
│   ├── cli.rs           # CLI argument and subcommand definitions
│   ├── vault.rs         # Password vault (add, list, get, update, delete, search)
│   ├── crypto.rs        # Encryption and decryption helpers
│   └── storage.rs       # Persistent storage (read/write vault file)
├── Cargo.toml
└── README.md
```

### Architecture Overview

```
User Input (CLI)
      │
      ▼
  cli.rs  ──► Parses commands and arguments
      │
      ▼
  vault.rs ──► Business logic (CRUD operations on password entries)
      │
      ├──► crypto.rs  ──► Encrypts / decrypts password data
      │
      └──► storage.rs ──► Reads / writes the encrypted vault file
```

The vault is stored as a single encrypted file on disk. The master password is used to derive an encryption key (via a key-derivation function), which is then used to encrypt and decrypt the vault contents. No plaintext passwords are ever written to disk.

## Installation

> **Requirements:** [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)

```bash
# Clone the repository
git clone https://github.com/coregatekit/passwm.git
cd passwm

# Build and install
cargo install --path .
```

## Usage

```bash
# Add a new password entry
passwm add --service github --username alice --password s3cr3t

# List all stored entries
passwm list

# Get the password for a service
passwm get --service github

# Update an existing entry
passwm update --service github --password new_s3cr3t

# Delete an entry
passwm delete --service github

# Search for entries
passwm search --query git
```

## License

This project is licensed under the [MIT License](LICENSE).
