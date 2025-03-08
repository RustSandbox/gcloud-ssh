#!/bin/bash

# Fix PATH for Google Cloud SSH Manager
# This script adds the installation directory to your PATH

# Define colors for pretty output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m' # No Color

echo -e "${BLUE}${BOLD}=== Google Cloud SSH Manager PATH Fix ===${NC}"

# Determine installation directory
if [ -x "/usr/local/bin/gcloud-ssh" ]; then
    INSTALL_DIR="/usr/local/bin"
elif [ -x "$HOME/.local/bin/gcloud-ssh" ]; then
    INSTALL_DIR="$HOME/.local/bin"
else
    echo -e "${YELLOW}Google Cloud SSH Manager doesn't seem to be installed.${NC}"
    echo -e "Please run the install.sh script first."
    exit 1
fi

# Detect shell
current_shell="$(basename "$SHELL")"
shell_config=""

if [ "$current_shell" = "bash" ]; then
    shell_config="$HOME/.bashrc"
elif [ "$current_shell" = "zsh" ]; then
    shell_config="$HOME/.zshrc"
else
    echo -e "${YELLOW}Unsupported shell: $current_shell${NC}"
    echo -e "Please add the following line to your shell configuration file manually:"
    echo -e "${BOLD}export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
    exit 1
fi

# Check if already in PATH
if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
    echo -e "${GREEN}$INSTALL_DIR is already in your PATH.${NC}"
    exit 0
fi

# Add to PATH
echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$shell_config"
echo -e "${GREEN}✅ Added $INSTALL_DIR to PATH in $shell_config${NC}"
echo -e "\nTo apply changes to your current terminal, run:"
echo -e "${BOLD}source $shell_config${NC}"

echo -e "\n${BLUE}You should now be able to run ${BOLD}gcloud-ssh${NC}${BLUE} from anywhere.${NC}" 