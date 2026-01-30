#!/bin/bash
set -e

# Determine if we need sudo
RUN_CMD=""
if [ "$(id -u)" -ne 0 ]; then
    if command -v sudo &> /dev/null; then
        RUN_CMD="sudo"
    else
        echo "This script requires root privileges to install packages. Please run as root or install sudo."
        exit 1
    fi
fi

echo "Updating package lists..."
$RUN_CMD apt-get update

echo "Installing system dependencies..."
$RUN_CMD apt-get install -y build-essential curl pkg-config git

echo "Installing Raylib build dependencies..."
$RUN_CMD apt-get install -y \
    cmake \
    clang \
    libclang-dev \
    libasound2-dev \
    libx11-dev \
    libxrandr-dev \
    libxi-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    libxcursor-dev \
    libxinerama-dev

echo "Installing Rust via rustup..."
if ! command -v rustc &> /dev/null; then
    # Download and install rustup
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Source the environment variables
    if [ -f "$HOME/.cargo/env" ]; then
        . "$HOME/.cargo/env"
    fi
else
    echo "Rust is already installed."
fi

echo "Verifying installation..."
if command -v rustc &> /dev/null; then
    rustc --version
    cargo --version
    echo "Setup complete! Please restart your terminal or run 'source $HOME/.cargo/env' to use Rust."
else
    echo "Rust installation verification failed. Please check the logs."
    exit 1
fi
