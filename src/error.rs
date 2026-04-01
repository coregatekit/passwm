use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswmError {
    #[error("Encryption failed: {0}")]
    EncryptionError(String),
    #[error("Decryption failed: invalid key or corrupted data")]
    DecryptionError,
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Entry not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, PasswmError>;
