//! Banner and prompt display

use colored::*;
use std::io::{self, Write};
use std::sync::atomic::Ordering;

use crate::state::CHANGE_COUNT;

/// Display banner
pub fn print_banner() {
    eprintln!(
        "\n{}",
        "╔══════════════════════════════════════════════════════════╗".cyan()
    );
    eprintln!(
        "{}",
        "║           TableTrace - Real-time DB Monitor              ║".cyan()
    );
    eprintln!(
        "{}",
        "╚══════════════════════════════════════════════════════════╝".cyan()
    );
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
