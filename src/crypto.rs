use argon2::Argon2;
use zeroize::Zeroizing;

use crate::error::{PasswmError, Result};

pub const KEY_LEN: usize = 32; // 256-bit key
pub const SALT_LEN: usize = 16;

/// Derives a key from the master password and salt using Argon2id
pub fn derive_key(master_password: &str, salt: &[u8]) -> Result<Zeroizing<[u8; KEY_LEN]>> {
    let mut key = Zeroizing::new([0u8; KEY_LEN]);
    Argon2::default().hash_password_into(master_password.as_bytes(), salt, key.as_mut())
    .map_err(|e| PasswmError::EncryptionError(e.to_string()))?;
    Ok(key)
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_derive_key_is_deterministic() {
      let salt = [0u8; SALT_LEN];
      let key1 = derive_key("master123", &salt).unwrap();
      let key2 = derive_key("master123", &salt).unwrap();
      assert_eq!(*key1, *key2); // -> same password and salt should yield the same key
  }

  #[test]
  fn test_derive_key_differs_with_different_password() {
      let salt = [0u8; SALT_LEN];
      let key1 = derive_key("password_a", &salt).unwrap();
      let key2 = derive_key("password_b", &salt).unwrap();
      assert_ne!(*key1, *key2); // -> different passwords should yield different keys
  }

  #[test]
  fn test_derive_key_differs_with_different_salt() {
      let salt1 = [0u8; SALT_LEN];
      let salt2 = [1u8; SALT_LEN];
      let key1 = derive_key("master123", &salt1).unwrap();
      let key2 = derive_key("master123", &salt2).unwrap();
      assert_ne!(*key1, *key2); // -> same password but different salt should yield different keys
  }
}