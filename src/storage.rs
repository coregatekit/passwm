use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct VaultFile {
    salt: String,
    ciphertext: String,
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::vault::{PasswordEntry, Vault};

    fn make_vault_with_entries() -> Vault {
        let mut vault = Vault::new();
        vault
            .add(PasswordEntry {
                service: "github".to_string(),
                username: "alice".to_string(),
                password: "s3cr3t".to_string(),
            })
            .unwrap();
        vault
            .add(PasswordEntry {
                service: "email".to_string(),
                username: "alice@gmail.com".to_string(),
                password: "g00gl3pass".to_string(),
            })
            .unwrap();
        vault
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        // Arrange
        let dir = tempdir().unwrap();
        let path = dir.path().join("vault.pwm");
        let vault = make_vault_with_entries();
        let password = "master_password";

        // Act
        save_vault(&vault, &path, password).unwrap();
        let loaded = load_vault(&path, password).unwrap();

        // Assert
        assert_eq!(vault.entries, loaded.entries);
    }
}
