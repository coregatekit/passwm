use crate::error::Result;
use crate::helper::copy_to_clipboard_with_clear;
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

pub fn cmd_get(vault: &Vault, service: &str) -> Result<()> {
    let entry = vault.get(service)?;
    println!("🔐 Service  : {}", entry.service);
    println!("👤 Username : {}", entry.username);
    println!("🔑 Password : {}", entry.password);

    copy_to_clipboard_with_clear(&entry.password)?;

    Ok(())
}

pub fn cmd_update(
    vault: &mut Vault,
    service: &str,
    username: Option<String>,
    password: Option<String>,
) -> Result<()> {
    vault.update(service, username, password)?;
    println!("✅ Updated entry for '{service}'");
    Ok(())
}

pub fn cmd_delete(vault: &mut Vault, service: &str) -> Result<()> {
    vault.delete(service)?;
    println!("🗑️ Deleted entry for '{service}'");
    Ok(())
}

pub fn cmd_search(vault: &Vault, query: &str) {
    let results = vault.search(query);
    if results.is_empty() {
        println!("🔍 No entries found matching '{query}'.");
        return;
    }
    println!("🔍 Found {} result(s):", results.len());
    println!("{:<20} {:<30}", "SERVICE", "USERNAME");
    println!("{}", "-".repeat(50));
    for entry in results {
        println!("{:<20} {:<30}", entry.service, entry.username);
    }
}
