# Simple Makefile for Google Cloud SSH Manager
# Created by Hamze Ghalebi, CTO at Welcome Place
# Part of a collection of open-source developer tools

# Configuration
BINARY = gcloud-ssh
INSTALL_DIR = $(HOME)/.local/bin

# Default: build in debug mode
all: build

# Build the application (debug mode)
build:
	cargo build

# Build optimized release
release:
	cargo build --release

# Run the application
run:
	cargo run

# Install (user level)
install: release
	mkdir -p $(INSTALL_DIR)
	cp target/release/hcloud $(INSTALL_DIR)/$(BINARY)
	chmod +x $(INSTALL_DIR)/$(BINARY)
	@echo "Installed to $(INSTALL_DIR)/$(BINARY)"
	@echo "Make sure $(INSTALL_DIR) is in your PATH"

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Show help
help:
	@echo "Simple Makefile for Google Cloud SSH Manager"
	@echo ""
	@echo "make          - Build in debug mode"
	@echo "make release  - Build optimized release"
	@echo "make run      - Run the application"
	@echo "make install  - Install to ~/.local/bin"
	@echo "make test     - Run tests"
	@echo "make clean    - Clean build artifacts"
	@echo "make help     - Show this help"

.PHONY: all build release run install test clean help 