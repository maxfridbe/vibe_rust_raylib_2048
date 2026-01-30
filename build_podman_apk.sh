#!/bin/bash
set -e

IMAGE_NAME="raylib2048-android-build-env"

# Ensure output directory exists
mkdir -p dist

echo "Building Android Build Environment Image (if needed)..."
podman build -t $IMAGE_NAME -f Dockerfile.android .

echo "Building Android APK..."
podman run --rm -v "$(pwd):/app:Z" -w /app $IMAGE_NAME /bin/bash -c '
set -e

# We might need to add package metadata to Cargo.toml if not present
if ! grep -q "\[package.metadata.android\]" Cargo.toml; then
    echo "Adding temporary Android metadata to Cargo.toml..."
    cat >> Cargo.toml <<EOF

[package.metadata.android]
package = "com.maxfridbe.raylib2048"
label = "Raylib 2048"
assets = "assets"
res = "res"
icon = "assets/icon.png"
build_targets = ["aarch64-linux-android"]
EOF
fi

# Build APK
cargo apk build --release

# Copy resulting APK to dist
APK_FILE=$(find target/release/apk -name "*.apk" | head -n 1)
if [ -n "$APK_FILE" ]; then
    cp "$APK_FILE" dist/raylib2048.apk
    echo "APK created at dist/raylib2048.apk"
else
    echo "Error: APK file not found."
    exit 1
fi
'