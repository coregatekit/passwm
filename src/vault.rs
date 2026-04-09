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
}
