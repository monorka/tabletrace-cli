//! Snapshot management

use tokio::io::{AsyncBufReadExt, BufReader};

use crate::constants::INPUT_CHANNEL_BUFFER;
use crate::db::fetch_all_rows;
use crate::display::print_warning;
use crate::input::select_tables_interactively;
use crate::types::TableSnapshots;

/// Initial table selection
pub async fn select_initial_tables(
    all_tables: &[(String, String)],
    interactive: bool,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    if interactive {
        let selected = select_tables_interactively(all_tables).await?;
        if selected.is_empty() {
            print_warning("No tables selected. Exiting.");
            return Ok(Vec::new());
        }
        Ok(selected)
    } else {
        Ok(all_tables.to_vec())
    }
}

/// Take snapshots
pub async fn take_snapshots(
    client: &tokio_postgres::Client,
    tables: &[(String, String)],
    snapshots: &TableSnapshots,
) {
    for (schema, table) in tables {
        let key = format!("{}.{}", schema, table);
        if let Ok(rows) = fetch_all_rows(client, schema, table).await {
            snapshots.lock().unwrap().insert(key, rows);
        }
    }
}

/// Setup input channel
pub fn setup_input_channel(interactive: bool) -> tokio::sync::mpsc::Receiver<String> {
    let (tx, rx) = tokio::sync::mpsc::channel::<String>(INPUT_CHANNEL_BUFFER);

    if interactive {
        tokio::spawn(async move {
            let stdin = BufReader::new(tokio::io::stdin());
            let mut lines = stdin.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if tx.send(line).await.is_err() {
                    break;
                }
            }
        });
    }

    rx
}
