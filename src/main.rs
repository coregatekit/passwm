use std::path::PathBuf;

use clap::Parser;
use passwm::{
    cli::{Cli, Commands},
    cmd::{cmd_add, cmd_delete, cmd_get, cmd_list, cmd_search, cmd_update},
    error::Result,
    helper::prompt_password,
    storage::{load_vault, save_vault, vault_exists},
    vault::Vault,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("❌ Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let vault_path = PathBuf::from(&cli.vault_path);

    let master_password = prompt_password("🔑 Enter maste password: ")?;

    let mut vault = if vault_exists(&vault_path) {
        load_vault(&vault_path, &master_password)?
    } else {
        println!("📦 No vault found. Creating a new vault...");
        Vault::new()
    };

    match cli.command {
        Commands::Add {
            service,
            username,
            password,
        } => {
            cmd_add(&mut vault, service, username, password)?;
        }
        Commands::List => cmd_list(&vault),
        Commands::Get { service } => cmd_get(&vault, &service)?,
        Commands::Update {
            service,
            username,
            password,
        } => {
            cmd_update(&mut vault, &service, username, password)?;
        }
        Commands::Delete { service } => cmd_delete(&mut vault, &service)?,
        Commands::Search { query } => cmd_search(&mut vault, &query),
    }

    save_vault(&vault, &vault_path, &master_password)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use passwm::{
        cmd::{cmd_add, cmd_delete, cmd_get, cmd_update},
        vault::{PasswordEntry, Vault},
    };

    fn make_entry(service: &str, username: &str, password: &str) -> PasswordEntry {
        PasswordEntry {
            service: service.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    /// --- cmd_add tests ---
    #[test]
    fn test_cmd_add_inserts_entry() {
        let mut vault = Vault::new();
        cmd_add(&mut vault, "github".into(), "alice".into(), "s3cr3t".into()).unwrap();
        assert!(vault.get("github").is_ok());
    }

    #[test]
    fn test_cmd_add_duplicate_fails() {
        let mut vault = Vault::new();
        cmd_add(&mut vault, "github".into(), "alice".into(), "s3cr3t".into()).unwrap();
        let result = cmd_add(&mut vault, "github".into(), "bob".into(), "passw0rd".into());
        assert!(result.is_err());
    }

    /// --- cmd_get tests ---
    #[test]
    fn test_cmd_get_existing_entry() {
        let mut vault = Vault::new();
        vault.add(make_entry("github", "alice", "s3cr3t")).unwrap();
        assert!(cmd_get(&vault, "github").is_ok());
    }

    #[test]
    fn test_cmd_get_nonexistent_fails() {
        let vault = Vault::new();
        assert!(cmd_get(&vault, "ghost").is_err());
    }

    /// --- cmd_update tests ---
    #[test]
    fn test_cmd_update_password_only() {
        let mut vault = Vault::new();
        vault.add(make_entry("github", "alice", "old")).unwrap();
        cmd_update(&mut vault, "github", None, Some("new_pass".into())).unwrap();
        assert_eq!(vault.get("github").unwrap().password, "new_pass");
    }

    #[test]
    fn test_cmd_update_nonexistent_fails() {
        let mut vault = Vault::new();
        let result = cmd_update(&mut vault, "ghost", None, Some("pass".into()));
        assert!(result.is_err());
    }

    /// --- cmd_delete tests ---
    #[test]
    fn test_cmd_delete_removes_entry() {
        let mut vault = Vault::new();
        vault.add(make_entry("github", "alice", "pass")).unwrap();
        cmd_delete(&mut vault, "github").unwrap();
        assert!(vault.get("github").is_err());
    }

    #[test]
    fn test_cmd_delete_nonexistent_fails() {
        let mut vault = Vault::new();
        assert!(cmd_delete(&mut vault, "ghost").is_err());
    }
}
