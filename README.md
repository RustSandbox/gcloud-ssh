# 🚀 Google Cloud SSH Manager

A Rust command-line utility that automates SSH key management and Google Cloud VM access with a rich, interactive terminal interface.

## ✨ Features

- **🔑 Automatic SSH Key Management**: Checks for an existing SSH key pair and generates one if needed.
- **📋 Interactive VM Selection**: Displays a list of your Google Cloud VMs with their details for easy selection.
- **🔒 Secure Key Deployment**: Automatically copies your public SSH key to the selected VM.
- **🖥️ Ready-to-Use SSH Command**: Generates the exact SSH command you need to connect to your VM.
- **🎨 Rich Terminal Interface**: Beautiful ASCII art, animations, and color-coded information.

## 🛠️ Prerequisites

Before using this tool, ensure you have:

1. [Rust and Cargo](https://www.rust-lang.org/tools/install) installed
2. [Google Cloud SDK (gcloud CLI)](https://cloud.google.com/sdk/docs/install) installed and configured
3. Active Google Cloud project with compute instances

## 📦 Installation

### Option 1: One-Step Setup (Recommended)

Run the setup script to build, install, and configure your PATH in one step:

```bash
./setup.sh
```

### Option 2: Using Makefile

For more control, use the Makefile:

```bash
# Build in debug mode
make

# Build optimized release
make release

# Install to ~/.local/bin
make install
```

Type `make help` to see all available commands.

### Option 3: Manual Installation

If you prefer to install manually:

```bash
# Build the release version
cargo build --release

# Copy to a directory in your PATH
mkdir -p ~/.local/bin
cp target/release/hcloud ~/.local/bin/gcloud-ssh
chmod +x ~/.local/bin/gcloud-ssh

# Add to PATH if needed
echo 'export PATH=$PATH:~/.local/bin' >> ~/.zshrc  # or ~/.bashrc
source ~/.zshrc  # or ~/.bashrc
```

## 🚀 Usage

Run the application:

```bash
gcloud-ssh
```

Command-line options:

```bash
# Show help
gcloud-ssh --help

# Show version
gcloud-ssh --version

# Check for updates
gcloud-ssh --update
```

The application will guide you through:

1. 🔍 Checking/generating SSH keys
2. 📊 Listing available VM instances
3. 🔖 Selecting a VM to connect to
4. 📤 Copying your SSH key to the VM
5. 📝 Showing the exact SSH command to connect

## 🧩 How It Works

1. The tool checks if you have an SSH key pair in `~/.ssh/id_rsa` and `~/.ssh/id_rsa.pub`.
2. If no key exists, it generates a new key pair using `gcloud compute ssh-keys create`.
3. It retrieves your VM instances using `gcloud compute instances list`.
4. It displays an interactive selection menu with your VMs.
5. Once you select a VM, it copies your public key to the VM's `~/.ssh/authorized_keys` file.
6. Finally, it shows you the exact SSH command to connect.

## 🎨 Terminal Interface

The application includes:

- **📊 ASCII Art Banner**: Beautiful welcome banner
- **🎬 Animations**: Typing effects, spinners, and progress bars
- **🎨 Color-Coded Information**: Different colors for different types of information
- **📦 Boxed Messages**: Important information displayed in stylish boxes

## ⚙️ Customization

Modify the `config.rs` file to customize:

- **Animation Settings**: Enable/disable animations and timing
- **Color Schemes**: Change the color theme
- **Emojis**: Customize emojis used for different messages

## 🔧 Troubleshooting

If you encounter issues:

- Ensure you're authenticated with gcloud: `gcloud auth login`
- Verify your active project: `gcloud config get-value project`
- Check VM access permissions in Google Cloud Console
- Run with verbose output: `RUST_LOG=debug gcloud-ssh`

## 👨‍💻 About the Author

I'm Hamze Ghalebi, CTO at Remolab, passionate about building tools that improve developer workflows. This Google Cloud SSH Manager is part of a collection of tools I originally built for my own use, and I've decided to open source it in case others find it helpful.

Many of the tools I create solve specific pain points in my daily workflow with cloud infrastructure and development environments. If you have any feedback or suggestions for improvements, please feel free to contribute!

### Connect with me:
- GitHub: [hghalebi](https://github.com/hghalebi)
- Twitter/X: [@hamzeml](https://twitter.com/hamzeml)
- Website: [linkedin](https://www.linkedin.com/in/hamze/)

### Support this project:
If you find this tool useful, please consider [sponsoring me on GitHub](https://github.com/sponsors/hghalebi) to support continued development and maintenance.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details. 