use crate::error::{PasswmError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PasswordEntry {
    pub service: String,
    pub username: String,
    pub password: String, // plaintext in memory will be encrypted on save
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Vault {
    pub entries: Vec<PasswordEntry>,
}

impl Vault {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: PasswordEntry) -> Result<()> {
        if self.entries.iter().any(|e| e.service == entry.service) {
            return Err(PasswmError::NotFound(format!(
                "Entry for '{}' already exists",
                entry.service
            )));
        }
        self.entries.push(entry);
        Ok(())
    }

    pub fn get(&self, service: &str) -> Result<&PasswordEntry> {
        self.entries
            .iter()
            .find(|e| e.service == service)
            .ok_or_else(|| PasswmError::NotFound(service.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::vault::{PasswordEntry, Vault};

    fn make_entry(service: &str, username: &str, password: &str) -> PasswordEntry {
        PasswordEntry {
            service: service.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    #[test]
    fn test_add_and_get_entry() {
        let mut vault = Vault::new();
        vault.add(make_entry("github", "alice", "s3cr3t")).unwrap();
        let entry = vault.get("github").unwrap();
        assert_eq!(entry.username, "alice");
        assert_eq!(entry.password, "s3cr3t");
    }

    #[test]
    fn test_add_duplicate_service_fails() {
        let mut vault = Vault::new();
        vault.add(make_entry("github", "alice", "pass1")).unwrap();
        assert!(vault.add(make_entry("github", "bob", "pass2")).is_err());
    }
}
