#!/bin/bash
set -e
echo "Running $1, $2, $3, $4, $5"

# Get program directory name as $1 and append "-$2" to it if $1 == "tendermint"
if [ "$1" = "tendermint" ] || [ "$1" = "reth" ]; then
    program_directory="${1}-$2"
else
    program_directory="$1"
fi
# If program_directory starts with loop, then set it to loop
if [[ $program_directory == loop* ]]; then
    program_directory="loop"
fi
if [[ $program_directory == fibonacci* ]]; then
    program_directory="fibonacci"
fi
if [[ $program_directory == sha256* ]]; then
    program_directory="sha256-$2"
fi
if [[ $program_directory == keccak256* ]]; then
    program_directory="keccak256-$2"
fi
if [[ $program_directory == rsp* ]]; then
    program_directory="rsp-$2"
fi
if [[ $program_directory == eddsa-verify* ]]; then
    program_directory="eddsa-verify-$2"
fi
if [[ $program_directory == ecdsa-verify* ]]; then
    program_directory="ecdsa-verify-$2"
fi
if [[ $program_directory == helios* ]]; then
    program_directory="helios-$2"
fi
if [[ $program_directory == groth16-proof-verify* ]]; then
    program_directory="groth-$2"
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

# Check for AVX-512 support
if lscpu | grep -q avx512; then
  # If AVX-512 is supported, add the specific features to RUSTFLAGS
  export RUSTFLAGS="-C target-cpu=native -C target-feature=+avx512ifma,+avx512vl"
else
  # If AVX-512 is not supported, just set target-cpu=native
  export RUSTFLAGS="-C target-cpu=native"
fi

# Set the logging level.
export RUST_LOG=debug

# Determine the features based on GPU existence.
if [ "$GPU_EXISTS" = true ]; then
  FEATURES="default"
else
  FEATURES="default"
fi

if [ "$2" == "risc0" ]; then
 if [ "$GPU_EXISTS" = true ]; then 
  FEATURES="risc0, cuda"
 else
 FEATURES="risc0" 
 fi
fi

# Run the benchmark and capture its exit status
CUDA_VISIBLE_DEVICES=0 cargo run \
    -p sp1-benchmarks-eval \
    --release \
    --no-default-features \
    --features "$FEATURES" \
    -- \
    --groth16 \
    --program "$1" \
    --prover "$2" \
    --hashfn "$3" \
    --shard-size "$4" \
    --filename "$5" \
    ${6:+--block-number $6}

exit $?
