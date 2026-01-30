#!/bin/bash
set -e

IMAGE_NAME="raylib2048-android-build-env"
# Use a pre-built Rust Android image to save significant time and complexity
BASE_IMAGE="docker.io/library/rust:bullseye"

# Ensure output directory exists
mkdir -p dist

echo "Creating Android Build Environment (this might take a long time on first run)..."

if [[ "$(podman images -q $IMAGE_NAME 2> /dev/null)" == "" ]]; then
    cat <<EOF | podman build -t $IMAGE_NAME -f - .
FROM $BASE_IMAGE

# Install dependencies for Android SDK and Raylib
RUN apt-get update && apt-get install -y \
    openjdk-11-jdk-headless \
    wget \
    unzip \
    cmake \
    ninja-build \
    python3 \
    pkg-config \
    libclang-dev \
    clang \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set up Android SDK
ENV ANDROID_HOME /opt/android-sdk
RUN mkdir -p $ANDROID_HOME && cd $ANDROID_HOME && \
    wget -q https://dl.google.com/android/repository/commandlinetools-linux-8512546_latest.zip -O cmdline-tools.zip && \
    unzip -q cmdline-tools.zip && rm cmdline-tools.zip && \
    mkdir -p cmdline-tools/latest && \
    mv cmdline-tools/bin cmdline-tools/latest/ && \
    mv cmdline-tools/lib cmdline-tools/latest/ && \
    mv cmdline-tools/source.properties cmdline-tools/latest/ && \
    mv cmdline-tools/NOTICE.txt cmdline-tools/latest/

ENV PATH $PATH:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools

# Accept licenses and install NDK/Platforms
RUN yes | sdkmanager --licenses && \
    sdkmanager "platform-tools" "platforms;android-33" "build-tools;33.0.2" "ndk;25.2.9519653"

ENV ANDROID_NDK_HOME $ANDROID_HOME/ndk/25.2.9519653

# Install cargo-apk
RUN cargo install cargo-apk

# Install Rust Android targets
RUN rustup target add aarch64-linux-android armv7-linux-androideabi

WORKDIR /app
EOF
fi

echo "Building Android APK..."
podman run --rm -v "$(pwd):/app:Z" -w /app $IMAGE_NAME /bin/bash -c '
set -e

# We might need to add package metadata to Cargo.toml if not present
if ! grep -q "[package.metadata.android]" Cargo.toml; then
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

# Note: Raylib-rs on Android usually requires specific crate features or android-activity.
# This attempt uses cargo-apk directly.
cargo apk build --release

# Copy the resulting APK to dist
find target/release/apk -name "*.apk" -exec cp {} dist/raylib2048.apk \;
'

echo "Done! If successful, the APK is at dist/raylib2048.apk"
