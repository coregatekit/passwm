use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "passwm", version, about, long_about = None)]
pub struct Cli {
    /// Path to vault file (default: ~/.passwm/vault.pwm)
    #[arg(long, global = true, default_value = "vault.pwm")]
    pub vault_path: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a new password entry
    Add {
        #[arg(short, long)]
        service: String,
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },

    /// List all stored entries (service and username only)
    List,

    /// Get the password for a specific service
    Get {
        #[arg(short, long)]
        service: String,
    },

    /// Update username or password for an existing entry
    Update {
        #[arg(short, long)]
        service: String,
        #[arg(short, long)]
        username: Option<String>,
        #[arg(short, long)]
        password: Option<String>,
    },

    /// Delete an entry for a specific service
    Delete {
        #[arg(short, long)]
        service: String,
    },

    /// Search entries by service name or username
    Search {
        #[arg(short, long)]
        query: String,
    },
}
