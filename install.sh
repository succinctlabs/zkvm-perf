#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Helper function to print an error message and exit.
error_exit() {
    echo "Error: $1 failed with exit code $?. Exiting."
    exit 1
}

# Install Rust and the nightly toolchain
echo 1 | curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh || error_exit "Installing Rust"
source $HOME/.cargo/env
yes |rustup install nightly || error_exit "Installing nightly toolchain"

# Install the Succinct toolchain
curl -L https://sp1.succinct.xyz | bash || error_exit "Installing Succinct toolchain"
sp1up || error_exit "Updating Succinct toolchain"
cargo prove --version || error_exit "Checking cargo prove version"

# Install the jolt toolchain
yes | cargo +nightly install --git https://github.com/a16z/jolt --force --bins jolt || error_exit "Installing jolt toolchain"
yes | jolt install-toolchain || error_exit "Installing jolt runtime"

# Install the Risc0 toolchain
yes | cargo install cargo-binstall || error_exit "Installing cargo-binstall"
yes | cargo binstall cargo-risczero || error_exit "Installing cargo-risczero"
yes | cargo risczero install || error_exit "Installing Risc0 toolchain"

echo "All installations completed successfully."