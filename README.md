# zkVM benchmarks

A benchmarking suite for zkVMs.

## Setup

Install Rust:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly
```

You might need to do `sudo sh` if the first command doesn't work.

Install the Succinct toolchain which is used for building the ELF files: https://succinctlabs.github.io/sp1/getting-started/install.html.

```
curl -L https://sp1.succinct.xyz | bash
source /home/ubuntu/.bashrc
sp1up
cargo prove --version
```

Make sure that `go` is installed on your system, to run the gnark wrapper.

Install the `jolt` [toolchain](https://jolt.a16zcrypto.com/usage/quickstart.html).

```
cargo install --git https://github.com/a16z/jolt --rev 845d39af373de078ee2616cf36a255f36f38334a --force --bins jolt
jolt install-toolchain
```

You might have to install these libraries for JOLT:

```
sudo apt-get update
sudo apt-get install libc6
```

Install the Risc0 toolchain:

```
cargo install cargo-binstall
cargo binstall cargo-risczero
cargo risczero install
```

Install docker (needed for running R0's STARK -> SNARK):

- https://docs.docker.com/engine/install/ubuntu/

Then, make sure you can run docker not in sudo mode:
https://docs.docker.com/engine/install/linux-postinstall/

**Note:** When benchmarking, make sure that you run 1 round of Fibonacci (or some other small program) to download the R0 docker image to make sure it doesn't contribute to benchmarking time.

**Note:** On Ubuntu 22.04, you might need to install libssl1.0 to use the Risc0 toolchain. You can follow these instructions (from [here](https://stackoverflow.com/questions/72133316/libssl-so-1-1-cannot-open-shared-object-file-no-such-file-or-directory/73604364#73604364)).

## Machine Requirements

We ran our benchmarks on a AWS r7i.16xlarge machine with 64 vCPUs and 512 GB of memory. Smaller programs require significantly less memory. Note that the machine has to support AVX-512 instructions, otherwise the command in `eval.sh` will not run. If your machine doesn't support AVX-512, then modify this command to remove those flags.

## Usage

You can conduct a sweep of the benchmarks by running either of the following commands:

```
python3 sweep.py
```

or running the following command for the JOLT sweep:

```
python3 sweep_jolt.py
```

To run a single benchmark, you can run:

```
./eval.sh (loop|fibonacci|ssz_withdrawals|tendermint) (sp1|risc0|jolt-zkvm) (poseidon|sha256|blake3|...)
```

Note that right now only poseidion is supported for all zkVMs, since we are interested in also profiling recursion.

Example SP1:

```
./eval.sh fibonacci sp1 poseidon 22 benchmark
```

Example JOLT:

```
./eval.sh fibonacci jolt-zkvm poseidon 22 benchmark
```

Example Risc0:

```
./eval.sh fibonacci risc0 poseidon 22 benchmark
```

## Debugging

Because Risc0 uses C++ for their prover, you may need to install the C++ compiler and libraries.

For Ubuntu/Debian-based systems:

```
# Update system repositories and installed packages
sudo apt update
sudo apt upgrade

# Install the latest version of GCC and libraries
sudo apt install build-essential
```

For CentOS/RHEL-based systems:

```
# Update system repositories and installed packages
sudo yum update

# Install the Development Tools group which includes GCC
sudo yum groupinstall "Development Tools"

# Install or update g++
sudo yum install -y gcc-c++
```

For Fedora:

```
# Update system repositories and installed packages
sudo dnf update

# Install the Development Tools group which includes GCC
sudo dnf groupinstall "Development Tools"
```
