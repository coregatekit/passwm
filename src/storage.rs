use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct VaultFile {
    salt: String,
    ciphertext: String,
}
