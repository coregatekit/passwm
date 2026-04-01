use argon2::Argon2;
use zeroize::Zeroizing;

use crate::error::{PasswmError, Result};

pub const KEY_LEN: usize = 32; // 256-bit key
pub const SALT_LEN: usize = 16;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_derive_key_is_deterministic() {
      let salt = [0u8; SALT_LEN];
      let key1 = derive_key("master123", &salt).unwrap();
      let key2 = derive_key("master123", &salt).unwrap();
      assert_eq!(*key1, *key2);
  }
}