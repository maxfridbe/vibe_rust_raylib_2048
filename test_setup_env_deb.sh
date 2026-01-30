#!/bin/bash
set -e

IMAGE_NAME="raylib2048-build-env"
BASE_IMAGE="debian:stable-slim"
SCRIPT_NAME="setup_env_deb.sh"

echo "Creating Containerfile..."
cat > Containerfile <<EOF
FROM $BASE_IMAGE

# Copy the setup script into the image
COPY $SCRIPT_NAME /tmp/$SCRIPT_NAME

# Make it executable
RUN chmod +x /tmp/$SCRIPT_NAME

# Run the setup script to install dependencies and Rust
# We need to source the env in the same RUN command or add it to the path for future layers/commands
RUN /tmp/$SCRIPT_NAME && \
    rm /tmp/$SCRIPT_NAME

# Add cargo to PATH for future commands
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
EOF

echo "Building build environment image: $IMAGE_NAME..."
podman build -t $IMAGE_NAME -f Containerfile .

echo "Running build test in $IMAGE_NAME..."
# Mount current directory to /app
# Run the build script
podman run --rm -v "$(pwd):/app:Z" -w /app $IMAGE_NAME /bin/bash -c "
    echo 'Inside container...'
    
    echo 'Verifying Rust installation...'
    rustc --version
    cargo --version
    
    echo 'Building project...'
    cargo build --release
    
    echo 'Build successful!'
"

echo "Cleaning up..."
rm Containerfile