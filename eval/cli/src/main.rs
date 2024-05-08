use clap::{command, Parser};
use csv::WriterBuilder;
use serde::Serialize;
use std::{
    fs::OpenOptions,
    time::{self, Instant},
};

#[cfg(feature = "jolt-zkvm")]
mod jolt;

#[cfg(feature = "risc0")]
mod risc0;

#[cfg(feature = "sp1")]
mod sp1;

pub trait PerformanceReportGenerator {
    fn get_report(args: &EvalArgs) -> PerformanceReport;
}

/// An identifier used to select the program to evaluate.
#[derive(clap::ValueEnum, Clone, PartialEq)]
#[clap(rename_all = "kebab_case")]
enum ProgramId {
    Loop,
    Fibonacci,
    SSZWithdrawals,
    Tendermint,
    Sha2Chain,
    Reth,
}

impl ProgramId {
    /// Convert the identifier to a string.
    fn to_string(&self) -> String {
        match self {
            ProgramId::Loop => "loop".to_string(),
            ProgramId::Fibonacci => "fibonacci".to_string(),
            ProgramId::SSZWithdrawals => "ssz-withdrawals".to_string(),
            ProgramId::Tendermint => "tendermint".to_string(),
            ProgramId::Sha2Chain => "sha2-chain".to_string(),
            ProgramId::Reth => "reth".to_string(),
        }
    }
}

/// An identifier used to select the prover to evaluate.
#[derive(clap::ValueEnum, Clone, PartialEq)]
/// These should match the feature flags in the Cargo.toml.
enum ProverId {
    Risc0,
    SP1,
    JoltZkvm,
}

impl ProverId {
    /// Convert the identifier to a string.
    fn to_string(&self) -> String {
        match self {
            ProverId::Risc0 => "risc0".to_string(),
            ProverId::SP1 => "sp1".to_string(),
            ProverId::JoltZkvm => "jolt-zkvm".to_string(),
        }
    }
}

/// An identifier used to select the hash function to evaluate.
#[derive(clap::ValueEnum, Clone, PartialEq)]
enum HashFnId {
    Sha256,
    Poseidon,
    Blake3,
    Keccak256,
}

impl HashFnId {
    /// Convert the identifier to a string.
    fn to_string(&self) -> String {
        match self {
            HashFnId::Sha256 => "sha-256".to_string(),
            HashFnId::Poseidon => "poseidon".to_string(),
            HashFnId::Blake3 => "blake3".to_string(),
            HashFnId::Keccak256 => "keccak256".to_string(),
        }
    }
}

/// The performance report of a zkVM on a program.
#[derive(Debug, Serialize, Default)]
pub struct PerformanceReport {
    /// The program that is being evaluated.
    pub program: String,

    /// The prover that is being evaluated.
    pub prover: String,

    /// The hash function that is being evaluated.
    pub hashfn: String,

    /// The shard size that is being evaluated.
    pub shard_size: u64,

    /// The number of shards.
    pub shards: usize,

    /// The reported number of cycles.
    ///
    /// Note that this number may vary based on the zkVM.
    pub cycles: u64,

    /// The reported speed in cycles per second.
    pub speed: f64,

    /// The reported duration of the execution in seconds.
    pub execution_duration: f64,

    /// The reported duration of the prover in seconds.
    pub prove_duration: f64,

    /// The reported duration of the core proving time in seconds.
    pub core_prove_duration: f64,

    /// The reported duration of the verifier in seconds.
    pub core_verify_duration: f64,

    /// The size of the core proof.
    pub core_proof_size: usize,

    /// The reported duration of the recursive proving time in seconds.
    pub recursive_prove_duration: f64,

    /// The reported duration of the verifier in seconds.
    pub recursive_verify_duration: f64,

    /// The size of the recursive proof in bytes.
    pub recursive_proof_size: usize,

    // Only applicable for SP1.
    pub compressed_proof_size: Option<usize>,
    pub compressed_proof_duration: Option<f64>,

    /// The time it takes to do a bn254 compress
    pub bn254_compress_duration: f64,

    /// The size of the bn254 proof in bytes.
    pub bn254_compress_proof_size: usize,

    // Groth16 related fields
    pub groth16_compress_duration: f64,
}

