/// Google Cloud SSH Manager - Enhanced Terminal Interface
/// 
/// This library module organizes the terminal interface enhancements
/// without modifying the core functionality of the application.

// Module declarations
pub mod banner;
pub mod config;
pub mod term_utils;
pub mod terminal_fx;

// Re-exports for easier access
pub use banner::*;
pub use config::*;
pub use term_utils::*;
pub use terminal_fx::*;

/// Initializes the enhanced terminal interface
/// 
/// # Returns
/// * `Result<(), std::io::Error>` - Success or error information
pub fn initialize() -> Result<(), std::io::Error> {
    // Ensure terminal is in a clean state
    term_utils::clear_screen();
    term_utils::reset_terminal();
    term_utils::show_cursor();
    
    Ok(())
}

/// Displays a welcome message with banner and help text
/// 
/// # Returns
/// * `Result<(), std::io::Error>` - Success or error information
pub fn display_welcome() -> Result<(), std::io::Error> {
    // Print the banner
    println!("{}", banner::main_banner());
    
    // Show welcome message with typing effect if animations are enabled
    if config::animations::ENABLED {
        terminal_fx::type_text(
            &format!("Welcome to {}! Let's set up your SSH access.", config::APP_TITLE),
            config::animations::TYPING_SPEED_MS
        );
    } else {
        println!(
            "Welcome to {}! Let's set up your SSH access.",
            config::APP_TITLE
        );
    }
    
    // Display help text if tutorial mode is enabled
    if config::help::TUTORIAL_MODE {
        let help_text = "This tool will guide you through the process of:\n\
                         1. Checking for an existing SSH key\n\
                         2. Creating a new key if needed\n\
                         3. Listing your Google Cloud VMs\n\
                         4. Selecting a VM to connect to\n\
                         5. Adding your SSH key to the VM\n\
                         6. Generating the SSH command for connection";
        
        let term_width = term_utils::get_terminal_size()
            .map(|size| size.width)
            .unwrap_or(config::layout::DEFAULT_FRAME_WIDTH);
        
        terminal_fx::framed_message(help_text, term_width);
    }
    
    // Display keyboard shortcuts if enabled
    if config::help::SHOW_TIPS {
        println!("\n{}", config::help::KEYBOARD_SHORTCUTS);
    }
    
    Ok(())
}

/// Performs terminal cleanup on application exit
/// 
/// # Returns
/// * `Result<(), std::io::Error>` - Success or error information
pub fn cleanup() -> Result<(), std::io::Error> {
    term_utils::reset_terminal();
    term_utils::show_cursor();
    
    Ok(())
}

/// Formats a VM list item with enhanced styling
/// 
/// # Arguments
/// * `index` - VM index in the list
/// * `name` - VM name
/// * `zone` - VM zone
/// * `ip` - Optional external IP address
/// 
/// # Returns
/// * `String` - Formatted VM list item
pub fn format_vm_list_item(index: usize, name: &str, zone: &str, ip: Option<&str>) -> String {
    banner::vm_list_item(index, name, zone, ip)
}

/// Formats and displays the SSH command in a visually appealing box
/// 
/// # Arguments
/// * `command` - The SSH command to display
/// 
/// # Returns
/// * `Result<(), std::io::Error>` - Success or error information
pub fn display_ssh_command(command: &str) -> Result<(), std::io::Error> {
    println!("{}", banner::ssh_command_box(command));
    Ok(())
}

/// Displays a section header with styling
/// 
/// # Arguments
/// * `title` - The section title
/// 
/// # Returns
/// * `Result<(), std::io::Error>` - Success or error information
pub fn display_section_header(title: &str) -> Result<(), std::io::Error> {
    println!("{}", banner::section_header(title));
    Ok(())
}

/// Displays a success message with animation
/// 
/// # Arguments
/// * `message` - The success message
/// 
/// # Returns
/// * `Result<(), std::io::Error>` - Success or error information
pub fn display_success(message: &str) -> Result<(), std::io::Error> {
    if config::animations::ENABLED {
        terminal_fx::fade_text(&banner::success_message(message), 1000);
    } else {
        println!("{}", banner::success_message(message));
    }
    Ok(())
}

/// Displays a processing animation while performing a task
/// 
/// # Arguments
/// * `message` - The processing message
/// * `duration_ms` - Duration of the animation in milliseconds
/// 
/// # Returns
/// * `Result<(), std::io::Error>` - Success or error information
pub fn display_processing(message: &str, duration_ms: u64) -> Result<(), std::io::Error> {
    if config::animations::ENABLED {
        terminal_fx::spinner(message, duration_ms);
    } else {
        println!("{}", message);
    }
    Ok(())
} 