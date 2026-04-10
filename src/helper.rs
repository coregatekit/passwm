use std::io::Write;

use crate::error::{PasswmError, Result};

/// stdin helper functions
pub fn prompt_password(prompt: &str) -> Result<String> {
    print!("{prompt}");
    std::io::stdout()
        .flush()
        .map_err(|e| PasswmError::StorageError(e.to_string()))?;
    rpassword::read_password().map_err(|e| PasswmError::StorageError(e.to_string()))
}
