# Raylib 2048 (Rust)

A 2048 game clone implemented in Rust using the Raylib library.

## Build Prerequisites

This project relies on `raylib-sys`, which requires several system libraries to build, particularly `clang` and `libclang`.

### Debian/Ubuntu Dependencies

To install the necessary build dependencies on a Debian-based system, you can run the provided setup script:

```bash
sudo ./setup_env_deb.sh
```

This script installs:
*   `build-essential`
*   `curl`, `pkg-config`, `git`
*   `cmake`
*   `clang`, `libclang-dev`
*   Raylib dependencies: `libasound2-dev`, `libx11-dev`, `libxrandr-dev`, `libxi-dev`, `libgl1-mesa-dev`, `libglu1-mesa-dev`, `libxcursor-dev`, `libxinerama-dev`
*   Rust (via `rustup`)

### Build Environment Verification (Podman)

A test script is included to verify the build environment using a clean Debian Podman container. This is useful for ensuring reproducibility and identifying missing dependencies without modifying your host system.

To build the environment image and run the build inside a container:

```bash
./test_setup_env_deb.sh
```

This will:
1.  Create a Docker/Podman image named `raylib2048-build-env` based on `debian:stable-slim`.
2.  Install all dependencies defined in `setup_env_deb.sh` inside the image.
3.  Mount the current directory into a container.
4.  Run `cargo build --release` inside the container to verify the build.

## Building the Project

Once dependencies are installed (either on your host or via the container), you can build the project using the standard Cargo commands or the helper script:

```bash
./build.sh
```

Or manually:

```bash
cargo build --release
```

The binary will be located at `target/release/raylib2048`.

## Libraries Used

*   **[raylib-rs](https://github.com/deltaphc/raylib-rs):** Rust bindings for the Raylib game development library.
*   **[rand](https://crates.io/crates/rand):** For random tile generation.
*   **std:** Standard library for file system and path handling.

## Building for Windows (on Linux)

You can cross-compile the project for Windows from a Linux host using MinGW-w64. Similar to the Linux build, a test script is provided to handle this in an isolated Podman container.

### Windows Build Environment (Podman)

To build the Windows environment image and compile the project:

```bash
./test_build_win.sh
```

This will:
1.  Create a Podman image named `raylib2048-win-build-env`.
2.  Install `mingw-w64`, `zip`, and the Rust Windows target.
3.  Compile the project for Windows (`x86_64-pc-windows-gnu`).
4.  Package the executable, assets, and documentation into `raylib2048-windows-x86_64.zip`.

### Manual Windows Cross-Compilation

If you have `mingw-w64` and the `x86_64-pc-windows-gnu` Rust target installed on your host, you can use the helper script:

```bash
./build_win.sh
```

## Directory Structure

*   `src/`: Source code (`main.rs`, `game.rs`).
*   `assets/`: Game assets (audio).
*   `setup_env_deb.sh`: Script to set up the build environment on Debian/Ubuntu.
*   `test_setup_env_deb.sh`: Script to verify the build environment using Podman.
*   `build.sh`: Helper script for building the release binary.
