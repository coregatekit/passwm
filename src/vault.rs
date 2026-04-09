use crate::error::{PasswmError, Result};
use serde::{Deserialize, Serialize, ser};

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

    pub fn list(&self) -> Vec<(&str, &str)> {
        self.entries
            .iter()
            .map(|e| (e.service.as_str(), e.username.as_str()))
            .collect()
    }

    pub fn update(
        &mut self,
        service: &str,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<()> {
        let entry = self
            .entries
            .iter_mut()
            .find(|e| e.service == service)
            .ok_or_else(|| PasswmError::NotFound(service.to_string()))?;

        if let Some(u) = username {
            entry.username = u;
        }
        if let Some(p) = password {
            entry.password = p;
        }

        println!("🟢 {} updated successfully!", service);
        Ok(())
    }

    pub fn delete(&mut self, service: &str) -> Result<()> {
        let before = self.entries.len();
        self.entries.retain(|e| e.service != service);
        if self.entries.len() == before {
            return Err(PasswmError::NotFound(service.to_string()));
        }
        return Ok(());
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

    #[test]
    fn test_get_nonexistent_returns_error() {
        let vault = Vault::new();
        assert!(vault.get("nonexistent").is_err());
    }

    #[test]
    fn test_list_returns_service_and_username_only() {
        let mut vault = Vault::new();
        vault.add(make_entry("github", "alice", "secret1")).unwrap();
        vault.add(make_entry("google", "alice", "secret2")).unwrap();
        let list = vault.list();
        assert_eq!(list.len(), 2);
        assert!(list.iter().all(|(_, _)| true)); // passwords not exposed
    }

    #[test]
    fn test_update_password() {
        let mut vault = Vault::new();
        vault.add(make_entry("github", "alice", "old")).unwrap();
        vault
            .update("github", None, Some("new_pass".to_string()))
            .unwrap();
        assert_eq!(vault.get("github").unwrap().password, "new_pass");
    }

    #[test]
    fn test_delete_entry() {
        let mut vault = Vault::new();
        vault.add(make_entry("github", "alice", "pass")).unwrap();
        vault.delete("github").unwrap();
        assert!(vault.get("github").is_err());
    }

    #[test]
    fn test_delete_nonexistent_fails() {
        let mut vault = Vault::new();
        assert!(vault.delete("github").is_err());
    }

    #[test]
    fn test_search_by_service() {
        let mut vault = Vault::new();
        vault.add(make_entry("github", "alice", "p1")).unwrap();
        vault.add(make_entry("gitlab", "bob", "p2")).unwrap();
        vault.add(make_entry("google", "alice", "p3")).unwrap();
        let results = vault.search("git");
        assert_eq!(results.len(), 2);
    }
}
