#!/bin/bash

# Google Cloud SSH Manager Installer
# This script installs the Google Cloud SSH Manager so it can be run from anywhere.

set -e  # Exit on any error

# Define colors for pretty output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m' # No Color

echo -e "${BLUE}${BOLD}=== Google Cloud SSH Manager Installer ===${NC}"
echo -e "${BLUE}This script will install Google Cloud SSH Manager system-wide.${NC}\n"

# Check if running with sudo/root permissions
if [ "$EUID" -ne 0 ]; then
    echo -e "${YELLOW}Note: You're running this script as a normal user.${NC}"
    echo -e "Installation will be for current user only.\n"
    INSTALL_DIR="$HOME/.local/bin"
else
    echo -e "${YELLOW}Note: You're running this script with sudo.${NC}"
    echo -e "Installation will be system-wide.\n"
    INSTALL_DIR="/usr/local/bin"
fi

# Create installation directory if it doesn't exist
if [ ! -d "$INSTALL_DIR" ]; then
    echo -e "${BLUE}Creating directory: $INSTALL_DIR${NC}"
    mkdir -p "$INSTALL_DIR"
fi

# Check if the binary exists
if [ ! -f "target/release/hcloud" ]; then
    echo -e "${RED}Error: Release binary not found!${NC}"
    echo -e "Please run ${BOLD}cargo build --release${NC} first."
    exit 1
fi

# Copy the binary to the installation directory
echo -e "${BLUE}Installing to $INSTALL_DIR/gcloud-ssh...${NC}"
cp target/release/hcloud "$INSTALL_DIR/gcloud-ssh"
chmod +x "$INSTALL_DIR/gcloud-ssh"

# Success message
echo -e "\n${GREEN}${BOLD}✅ Installation successful!${NC}"
echo -e "${GREEN}You can now run ${BOLD}gcloud-ssh${NC}${GREEN} from anywhere.${NC}"

# Check if the installation directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "\n${YELLOW}Warning: $INSTALL_DIR is not in your PATH.${NC}"
    
    # Detect shell
    current_shell="$(basename "$SHELL")"
    shell_config=""
    
    if [ "$current_shell" = "bash" ]; then
        shell_config="$HOME/.bashrc"
    elif [ "$current_shell" = "zsh" ]; then
        shell_config="$HOME/.zshrc"
    fi
    
    if [ -n "$shell_config" ]; then
        echo -e "Would you like to add it to your PATH automatically? (y/n)"
        read -r auto_add_path
        
        if [[ "$auto_add_path" =~ ^[Yy]$ ]]; then
            echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$shell_config"
            echo -e "${GREEN}✅ Added to PATH in $shell_config${NC}"
            echo -e "Run the following command to apply changes to your current terminal:"
            echo -e "${BOLD}source $shell_config${NC}"
        else
            echo -e "\nTo add it manually, run this command:"
            echo -e "${BOLD}echo 'export PATH=\"\$PATH:$INSTALL_DIR\"' >> $shell_config${NC}"
            echo -e "Then run: ${BOLD}source $shell_config${NC}"
        fi
    else
        echo -e "\nTo add it manually, add this line to your shell configuration file:"
        echo -e "${BOLD}export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
    fi
fi

echo -e "\n${BLUE}Thank you for installing Google Cloud SSH Manager!${NC}" 