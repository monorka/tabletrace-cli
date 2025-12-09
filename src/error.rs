//! Error type definitions module

use thiserror::Error;

/// TableTrace error type
#[derive(Error, Debug)]
pub enum TableTraceError {
    /// Database connection error
    #[error("Database connection failed: {0}")]
    ConnectionFailed(#[from] tokio_postgres::Error),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Unknown preset
    #[error("Unknown preset '{0}'. Available: supabase, postgres")]
    UnknownPreset(String),

    /// Database name not specified
    #[error("Database name is required. Use --database or --preset")]
    DatabaseRequired,

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, TableTraceError>;

impl TableTraceError {
    /// Create a configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::ConfigError(msg.into())
    }
}
