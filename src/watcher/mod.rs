//! Watch loop module
//!
//! Provides functionality for monitoring table changes.

mod changes;
mod handlers;
mod snapshot;
mod stats;

use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use tokio_postgres::NoTls;
use tracing::error;

use crate::constants::display::PROMPT_CLEAR_WIDTH;
use crate::db::{get_all_tables, get_table_stats, has_stats_changes};
use crate::display::{
    print_banner, print_change, print_connected, print_connecting, print_connection_error,
    print_inline_diff, print_interactive_hint, print_prompt, print_warning, print_watching_tables,
};
use crate::state::{CHANGE_COUNT, CONNECTION_LOST};
use crate::types::{ChangeHistory, TableSnapshots, WatchConfig};

use changes::{add_to_history, collect_cycle_changes, create_change_event};
use handlers::process_user_input;
use snapshot::{select_initial_tables, setup_input_channel, take_snapshots};
use stats::debounce_stats;

/// Main watch loop
pub async fn watch_tables(config: WatchConfig) -> Result<(), Box<dyn std::error::Error>> {
    print_banner();
    print_connecting();

    // Connect to database
    let conn_str = config.connection.to_connection_string();
    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await?;

    // Maintain connection in separate task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("Connection error: {}", e);
            CONNECTION_LOST.store(true, Ordering::SeqCst);
        }
    });

    print_connected();

    // Get available tables
    let all_tables = get_all_tables(&client, &config.schema).await?;
    if all_tables.is_empty() {
        print_warning("No tables found in database.");
        return Ok(());
    }

    // Select tables to watch
    let mut watch_tables = select_initial_tables(&all_tables, config.interactive).await?;
    if watch_tables.is_empty() {
        print_warning("No tables selected to watch.");
        return Ok(());
    }

    print_watching_tables(&watch_tables, "👁 Watching");

    if config.interactive {
        print_interactive_hint();
    }

    // Initialize shared state
    let history: ChangeHistory = Arc::new(Mutex::new(Vec::new()));
    let snapshots: TableSnapshots = Arc::new(Mutex::new(HashMap::new()));
    let client_arc = Arc::new(client);

    // Take initial snapshots
    take_snapshots(&client_arc, &watch_tables, &snapshots).await;

    let mut prev_stats = get_table_stats(&client_arc, &watch_tables).await?;
    let mut change_counter: usize = 0;

    // Setup input channel
    let mut rx = setup_input_channel(config.interactive);
    if config.interactive {
        print_prompt();
    }

    // Main loop
    loop {
        if config.interactive {
            process_user_input(
                &mut rx,
                &all_tables,
                &mut watch_tables,
                &snapshots,
                &history,
                &client_arc,
                &mut prev_stats,
            )
            .await;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(config.interval)).await;

        // Get statistics
        let current_stats = match get_table_stats(&client_arc, &watch_tables).await {
            Ok(stats) => stats,
            Err(e) => {
                print_connection_error(e.as_ref());
                std::process::exit(1);
            }
        };

        // Debounce
        let final_stats = if has_stats_changes(&current_stats, &prev_stats) {
            debounce_stats(&client_arc, &watch_tables, &current_stats).await
        } else {
            current_stats.clone()
        };

        // Collect and display changes
        let cycle_result =
            collect_cycle_changes(&final_stats, &prev_stats, &snapshots, &client_arc).await;

        if !cycle_result.diffs.is_empty() {
            change_counter += 1;
            CHANGE_COUNT.store(change_counter, Ordering::Relaxed);

            let change = create_change_event(
                change_counter,
                &cycle_result.tables,
                &cycle_result.change_types,
                cycle_result.total_rows,
            );

            if config.interactive {
                eprintln!("\r{}", " ".repeat(PROMPT_CLEAR_WIDTH));
            }
            print_change(&change, config.interactive);
            if config.interactive {
                print_inline_diff(&cycle_result.diffs);
                print_prompt();
            }

            add_to_history(&history, change, cycle_result.diffs);
        }

        prev_stats = final_stats;
        io::stdout().flush().ok();
    }
}
