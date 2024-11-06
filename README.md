# ZKVM-Perf And You 🫵🏻

Welcome to ZKVM-Perf, a powerful benchmarking tool for ZKVM implementations. This guide covers both automated workflow-based benchmarking and manual benchmarking processes.

## Automated Benchmarking

### Quick Start Guide

1. Go to the [Actions tab](https://github.com/succinctlabs/zkvm-perf/actions) in the ZKVM-Perf repository.
2. Click on the "Execute ZKVM-Perf (Matrix)" workflow.
3. Click the "Run workflow" button.
4. (Optional) Customize parameters or use defaults.
5. Click the green "Run workflow" button at the bottom.
6. Wait for all matrix jobs to complete.
7. Check individual job results and download CSV artifacts for detailed analysis.

### Workflow Details

The benchmarking process is split into two main workflows:

1. `adhoc-matrix.yml`: Orchestrates the overall benchmarking process.
2. `run-on-runner.yml`: Executes the actual benchmarks on EC2 instances.

#### adhoc-matrix.yml

This workflow sets up the benchmarking environment and triggers individual benchmark runs.

##### Inputs

- `provers`: Provers to use (comma-separated, default: 'sp1')
- `programs`: Programs to benchmark (comma-separated, default: 'loop10k, loop100k, loop1m, loop3m, loop10m, loop30m, loop100m, fibonacci,tendermint,reth1,reth2')
- `filename`: Filename for the benchmark (default: 'benchmark')
- `trials`: Number of trials to run (default: '1')
- `sp1_ref`: SP1 reference (commit hash or branch name, default: 'dev')
- `additional_params`: Additional parameters as JSON (default: '{"hashfns":"poseidon","shard_sizes":"22"}')

##### Matrix Strategy

The workflow runs benchmarks on two types of EC2 instances:

- GPU: g6.16xlarge
- CPU: r7i.16xlarge

#### run-on-runner.yml

This workflow is triggered by `adhoc-matrix.yml` and runs the actual benchmarks on the specified EC2 instance.

##### Key Steps

1. Sets up the Docker environment.
2. Builds the Docker image with the specified SP1 reference.
3. Runs the benchmark using the `sweep.py` script.
4. Uploads the benchmark results as artifacts.

### Running Automated Benchmarks

1. **Navigate to the Actions Tab**
   Go to the [Actions tab](https://github.com/succinctlabs/zkvm-perf/actions) in the repository.

2. **Select the Workflow**
   Click on "Execute ZKVM-Perf (Matrix)".

3. **Configure the Run**

   - You can use the default settings for a quick start.
   - Customize inputs as needed.

4. **Start the Benchmark**
   Click "Run workflow".

5. **Monitor Progress**

   - The workflow will start two jobs: one for GPU and one for CPU.
   - Each job will trigger a separate `run-on-runner` workflow.

6. **Access Results**
   - Once complete, each job will upload its results as an artifact.
   - Download the artifacts to analyze the benchmark data.
   - A combined results file will also be available.

## Manual Benchmarking

### Setup

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
   rzup install
   cargo risczero --version
   ```

4. Install [Docker](https://docs.docker.com/engine/install/ubuntu/).

5. If using NVIDIA GPUs, install the [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html).

**Note:** Run one round of a small program (e.g., Fibonacci) to download the R0 docker image before benchmarking to avoid affecting benchmark times.

**Note:** On Ubuntu 22.04, you might need to install libssl1.0 for the Risc0 toolchain. Follow these [instructions](https://stackoverflow.com/questions/72133316/libssl-so-1-1-cannot-open-shared-object-file-no-such-file-or-directory/73604364#73604364).

### Running a Manual Sweep

To conduct a sweep of the benchmarks:

```sh
python3 sweep.py [options]
```

Available options:

- `--filename`: Filename for the benchmark (default: "benchmark")
- `--trials`: Number of trials to run (default: 1)
- `--programs`: List of programs to benchmark (choices: loop10k, loop100k, loop1m, loop3m, loop10m, loop30m, loop100m, fibonacci, tendermint, reth1, reth2)
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

```
./eval.sh fibonacci sp1 poseidon 22 benchmark
./eval.sh fibonacci jolt-zkvm poseidon 22 benchmark
./eval.sh fibonacci risc0 poseidon 22 benchmark
./eval.sh reth sp1 poseidon 22 benchmark 19409768
```

## Analyzing Results

- Each benchmark run produces a CSV file with detailed performance metrics.
- The CSV includes the instance type, allowing for easy comparison between GPU and CPU performance.
- Use the combined results file for a comprehensive view of all benchmarks.

## Troubleshooting

If you encounter issues:

1. Check the logs of both the matrix job and the individual runner jobs.
2. Ensure your AWS credentials and permissions are correctly set up.
3. Verify that the SP1 reference is valid and accessible.
4. For GPU jobs, confirm that GPU support is properly configured in the EC2 instance.

### Common Issues

For C++ compiler and library issues:

Ubuntu/Debian:

```sh
sudo apt update && sudo apt upgrade
sudo apt install build-essential libc6
```

CentOS/RHEL:

```
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

## Contributing

We welcome contributions to improve ZKVM-Perf! If you encounter issues or have suggestions:

1. Check existing issues in the repository.
2. If your issue is new, create a detailed bug report or feature request.
3. For code contributions, please submit a pull request with a clear description of your changes.

Happy benchmarking! 🚀
