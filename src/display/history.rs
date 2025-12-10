//! History display

use colored::*;

use super::colorize_change_type;
use super::diff::print_detail_diffs;
use crate::types::{ChangeHistory, ChangeRecord};

/// Display watching tables list
pub fn print_watching_tables(tables: &[(String, String)], prefix: &str) {
    let suffix = if tables.len() == 1 { "" } else { "s" };
    eprintln!(
        "\n{} ({} table{})",
        prefix.cyan().bold(),
        tables.len(),
        suffix
    );
    for (i, (schema, table)) in tables.iter().enumerate() {
        eprintln!("  {} {}.{}", format!("[{}]", i + 1).cyan(), schema, table);
    }
    eprintln!();
}

/// Display change history
pub fn print_history(history: &ChangeHistory) {
    let h = history.lock().unwrap();
    if h.is_empty() {
        eprintln!(
            "\n{}",
            "No changes recorded yet. Make some changes to your database!".dimmed()
        );
        return;
    }

    eprintln!("\n{}", "═══ Change History ═══".cyan().bold());
    for r in h.iter() {
        print_history_item(r);
    }
    eprintln!("\n{}", "Type a number to see details (e.g., '1')".dimmed());
}

/// Display single history item (also used for real-time change notification)
pub fn print_change_line(record: &ChangeRecord, indent: &str) {
    let c = &record.change;
    let ct = colorize_change_type(&c.change_type);
    let diff_hint = if !record.diffs.is_empty() {
        format!(" [{} row diff]", record.diffs.len())
            .dimmed()
            .to_string()
    } else {
        String::new()
    };

    let row_suffix = if c.row_count > 1 { "s" } else { "" };

    let table_display = if c.schema.is_empty() {
        c.table.clone()
    } else {
        format!("{}.{}", c.schema, c.table)
    };

    eprintln!(
        "{}{} [{}] {} {} ({} row{}){}",
        indent,
        format!("#{}", c.id).cyan().bold(),
        c.timestamp.dimmed(),
        ct,
        table_display,
        c.row_count,
        row_suffix,
        diff_hint
    );
}

/// Display single history item (for history list)
fn print_history_item(record: &ChangeRecord) {
    print_change_line(record, "  ");
}

/// Show details
pub fn show_details(history: &ChangeHistory, id: usize) {
    let record = history
        .lock()
        .unwrap()
        .iter()
        .find(|r| r.change.id == id)
        .cloned();

    match record {
        Some(r) => print_detail_view(&r),
        None => {
            eprintln!(
                "{} Change #{} not found. Type 'l' to list all changes.",
                "✗".red(),
                id
            );
        }
    }
}

/// Display detail view
fn print_detail_view(record: &ChangeRecord) {
    let c = &record.change;
    let ct = match c.change_type.as_str() {
        "INSERT" => c.change_type.green().bold(),
        "UPDATE" => c.change_type.yellow().bold(),
        "DELETE" => c.change_type.red().bold(),
        _ => c.change_type.normal(),
    };

    eprintln!();
    eprintln!(
        "{}",
        "╔══════════════════════════════════════════════════════════╗".cyan()
    );
    eprintln!(
        "║  {} #{}: {} on {}.{}",
        "Change".cyan().bold(),
        c.id.to_string().cyan(),
        ct,
        c.schema,
        c.table
    );
    eprintln!(
        "║  {}: {}   {}: {} row(s)",
        "Time".dimmed(),
        c.timestamp,
        "Affected".dimmed(),
        c.row_count
    );
    eprintln!(
        "{}",
        "╠══════════════════════════════════════════════════════════╣".cyan()
    );

    if record.diffs.is_empty() {
        eprintln!("║  {}", "No detailed diff available.".dimmed());
    } else {
        print_detail_diffs(&record.diffs);
    }

    eprintln!(
        "{}",
        "╚══════════════════════════════════════════════════════════╝".cyan()
    );
    eprintln!();
}
