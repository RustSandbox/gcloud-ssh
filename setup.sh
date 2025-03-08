#!/bin/bash

# Simple setup script for Google Cloud SSH Manager
# This script builds, installs, and sets up your PATH in one command
# Created by Hamze Ghalebi, CTO at Welcome Place
# Part of a collection of open-source developer tools

echo "=== Setting up Google Cloud SSH Manager ==="
echo

# Build the release version
echo "Step 1: Building release version..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Error: Build failed!"
    exit 1
fi
echo "Build successful!"
echo

# Install the binary
echo "Step 2: Installing application..."
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"
cp target/release/hcloud "$INSTALL_DIR/gcloud-ssh"
chmod +x "$INSTALL_DIR/gcloud-ssh"
echo "Installed to $INSTALL_DIR/gcloud-ssh"
echo

# Update PATH if needed
echo "Step 3: Checking PATH configuration..."
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "$INSTALL_DIR is not in your PATH"
    
    # Detect shell
    SHELL_CONFIG=""
    if [[ "$SHELL" == *"zsh"* ]]; then
        SHELL_CONFIG="$HOME/.zshrc"
    elif [[ "$SHELL" == *"bash"* ]]; then
        SHELL_CONFIG="$HOME/.bashrc"
    fi
    
    if [ -n "$SHELL_CONFIG" ]; then
        echo "Adding $INSTALL_DIR to your PATH in $SHELL_CONFIG"
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG"
        echo "To apply changes immediately, run: source $SHELL_CONFIG"
    else
        echo "Please add this line to your shell configuration file:"
        echo "export PATH=\"\$PATH:$INSTALL_DIR\""
    fi
else
    echo "$INSTALL_DIR is already in your PATH"
fi

echo
echo "=== Setup complete! ==="
echo "Run 'gcloud-ssh' to use the application" 