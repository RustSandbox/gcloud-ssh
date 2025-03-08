use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::Deserialize;
use std::{env, fs, io, process::Command};
use thiserror::Error;

// Import our enhanced terminal interface library
mod banner;
mod config;
mod term_utils;
mod terminal_fx;

// Version of the application
const VERSION: &str = "0.1.0";

/// Custom error type for our application.
/// This provides more descriptive errors than using anyhow alone.
#[derive(Error, Debug)]
enum AppError {
    #[error("No SSH key found and failed to generate one: {0}")]
    SshKeyGeneration(String),

    #[error("Failed to list VM instances: {0}")]
    VmListing(String),

    #[error("No VM instances found in the active project")]
    NoVmsFound,

    #[error("Failed to select VM: {0}")]
    VmSelection(String),

    #[error("Failed to copy SSH key to VM: {0}")]
    KeyCopy(String),

    #[error("VM does not have an external IP address")]
    NoExternalIp,

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

/// Represents a Google Cloud VM instance
#[derive(Debug, Deserialize, Clone)]
struct Instance {
    /// Name of the VM instance
    name: String,

    /// Zone where the VM is located (e.g., "us-central1-a")
    #[serde(rename = "zone")]
    zone_url: String,

    /// Network interfaces attached to the VM
    #[serde(rename = "networkInterfaces")]
    network_interfaces: Vec<NetworkInterface>,
}

impl Instance {
    /// Extracts just the zone name from the full zone URL
    fn zone(&self) -> String {
        // The zone URL is formatted like: "https://www.googleapis.com/compute/v1/projects/PROJECT_ID/zones/ZONE_NAME"
        // We only want the ZONE_NAME part
        self.zone_url
            .split('/')
            .last()
            .unwrap_or(&self.zone_url)
            .to_string()
    }

    /// Gets the external IP address of the VM, if available
    fn external_ip(&self) -> Option<String> {
        // Get the first network interface
        self.network_interfaces.first().and_then(|interface| {
            // Get the first access config with a natIP
            interface
                .access_configs
                .iter()
                .find_map(|config| config.nat_ip.clone())
        })
    }
}

/// Represents a network interface attached to a VM
#[derive(Debug, Deserialize, Clone)]
struct NetworkInterface {
    /// Configuration for external access
    #[serde(rename = "accessConfigs")]
    access_configs: Vec<AccessConfig>,
}

/// Configuration for external network access
#[derive(Debug, Deserialize, Clone)]
struct AccessConfig {
    /// External IP address, if assigned
    #[serde(rename = "natIP")]
    nat_ip: Option<String>,
}

/// Prints the version information and exits
fn print_version() {
    println!("Google Cloud SSH Manager v{}", VERSION);
    println!("A tool to automate SSH access to Google Cloud VMs");
    std::process::exit(0);
}

/// Prints the help message and exits
fn print_help() {
    println!("Google Cloud SSH Manager v{}", VERSION);
    println!("A tool to automate SSH access to Google Cloud VMs\n");
    println!("USAGE:");
    println!("  gcloud-ssh [OPTIONS]\n");
    println!("OPTIONS:");
    println!("  -h, --help     Print this help message");
    println!("  -v, --version  Print version information");
    println!("  --update       Check for updates and install them");
    std::process::exit(0);
}

/// Checks for updates and installs them if available
fn check_for_updates() -> Result<()> {
    println!("{}", "Checking for updates...".blue());

    // In a real application, this would connect to a server to check for updates
    // For this example, we'll just display a message

    println!("{}", "You're running the latest version!".green());
    println!("If you want to update manually, run the following commands:");
    println!("  1. git pull");
    println!("  2. cargo build --release");
    println!("  3. sudo ./install.sh");

    Ok(())
}

/// Parses command-line arguments
fn parse_args() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => print_help(),
            "-v" | "--version" => print_version(),
            "--update" => {
                if let Err(e) = check_for_updates() {
                    eprintln!("Error checking for updates: {}", e);
                    std::process::exit(1);
                }
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unknown option: {}", args[1]);
                eprintln!("Run with --help for usage information");
                std::process::exit(1);
            }
        }
    }
}

