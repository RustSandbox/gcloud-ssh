/// This module provides terminal effects and animations
/// to enhance the user experience without modifying core functionality.
use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

/// Creates a typing effect for text, simulating someone typing
///
/// # Arguments
/// * `text` - The text to display with typing effect
/// * `delay_ms` - Delay between characters in milliseconds
pub fn type_text(text: &str, delay_ms: u64) {
    for c in text.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(delay_ms));
    }
    println!();
}

/// Displays a loading spinner with message
///
/// # Arguments
/// * `message` - The message to display next to the spinner
/// * `duration_ms` - How long to show the spinner in milliseconds
pub fn spinner(message: &str, duration_ms: u64) {
    let spinner_chars = ["⠋", "⠙", "⠸", "⠴", "⠦", "⠇"];
    let interval = Duration::from_millis(80);
    let iterations = duration_ms / 80;

    for i in 0..iterations {
        print!(
            "\r{} {}",
            spinner_chars[i as usize % spinner_chars.len()],
            message
        );
        io::stdout().flush().unwrap();
        sleep(interval);
    }
    print!("\r");
    // Clear the line
    print!("\r{}\r", " ".repeat(message.len() + 2));
    io::stdout().flush().unwrap();
}

/// Creates a progress bar effect
///
/// # Arguments
/// * `message` - The message to display with the progress bar
/// * `total` - Total number of steps
/// * `duration_ms` - Total duration of the progress bar in milliseconds
pub fn progress_bar(message: &str, total: u64, duration_ms: u64) {
    let width = 30;
    let step_duration = duration_ms / total;

    for i in 1..=total {
        let percentage = (i as f64 / total as f64) * 100.0;
        let filled = (width as f64 * i as f64 / total as f64) as usize;
        let empty = width - filled;

        print!(
            "\r{} [{}{}] {:.1}%",
            message,
            "█".repeat(filled),
            " ".repeat(empty),
            percentage
        );
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(step_duration));
    }
    println!();
}

/// Displays a framed message in the terminal
///
/// # Arguments
/// * `message` - The message to display in the frame
/// * `width` - Width of the frame
pub fn framed_message(message: &str, width: usize) {
    let top = "┌".to_owned() + &"─".repeat(width - 2) + "┐";
    let bottom = "└".to_owned() + &"─".repeat(width - 2) + "┘";
    
    println!("{}", top);
    
    // Split message into lines that fit within the frame
    let max_line_width = width - 4;
    let mut current_line = String::new();
    
    for word in message.split_whitespace() {
        if current_line.len() + word.len() < max_line_width {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else if !current_line.is_empty() {
            let padding = " ".repeat(width - 4 - current_line.len());
            println!("│ {} {} │", current_line, padding);
            current_line = word.to_string();
        } else {
            // Word is too long, need to split it
            current_line = word.to_string();
        }
    }
    
    if !current_line.is_empty() {
        let padding = " ".repeat(width - 4 - current_line.len());
        println!("│ {} {} │", current_line, padding);
    }
    
    println!("{}", bottom);
}

/// Creates a fading effect for text
///
/// # Arguments
/// * `text` - The text to fade in and out
/// * `duration_ms` - Total duration of the effect in milliseconds
pub fn fade_text(text: &str, duration_ms: u64) {
    let half_duration = duration_ms / 2;
    let steps = 10;
    let step_duration = half_duration / steps;

    // Fade in
    for i in 1..=steps {
        print!("\r");
        let opacity = i as f64 / steps as f64;
        let gray_level = (opacity * 24.0) as u8;
        print!("\x1b[38;5;{}m{}\x1b[0m", 232 + gray_level, text);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(step_duration));
    }

    // Fully visible pause
    sleep(Duration::from_millis(half_duration));

    // Fade out
    for i in (1..=steps).rev() {
        print!("\r");
        let opacity = i as f64 / steps as f64;
        let gray_level = (opacity * 24.0) as u8;
        print!("\x1b[38;5;{}m{}\x1b[0m", 232 + gray_level, text);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(step_duration));
    }

    // Clear line
    print!("\r{}\r", " ".repeat(text.len()));
    io::stdout().flush().unwrap();
}
