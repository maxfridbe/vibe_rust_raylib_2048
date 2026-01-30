#!/bin/bash
set -e

# Ensure output directory exists
mkdir -p dist

echo "Starting Podman container for build..."

# Check if podman exists
if ! command -v podman &> /dev/null; then
    echo "Podman could not be found. Please install podman."
    exit 1
fi

podman run --rm --cgroup-manager=cgroupfs --network host -v $(pwd):/app -w /app docker.io/library/rust:bullseye /bin/bash -c '
set -e
echo "Updating apt..."
apt-get update > /dev/null
echo "Installing dependencies..."
apt-get install -y cmake clang libclang-dev libasound2-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev libglu1-mesa-dev libxcursor-dev libxinerama-dev rpm wget file desktop-file-utils > /dev/null

echo "Installing Cargo tools (this may take a while)..."
cargo install cargo-deb
cargo install cargo-generate-rpm

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
./squashfs-root/AppRun --appdir AppDir --output appimage --desktop-file AppDir/usr/share/applications/raylib2048.desktop --icon-file AppDir/usr/share/icons/hicolor/256x256/apps/raylib2048.png

mv *.AppImage dist/

echo "Builds complete. Artifacts are in dist/ folder."
'