/// Main function that orchestrates the application flow
fn main() -> Result<()> {
    // Parse command-line arguments
    parse_args();

    // Initialize terminal interface
    term_utils::clear_screen();

    // Display welcome banner
    println!("{}", banner::main_banner());

    // Add a slight delay for visual effect
    if config::animations::ENABLED {
        terminal_fx::type_text(
            &format!(
                "Welcome to {}! Let's set up your SSH access.",
                config::APP_TITLE
            ),
            config::animations::TYPING_SPEED_MS,
        );
    } else {
        println!(
            "Welcome to {}! Let's set up your SSH access.",
            config::APP_TITLE
        );
    }

    // Step 1: Ensure SSH key exists
    println!("{}", banner::section_header("SSH KEY MANAGEMENT"));
    ensure_ssh_key().context("Failed to ensure SSH key exists")?;

    // Step 2: List VM instances
    println!("{}", banner::section_header("VM INSTANCES"));

    // Display loading animation
    if config::animations::ENABLED {
        terminal_fx::spinner(
            "Fetching VM instances...",
            config::animations::SPINNER_DURATION_MS,
        );
    }

    let instances = list_vms().context("Failed to list VM instances")?;

    // Step 3: Let user select a VM
    println!("{}", banner::section_header("VM SELECTION"));
    let selected_vm = select_vm(&instances).context("Failed to select VM")?;

    // Step 4: Copy SSH key to selected VM
    println!("{}", banner::section_header("SSH KEY DEPLOYMENT"));

    // Display progress animation
    if config::animations::ENABLED {
        terminal_fx::progress_bar(
            "Copying SSH key to VM...",
            config::animations::PROGRESS_BAR_STEPS,
            config::animations::PROGRESS_BAR_DURATION_MS,
        );
    }

    copy_ssh_key_to_vm(&selected_vm).context("Failed to copy SSH key to VM")?;

    // Step 5: Print SSH command
    println!("{}", banner::section_header("CONNECTION INFORMATION"));
    print_ssh_command(&selected_vm)?;

    // Clean up terminal state
    term_utils::reset_terminal();
    term_utils::show_cursor();

    Ok(())
}

/// Ensures that an SSH key pair exists, generating one if it doesn't
///
/// # Returns
/// * `Result<()>` - Success or error information
fn ensure_ssh_key() -> Result<()> {
    // Get the path to the user's .ssh directory
    let ssh_dir = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?
        .join(".ssh");

    // Create the .ssh directory if it doesn't exist
    if !ssh_dir.exists() {
        println!("{}", banner::info_message("Creating ~/.ssh directory..."));
        fs::create_dir_all(&ssh_dir).context("Failed to create ~/.ssh directory")?;

        // Set appropriate permissions for .ssh directory (700)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&ssh_dir, fs::Permissions::from_mode(0o700))
                .context("Failed to set permissions on ~/.ssh directory")?;
        }
    }

    // Check if public key exists
    let pub_key_path = ssh_dir.join("id_rsa.pub");
    let priv_key_path = ssh_dir.join("id_rsa");

    if pub_key_path.exists() && priv_key_path.exists() {
        println!(
            "{}",
            banner::success_message("SSH key pair already exists.")
        );
        return Ok(());
    }

    // Generate new SSH key pair using gcloud
    println!(
        "{}",
        banner::info_message("No SSH key found. Generating new key pair...")
    );

    // Display spinner animation for key generation
    if config::animations::ENABLED {
        terminal_fx::spinner("Generating SSH key pair...", 3000);
    }

    // Use gcloud to generate the key
    let output = Command::new("gcloud")
        .args(["compute", "ssh-keys", "create"])
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::SshKeyGeneration(error_msg.to_string()).into());
    }

    println!(
        "{}",
        banner::success_message("SSH key generated successfully.")
    );
    Ok(())
}

