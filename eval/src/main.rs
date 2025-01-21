mod risc0;
mod sp1;
mod types;
mod utils;

use std::{
    fs::{create_dir_all, OpenOptions},
    path::PathBuf,
};

use clap::{command, Parser};
use csv::WriterBuilder;
use serde::Serialize;
use types::*;

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
    shard_size: u64,
    #[arg(long)]
    filename: String,
    #[arg(long)]
    block_number: Option<u64>,
    #[arg(long)]
    groth16: bool,
    #[arg(long)]
    plonk: bool,
}

/// The performance report of a zkVM on a program.
#[derive(Debug, Serialize, Default)]
pub struct PerformanceReport {
    pub priority: usize,
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
    /// The speed of the core proving time in KHz.
    pub core_khz: f64,

    /// The reported duration of the recursive proving time in seconds.
    pub compress_prove_duration: f64,
    /// The reported duration of the verifier in seconds.
    pub compress_verify_duration: f64,
    /// The size of the recursive proof in bytes.
    pub compress_proof_size: usize,

    /// The reported duration of the shrink proving time in seconds.
    pub shrink_prove_duration: f64,
    /// The reported duration of the wrap proving time in seconds.
    pub wrap_prove_duration: f64,
    /// The reported duration of the groth16 proving time in seconds.
    pub groth16_prove_duration: f64,
    /// The reported duration of the plonk proving time in seconds.
    pub plonk_prove_duration: f64,

    /// The overall speed in KHz.
    pub overall_khz: f64,
    /// The number of hashes per second
    pub hashes_per_second: Option<f64>,
    /// The number of bytes hashed per second
    pub hash_bytes_per_second: Option<f64>,
    /// The gas used by the program (if RSP)
    pub gas: Option<u64>,
}

fn main() {
    // Setup the logger.
    sp1_core_machine::utils::setup_logger();

    let args = EvalArgs::parse();

    // Select the correct implementation based on the prover.
    let report = match args.prover {
        ProverId::Risc0 => risc0::Risc0Evaluator::eval(&args),
        ProverId::SP1 => sp1::SP1Evaluator::eval(&args),
    };

    // Create the results directory if it doesn't exist.
    let results_dir = PathBuf::from("benchmarks");
    create_dir_all(&results_dir).unwrap();

    // Create the file.
    let filename = format!("{}_{}.csv", args.filename, env!("VERGEN_GIT_SHA"));
    let path = results_dir.join(filename);
    let file = OpenOptions::new().create(true).append(true).open(path.clone()).unwrap();

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
                "core_khz",
                "compress_prove_duration",
                "compress_verify_duration",
                "compress_proof_size",
                "overall_khz",
                "priority",
                "hashes_per_second",
                "hash_bytes_per_second",
                "gas",
                "shrink_prove_duration",
                "wrap_prove_duration",
                "groth16_prove_duration",
                "plonk_prove_duration",
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
            report.core_khz.to_string(),
            report.compress_prove_duration.to_string(),
            report.compress_verify_duration.to_string(),
            report.compress_proof_size.to_string(),
            report.overall_khz.to_string(),
            report.priority.to_string(),
            report.hashes_per_second.map(|x| x.to_string()).unwrap_or_default(),
            report.hash_bytes_per_second.map(|x| x.to_string()).unwrap_or_default(),
            report.gas.map(|x| x.to_string()).unwrap_or_default(),
            report.shrink_prove_duration.to_string(),
            report.wrap_prove_duration.to_string(),
            report.groth16_prove_duration.to_string(),
            report.plonk_prove_duration.to_string(),
        ])
        .unwrap();
    writer.flush().unwrap();

    let latest_filename = "benchmarks_latest.csv";
    let latest_path = results_dir.join(latest_filename);
    std::fs::copy(&path, &latest_path).unwrap();
}
