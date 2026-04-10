use crate::error::Result;
use crate::vault::{PasswordEntry, Vault};

pub fn cmd_add(
    vault: &mut Vault,
    service: String,
    username: String,
    password: String,
) -> Result<()> {
    vault.add(PasswordEntry {
        service: service.clone(),
        username: username.clone(),
        password: password.clone(),
    })?;
    println!("✅ Added entry for '{service}'");
    Ok(())
}

pub fn cmd_list(vault: &Vault) {
    let entries = vault.list();
    if entries.is_empty() {
        println!("📭 No entries found.");
        return;
    }
    println!("{:<20} {:<30}", "SERVICE", "USERNAME");
    println!("{}", "-".repeat(50));
    for (service, username) in entries {
        println!("{:<20} {:<30}", service, username);
    }
}
