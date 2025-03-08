# Google Cloud SSH Manager

A Rust command-line utility that automates SSH key management and Google Cloud VM access.

## Features

- **Automatic SSH Key Management**: Checks for an existing SSH key pair and generates one if needed.
- **Interactive VM Selection**: Displays a list of your Google Cloud VMs with their details for easy selection.
- **Secure Key Deployment**: Automatically copies your public SSH key to the selected VM.
- **Ready-to-Use SSH Command**: Generates the exact SSH command you need to connect to your VM.

## Prerequisites

Before using this tool, ensure you have:

1. [Rust and Cargo](https://www.rust-lang.org/tools/install) installed
2. [Google Cloud SDK (gcloud CLI)](https://cloud.google.com/sdk/docs/install) installed and configured
3. Active Google Cloud project with compute instances

## Installation

Clone the repository and build the project:

```bash
git clone <repository-url>
cd gcloud-ssh-manager
cargo build --release
```

The executable will be available at `target/release/hcloud`.

## Usage

Simply run the application without any arguments:

```bash
cargo run
```

Or use the compiled binary:

```bash
./target/release/hcloud
```

The application will guide you through the process:

1. Check/generate SSH keys
2. List available VM instances
3. Let you select a VM
4. Copy your SSH key to the VM
5. Show you the exact SSH command to connect

## How It Works

1. The tool first checks if you have an SSH key pair in the standard location (`~/.ssh/id_rsa` and `~/.ssh/id_rsa.pub`).
2. If no key exists, it uses `gcloud compute ssh-keys create` to generate a new key pair.
3. It retrieves a list of your VM instances using `gcloud compute instances list`.
4. It displays an interactive selection menu showing VM names, zones, and IP addresses.
5. Once you select a VM, it copies your public key to the VM's `~/.ssh/authorized_keys` file.
6. Finally, it constructs and displays the SSH command you can use to connect to the VM.

## Troubleshooting

If you encounter issues:

- Ensure you're authenticated with gcloud: `gcloud auth login`
- Verify your active project: `gcloud config get-value project`
- Check VM access permissions in Google Cloud Console
- Run with verbose output: `RUST_LOG=debug cargo run`

## License

This project is licensed under the MIT License - see the LICENSE file for details. 