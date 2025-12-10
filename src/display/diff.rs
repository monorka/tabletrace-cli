//! Diff display

use colored::*;

use super::{extract_table_and_column, format_diff_values, get_change_symbol};
use crate::constants::display::MAX_INLINE_DIFF_ROWS;
use crate::types::RowDiff;

/// Display inline diff
#[allow(dead_code)]
pub fn print_inline_diff(diffs: &[RowDiff]) {
    let mut current_table = String::new();

    for (shown, diff) in diffs.iter().enumerate() {
        if shown >= MAX_INLINE_DIFF_ROWS {
            break;
        }

        let (table_name, pk_col) = extract_table_and_column(&diff.pk_column);

        // Show separator when table changes
        if !table_name.is_empty() && table_name != current_table {
            if !current_table.is_empty() {
                eprintln!();
            }
            eprintln!("  {}", format!("── {} ──", table_name).dimmed());
            current_table = table_name;
        }

        print_diff_line(diff, &pk_col);
    }

    if diffs.len() > MAX_INLINE_DIFF_ROWS {
        eprintln!(
            "    {}",
            format!("...and {} more rows", diffs.len() - MAX_INLINE_DIFF_ROWS).dimmed()
        );
    }
}

/// Display single diff line
#[allow(dead_code)]
fn print_diff_line(diff: &RowDiff, pk_col: &str) {
    let symbol = get_change_symbol(&diff.change_type);
    let values = format_diff_values(diff, pk_col);

    if !values.is_empty() {
        eprintln!(
            "    {} {} {{ {} }}",
            symbol,
            format!("{}={}", pk_col, diff.pk_value).cyan(),
            values.join(", ")
        );
    }
}

/// Display detail view diffs
pub fn print_detail_diffs(diffs: &[RowDiff]) {
    let mut current_table = String::new();

    for d in diffs {
        let (table_name, pk_col) = extract_table_and_column(&d.pk_column);

        // Show separator when table changes
        if !table_name.is_empty() && table_name != current_table {
            if !current_table.is_empty() {
                eprintln!("║");
                eprintln!(
                    "{}",
                    "╠───────────────────────────────────────────────────────────╣".dimmed()
                );
            }
            eprintln!("║  {}", format!("📋 {}", table_name).cyan().bold());
            eprintln!(
                "{}",
                "╠───────────────────────────────────────────────────────────╣".dimmed()
            );
            current_table = table_name;
        }

        let sym = get_change_symbol(&d.change_type);

        eprintln!("║");
        eprintln!(
            "║  {} {} = {}",
            sym,
            pk_col.cyan(),
            d.pk_value.cyan().bold()
        );

        print_detail_diff_values(d, &pk_col);
    }
}

/// Display detail view values
fn print_detail_diff_values(diff: &RowDiff, pk_col: &str) {
    match diff.change_type.as_str() {
        "added" => {
            if let Some(nv) = &diff.new_values {
                for (k, v) in nv {
                    if k != pk_col {
                        eprintln!("║      {}: {}", k.dimmed(), v.green());
                    }
                }
            }
        }
        "removed" => {
            if let Some(ov) = &diff.old_values {
                for (k, v) in ov {
                    if k != pk_col {
                        eprintln!("║      {}: {}", k.dimmed(), v.red().strikethrough());
                    }
                }
            }
        }
        "modified" => {
            for col in &diff.changed_columns {
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
                eprintln!("║      {}: {} → {}", col, ov.white(), nv.yellow());
            }
        }
        _ => {}
    }
}
