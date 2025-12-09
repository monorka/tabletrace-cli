//! Statistics processing

use std::collections::HashMap;

use crate::constants::db::{DEBOUNCE_INTERVAL_MS, DEBOUNCE_MAX_ITERATIONS};
use crate::db::{get_table_stats, has_stats_changes};
use crate::types::TableStats;

/// Debounce statistics
///
/// When changes are detected, wait until stable before returning final statistics
pub async fn debounce_stats(
    client: &tokio_postgres::Client,
    tables: &[(String, String)],
    current_stats: &HashMap<(String, String), TableStats>,
) -> HashMap<(String, String), TableStats> {
    let mut last_stats = current_stats.clone();
    let mut final_stats = current_stats.clone();

    for _ in 0..DEBOUNCE_MAX_ITERATIONS {
        tokio::time::sleep(tokio::time::Duration::from_millis(DEBOUNCE_INTERVAL_MS)).await;
        match get_table_stats(client, tables).await {
            Ok(new_stats) => {
                if !has_stats_changes(&new_stats, &last_stats) {
                    final_stats = new_stats;
                    break;
                }
                last_stats = new_stats.clone();
                final_stats = new_stats;
            }
            Err(_) => break,
        }
    }

    final_stats
}

/// Detect changes
pub fn detect_changes(stats: &TableStats, prev: &TableStats) -> Vec<(&'static str, i64)> {
    let mut detected = Vec::new();
    if stats.n_tup_ins > prev.n_tup_ins {
        detected.push(("INSERT", stats.n_tup_ins - prev.n_tup_ins));
    }
    if stats.n_tup_upd > prev.n_tup_upd {
        detected.push(("UPDATE", stats.n_tup_upd - prev.n_tup_upd));
    }
    if stats.n_tup_del > prev.n_tup_del {
        detected.push(("DELETE", stats.n_tup_del - prev.n_tup_del));
    }
    detected
}
