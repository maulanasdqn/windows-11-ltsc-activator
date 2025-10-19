use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub kms_server: String,
    pub product_key: String,
    pub dry_run: bool,
    pub verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            kms_server: "kms.digiboy.ir".to_string(),
            product_key: "M7XTQ-FN8P6-TTKYV-9D4CC-J462D".to_string(),
            dry_run: false,
            verbose: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LicenseFile {
    pub name: String,
    pub data: &'static [u8],
    pub relative_path: PathBuf,
}

#[derive(Error, Debug)]
pub enum ActivationError {
    #[error("Insufficient privileges - must run as administrator")]
    InsufficientPrivileges,

    #[error("Failed to create directory: {0}")]
    DirectoryCreation(String),

    #[error("Failed to write file: {0}")]
    FileWrite(String),

    #[error("Command execution failed: {0}")]
    CommandExecution(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("License file not found: {0}")]
    LicenseNotFound(String),

    #[error("Activation failed: {0}")]
    ActivationFailed(String),
}

pub type Result<T> = std::result::Result<T, ActivationError>;
