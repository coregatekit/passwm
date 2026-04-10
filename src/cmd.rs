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
