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

## Planned Software Design

> **Note:** The Rust implementation has not been added to the repository yet. The module layout and architecture below describe the intended design.

```text
passwm/             (planned)
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

```text
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

Design goal: The vault will be stored as a single encrypted file on disk. The master password will be used to derive an encryption key (via a key-derivation function), which will then be used to encrypt and decrypt the vault contents. Plaintext passwords are not intended to be written to disk.

## Installation

> **Note:** The Rust implementation of `passwm` is not yet published in this repository. There is no installable CLI binary at this time; the layout and commands shown below describe the planned design. Installation instructions will be added once the Rust sources and `Cargo.toml` are available.

<!--
Planned installation options (once the Rust project is added):

- From source:
  ```bash
  git clone https://github.com/coregatekit/passwm.git
  cd passwm
  cargo install --path .
  ```

- From crates.io (planned):
  ```bash
  cargo install passwm
  ```
-->

## Planned Usage

> **Note:** The following examples show the target CLI syntax once the tool is implemented. They do not reflect working commands in the current repository.

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
