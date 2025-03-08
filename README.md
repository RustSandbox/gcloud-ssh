# 🚀 Google Cloud SSH Manager

A Rust command-line utility that automates SSH key management and Google Cloud VM access with a rich, interactive terminal interface.

## ✨ Features

- **🔑 Automatic SSH Key Management**: Checks for an existing SSH key pair and generates one if needed.
- **📋 Interactive VM Selection**: Displays a list of your Google Cloud VMs with their details for easy selection.
- **🔒 Secure Key Deployment**: Automatically copies your public SSH key to the selected VM.
- **🖥️ Ready-to-Use SSH Command**: Generates the exact SSH command you need to connect to your VM.
- **🎨 Rich Terminal Interface**: Beautiful ASCII art, animations, and color-coded information.
- **📱 Responsive Design**: Adapts to your terminal size for optimal display.

## 🛠️ Prerequisites

Before using this tool, ensure you have:

1. [Rust and Cargo](https://www.rust-lang.org/tools/install) installed
2. [Google Cloud SDK (gcloud CLI)](https://cloud.google.com/sdk/docs/install) installed and configured
3. Active Google Cloud project with compute instances

## 📦 Installation

### Option 1: Quick Install (Recommended)

Build and install the application system-wide:

```bash
# Build the release version
cargo build --release

# Install system-wide (requires sudo)
sudo ./install.sh

# Or install for current user only
./install.sh
```

After installation, you can run the application from anywhere:

```bash
gcloud-ssh
```

### Option 2: Manual Installation

If you prefer to install manually:

```bash
# Build the release version
cargo build --release

# Copy to a directory in your PATH
cp target/release/hcloud ~/.local/bin/gcloud-ssh
chmod +x ~/.local/bin/gcloud-ssh

# Make sure ~/.local/bin is in your PATH
echo 'export PATH=$PATH:~/.local/bin' >> ~/.bashrc  # or ~/.zshrc
source ~/.bashrc  # or ~/.zshrc
```

## 🚀 Usage

Run the application:

```bash
gcloud-ssh
```

You can also use these command-line options:

```bash
# Show help
gcloud-ssh --help

# Show version
gcloud-ssh --version

# Check for updates
gcloud-ssh --update
```

The application will guide you through the process with a beautiful interface:

1. 🔍 Check/generate SSH keys
2. 📊 List available VM instances
3. 🔖 Let you select a VM
4. 📤 Copy your SSH key to the VM
5. 📝 Show you the exact SSH command to connect

## 🧩 How It Works

1. The tool first checks if you have an SSH key pair in the standard location (`~/.ssh/id_rsa` and `~/.ssh/id_rsa.pub`).
2. If no key exists, it uses `gcloud compute ssh-keys create` to generate a new key pair.
3. It retrieves a list of your VM instances using `gcloud compute instances list`.
4. It displays an interactive selection menu showing VM names, zones, and IP addresses.
5. Once you select a VM, it copies your public key to the VM's `~/.ssh/authorized_keys` file.
6. Finally, it constructs and displays the SSH command you can use to connect to the VM.

## 🎨 Terminal Interface Features

The application includes several terminal interface enhancements:

- **📊 ASCII Art Banner**: A beautiful banner greets you on startup
- **🎬 Animations**: Typing effects, spinners, and progress bars
- **🎨 Color-Coded Information**: Different colors for different types of information
- **📦 Boxed Messages**: Important information is displayed in stylish boxes
- **📏 Responsive Layout**: Adapts to your terminal size
- **✨ Emoji Support**: Uses emojis for better visual cues

## ⚙️ Customization

You can customize the interface by modifying the `config.rs` file:

- **Animation Settings**: Enable/disable animations and adjust their timing
- **Color Schemes**: Change the color theme to match your preferences
- **Layout Options**: Adjust the terminal layout settings
- **Help Text**: Configure the level of guidance provided
- **Emojis**: Customize the emojis used for different message types

## 🔄 Updating the Application

To update to the latest version:

```bash
# Check for updates
gcloud-ssh --update

# Or update manually
git pull
cargo build --release
sudo ./install.sh
```

## 🔧 Troubleshooting

If you encounter issues:

- Ensure you're authenticated with gcloud: `gcloud auth login`
- Verify your active project: `gcloud config get-value project`
- Check VM access permissions in Google Cloud Console
- Run with verbose output: `RUST_LOG=debug gcloud-ssh`
- Make sure the application is correctly installed in your PATH

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details. 