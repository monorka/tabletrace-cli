//! User input handlers

use std::collections::HashMap;
use std::sync::atomic::Ordering;

use crate::db::get_table_stats;
use crate::display::{
    print_prompt, print_table_selection_prompt, print_warning, print_watching_tables,
};
use crate::input::{handle_input, parse_selection_input};
use crate::state::{CHANGE_COUNT, SELECTING_TABLES};
use crate::types::{ChangeHistory, TableSnapshots, TableStats};

use super::snapshot::take_snapshots;

/// Process user input
pub async fn process_user_input(
    rx: &mut tokio::sync::mpsc::Receiver<String>,
    all_tables: &[(String, String)],
    watch_tables: &mut Vec<(String, String)>,
    snapshots: &TableSnapshots,
    history: &ChangeHistory,
    client: &tokio_postgres::Client,
    prev_stats: &mut HashMap<(String, String), TableStats>,
) {
    while let Ok(input) = rx.try_recv() {
        eprintln!();

        if SELECTING_TABLES.load(Ordering::SeqCst) {
            if let Some(tables) =
                handle_table_selection(&input, all_tables, snapshots, history, client, prev_stats)
                    .await
            {
                *watch_tables = tables;
                print_watching_tables(watch_tables, "✓ Now watching");
            }
            print_prompt();
            continue;
        }

        let trimmed = input.trim();
        if trimmed == "w" || trimmed == "watching" {
            print_watching_tables(watch_tables, "👁 Watching");
            print_prompt();
            continue;
        }

        handle_input(&input, history);

        if SELECTING_TABLES.load(Ordering::SeqCst) {
            print_table_selection_prompt(all_tables);
        }

        print_prompt();
    }
}

/// Handle table selection
async fn handle_table_selection(
    input: &str,
    all_tables: &[(String, String)],
    snapshots: &TableSnapshots,
    history: &ChangeHistory,
    client: &tokio_postgres::Client,
    prev_stats: &mut HashMap<(String, String), TableStats>,
) -> Option<Vec<(String, String)>> {
    SELECTING_TABLES.store(false, Ordering::SeqCst);

    let input = input.trim();
    if input.is_empty() {
        print_warning("Selection cancelled. Continuing with current tables.");
        return None;
    }

    let new_watch_tables = if input.to_lowercase() == "all" {
        all_tables.to_vec()
    } else {
        let indices = parse_selection_input(input, all_tables.len());
        indices.iter().map(|&i| all_tables[i].clone()).collect()
    };

    if new_watch_tables.is_empty() {
        print_warning("No valid tables selected. Continuing with current tables.");
        return None;
    }

    // Clear state
    snapshots.lock().unwrap().clear();
    history.lock().unwrap().clear();
    CHANGE_COUNT.store(0, Ordering::Relaxed);

    // Take new snapshots
    take_snapshots(client, &new_watch_tables, snapshots).await;

    // Update statistics
    if let Ok(new_stats) = get_table_stats(client, &new_watch_tables).await {
        *prev_stats = new_stats;
    }

    Some(new_watch_tables)
}
