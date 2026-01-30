#!/bin/bash
set -e

# 1. Check/Install Dependencies
if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "MinGW-w64 not found. Installing..."
    if [ "$(id -u)" -ne 0 ] && command -v sudo &> /dev/null; then
        sudo apt-get update && sudo apt-get install -y mingw-w64 zip
    elif [ "$(id -u)" -eq 0 ]; then
        apt-get update && apt-get install -y mingw-w64 zip
    else
        echo "Please install 'mingw-w64' and 'zip' manually."
        echo "Example: sudo apt-get install mingw-w64 zip"
        exit 1
    fi
fi

# Ensure 'zip' is installed too
if ! command -v zip &> /dev/null; then
     echo "Installing zip..."
     if [ "$(id -u)" -ne 0 ] && command -v sudo &> /dev/null; then
        sudo apt-get install -y zip
     elif [ "$(id -u)" -eq 0 ]; then
        apt-get install -y zip
     fi
fi

# 2. Add Rust Target
echo "Adding Rust Windows target..."
rustup target add x86_64-pc-windows-gnu

# 3. Build for Windows
echo "Building for Windows (x86_64-pc-windows-gnu)..."

# Configure the 'cc' and 'cmake' crates to use the MinGW compilers
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++
export AR_x86_64_pc_windows_gnu=x86_64-w64-mingw32-ar
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc

# Sometimes Raylib's build script needs explicit help finding the cross-compiler for CMake
export CMAKE_C_COMPILER=x86_64-w64-mingw32-gcc
export CMAKE_CXX_COMPILER=x86_64-w64-mingw32-g++
export CMAKE_SYSTEM_NAME=Windows

cargo build --release --target x86_64-pc-windows-gnu

# 4. Package
echo "Packaging..."
DIST_DIR="dist_win"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

cp target/x86_64-pc-windows-gnu/release/raylib2048.exe "$DIST_DIR/"
cp -r assets "$DIST_DIR/"
cp README.md "$DIST_DIR/"

# Create Zip
echo "Creating ZIP archive..."
zip -r raylib2048-windows-x86_64.zip "$DIST_DIR"

echo "Done! Artifact: raylib2048-windows-x86_64.zip"
