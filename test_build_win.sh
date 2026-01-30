#!/bin/bash
set -e

IMAGE_NAME="raylib2048-win-build-env"
BASE_IMAGE="docker.io/library/rust:bullseye"

echo "Creating Containerfile for Windows Cross-Compilation..."
cat > Containerfile.win <<EOF
FROM $BASE_IMAGE

# Install MinGW-w64 for cross-compiling and zip for packaging
RUN apt-get update && \
    apt-get install -y mingw-w64 zip cmake clang libclang-dev && \
    rm -rf /var/lib/apt/lists/*

# Add the Windows Rust target
RUN rustup target add x86_64-pc-windows-gnu

WORKDIR /app
EOF

echo "Building cross-compilation image: $IMAGE_NAME..."
podman build -t $IMAGE_NAME -f Containerfile.win .

echo "Running Windows build in container..."
# Mount current directory
# Run the build script
# Using :Z for SELinux (common on Fedora/RHEL/CentOS)
podman run --rm -v "$(pwd):/app:Z" -w /app $IMAGE_NAME /bin/bash -c "
    chmod +x build_win.sh
    ./build_win.sh
"

echo "Build complete. Check for dist/raylib2048-windows-x86_64.zip"
rm Containerfile.win
