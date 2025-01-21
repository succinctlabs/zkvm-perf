#!/bin/bash

set -euo pipefail

# Constants
RUSTUP_TOOLCHAIN_NAME="succinct"
ROOT_DIR="$HOME/.sp1"
TOOLCHAINS_DIR="$ROOT_DIR/toolchains"

# Function to get the target triple
get_target() {
    local target=$(rustc -vV | grep "host:" | awk '{print $2}')
    if [[ "$target" == *"-musl" ]]; then
        target="${target/musl/gnu}"
    fi
    echo "$target"
}

# Ensure Rust and Rustup are installed
if ! command -v rustup &>/dev/null; then
    echo "Rust is not installed. Please install Rust from https://rustup.rs/ and try again."
    exit 1
fi

# Prepare directories
mkdir -p "$ROOT_DIR" "$TOOLCHAINS_DIR"

# Get target triple
TARGET=$(get_target)
TOOLCHAIN_ASSET_NAME="rust-toolchain-${TARGET}.tar.gz"
TOOLCHAIN_ARCHIVE_PATH="$ROOT_DIR/$TOOLCHAIN_ASSET_NAME"
TOOLCHAIN_URL="https://github.com/succinctlabs/rust/releases/download/v1.82.0/$TOOLCHAIN_ASSET_NAME"

# Download the toolchain
echo "Downloading toolchain from $TOOLCHAIN_URL..."
curl -fSL "$TOOLCHAIN_URL" -o "$TOOLCHAIN_ARCHIVE_PATH"

# Unpack the toolchain
NEW_TOOLCHAIN_DIR="$TOOLCHAINS_DIR/1.82.0"
mkdir -p "$NEW_TOOLCHAIN_DIR"
echo "Unpacking toolchain to $NEW_TOOLCHAIN_DIR..."
tar -xzf "$TOOLCHAIN_ARCHIVE_PATH" -C "$NEW_TOOLCHAIN_DIR" --strip-components=1

# Link the toolchain to rustup
echo "Linking toolchain to rustup as $RUSTUP_TOOLCHAIN_NAME..."
rustup toolchain link "$RUSTUP_TOOLCHAIN_NAME" "$NEW_TOOLCHAIN_DIR"

# Ensure permissions for binaries
BIN_DIR="$NEW_TOOLCHAIN_DIR/bin"
RUSTLIB_BIN_DIR="$NEW_TOOLCHAIN_DIR/lib/rustlib/$TARGET/bin"

for DIR in "$BIN_DIR" "$RUSTLIB_BIN_DIR"; do
    if [[ -d "$DIR" ]]; then
        chmod -R 755 "$DIR"
    fi
done

echo "Toolchain installed and linked successfully."
