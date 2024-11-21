use core::time;
use std::{
    env, fs,
    time::{Duration, Instant},
};

use sp1_reth_primitives::SP1RethInput;

use crate::{EvalArgs, ProgramId, ProverId};

pub fn get_elf(args: &EvalArgs) -> String {
    let mut program_dir = args.program.to_string();
    if args.program == ProgramId::Tendermint || args.program == ProgramId::Reth {
        program_dir += "-";
        program_dir += args.prover.to_string().as_str();
    }
    if program_dir.starts_with("loop") {
        program_dir = "loop".to_string();
    }
    if program_dir.starts_with("fibonacci") {
        program_dir = "fibonacci".to_string();
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
    if program_dir.starts_with("rsp") {
        program_dir = "rsp".to_string();
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

pub fn gas_amount(program: &ProgramId) -> Option<u64> {
    let amount = match program {
        ProgramId::Rsp20526626 => 12_121_809,
        ProgramId::Rsp20526627 => 16_515_842,
        ProgramId::Rsp20526628 => 13_311_631,
        ProgramId::Rsp20526629 => 16_995_405,
        ProgramId::Rsp20526630 => 16_936_272,
        ProgramId::Rsp20528708 => 13_218_606,
        ProgramId::Rsp20528709 => 16_512_503,
        ProgramId::Rsp20528710 => 11_570_190,
        ProgramId::Rsp20528711 => 12_942_861,
        ProgramId::Rsp20528712 => 14_753_755,
        _ => return None,
    };

    Some(amount)
}

/// The hashes per second are given by the block size of the hash function
pub fn hashes_per_second(program: &ProgramId, core_compress_duration: Duration) -> Option<f64> {
    let word_size_bytes: u64 = match program {
        ProgramId::Sha256100kb
        | ProgramId::Sha256300kb
        | ProgramId::Sha2561mb
        | ProgramId::Sha2563mb
        | ProgramId::Sha25610mb => 64,
        ProgramId::Keccak256100kb
        | ProgramId::Keccak256300kb
        | ProgramId::Keccak2561mb
        | ProgramId::Keccak2563mb
        | ProgramId::Keccak25610mb => 136,
        _ => return None,
    };

    let num_of_bytes = hash_input_size_bytes(program).expect("We should have an input size if we have a word size");
    let hashes_total = num_of_bytes / word_size_bytes;

    let duration = core_compress_duration.as_secs_f64();
    
    Some(hashes_total as f64 / duration)
}

/// The number of bytes hashed per second
///
/// Given by dividing the number of bytes hashed by the duration of the proving
pub fn hash_bytes_per_second(program: &ProgramId, core_compress_duration: Duration) -> Option<f64> {
    let num_of_bytes = hash_input_size_bytes(program)?;

    let num_of_bytes = num_of_bytes as f64;

    let duration = core_compress_duration.as_secs_f64();

    Some(num_of_bytes / duration)
}

/// The number of bytes we pass in as input for each program type
pub fn hash_input_size_bytes(program: &ProgramId) -> Option<u64> {
    let num_of_bytes = match program {
        ProgramId::Sha256100kb => 102400,
        ProgramId::Sha256300kb => 102400 * 3,
        ProgramId::Sha2561mb => 1048576,
        ProgramId::Sha2563mb => 1048576 * 3,
        ProgramId::Sha25610mb => 1048576 * 10,
        ProgramId::Keccak256100kb => 102400,
        ProgramId::Keccak256300kb => 102400 * 3,
        ProgramId::Keccak2561mb => 1048576,
        ProgramId::Keccak2563mb => 1048576 * 3,
        ProgramId::Keccak25610mb => 1048576 * 10,
        _ => return None,
    };

    Some(num_of_bytes)
}

// for now just for RSP
//pub fn raw_input(program: &ProgramId) -> Option<&[u8]> {
//    let raw = match program {
//        ProgramId::Rsp20526626 => {
//            include_bytes!("../../fixtures/20526626.bin")
//        }
//        ProgramId::Rsp20526627 => {
//            include_bytes!("../../fixtures/20526627.bin")
//        }
//        ProgramId::Rsp20526628 => {
//            include_bytes!("../../fixtures/20526628.bin")
//        }
//        ProgramId::Rsp20526629 => {
//            include_bytes!("../../fixtures/20526629.bin")
//        }
//        ProgramId::Rsp20526630 => {
//            include_bytes!("../../fixtures/20526630.bin")
//        }
//        ProgramId::Rsp20528708 => {
//            include_bytes!("../../fixtures/20528708.bin")
//        }
//        ProgramId::Rsp20528709 => {
//            include_bytes!("../../fixtures/20528709.bin")
//        }
//        ProgramId::Rsp20528710 => {
//            include_bytes!("../../fixtures/20528710.bin")
//        }
//        ProgramId::Rsp20528711 => {
//            include_bytes!("../../fixtures/20528711.bin")
//        }
//        ProgramId::Rsp20528712 => {
//            include_bytes!("../../fixtures/20528712.bin")
//        },
//        _ => return None,
//    };
//
//    Some(raw)
//}
