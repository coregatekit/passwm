use std::{io::Write, path::PathBuf};

use crate::error::{PasswmError, Result};

/// stdin helper functions
pub fn prompt_password(prompt: &str) -> Result<String> {
    print!("{prompt}");
    std::io::stdout()
        .flush()
        .map_err(|e| PasswmError::StorageError(e.to_string()))?;
    rpassword::read_password().map_err(|e| PasswmError::StorageError(e.to_string()))
}

/// Resolve vault path
/// - if user provides --vault-path option, use that
/// - otherwise, use default path ~/.passwm/vault.pwm
/// - create parent directory if it doesn't exist
pub fn resolve_vault_path(vault_path: Option<String>) -> Result<PathBuf> {
    let path = match vault_path {
        Some(p) => PathBuf::from(p),
        None => "".into(),
    };

    Ok(path)
}

/// Get default vault path: ~/.passwm/vault.pwm
pub fn default_vault_path() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| PasswmError::StorageError("Cannot find home directory".to_string()))?;
    Ok(home.join(".passwm").join("vault.pwm"))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    /// --- resolve_vault_path -----------------------------

    #[test]
    fn test_resolve_vault_path_uses_provided_path() {
        let dir = tempfile::tempdir().unwrap();
        let custom = dir.path().join("custom.pwm").to_str().unwrap().to_string();

        let resolved = resolve_vault_path(Some(custom.clone())).unwrap();

        assert_eq!(resolved, PathBuf::from(custom));
    }

    #[test]
    fn test_resolve_vault_path_defaults_to_home() {
        // if no path provided, should return with .passwm/vault.pwm under home
        let resolved = resolve_vault_path(None).unwrap();

        assert!(resolved.ends_with(".passwm/vault.pwm"));
    }

    /// --- default_vault_path -----------------------------

    #[test]
    fn test_default_vault_path_is_under_home() {
        let path = default_vault_path().unwrap();
        let home = dirs::home_dir().unwrap();

        assert!(path.starts_with(&home));
        assert_eq!(path.file_name().unwrap(), "vault.pwm");
    }

    #[test]
    fn test_default_vault_path_parent_is_dot_passwm() {
        let path = default_vault_path().unwrap();
        let parent = path.parent().unwrap();

        assert_eq!(parent.file_name().unwrap(), ".passwm");
    }
}
