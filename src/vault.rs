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
