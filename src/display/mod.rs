//! Display module
//!
//! Provides UI display functionality.

mod banner;
mod change;
mod diff;
mod history;
mod messages;

// Public API
pub use banner::{print_banner, print_help, print_interactive_hint, print_prompt};
pub use change::print_change;
pub use diff::print_inline_diff;
pub use history::{print_history, print_watching_tables, show_details};
pub use messages::{
    print_connected, print_connecting, print_connection_error, print_goodbye, print_success,
    print_table_selection_prompt, print_warning,
};

use colored::ColoredString;
use colored::*;

use crate::types::RowDiff;

/// Colorize change type
pub(crate) fn colorize_change_type(change_type: &str) -> ColoredString {
    match change_type {
        "INSERT" => change_type.green(),
        "UPDATE" => change_type.yellow(),
        "DELETE" => change_type.red(),
        _ => change_type.normal(),
    }
}

/// Extract table name and column name from pk_column
pub(crate) fn extract_table_and_column(pk_column: &str) -> (String, String) {
    let parts: Vec<&str> = pk_column.rsplitn(2, '.').collect();
    if parts.len() == 2 {
        (parts[1].to_string(), parts[0].to_string())
    } else {
        (String::new(), pk_column.to_string())
    }
}

/// Get symbol for change type
pub(crate) fn get_change_symbol(change_type: &str) -> ColoredString {
    match change_type {
        "added" => "+".green().bold(),
        "removed" => "-".red().bold(),
        "modified" => "~".yellow().bold(),
        _ => " ".normal(),
    }
}

/// Format row diff values
pub(crate) fn format_diff_values(diff: &RowDiff, pk_col: &str) -> Vec<String> {
    match diff.change_type.as_str() {
        "added" => diff
            .new_values
            .as_ref()
            .map(|nv| {
                nv.iter()
                    .filter(|(k, _)| *k != pk_col)
                    .map(|(k, v)| format!("{}={}", k.dimmed(), v.green()))
                    .collect()
            })
            .unwrap_or_default(),
        "removed" => diff
            .old_values
            .as_ref()
            .map(|ov| {
                ov.iter()
                    .filter(|(k, _)| *k != pk_col)
                    .map(|(k, v)| format!("{}={}", k.dimmed(), v.red()))
                    .collect()
            })
            .unwrap_or_default(),
        "modified" => diff
            .changed_columns
            .iter()
            .map(|col| {
                let ov = diff
                    .old_values
                    .as_ref()
                    .and_then(|v| v.get(col))
                    .map(|s| s.as_str())
                    .unwrap_or("?");
                let nv = diff
                    .new_values
                    .as_ref()
                    .and_then(|v| v.get(col))
                    .map(|s| s.as_str())
                    .unwrap_or("?");
                format!("{}: {} → {}", col, ov.white(), nv.yellow())
            })
            .collect(),
        _ => vec![],
    }
}
