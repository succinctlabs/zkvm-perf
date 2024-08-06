# SP1 Benchmarks

A suite of benchmarks to evaluate SP1's performance.

## Setup

Install Rust:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly
```

Install the [SP1 toolchain](https://docs.succinct.xyz/getting-started/install.html):

```sh
curl -L https://sp1.succinct.xyz | bash
source ~/.bashrc
sp1up
cargo prove --version
```

Install the [Risc0 toolchain](https://dev.risczero.com/api/zkvm/install):

```sh
curl -L https://risczero.com/install | bash
source ~/.bashrc
rzup
cargo risczero --version
```

Install [Docker](https://docs.docker.com/engine/install/ubuntu/).
* https://docs.docker.com/engine/install/ubuntu/
* https://docs.docker.com/engine/install/linux-postinstall/

Install the [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html) if you are using NVIDIA GPUs.
- https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html

**Note:** When benchmarking, make sure that you run 1 round of Fibonacci (or some other small program) to download the R0 docker image to make sure it doesn't contribute to benchmarking time.

**Note:** On Ubuntu 22.04, you might need to install libssl1.0 to use the Risc0 toolchain. You can follow these instructions (from [here](https://stackoverflow.com/questions/72133316/libssl-so-1-1-cannot-open-shared-object-file-no-such-file-or-directory/73604364#73604364)).

## Usage

You can conduct a sweep of the benchmarks by running either of the following commands:

```sh
python3 sweep.py
```

To run a single benchmark, you can run:

```sh
./eval.sh (loop|fibonacci|ssz_withdrawals|tendermint) (sp1|risc0|jolt-zkvm) (poseidon|sha256|blake3|...)
```

Note that right now only poseidion is supported for all zkVMs, since we are interested in also profiling recursion.

Example SP1:

```sh
./eval.sh fibonacci sp1 poseidon 22 benchmark
```

Example JOLT:

```sh
./eval.sh fibonacci jolt-zkvm poseidon 22 benchmark
```

Example Risc0:

```sh
./eval.sh fibonacci risc0 poseidon 22 benchmark
```

Note for benchmarking the Reth program, you must also pass in a block number:

```sh
./eval.sh reth sp1 poseidon 22 benchmark 19409768
```

The inputs for these blocks have already been generated [here](./eval/cli/blocks/). You can add more
blocks by using the [SP1-Reth](https://github.com/succinctlabs/sp1-reth) script.

## Common Issues

Because Risc0 uses C++ for their prover, you may need to install the C++ compiler and libraries.

For Ubuntu/Debian-based systems:

```sh
# Update system repositories and installed packages
sudo apt update
sudo apt upgrade

# Install the latest version of GCC and libraries
sudo apt install build-essential libc6
```

For CentOS/RHEL-based systems:

```sh
# Update system repositories and installed packages
sudo yum update

# Install the Development Tools group which includes GCC
sudo yum groupinstall "Development Tools"

# Install or update g++
sudo yum install -y gcc-c++
```

For Fedora:

```sh
# Update system repositories and installed packages
sudo dnf update

# Install the Development Tools group which includes GCC
sudo dnf groupinstall "Development Tools"
```

Setting up NVIDIA:

```sh
sudo nvidia-ctk runtime configure --runtime=docker
sudo systemctl restart docker
```