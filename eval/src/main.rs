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
    /// The speed of the core proving time in KHz.
    pub core_khz: f64,
    /// The reported duration of the recursive proving time in seconds.
    pub compress_prove_duration: f64,
    /// The reported duration of the verifier in seconds.
    pub compress_verify_duration: f64,
    /// The size of the recursive proof in bytes.
    pub compress_proof_size: usize,
    /// The overall speed in KHz.
    pub overall_khz: f64,
}

fn main() {
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
        ])
        .unwrap();
    writer.flush().unwrap();

    let latest_filename = "benchmarks_latest.csv";
    let latest_path = results_dir.join(latest_filename);
    std::fs::copy(&path, &latest_path).unwrap();
}
