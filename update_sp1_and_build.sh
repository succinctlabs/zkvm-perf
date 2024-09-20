#!/bin/bash

set -e

# Get the SP1 revision or branch from environment variable or use default
SP1_REF=${SP1_REF:-"2e8b0a8"}
echo "Using SP1_REF: $SP1_REF"

# Check if we should run the build
RUN_BUILD=${RUN_BUILD:-"false"}

# Path to the Cargo.toml file
CARGO_TOML="eval/Cargo.toml"

# Create a backup of the original file
cp "$CARGO_TOML" "$CARGO_TOML.bak"

# List of SP1 dependencies to update
SP1_DEPS=("sp1-prover" "sp1-core-executor" "sp1-core-machine" "sp1-cuda" "sp1-stark")

# Determine the sed in-place flag based on the OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    SED_IN_PLACE=(-i "")
else
    SED_IN_PLACE=(-i)
fi

# Update the Cargo.toml file
if [[ $SP1_REF =~ ^[0-9a-f]{7,40}$ ]]; then
    # It's a commit hash
    for dep in "${SP1_DEPS[@]}"; do
        sed "${SED_IN_PLACE[@]}" "s|^$dep = { git = \"https://github.com/succinctlabs/sp1.*branch = \"[^\"]*\".*$|$dep = { git = \"https://github.com/succinctlabs/sp1\", rev = \"$SP1_REF\" }|" "$CARGO_TOML"
        sed "${SED_IN_PLACE[@]}" "s|^$dep = { git = \"https://github.com/succinctlabs/sp1.*rev = \"[^\"]*\".*$|$dep = { git = \"https://github.com/succinctlabs/sp1\", rev = \"$SP1_REF\" }|" "$CARGO_TOML"
    done
    echo "Updated Cargo.toml with new SP1 commit hash"
else
    # It's a branch name
    for dep in "${SP1_DEPS[@]}"; do
        sed "${SED_IN_PLACE[@]}" "s|^$dep = { git = \"https://github.com/succinctlabs/sp1.*branch = \"[^\"]*\".*$|$dep = { git = \"https://github.com/succinctlabs/sp1\", branch = \"$SP1_REF\" }|" "$CARGO_TOML"
        sed "${SED_IN_PLACE[@]}" "s|^$dep = { git = \"https://github.com/succinctlabs/sp1.*rev = \"[^\"]*\".*$|$dep = { git = \"https://github.com/succinctlabs/sp1\", branch = \"$SP1_REF\" }|" "$CARGO_TOML"
    done
    echo "Updated Cargo.toml with new SP1 branch"
fi

# Show the full diff
echo "Full diff of Cargo.toml changes:"
diff -u "$CARGO_TOML.bak" "$CARGO_TOML" || true

# Remove the backup file
rm "$CARGO_TOML.bak"

# Run cargo build only if RUN_BUILD is set to "true"
if [ "$RUN_BUILD" = "true" ]; then
    echo "Running cargo build..."
    cargo build
    echo "Build process completed"
else
    echo "Skipping build process"
fi