//! Change collection and event creation

use chrono::Local;
use std::collections::{HashMap, HashSet};

use crate::constants::display::MAX_HISTORY_SIZE;
use crate::db::{fetch_all_rows, get_primary_key};
use crate::diff::calculate_all_diffs;
use crate::types::{ChangeHistory, ChangeRecord, RowDiff, TableChange, TableSnapshots, TableStats};

use super::stats::detect_changes;

/// Cycle change result
pub struct CycleResult {
    pub diffs: Vec<RowDiff>,
    pub tables: Vec<String>,
    pub change_types: HashSet<String>,
    pub total_rows: i64,
}

/// Collect changes during cycle
pub async fn collect_cycle_changes(
    final_stats: &HashMap<(String, String), TableStats>,
    prev_stats: &HashMap<(String, String), TableStats>,
    snapshots: &TableSnapshots,
    client: &tokio_postgres::Client,
) -> CycleResult {
    let mut result = CycleResult {
        diffs: Vec::new(),
        tables: Vec::new(),
        change_types: HashSet::new(),
        total_rows: 0,
    };

    for ((schema, table), stats) in final_stats {
        let key = (schema.clone(), table.clone());
        let full_key = format!("{}.{}", schema, table);

        if let Some(prev) = prev_stats.get(&key) {
            let detected = detect_changes(stats, prev);

            if !detected.is_empty() {
                let diffs =
                    calculate_table_diffs(client, schema, table, &full_key, snapshots).await;

                result.diffs.extend(diffs);

                for (change_type, count) in &detected {
                    result.total_rows += count;
                    result.change_types.insert(change_type.to_string());
                }

                if !result.tables.contains(&full_key) {
                    result.tables.push(full_key);
                }
            }
        }
    }

    result
}

/// Calculate table diffs
async fn calculate_table_diffs(
    client: &tokio_postgres::Client,
    schema: &str,
    table: &str,
    full_key: &str,
    snapshots: &TableSnapshots,
) -> Vec<RowDiff> {
    let new_rows = fetch_all_rows(client, schema, table)
        .await
        .unwrap_or_default();
    let old_rows = snapshots
        .lock()
        .unwrap()
        .get(full_key)
        .cloned()
        .unwrap_or_default();
    let pk_col = get_primary_key(client, schema, table)
        .await
        .unwrap_or_else(|| "id".to_string());

    let mut diffs = calculate_all_diffs(&old_rows, &new_rows, &pk_col);

    // Tag with table name
    for diff in &mut diffs {
        diff.pk_column = format!("{}.{}", full_key, diff.pk_column);
    }

    // Update snapshot
    snapshots
        .lock()
        .unwrap()
        .insert(full_key.to_string(), new_rows);

    diffs
}

/// Create change event
pub fn create_change_event(
    id: usize,
    tables: &[String],
    change_types: &HashSet<String>,
    total_rows: i64,
) -> TableChange {
    let mut types: Vec<String> = change_types.iter().cloned().collect();
    types.sort();
    let change_type_str = types.join("+");

    let table_str = if tables.len() == 1 {
        tables[0].clone()
    } else {
        format!("{} tables", tables.len())
    };

    TableChange {
        id,
        timestamp: Local::now().format("%H:%M:%S").to_string(),
        schema: String::new(),
        table: table_str,
        change_type: change_type_str,
        row_count: total_rows,
    }
}

/// Add to history
pub fn add_to_history(history: &ChangeHistory, change: TableChange, diffs: Vec<RowDiff>) {
    let mut h = history.lock().unwrap();
    h.push(ChangeRecord { change, diffs });
    if h.len() > MAX_HISTORY_SIZE {
        h.remove(0);
    }
}
