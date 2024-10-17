import argparse
import subprocess
from itertools import product


def run_benchmark(
    filename, trials, programs, provers, hashfns, shard_sizes, block_1, block_2
):
    option_combinations = product(programs, provers, hashfns, shard_sizes)
    for program, prover, hashfn, shard_size in option_combinations:
        first_shard_size = shard_sizes[0]
        if (
            prover != "sp1" and shard_size != first_shard_size
        ):  # Only sp1 supports different shard sizes
            continue
        print(f"Running: {program}, {prover}, {hashfn}, {shard_size}")
        for _ in range(trials):
            if program == "reth1":
                subprocess.run(
                    [
                        "bash",
                        "eval.sh",
                        "reth",
                        prover,
                        hashfn,
                        str(shard_size),
                        filename,
                        block_1,
                    ]
                )
            elif program == "reth2":
                subprocess.run(
                    [
                        "bash",
                        "eval.sh",
                        "reth",
                        prover,
                        hashfn,
                        str(shard_size),
                        filename,
                        block_2,
                    ]
                )
            else:
                subprocess.run(
                    [
                        "bash",
                        "eval.sh",
                        program,
                        prover,
                        hashfn,
                        str(shard_size),
                        filename,
                    ]
                )


def main():
    parser = argparse.ArgumentParser(
        description="Run benchmarks with various combinations of options."
    )
    parser.add_argument(
        "--filename", default="benchmark", help="Filename for the benchmark"
    )
    parser.add_argument("--trials", type=int, default=1, help="Number of trials to run")
    parser.add_argument(
        "--programs",
        nargs="+",
        default=[
            "loop10k",
            "loop100k",
            "loop1m",
            "loop3m",
            "loop10m",
            "loop30m",
            "loop100m",
            "fibonacci",
            "tendermint",
            "reth1",
            "reth2",
        ],
        help="List of programs to benchmark",
        choices=[
            "loop10k",
            "loop100k",
            "loop1m",
            "loop3m",
            "loop10m",
            "loop30m",
            "loop100m",
            "fibonacci",
            "tendermint",
            "reth1",
            "reth2",
        ],
    )
    parser.add_argument(
        "--provers",
        nargs="+",
        default=["sp1"],
        help="List of provers to use",
        choices=["sp1", "risc0"],
    )
    parser.add_argument(
        "--hashfns",
        nargs="+",
        default=["poseidon"],
        help="List of hash functions to use",
        choices=["poseidon"],
    )
    parser.add_argument(
        "--shard-sizes",
        type=int,
        nargs="+",
        default=[21],
        help="List of shard sizes to use",
    )
    parser.add_argument("--block-1", default="17106222", help="Block number for reth1")
    parser.add_argument("--block-2", default="19409768", help="Block number for reth2")

    args = parser.parse_args()

    run_benchmark(
        args.filename,
        args.trials,
        args.programs,
        args.provers,
        args.hashfns,
        args.shard_sizes,
        args.block_1,
        args.block_2,
    )


if __name__ == "__main__":
    main()
