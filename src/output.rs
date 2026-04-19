//! Color and formatting utilities for CLI output.

use colored::*;

/// Print a success message with green color
pub fn success(msg: &str) {
    println!("{}", msg.green().bold());
}

/// Print an error message with red color
pub fn error(msg: &str) {
    eprintln!("{}", format!("[ERROR] {}", msg).red().bold());
}

/// Print a warning message with yellow color
pub fn warning(msg: &str) {
    println!("{}", format!("[WARNING] {}", msg).yellow().bold());
}

/// Print an info message with blue color
pub fn info(msg: &str) {
    println!("{}", msg.blue().bold());
}

/// Print a highlight message with cyan color
pub fn highlight(msg: &str) {
    println!("{}", msg.cyan());
}

/// Print a header with bright colors
pub fn header(msg: &str) {
    println!("\n{}", msg.bold());
    println!("{}", "─".repeat(msg.len()));
}

/// Print a section header
pub fn section(msg: &str) {
    println!("{}", msg.bold());
}

/// Format a label-value pair
pub fn pair(label: &str, value: &str) {
    println!("  {} {}", label.cyan(), value.bright_white());
}

/// Format a numbered list item
pub fn list_item(idx: usize, content: &str) {
    println!("{}. {}", idx, content);
}

/// Format pronunciation with special styling
pub fn pronunciation(symbol: &str, ipa: &str) {
    println!("  {} -> {}",
        symbol.bright_yellow().bold(),
        ipa.bright_cyan()
    );
}

/// Print success with checkmark
pub fn checkmark(msg: &str) {
    println!("{} {}", "[OK]".green().bold(), msg.green());
}

/// Print failure with X mark
pub fn cross(msg: &str) {
    println!("{} {}", "[FAIL]".red().bold(), msg.red());
}

