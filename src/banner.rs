/// This module provides fancy terminal banners and decorative elements
/// to enhance the visual appearance of the application.

use colored::*;

/// Returns the main application banner with Google Cloud SSH Manager title
/// 
/// # Returns
/// * A colorful banner string ready to be printed to the terminal
pub fn main_banner() -> String {
    let border = "═".repeat(60);
    
    format!(
        r#"
{}
   {}
   {}   {}
   {}  ╱╱  {}
   {} ╱╱   {}
   {}╱╱    {}
   {}      {}
{}
"#,
        border.bright_blue(),
        " ██████╗  ██████╗██╗      ██████╗ ██╗   ██╗██████╗ ".bright_cyan(),
        "██╔════╝ ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗".bright_cyan(),
        "SSH MANAGER".bright_white().bold(),
        "██║  ███╗██║     ██║     ██║   ██║██║   ██║██║  ██║".bright_cyan(),
        "v0.1.0".bright_white(),
        "██║   ██║██║     ██║     ██║   ██║██║   ██║██║  ██║".bright_cyan(),
        "Secure • Fast • Simple".bright_white().italic(),
        "╚██████╔╝╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝".bright_cyan(),
        "by Your Name".bright_black(),
        " ╚═════╝  ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝ ".bright_cyan(),
        "Rust-powered CLI tool".bright_black().italic(),
        border.bright_blue()
    )
}

/// Returns a section header for different parts of the application
/// 
/// # Arguments
/// * `title` - The section title
/// 
/// # Returns
/// * A formatted section header
pub fn section_header(title: &str) -> String {
    let pad_len = (50 - title.len()) / 2;
    let padding = "─".repeat(pad_len);
    
    format!(
        "\n{} {} {}\n",
        padding.bright_blue(),
        title.bright_white().bold(),
        padding.bright_blue()
    )
}

/// Returns a formatted success message
/// 
/// # Arguments
/// * `message` - The success message
/// 
/// # Returns
/// * A formatted success message
pub fn success_message(message: &str) -> String {
    format!("✅ {}", message.green().bold())
}

/// Returns a formatted information message
/// 
/// # Arguments
/// * `message` - The info message
/// 
/// # Returns
/// * A formatted info message
pub fn info_message(message: &str) -> String {
    format!("ℹ️  {}", message.blue())
}

/// Returns a formatted box with the SSH command
/// 
/// # Arguments
/// * `command` - The SSH command
/// 
/// # Returns
/// * A formatted box containing the SSH command
pub fn ssh_command_box(command: &str) -> String {
    let width = command.len() + 8;
    let horizontal = "─".repeat(width);
    
    format!(
        r#"
┌{}┐
│   {}   │
└{}┘
"#,
        horizontal.bright_blue(),
        command.bright_white().bold(),
        horizontal.bright_blue()
    )
}

/// Returns a spinner animation frame for progress indication
/// 
/// # Arguments
/// * `frame` - The animation frame number (0-3)
/// 
/// # Returns
/// * A character representing the current spinner frame
pub fn spinner_frame(frame: usize) -> &'static str {
    match frame % 4 {
        0 => "⠋",
        1 => "⠙",
        2 => "⠸",
        3 => "⠴",
        _ => "⠦",
    }
}

/// Returns a formatted VM list item
/// 
/// # Arguments
/// * `index` - The VM index number
/// * `name` - The VM name
/// * `zone` - The VM zone
/// * `ip` - The VM IP address, if available
/// 
/// # Returns
/// * A formatted VM list item
pub fn vm_list_item(index: usize, name: &str, zone: &str, ip: Option<&str>) -> String {
    let ip_display = match ip {
        Some(ip) => format!("🌐 {}", ip.bright_white()),
        None => "⚠️  No external IP".bright_black().to_string(),
    };
    
    let index_str = format!("[{}]", index + 1);
    
    format!(
        "{} {} {} {}",
        index_str.bright_yellow().bold(),
        name.bright_cyan().bold(),
        format!("({})", zone).bright_black(),
        ip_display
    )
} 