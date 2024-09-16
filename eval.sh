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

# If the prover is sp1, then build the program.
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

cd ../../

echo "Running eval script"

# Detect whether we're on an instance with a GPU.
if nvidia-smi > /dev/null 2>&1; then
  GPU_EXISTS=true
else
  GPU_EXISTS=false
fi

# Set the compilation flags.
# if [ "$GPU_EXISTS" = false ]; then
#   export RUSTFLAGS='-C target-cpu=native'
# fi

# Set the logging level.
export RUST_LOG=info

# Determine the features based on GPU existence.
if [ "$GPU_EXISTS" = true ]; then
  FEATURES="cuda"
else
  FEATURES="default"
fi

# Run the benchmark.
cargo run \
    -p sp1-benchmarks-eval \
    --release \
    --no-default-features \
    --features $FEATURES \
    -- \
    --program $1 \
    --prover $2 \
    --hashfn $3 \
    --shard-size $4 \
    --filename $5 \
    ${6:+--block-number $6}