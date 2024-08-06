import os
import subprocess
from itertools import product

filename = "benchmark"
trials = 1
options_program = ['loop', 'fibonacci', 'tendermint', 'reth1', 'reth2']
options_prover = ["sp1", "risc0"]
options_hashfn = ['poseidon']
options_shard_size = [22]
options_block_1 = "17106222"
options_block_2 = "19409768"

option_combinations = product(options_program, options_prover, options_hashfn, options_shard_size)
for program, prover, hashfn, shard_size in option_combinations:
    first_shard_size = options_shard_size[0]
    if prover != 'sp1' and shard_size != first_shard_size:  # Only sp1 supports different shard sizes
        continue
    print(f"Running: {program}, {prover}, {hashfn}, {shard_size}")
    for _ in range(trials):
        if program == "reth1":
            subprocess.run(['bash', 'eval.sh', "reth", prover, hashfn, str(shard_size), filename, options_block_1])
        elif program == "reth2":
            subprocess.run(['bash', 'eval.sh', "reth", prover, hashfn, str(shard_size), filename, options_block_2])
        else:
            subprocess.run(['bash', 'eval.sh', program, prover, hashfn, str(shard_size), filename])
