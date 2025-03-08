/// This module provides utility functions for terminal operations and detection
/// to enhance the user experience without modifying core functionality.

use std::{
    io::{self, Write},
    process::Command,
};

/// Represents terminal dimensions
#[derive(Debug, Clone, Copy)]
pub struct TerminalSize {
    /// Width in columns
    pub width: usize,
    /// Height in rows
    pub height: usize,
}

/// Detects the terminal size
/// 
/// # Returns
/// * `Option<TerminalSize>` - Terminal dimensions if detection succeeds
pub fn get_terminal_size() -> Option<TerminalSize> {
    #[cfg(unix)]
    {
        // Try using stty size command
        let output = Command::new("stty")
            .args(["size"])
            .output()
            .ok()?;
        
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let values: Vec<&str> = output_str.trim().split_whitespace().collect();
            
            if values.len() == 2 {
                let height = values[0].parse::<usize>().ok()?;
                let width = values[1].parse::<usize>().ok()?;
                return Some(TerminalSize { width, height });
            }
        }
    }
    
    // Fallback to tput
    let width = Command::new("tput")
        .args(["cols"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().parse::<usize>().ok()
            } else {
                None
            }
        });
    
    let height = Command::new("tput")
        .args(["lines"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().parse::<usize>().ok()
            } else {
                None
            }
        });
    
    match (width, height) {
        (Some(w), Some(h)) => Some(TerminalSize { width: w, height: h }),
        _ => None,
    }
}

/// Checks if the terminal supports ANSI colors
/// 
/// # Returns
/// * `bool` - True if the terminal supports ANSI colors
pub fn supports_color() -> bool {
    // Check TERM environment variable
    if let Ok(term) = std::env::var("TERM") {
        return !term.is_empty() && term != "dumb";
    }
    
    // Try using tput colors
    Command::new("tput")
        .args(["colors"])
        .output()
        .map(|output| {
            if output.status.success() {
                if let Ok(colors) = String::from_utf8_lossy(&output.stdout).trim().parse::<i32>() {
                    return colors > 0;
                }
            }
            false
        })
        .unwrap_or(false)
}

/// Clears the terminal screen
pub fn clear_screen() {
    if Command::new("clear").status().is_ok() {
        // The clear command worked, nothing else to do
        return;
    }
    
    // Fallback to ANSI escape codes
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

/// Moves the cursor to a specific position
/// 
/// # Arguments
/// * `row` - Row position (1-based)
/// * `col` - Column position (1-based)
pub fn goto_xy(row: usize, col: usize) {
    print!("\x1B[{};{}H", row, col);
    io::stdout().flush().unwrap();
}

/// Returns the terminal to normal mode (resets attributes)
pub fn reset_terminal() {
    print!("\x1B[0m");
    io::stdout().flush().unwrap();
}

/// Hides the cursor
pub fn hide_cursor() {
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();
}

/// Shows the cursor
pub fn show_cursor() {
    print!("\x1B[?25h");
    io::stdout().flush().unwrap();
}

/// Centers text in the terminal
/// 
/// # Arguments
/// * `text` - Text to center
/// * `width` - Available width (0 for auto-detect)
/// 
/// # Returns
/// * `String` - Centered text with appropriate padding
pub fn center_text(text: &str, width: usize) -> String {
    let term_width = if width > 0 {
        width
    } else if let Some(size) = get_terminal_size() {
        size.width
    } else {
        80 // Default fallback width
    };
    
    if text.len() >= term_width {
        return text.to_string();
    }
    
    let padding = (term_width - text.len()) / 2;
    format!("{}{}", " ".repeat(padding), text)
}

/// Wraps text to fit within a specified width
/// 
/// # Arguments
/// * `text` - Text to wrap
/// * `width` - Maximum width per line
/// 
/// # Returns
/// * `Vec<String>` - Lines of text wrapped to fit the width
pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_line = String::new();
    
    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 <= width {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            if !current_line.is_empty() {
                result.push(current_line);
                current_line = word.to_string();
            } else {
                // Word is too long for the width, need to break it up
                result.push(word[..width.min(word.len())].to_string());
                if word.len() > width {
                    current_line = word[width.min(word.len())..].to_string();
                } else {
                    current_line = String::new();
                }
            }
        }
    }
    
    if !current_line.is_empty() {
        result.push(current_line);
    }
    
    result
} 