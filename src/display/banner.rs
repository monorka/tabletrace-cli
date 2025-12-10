//! Banner and prompt display

use colored::*;
use std::io::{self, Write};
use std::sync::atomic::Ordering;
use terminal_size::{terminal_size, Width};

use crate::state::CHANGE_COUNT;

/// Large ASCII art banner (requires 90+ columns)
const BANNER_LARGE: &str = r#"
  ████████╗ █████╗ ██████╗ ██╗     ███████╗  ████████╗██████╗  █████╗  ██████╗███████╗
  ╚══██╔══╝██╔══██╗██╔══██╗██║     ██╔════╝  ╚══██╔══╝██╔══██╗██╔══██╗██╔════╝██╔════╝
     ██║   ███████║██████╔╝██║     █████╗       ██║   ██████╔╝███████║██║     █████╗
     ██║   ██╔══██║██╔══██╗██║     ██╔══╝       ██║   ██╔══██╗██╔══██║██║     ██╔══╝
     ██║   ██║  ██║██████╔╝███████╗███████╗     ██║   ██║  ██║██║  ██║╚██████╗███████╗
     ╚═╝   ╚═╝  ╚═╝╚═════╝ ╚══════╝╚══════╝     ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚══════╝
"#;

/// Medium ASCII art banner (requires 60+ columns)
const BANNER_MEDIUM: &str = r#"
  ╔════════════════════════════════════════════════════╗
  ║     ▀█▀ █▀█ █▄▄ █   █▀▀   ▀█▀ █▀█ █▀█ █▀▀ █▀▀     ║
  ║      █  █▀█ █▄█ █▄▄ ██▄    █  █▀▄ █▀█ █▄▄ ██▄     ║
  ╚════════════════════════════════════════════════════╝
"#;

/// Small banner for narrow terminals
const BANNER_SMALL: &str = r#"
╭──────────────────────────╮
│   Table Trace CLI        │
╰──────────────────────────╯
"#;

/// Get terminal width
fn get_terminal_width() -> u16 {
    terminal_size()
        .map(|(Width(w), _)| w)
        .unwrap_or(80)
}

/// Display banner
pub fn print_banner() {
    let width = get_terminal_width();

    if width >= 90 {
        // Large ASCII art
        for line in BANNER_LARGE.lines() {
            eprintln!("{}", line.cyan());
        }
        eprintln!(
            "{}  {}",
            "                     ".dimmed(),
            "Real-time PostgreSQL Monitor".dimmed()
        );
    } else if width >= 60 {
        // Medium banner
        for line in BANNER_MEDIUM.lines() {
            eprintln!("{}", line.cyan());
        }
    } else {
        // Small banner
        for line in BANNER_SMALL.lines() {
            eprintln!("{}", line.cyan());
        }
    }
    eprintln!();
}

/// Display prompt
pub fn print_prompt() {
    let count = CHANGE_COUNT.load(Ordering::Relaxed);
    if count > 0 {
        eprint!(
            "\r{} {} ",
            format!("[{} changes]", count).cyan(),
            "(l=list, w=watching, r=reset, q=quit) >".dimmed()
        );
    } else {
        eprint!(
            "\r{} ",
            "Waiting for changes... (h=help, r=reset, q=quit) >".dimmed()
        );
    }
    io::stderr().flush().ok();
}

/// Display interactive mode command hints
pub fn print_interactive_hint() {
    eprintln!(
        "\n{}",
        "┌─────────────────────────────────────────────────────────┐".dimmed()
    );
    eprintln!(
        "{}",
        "│  Commands: [number] details | h help | l list | q quit  │".dimmed()
    );
    eprintln!(
        "{}",
        "└─────────────────────────────────────────────────────────┘".dimmed()
    );
    eprintln!();
}

/// Display help
pub fn print_help() {
    eprintln!();
    eprintln!("{}", "╭─────────────────────────────────────────╮".cyan());
    eprintln!("{}", "│           Available Commands            │".cyan());
    eprintln!("{}", "├─────────────────────────────────────────┤".cyan());
    eprintln!(
        "│  {}       Show change details        │",
        "1, 2, ...".yellow()
    );
    eprintln!("│  {}            List all changes         │", "l".yellow());
    eprintln!("│  {}            Clear history            │", "c".yellow());
    eprintln!("│  {}            Show watching tables     │", "w".yellow());
    eprintln!("│  {}            Reset table selection    │", "r".yellow());
    eprintln!("│  {}            Show this help           │", "h".yellow());
    eprintln!("│  {}            Quit                     │", "q".yellow());
    eprintln!("{}", "╰─────────────────────────────────────────╯".cyan());
    eprintln!();
}
