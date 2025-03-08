/// This module provides configuration options for the application
/// These settings control the appearance and behavior of the terminal interface

/// Application title displayed in various places
pub const APP_TITLE: &str = "Google Cloud SSH Manager";

/// Application version
pub const APP_VERSION: &str = "0.1.0";

/// Application tagline
pub const APP_TAGLINE: &str = "Secure • Fast • Simple";

/// Default author name (can be customized)
pub const AUTHOR: &str = "Your Name";

/// Configuration for terminal animations
pub mod animations {
    /// Whether to enable animations
    pub const ENABLED: bool = true;

    /// Typing effect speed (milliseconds per character)
    pub const TYPING_SPEED_MS: u64 = 10;

    /// Default spinner duration in milliseconds
    pub const SPINNER_DURATION_MS: u64 = 1000;

    /// Progress bar steps
    pub const PROGRESS_BAR_STEPS: u64 = 20;

    /// Progress bar duration in milliseconds
    pub const PROGRESS_BAR_DURATION_MS: u64 = 1500;
}

/// Configuration for terminal colors and styles
pub mod styles {
    /// Primary color for titles and important information
    pub const PRIMARY_COLOR: &str = "cyan";

    /// Secondary color for sub-headings and highlights
    pub const SECONDARY_COLOR: &str = "yellow";

    /// Success message color
    pub const SUCCESS_COLOR: &str = "green";

    /// Information message color
    pub const INFO_COLOR: &str = "blue";

    /// Warning message color
    pub const WARNING_COLOR: &str = "yellow";

    /// Error message color
    pub const ERROR_COLOR: &str = "red";

    /// Box drawing style: "single", "double", "rounded", or "bold"
    pub const BOX_STYLE: &str = "rounded";
}

/// Configuration for terminal UI layout
pub mod layout {
    /// Terminal width in characters (0 for auto-detect)
    pub const TERMINAL_WIDTH: usize = 0;

    /// Padding size for framed messages
    pub const FRAME_PADDING: usize = 2;

    /// Default frame width if terminal width detection fails
    pub const DEFAULT_FRAME_WIDTH: usize = 80;

    /// Horizontal rule character
    pub const HORIZONTAL_RULE_CHAR: &str = "─";

    /// Indent size for list items
    pub const LIST_INDENT: usize = 2;
}

/// Configuration for help messages
pub mod help {
    /// Tutorial mode (show more detailed help)
    pub const TUTORIAL_MODE: bool = true;

    /// Show tips and hints during operation
    pub const SHOW_TIPS: bool = true;

    /// Keyboard shortcuts help text
    pub const KEYBOARD_SHORTCUTS: &str = "Keyboard shortcuts:\n\
        - Press ↑/↓ to navigate\n\
        - Press Enter to select\n\
        - Press q to quit at any prompt";
}

/// Emoji sets for different message types
pub mod emojis {
    /// Success indicators
    pub const SUCCESS: &[&str] = &["✅", "🎉", "🚀"];

    /// Information indicators
    pub const INFO: &[&str] = &["ℹ️", "💡", "🔍"];

    /// Warning indicators
    pub const WARNING: &[&str] = &["⚠️", "🔔", "❗"];

    /// Error indicators
    pub const ERROR: &[&str] = &["❌", "🛑", "💔"];

    /// SSH-related icons
    pub const SSH: &[&str] = &["🔑", "🔒", "🖥️", "🌐"];

    /// IP address icon
    pub const IP_ADDRESS: &str = "🌐";

    /// VM instance icon
    pub const VM: &str = "🖥️";

    /// SSH key icon
    pub const KEY: &str = "🔑";

    /// Zone/location icon
    pub const ZONE: &str = "📍";
}
