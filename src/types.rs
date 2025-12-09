//! Data type definitions module

use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::constants::{defaults, presets};
use crate::error::{Result, TableTraceError};

/// Data type representing a single row in a table
pub type RowData = HashMap<String, String>;

/// Type for holding change history
pub type ChangeHistory = Arc<Mutex<Vec<ChangeRecord>>>;

/// Type for holding table snapshots
pub type TableSnapshots = Arc<Mutex<HashMap<String, Vec<RowData>>>>;

/// Table change event
#[derive(Debug, Clone, Serialize)]
pub struct TableChange {
    pub id: usize,
    pub timestamp: String,
    pub table: String,
    pub schema: String,
    pub change_type: String,
    pub row_count: i64,
}

/// Row-level diff information
#[derive(Debug, Clone)]
pub struct RowDiff {
    pub pk_column: String,
    pub pk_value: String,
    pub change_type: String,
    pub old_values: Option<RowData>,
    pub new_values: Option<RowData>,
    pub changed_columns: Vec<String>,
}

/// Change record (event + diff)
#[derive(Debug, Clone)]
pub struct ChangeRecord {
    pub change: TableChange,
    pub diffs: Vec<RowDiff>,
}

/// Table statistics (from pg_stat_user_tables)
#[derive(Debug, Clone)]
pub struct TableStats {
    pub n_tup_ins: i64,
    pub n_tup_upd: i64,
    pub n_tup_del: i64,
}

/// Connection configuration
#[derive(Clone)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
}

impl std::fmt::Debug for ConnectionConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectionConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("database", &self.database)
            .field("user", &self.user)
            .field("password", &"[REDACTED]")
            .finish()
    }
}

impl ConnectionConfig {
    /// Create a new connection configuration
    pub fn new(
        host: impl Into<String>,
        port: u16,
        database: impl Into<String>,
        user: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            host: host.into(),
            port,
            database: database.into(),
            user: user.into(),
            password: password.into(),
        }
    }

    /// Generate connection string
    pub fn to_connection_string(&self) -> String {
        format!(
            "host={} port={} user={} password={} dbname={}",
            self.host, self.port, self.user, self.password, self.database
        )
    }

    /// Create connection configuration from preset
    pub fn from_preset(preset: &str) -> Result<Self> {
        match preset {
            "supabase" | "supabase-local" => Ok(Self {
                host: defaults::HOST.to_string(),
                port: presets::SUPABASE_PORT,
                database: presets::SUPABASE_DATABASE.to_string(),
                user: presets::SUPABASE_USER.to_string(),
                password: presets::SUPABASE_PASSWORD.to_string(),
            }),
            "postgres" | "pg" => Ok(Self {
                host: defaults::HOST.to_string(),
                port: defaults::PORT,
                database: presets::SUPABASE_DATABASE.to_string(),
                user: defaults::USER.to_string(),
                password: presets::SUPABASE_PASSWORD.to_string(),
            }),
            _ => Err(TableTraceError::UnknownPreset(preset.to_string())),
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.host.is_empty() {
            return Err(TableTraceError::config("Host cannot be empty"));
        }
        if self.database.is_empty() {
            return Err(TableTraceError::DatabaseRequired);
        }
        if self.user.is_empty() {
            return Err(TableTraceError::config("User cannot be empty"));
        }
        Ok(())
    }
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            host: defaults::HOST.to_string(),
            port: defaults::PORT,
            database: String::new(),
            user: defaults::USER.to_string(),
            password: String::new(),
        }
    }
}

/// Watch configuration
#[derive(Debug, Clone)]
pub struct WatchConfig {
    pub connection: ConnectionConfig,
    pub schema: String,
    pub interval: u64,
    pub interactive: bool,
}

impl WatchConfig {
    /// Create a new watch configuration
    pub fn new(connection: ConnectionConfig, schema: impl Into<String>) -> Self {
        Self {
            connection,
            schema: schema.into(),
            interval: defaults::POLLING_INTERVAL_MS,
            interactive: true,
        }
    }

    /// Set polling interval
    pub fn with_interval(mut self, interval: u64) -> Self {
        self.interval = interval;
        self
    }

    /// Set interactive mode
    pub fn with_interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        self.connection.validate()?;
        if self.interval == 0 {
            return Err(TableTraceError::config(
                "Polling interval must be greater than 0",
            ));
        }
        Ok(())
    }
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            connection: ConnectionConfig::default(),
            schema: defaults::SCHEMA.to_string(),
            interval: defaults::POLLING_INTERVAL_MS,
            interactive: true,
        }
    }
}
