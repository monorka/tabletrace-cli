//! CLI parser module

use clap::{Parser, Subcommand};
use std::env;

use crate::constants::defaults;
use crate::error::{Result, TableTraceError};
use crate::types::{ConnectionConfig, WatchConfig};

/// Environment variable name for PostgreSQL password
const PGPASSWORD_ENV: &str = "PGPASSWORD";

#[derive(Parser)]
#[command(name = "tabletrace")]
#[command(author = "Monorka Inc.")]
#[command(version = "0.1.0")]
#[command(about = "Real-time PostgreSQL change monitoring", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Watch PostgreSQL tables for changes in real-time
    Watch {
        /// Preset connection: 'supabase' (local Docker), 'postgres' (local default)
        #[arg(long)]
        preset: Option<String>,
        /// Database host
        #[arg(short = 'H', long, default_value = defaults::HOST)]
        host: String,
        /// Database port
        #[arg(short = 'P', long, default_value_t = defaults::PORT)]
        port: u16,
        /// Database name
        #[arg(short, long)]
        database: Option<String>,
        /// Database user
        #[arg(short, long, default_value = defaults::USER)]
        user: String,
        /// Database password (or use PGPASSWORD environment variable)
        #[arg(short = 'W', long)]
        password: Option<String>,
        /// Schema to filter tables (use 'all' for all schemas)
        #[arg(short, long, default_value = defaults::SCHEMA)]
        schema: String,
        /// Polling interval in milliseconds
        #[arg(short, long, default_value_t = defaults::POLLING_INTERVAL_MS)]
        interval: u64,
        /// Enable interactive mode (keyboard input for details) [default: true]
        #[arg(long, default_value = "true")]
        interactive: bool,
    },
}

impl Commands {
    /// Generate WatchConfig from Watch command
    pub fn into_watch_config(self) -> Result<WatchConfig> {
        match self {
            Commands::Watch {
                preset,
                host,
                port,
                database,
                user,
                password,
                schema,
                interval,
                interactive,
            } => {
                let connection = match preset {
                    Some(p) => ConnectionConfig::from_preset(&p)?,
                    None => {
                        let db = database.ok_or(TableTraceError::DatabaseRequired)?;
                        let pass = resolve_password(password);
                        ConnectionConfig::new(host, port, db, user, pass)
                    }
                };

                let config = WatchConfig::new(connection, schema)
                    .with_interval(interval)
                    .with_interactive(interactive);

                config.validate()?;
                Ok(config)
            }
        }
    }
}

/// Resolve password from argument or environment variable
fn resolve_password(password: Option<String>) -> String {
    password
        .or_else(|| env::var(PGPASSWORD_ENV).ok())
        .unwrap_or_default()
}
