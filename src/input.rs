//! User input handling module

use colored::*;
use std::io::{self, Write};
use std::sync::atomic::Ordering;

use crate::display::{print_goodbye, print_help, print_history, print_success, show_details};
use crate::state::{CHANGE_COUNT, SELECTING_TABLES};
use crate::types::ChangeHistory;

/// Handle user input
pub fn handle_input(input: &str, history: &ChangeHistory) {
    let t = input.trim();
    if t.is_empty() {
        return;
    }

    match t {
        "q" | "quit" | "exit" => {
            print_goodbye();
            std::process::exit(0);
        }
        "h" | "help" => print_help(),
        "l" | "list" => print_history(history),
        "c" | "clear" => clear_history(history),
        "r" | "reset" | "reselect" => {
            SELECTING_TABLES.store(true, Ordering::SeqCst);
        }
        _ => handle_unknown_input(t, history),
    }
}

/// Clear history
fn clear_history(history: &ChangeHistory) {
    history.lock().unwrap().clear();
    CHANGE_COUNT.store(0, Ordering::Relaxed);
    print_success("✓ History cleared.");
}

/// Handle unknown input
fn handle_unknown_input(input: &str, history: &ChangeHistory) {
    if let Ok(num) = input.parse::<usize>() {
        show_details(history, num);
    } else {
        eprintln!(
            "{} Unknown command '{}'. Type 'h' for help.",
            "?".yellow(),
            input
        );
    }
}

/// Interactive UI for table selection
pub async fn select_tables_interactively(
    all_tables: &[(String, String)],
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    eprintln!("\n{}", "Available tables:".cyan().bold());
    eprintln!();

    for (i, (schema, table)) in all_tables.iter().enumerate() {
        eprintln!("  {} {}.{}", format!("[{}]", i + 1).cyan(), schema, table);
    }

    eprintln!();
    eprintln!(
        "{}",
        "Select tables (e.g., 1,3,5 or 1-3 or 1,4-6), 'all', or empty to cancel:".cyan()
    );
    eprint!("{} ", "> ".cyan());
    io::stderr().flush().ok();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input.is_empty() {
        return Ok(Vec::new());
    }

    if input.to_lowercase() == "all" {
        return Ok(all_tables.to_vec());
    }

    let indices = parse_selection_input(input, all_tables.len());
    let selected: Vec<_> = indices
        .into_iter()
        .map(|idx| all_tables[idx].clone())
        .collect();

    // Remove duplicates
    let mut unique = Vec::new();
    for table in selected {
        if !unique.contains(&table) {
            unique.push(table);
        }
    }

    Ok(unique)
}

/// Parse selection input (e.g., "1,3,5" or "1-3" or "1,4-6,9")
pub fn parse_selection_input(input: &str, max: usize) -> Vec<usize> {
    let mut indices = Vec::new();

    for part in input.split(',') {
        let part = part.trim();
        if part.contains('-') {
            parse_range(part, max, &mut indices);
        } else {
            parse_single(part, max, &mut indices);
        }
    }

    indices
}

/// Parse range (e.g., "1-3")
fn parse_range(part: &str, max: usize, indices: &mut Vec<usize>) {
    let range_parts: Vec<&str> = part.split('-').collect();
    if range_parts.len() != 2 {
        return;
    }

    let start = range_parts[0].trim().parse::<usize>().ok();
    let end = range_parts[1].trim().parse::<usize>().ok();

    if let (Some(start), Some(end)) = (start, end) {
        let start = start.max(1);
        let end = end.min(max);
        if start <= end {
            for i in start..=end {
                let idx = i - 1;
                if !indices.contains(&idx) {
                    indices.push(idx);
                }
            }
        }
    }
}

/// Parse single number (e.g., "3")
fn parse_single(part: &str, max: usize, indices: &mut Vec<usize>) {
    if let Ok(num) = part.parse::<usize>() {
        if num > 0 && num <= max {
            let idx = num - 1;
            if !indices.contains(&idx) {
                indices.push(idx);
            }
        } else if num > max {
            eprintln!("{} Invalid number: {}", "⚠".yellow(), num);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_numbers() {
        assert_eq!(parse_selection_input("1", 5), vec![0]);
        assert_eq!(parse_selection_input("1,3", 5), vec![0, 2]);
        assert_eq!(parse_selection_input("1, 3, 5", 5), vec![0, 2, 4]);
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_selection_input("1-3", 5), vec![0, 1, 2]);
        assert_eq!(parse_selection_input("2-4", 5), vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_mixed() {
        assert_eq!(parse_selection_input("1,3-5", 5), vec![0, 2, 3, 4]);
        assert_eq!(parse_selection_input("1-2,4", 5), vec![0, 1, 3]);
    }

    #[test]
    fn test_parse_out_of_bounds() {
        assert_eq!(parse_selection_input("1,10", 5), vec![0]);
        assert_eq!(parse_selection_input("0,1", 5), vec![0]);
    }

    #[test]
    fn test_parse_duplicates() {
        assert_eq!(parse_selection_input("1,1,2", 5), vec![0, 1]);
    }
}
