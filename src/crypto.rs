use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::{Aead, OsRng, rand_core::RngCore}};
use argon2::Argon2;
use zeroize::Zeroizing;

use crate::error::{PasswmError, Result};

pub const KEY_LEN: usize = 32; // 256-bit key
pub const NONCE_LEN: usize = 12; // 96-bit nonce for AES-GCM
pub const SALT_LEN: usize = 16;

/// Derives a key from the master password and salt using Argon2id
pub fn derive_key(master_password: &str, salt: &[u8]) -> Result<Zeroizing<[u8; KEY_LEN]>> {
    let mut key = Zeroizing::new([0u8; KEY_LEN]);
    Argon2::default().hash_password_into(master_password.as_bytes(), salt, key.as_mut())
    .map_err(|e| PasswmError::EncryptionError(e.to_string()))?;
    Ok(key)
}

/// Encrypt plaintext using AES-256-GCM with the derived key
/// Returns: [nonce 12 bytes || ciphertext]
pub fn encrypt(key: &[u8; KEY_LEN], plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher.encrypt(nonce, plaintext).map_err(|e| PasswmError::EncryptionError(e.to_string()))?;

    // Prepend nonce to ciphertext for storage
    let mut output = nonce_bytes.to_vec();
    output.extend_from_slice(&ciphertext);
    Ok(output)
}

/// Decrypt ciphertext using AES-256-GCM with the derived key
/// Expects input format: [nonce 12 bytes || ciphertext]
pub fn decrypt(key: &[u8; KEY_LEN], data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < NONCE_LEN {
      return Err(PasswmError::DecryptionError);
    }

    let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.decrypt(nonce, ciphertext).map_err(|_| PasswmError::DecryptionError)
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

  #[test]
  fn test_encrypt_produces_different_ciphertext_each_time() {
    let key = [42u8; KEY_LEN];
    let plaintext = b"same password";
    let ct1 = encrypt(&key, plaintext).unwrap();
    let ct2 = encrypt(&key, plaintext).unwrap();
    assert_ne!(ct1, ct2); // -> same plaintext and key should yield different ciphertexts each time
  }

  #[test]
  fn test_encrypt_decrypt_roundtrip() {
      let key = [42u8; KEY_LEN];
      let plaintext = b"my secret data";
      let encrypted = encrypt(&key, plaintext).unwrap();
      let decrypted = decrypt(&key, &encrypted).unwrap();
      assert_eq!(decrypted, plaintext);
  }
}