/// The argument passed through the CLI.
#[derive(Parser, Clone)]
#[command(about = "Evaluate the performance of a zkVM on a program.")]
pub struct EvalArgs {
    #[arg(long)]
    program: ProgramId,

    #[arg(long)]
    prover: ProverId,

    #[arg(long)]
    hashfn: HashFnId,

    #[arg(long)]
    pub shard_size: u64,

    #[arg(long)]
    pub filename: String,
}

pub fn get_elf(args: &EvalArgs) -> String {
    // The path to the ELF file that contains the wrapped program compiled for the zkVM.
    let mut program_dir = args.program.to_string();
    if args.program == ProgramId::Tendermint || args.program == ProgramId::Reth {
        program_dir += "-";
        program_dir += args.prover.to_string().as_str();
    }
    let mut elf_path = format!(
        "../programs/{}/target/riscv32im-succinct-zkvm-elf/release/{}",
        program_dir, program_dir
    );
    if args.prover == ProverId::Risc0 {
        elf_path = format!(
            "../programs/{}/target/riscv32im-risc0-zkvm-elf/release/{}",
            program_dir, program_dir
        );
    }

    println!("elf path: {}", elf_path);
    elf_path
}

pub fn time_operation<T, F: FnOnce() -> T>(operation: F) -> (T, time::Duration) {
    let start = Instant::now();
    let result = operation();
    let duration = start.elapsed();
    (result, duration)
}

fn main() {
    let args = EvalArgs::parse();

    // Select the correct implementation based on the prover. To reduce compilation time, we
    // use feature gates.
    let report = match args.prover {
        ProverId::Risc0 => {
            #[cfg(feature = "risc0")]
            {
                risc0::Risc0PerformanceReportGenerator::get_report(&args)
            }
            #[cfg(not(feature = "risc0"))]
            {
                PerformanceReport::default()
            }
        }
        ProverId::SP1 => {
            #[cfg(feature = "sp1")]
            {
                sp1::SP1PerformanceReportGenerator::get_report(&args)
            }
            #[cfg(not(feature = "sp1"))]
            {
                PerformanceReport::default()
            }
        }
        ProverId::JoltZkvm => {
            #[cfg(feature = "jolt-zkvm")]
            {
                jolt::JoltPerformanceReportGenerator::get_report(&args)
            }
            #[cfg(not(feature = "jolt-zkvm"))]
            {
                PerformanceReport::default()
            }
        }
    };

    let filename = format!("{}.csv", args.filename);
    // Open a file for writing the report.
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();

    // Write the row and the header, if needed.
    let mut writer = WriterBuilder::new().from_writer(&file);
    if file.metadata().unwrap().len() == 0 {
        writer
            .write_record(&[
                "program",
                "prover",
                "hashfn",
                "shard_size",
                "shards",
                "cycles",
                "speed",
                "execution_duration",
                "prove_duration",
                "core_prove_duration",
                "core_verify_duration",
                "core_proof_size",
                "recursive_prove_duration",
                "recursive_verify_duration",
                "recursive_proof_size",
                "compressed_proof_size",
                "compressed_proof_duration",
                "bn254_compress_duration",
                "bn254_compress_proof_size",
                "groth16_compress_duration",
            ])
            .unwrap();
    }
    writer
        .serialize(&[
            report.program,
            report.prover,
            report.hashfn,
            report.shard_size.to_string(),
            report.shards.to_string(),
            report.cycles.to_string(),
            report.speed.to_string(),
            report.execution_duration.to_string(),
            report.prove_duration.to_string(),
            report.core_prove_duration.to_string(),
            report.core_verify_duration.to_string(),
            report.core_proof_size.to_string(),
            report.recursive_prove_duration.to_string(),
            report.recursive_verify_duration.to_string(),
            report.recursive_proof_size.to_string(),
            report.compressed_proof_size.unwrap_or(0).to_string(),
            report.compressed_proof_duration.unwrap_or(0.0).to_string(),
            report.bn254_compress_duration.to_string(),
            report.bn254_compress_proof_size.to_string(),
            report.groth16_compress_duration.to_string(),
        ])
        .unwrap();
    writer.flush().unwrap();
}
