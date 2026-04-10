use std::path::PathBuf;

use clap::Parser;
use passwm::{
    cli::{Cli, Commands},
    error::Result,
    helper::prompt_password,
    storage::{load_vault, vault_exists},
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

    Ok(())
}
