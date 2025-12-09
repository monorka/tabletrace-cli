//! TableTrace CLI - Real-time PostgreSQL change monitoring
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                         main.rs                             │
//! │                      (Entry Point)                          │
//! └─────────────────────────────────────────────────────────────┘
//!                              │
//!                              ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │                         cli.rs                              │
//! │                  (CLI Parser & Config)                      │
//! └─────────────────────────────────────────────────────────────┘
//!                              │
//!                              ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │                       watcher.rs                            │
//! │                     (Watch Loop)                            │
//! └─────────────────────────────────────────────────────────────┘
//!           │              │              │
//!           ▼              ▼              ▼
//! ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
//! │   db.rs     │  │  diff.rs    │  │  display/   │
//! │ (DB Ops)    │  │ (Diff Calc) │  │  (Display)  │
//! └─────────────┘  └─────────────┘  └─────────────┘
//! ```
//!
//! # Module Structure
//!
//! - `cli`: Command line argument parsing
//! - `constants`: Constant definitions
//! - `db`: Database operations
//! - `diff`: Diff calculation
//! - `display`: Display handling (submodule)
//! - `error`: Error type definitions
//! - `input`: User input handling
//! - `state`: Global state management
//! - `types`: Data type definitions
//! - `watcher`: Watch loop

mod cli;
mod constants;
mod db;
mod diff;
mod display;
mod error;
mod input;
mod state;
mod types;
mod watcher;

use clap::Parser;

use cli::Cli;
use watcher::watch_tables;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let cli = Cli::parse();

    let config = match cli.command.into_watch_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    watch_tables(config).await?;

    Ok(())
}

/// Initialize logging configuration
fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::WARN.into()),
        )
        .with_target(false)
        .init();
}
