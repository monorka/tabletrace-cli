//! Message display

use colored::*;

/// Display connecting message
pub fn print_connecting() {
    eprintln!("\n{}", "Connecting to PostgreSQL...".dimmed());
}

/// Display connected message
pub fn print_connected() {
    eprintln!("{} {}\n", "✓".green(), "Connected!".green());
}

/// Display table selection prompt
pub fn print_table_selection_prompt(all_tables: &[(String, String)]) {
    eprintln!("\n{}", "Available tables:".cyan().bold());
    for (i, (schema, table)) in all_tables.iter().enumerate() {
        eprintln!("  {} {}.{}", format!("[{}]", i + 1).cyan(), schema, table);
    }
    eprintln!();
    eprintln!(
        "{}",
        "Enter numbers (e.g., 1,3,5 or 1-3 or 1,4-6), 'all', or empty to cancel:".cyan()
    );
}

/// Display connection error
pub fn print_connection_error(error: &dyn std::error::Error) {
    eprintln!(
        "\n{} {} {}",
        "✗".red().bold(),
        "Connection error:".red(),
        error
    );
    eprintln!("{}", "Exiting...".red());
}

/// Display goodbye message
pub fn print_goodbye() {
    eprintln!("\n{}", "Goodbye! 👋".cyan());
}

/// Display warning message
pub fn print_warning(message: &str) {
    eprintln!("{}", message.yellow());
}

/// Display success message
pub fn print_success(message: &str) {
    eprintln!("{}", message.dimmed());
}
