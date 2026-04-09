use std::path::Path;

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};

use crate::{
    crypto::{self, KEY_LEN, encrypt},
    error::{PasswmError, Result},
    vault::Vault,
};

// ──────  VaultFile structure ─────────────────────────────────────
// Json format for storing the vault on disk
// salt         : random 16 bytes (base64-encoded) used for key derivation
// ciphertext   : [nonce(12B) | encrypted_vault | auth_tag(16B)] (base64-encoded)

#[derive(Serialize, Deserialize)]
struct VaultFile {
    salt: String,
    ciphertext: String,
}

// ──────  Public API for storage ─────────────────────────────────────

/// Encrypt vault and save to disk
/// Generate new salt every time to ensure different ciphertexts for the same vault and password
pub fn save_vault(vault: &Vault, path: &Path, master_password: &str) -> Result<()> {
    let plaintext = serde_json::to_vec(vault)?;

    let salt = crypto::generate_salt();
    let key = crypto::derive_key(master_password, &salt)?;

    let ciphertext = encrypt(&key, &plaintext)?;

    let vault_file = VaultFile {
        salt: BASE64.encode(salt),
        ciphertext: BASE64.encode(&ciphertext),
    };

    let file_bytes = serde_json::to_vec(&vault_file)?;
    std::fs::write(path, file_bytes).map_err(|e| PasswmError::StorageError(e.to_string()))?;

    Ok(())
}

/// Load from disk and decrypt into Vault
pub fn load_vault(path: &Path, master_password: &str) -> Result<Vault> {
    let file_bytes = std::fs::read(path).map_err(|e| PasswmError::StorageError(e.to_string()))?;

    let vault_file: VaultFile = serde_json::from_slice(&file_bytes)?;

    let salt = BASE64
        .decode(&vault_file.salt)
        .map_err(|e| PasswmError::StorageError(e.to_string()))?;
    let ciphertext = BASE64
        .decode(&vault_file.ciphertext)
        .map_err(|e| PasswmError::StorageError(e.to_string()))?;

    let key = crypto::derive_key(master_password, &salt)?;
    let key_array: &[u8; KEY_LEN] = key
        .as_ref()
        .try_into()
        .map_err(|_| PasswmError::DecryptionError)?;

    let plaintext = crypto::decrypt(key_array, &ciphertext)?;

    let vault: Vault = serde_json::from_slice(&plaintext)?;

    Ok(vault)
}
// ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
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
        let dir = tempdir().unwrap();
        let path = dir.path().join("vault.pwm");
        let vault = make_vault_with_entries();
        let password = "master_password";

        save_vault(&vault, &path, password).unwrap();
        let loaded = load_vault(&path, password).unwrap();

        assert_eq!(vault.entries, loaded.entries);
    }

    #[test]
    fn test_save_creates_file_on_disk() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("vault.pwm");
        assert!(!path.exists());

        save_vault(&Vault::new(), &path, "pass").unwrap();

        assert!(path.exists());
    }
}
