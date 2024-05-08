#!/bin/bash
set -e

echo "Running $1, $2, $3, $4, $5"

# Get program directory name as $1 and append "-$2" to it if $1 == "tendermint"
if [ "$1" = "tendermint" ] || [ "$1" = "reth" ]; then
    program_directory="${1}-$2"
else
    program_directory="$1"
fi

echo "Building program"

# cd to program directory computed above
cd "programs/$program_directory"

# If the prover is not jolt-zkvm, then build the program.
if [ "$2" == "sp1" ]; then
    # The reason we don't just use `cargo prove build` from the SP1 CLI is we need to pass a --features ...
    # flag to select between sp1 and risc0.
    RUSTFLAGS="-C passes=loweratomic -C link-arg=-Ttext=0x00200800 -C panic=abort" \
        RUSTUP_TOOLCHAIN=succinct \
        CARGO_BUILD_TARGET=riscv32im-succinct-zkvm-elf \
        cargo build --release --ignore-rust-version --features $2
fi
# If the prover is risc0, then build the program.
if [ "$2" == "risc0" ]; then
    echo "Building Risc0"
    # Use the risc0 toolchain.
    RUSTFLAGS="-C passes=loweratomic -C link-arg=-Ttext=0x00200800 -C panic=abort" \
        RUSTUP_TOOLCHAIN=risc0 \
        CARGO_BUILD_TARGET=riscv32im-risc0-zkvm-elf \
        cargo build --release --ignore-rust-version --features $2
fi


cd ../../eval

echo "Running eval script"

# Runs the eval for the specified program, prover & hash function with conditional compilation based on the feature flag.

RUST_LOG=info RUSTFLAGS='-C target-cpu=native -C target_feature=+avx512ifma,+avx512vl' cargo run -p eval --release --no-default-features --features $2 -- --program $1 --prover $2 --hashfn $3 --shard-size $4 --filename $5

# TODO: conditionally run with avx flags based on the machine's architecture.
#RUST_LOG=info RUSTFLAGS='-C target-cpu=native' cargo run -p eval --release --no-default-features --features $2 -- --program $1 --prover $2 --hashfn $3 --shard-size $4

cd ../

# Get the current commit hash
commit_hash=$(git rev-parse HEAD)

# Write the commit hash to COMMIT_HASH file
echo "$commit_hash" > COMMIT_HASH
