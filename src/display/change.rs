//! Change event display

use colored::*;

use crate::types::TableChange;

/// Display change event
pub fn print_change(change: &TableChange, interactive: bool) {
    let (icon, ct) = get_change_icon_and_color(change);

    let table_display = if change.schema.is_empty() {
        change.table.clone()
    } else {
        format!("{}.{}", change.schema, change.table)
    };

    let row_suffix = if change.row_count > 1 { "s" } else { "" };

    if interactive {
        eprintln!(
            "{} {} [{}] {} {} ({} row{})",
            icon,
            format!("#{}", change.id).cyan().bold(),
            change.timestamp.dimmed(),
            ct,
            table_display,
            change.row_count,
            row_suffix
        );
    } else {
        eprintln!(
            "{} [{}] {} {} ({} row{})",
            icon,
            change.timestamp.dimmed(),
            ct,
            table_display,
            change.row_count,
            row_suffix
        );
    }
}

/// Get icon and color for change type
fn get_change_icon_and_color(change: &TableChange) -> (ColoredString, ColoredString) {
    if change.change_type.contains('+') {
        ("±".magenta(), change.change_type.magenta())
    } else {
        match change.change_type.as_str() {
            "INSERT" => ("+".green(), change.change_type.green()),
            "UPDATE" => ("~".yellow(), change.change_type.yellow()),
            "DELETE" => ("-".red(), change.change_type.red()),
            _ => ("•".normal(), change.change_type.normal()),
        }
    }
}