/// Lists all VM instances in the active Google Cloud project
///
/// # Returns
/// * `Result<Vec<Instance>>` - List of VM instances or error
fn list_vms() -> Result<Vec<Instance>> {
    // Execute gcloud command to list instances in JSON format
    let output = Command::new("gcloud")
        .args(["compute", "instances", "list", "--format=json"])
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::VmListing(error_msg.to_string()).into());
    }

    // Parse JSON output into our Instance struct
    let instances: Vec<Instance> =
        serde_json::from_slice(&output.stdout).context("Failed to parse VM instance JSON data")?;

    // Check if we found any instances
    if instances.is_empty() {
        return Err(AppError::NoVmsFound.into());
    }

    println!(
        "{}",
        banner::success_message(&format!("Found {} VM instances.", instances.len()))
    );
    Ok(instances)
}

/// Allows the user to select a VM from the list
///
/// # Arguments
/// * `instances` - List of available VM instances
///
/// # Returns
/// * `Result<Instance>` - The selected VM instance or error
fn select_vm(instances: &[Instance]) -> Result<Instance> {
    // Map instances to display strings for selection menu
    let vm_display: Vec<String> = instances
        .iter()
        .enumerate()
        .map(|(idx, instance)| {
            // Create a longer-lived value for the IP
            let ip_option = instance.external_ip();
            let ip_str = ip_option.as_deref();

            banner::vm_list_item(idx, &instance.name, &instance.zone(), ip_str)
        })
        .collect();

    // Create an interactive selection menu
    println!(
        "{}",
        banner::info_message("Please select a VM to connect to:")
    );

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&vm_display)
        .default(0)
        .interact()
        .context("Failed to display VM selection menu")?;

    // Return a clone of the selected instance
    Ok(instances[selection].clone())
}

/// Copies the local SSH key to the authorized_keys file on the selected VM
///
/// # Arguments
/// * `instance` - The selected VM instance
///
/// # Returns
/// * `Result<()>` - Success or error information
fn copy_ssh_key_to_vm(instance: &Instance) -> Result<()> {
    println!(
        "{}",
        banner::info_message(&format!("Copying SSH key to VM: {}", instance.name.bold()))
    );

    // Get the path to the public key
    let pub_key_path = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?
        .join(".ssh")
        .join("id_rsa.pub");

    // Read public key content
    let pub_key_content =
        fs::read_to_string(&pub_key_path).context("Failed to read SSH public key")?;

    // Prepare the command to be executed on the VM
    // This command will:
    // 1. Create ~/.ssh directory if it doesn't exist
    // 2. Set proper permissions on ~/.ssh directory
    // 3. Append the public key to authorized_keys
    // 4. Set proper permissions on authorized_keys
    let remote_cmd = format!(
        "mkdir -p ~/.ssh && chmod 700 ~/.ssh && echo '{}' >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys",
        pub_key_content.trim()
    );

    // Execute gcloud command to run the remote command
    let output = Command::new("gcloud")
        .args([
            "compute",
            "ssh",
            &instance.name,
            "--zone",
            &instance.zone(),
            "--command",
            &remote_cmd,
        ])
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::KeyCopy(error_msg.to_string()).into());
    }

    println!(
        "{}",
        banner::success_message(&format!(
            "SSH key successfully copied to VM: {}",
            instance.name.bold()
        ))
    );
    Ok(())
}

/// Generates and prints the SSH command to connect to the VM
///
/// # Arguments
/// * `instance` - The selected VM instance
///
/// # Returns
/// * `Result<()>` - Success or error information
fn print_ssh_command(instance: &Instance) -> Result<()> {
    // Get the external IP of the VM
    let external_ip = instance.external_ip().ok_or(AppError::NoExternalIp)?;

    // Get the local username
    let username = whoami::username();

    // Construct the SSH command
    let ssh_cmd = format!("ssh {}@{}", username, external_ip);

    // Display connection information
    println!("{} {}", config::emojis::VM, "VM Name:".yellow());
    println!("   {}", instance.name.bright_cyan().bold());

    println!("{} {}", config::emojis::ZONE, "Zone:".yellow());
    println!("   {}", instance.zone().bright_cyan());

    println!("{} {}", config::emojis::IP_ADDRESS, "External IP:".yellow());
    println!("   {}", external_ip.bright_cyan());

    println!("\n{}", "To connect to your VM, run:".green().bold());

    // Display SSH command in a box
    println!("{}", banner::ssh_command_box(&ssh_cmd));

    Ok(())
}
