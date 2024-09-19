# SP1 Benchmarks

A suite of benchmarks to evaluate SP1's performance.

## Setup

1. Install Rust:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly
```

2. Install the [SP1 toolchain](https://docs.succinct.xyz/getting-started/install.html):

```sh
curl -L https://sp1.succinct.xyz | bash
source ~/.bashrc
sp1up
cargo prove --version
```

3. Install the [Risc0 toolchain](https://dev.risczero.com/api/zkvm/install):

```sh
curl -L https://risczero.com/install | bash
source ~/.bashrc
rzup
cargo risczero --version
```

4. Install [Docker](https://docs.docker.com/engine/install/ubuntu/).

5. If using NVIDIA GPUs, install the [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html).

**Note:** Run one round of a small program (e.g., Fibonacci) to download the R0 docker image before benchmarking to avoid affecting benchmark times.

**Note:** On Ubuntu 22.04, you might need to install libssl1.0 for the Risc0 toolchain. Follow these [instructions](https://stackoverflow.com/questions/72133316/libssl-so-1-1-cannot-open-shared-object-file-no-such-file-or-directory/73604364#73604364).

## Usage

### Running a Sweep

To conduct a sweep of the benchmarks:

```sh
python3 sweep.py [options]
```

Available options:
- `--filename`: Filename for the benchmark (default: "benchmark")
- `--trials`: Number of trials to run (default: 1)
- `--programs`: List of programs to benchmark (choices: loop, fibonacci, tendermint, reth1, reth2)
- `--provers`: List of provers to use (choices: sp1, risc0)
- `--hashfns`: List of hash functions to use (currently only poseidon is supported)
- `--shard-sizes`: List of shard sizes to use
- `--block-1`: Block number for reth1 (default: "17106222")
- `--block-2`: Block number for reth2 (default: "19409768")

### Running a Single Benchmark

To run a single benchmark:

```sh
./eval.sh <program> <prover> <hashfn> <shard_size> <filename> [block_number]
```

Examples:

```sh
./eval.sh fibonacci sp1 poseidon 22 benchmark
./eval.sh fibonacci jolt-zkvm poseidon 22 benchmark
./eval.sh fibonacci risc0 poseidon 22 benchmark
./eval.sh reth sp1 poseidon 22 benchmark 19409768
```

Certainly. I'll add a note about authorization for the GitHub Action. Here's the updated section on Adhoc Performance Tests:

## Adhoc Performance Tests

You can run adhoc performance tests using the GitHub Actions workflow defined in `adhoc.yaml`. This workflow allows you to customize various parameters for your benchmarks, including the SP1 reference version.

**Note:** To run this GitHub Action, you need to be authorized on the original repository. If you're not authorized, you can fork the repository and run it on your own infrastructure by setting up the necessary secrets and modifying the workflow as needed.

To run an adhoc test (if authorized):

1. Go to the "Actions" tab in your GitHub repository.
2. Select the "Execute ZKVM-Perf" workflow.
3. Click "Run workflow".
4. Fill in the following parameters:
   - Instance type (e.g., g6.16xlarge, r7i.16xlarge)
   - Enable GPU usage (true/false)
   - AMI ID
   - Provers to use (sp1, risc0)
   - Programs to benchmark (loop, fibonacci, tendermint, reth1, reth2)
   - Filename for the benchmark
   - Number of trials to run
   - Hash functions to use (currently only poseidon)
   - Shard sizes to use
   - SP1 reference (commit hash or branch name) - This allows you to specify which version of SP1 to use for the benchmarks

The workflow will start an EC2 instance with the specified configuration, update the SP1 reference in the Cargo.toml file, run the benchmarks, and then stop the instance. Results will be available in the workflow logs.

If you're running this on your own infrastructure:

1. Fork the repository to your own GitHub account.
2. Set up the necessary secrets in your forked repository's settings:
   - AWS_ACCESS_KEY_ID
   - AWS_SECRET_ACCESS_KEY
   - AWS_REGION
   - AWS_SUBNET_ID
   - AWS_SG_ID
   - GH_PAT (a GitHub Personal Access Token with necessary permissions)
3. Modify the `adhoc.yaml` workflow file if needed to fit your infrastructure setup.
4. Follow the steps above to run the workflow from your forked repository's Actions tab.

Remember to manage your AWS resources responsibly and ensure you understand the costs associated with running EC2 instances for benchmarking.

### SP1 Reference Update

The workflow uses a script (`update_sp1_and_build.sh`) to update the SP1 reference in the `eval/Cargo.toml` file before running the benchmarks. This allows you to easily test different versions of SP1 without manually editing the Cargo.toml file.

When you specify an SP1 reference in the workflow input:
- If it's a commit hash, the script will update the `rev` field for SP1 dependencies.
- If it's a branch name, the script will update the `branch` field for SP1 dependencies.

This update is performed both in the GitHub Actions environment and in the Docker container used for benchmarking, ensuring consistency across the entire benchmark process.

## Common Issues

For C++ compiler and library issues:

Ubuntu/Debian:
```sh
sudo apt update && sudo apt upgrade
sudo apt install build-essential libc6
```

CentOS/RHEL:
```sh
sudo yum update
sudo yum groupinstall "Development Tools"
sudo yum install -y gcc-c++
```

Fedora:
```sh
sudo dnf update
sudo dnf groupinstall "Development Tools"
```

Setting up NVIDIA:
```sh
sudo nvidia-ctk runtime configure --runtime=docker
sudo systemctl restart docker
```