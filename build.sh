#!/bin/bash
set -e

echo "Building release binary..."
cargo build --release

echo "Build complete. Binary is at target/release/raylib2048"
