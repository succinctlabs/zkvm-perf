use core::time;
use std::{env, fs, time::Instant};

use sp1_reth_primitives::SP1RethInput;

use crate::{EvalArgs, ProgramId, ProverId};

pub fn get_elf(args: &EvalArgs) -> String {
    let mut program_dir = args.program.to_string();
    if args.program == ProgramId::Tendermint ||
        args.program == ProgramId::Reth
    {
        program_dir += "-";
        program_dir += args.prover.to_string().as_str();
    }
    if program_dir.starts_with("loop") {
        program_dir = "loop".to_string();
    }
    if program_dir.starts_with("sha256") {
        program_dir = "sha256".to_string();
        program_dir += "-";
        program_dir += args.prover.to_string().as_str();
    }
    if program_dir.starts_with("keccak256") {
        program_dir = "keccak256".to_string();
        program_dir += "-";
        program_dir += args.prover.to_string().as_str();
    }

    let current_dir = env::current_dir().expect("Failed to get current working directory");

    let mut elf_path = current_dir.join(format!(
        "programs/{}/target/riscv32im-succinct-zkvm-elf/release/{}",
        program_dir, program_dir
    ));

    if args.prover == ProverId::Risc0 {
        elf_path = current_dir.join(format!(
            "programs/{}/target/riscv32im-risc0-zkvm-elf/release/{}",
            program_dir, program_dir
        ));
    }

    let elf_path_str = elf_path.to_str().expect("Failed to convert path to string").to_string();
    println!("elf path: {}", elf_path_str);
    elf_path_str
}

pub fn get_reth_input(args: &EvalArgs) -> SP1RethInput {
    if let Some(block_number) = args.block_number {
        let current_dir = env::current_dir().expect("Failed to get current working directory");

        let blocks_dir = current_dir.join("eval").join("blocks");

        let file_path = blocks_dir.join(format!("{}.bin", block_number));

        if let Ok(bytes) = fs::read(file_path) {
            bincode::deserialize(&bytes).expect("Unable to deserialize input")
        } else {
            let blocks: Vec<String> = fs::read_dir(&blocks_dir)
                .unwrap_or_else(|_| panic!("Failed to read blocks directory: {:?}", blocks_dir))
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        e.path().file_stem().and_then(|n| n.to_str().map(String::from))
                    })
                })
                .collect();

            panic!(
                "Block {} not supported. Please choose from: {}",
                block_number,
                blocks.join(", ")
            );
        }
    } else {
        panic!("Block number is required for Reth program");
    }
}

pub fn time_operation<T, F: FnOnce() -> T>(operation: F) -> (T, time::Duration) {
    let start = Instant::now();
    let result = operation();
    let duration = start.elapsed();
    (result, duration)
}
