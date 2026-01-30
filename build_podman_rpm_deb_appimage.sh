#!/bin/bash
set -e

# Ensure output directory exists
mkdir -p dist

IMAGE_NAME="raylib2048-linux-build-env"

# Check if podman exists
if ! command -v podman &> /dev/null; then
    echo "Podman could not be found. Please install podman."
    exit 1
fi

# Build the build environment image if it doesn't exist
if [[ "$(podman images -q $IMAGE_NAME 2> /dev/null)" == "" ]]; then
    echo "Building build environment image..."
    cat <<EOF | podman build -t $IMAGE_NAME -f - .
FROM docker.io/library/rust:bullseye
RUN apt-get update && \
    apt-get install -y cmake clang libclang-dev libasound2-dev libx11-dev \
    libxrandr-dev libxi-dev libgl1-mesa-dev libglu1-mesa-dev libxcursor-dev \
    libxinerama-dev rpm wget file desktop-file-utils && \
    rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-deb cargo-generate-rpm
EOF
fi

echo "Running build in container..."
podman run --rm -v "$(pwd):/app:Z" -w /app $IMAGE_NAME /bin/bash -c '
set -e

echo "Building project (Release)..."
cargo build --release

echo "Building .deb package..."
cargo deb --output dist/

echo "Building .rpm package..."
cargo generate-rpm
mv target/generate-rpm/*.rpm dist/

echo "Building AppImage..."
# Clean up previous AppDir
rm -rf AppDir

# Create AppDir structure
mkdir -p AppDir/usr/bin
mkdir -p AppDir/usr/share/raylib2048/assets
mkdir -p AppDir/usr/share/icons/hicolor/256x256/apps
mkdir -p AppDir/usr/share/applications

cp target/release/raylib2048 AppDir/usr/bin/
cp -r assets/* AppDir/usr/share/raylib2048/assets/

# Create desktop file
cat > AppDir/usr/share/applications/raylib2048.desktop <<EOF
[Desktop Entry]
Type=Application
Name=Raylib 2048
Exec=raylib2048
Icon=raylib2048
Categories=Game;
Terminal=false
EOF

# Fetch Icon (Using Raylib logo as placeholder)
wget -q -O AppDir/usr/share/icons/hicolor/256x256/apps/raylib2048.png https://raw.githubusercontent.com/raysan5/raylib/master/logo/raylib_256x256.png

# Get LinuxDeploy
if [ ! -f linuxdeploy ]; then
    wget -q -O linuxdeploy https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
    chmod +x linuxdeploy
fi

# Extract linuxdeploy (avoid FUSE issues)
./linuxdeploy --appimage-extract > /dev/null

# Run LinuxDeploy
./squashfs-root/AppRun --appdir AppDir --output appimage \
    --desktop-file AppDir/usr/share/applications/raylib2048.desktop \
    --icon-file AppDir/usr/share/icons/hicolor/256x256/apps/raylib2048.png

mv *.AppImage dist/

echo "Builds complete. Artifacts are in dist/ folder."
'