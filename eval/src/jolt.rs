use std::time::Instant;

use jolt::{
    host::{analyze::ProgramSummary, Program},
    Jolt, JoltPreprocessing, Proof, RV32IJoltVM, F, G,
};

use crate::{EvalArgs, PerformanceReport, PerformanceReportGenerator, ProgramId};
use fibonacci_jolt::{analyze_fibonacci, preprocess_fibonacci, prove_fibonacci};
use loop_jolt::{analyze_loop_jolt, preprocess_loop_jolt, prove_loop_jolt};
use sha2_chain_jolt::{analyze_sha2_chain, preprocess_sha2_chain, prove_sha2_chain};
use tendermint_jolt::{analyze_tendermint, preprocess_tendermint, prove_tendermint};

pub struct JoltPerformanceReportGenerator {}

struct ProveAndVerifyResult {
    total_cycles: u64,
    prove_duration: f64,
    verify_duration: f64,
    proof_size: usize,
}

fn get_jolt_statistics<P>(
    analyze: fn() -> ProgramSummary,
    program: Program,
    preprocessing: JoltPreprocessing<F, G>,
    prove: P,
) -> ProveAndVerifyResult
where
    P: Fn(Program, JoltPreprocessing<F, G>) -> Proof,
{
    // Get the cycle count of the program.
    let summary = analyze();
    let instruction_counts = summary.analyze::<F>();
    let total_cycles = instruction_counts.iter().map(|(_, count)| count).sum::<usize>();
    println!("Total cycles: {}", total_cycles);

    // Generate the proof.
    let prove_start = Instant::now();
    let proof = prove(program, preprocessing.clone());
    let prove_duration = prove_start.elapsed().as_secs_f64();

    // Get the proof size.
    let proof_size = proof.size().expect("Failed to get proof size");

    // Verify the proof.
    let verify_start = Instant::now();
    let _ = RV32IJoltVM::verify(preprocessing, proof.proof, proof.commitments);
    let verify_duration = verify_start.elapsed().as_secs_f64();

    ProveAndVerifyResult {
        total_cycles: total_cycles as u64,
        prove_duration,
        verify_duration,
        proof_size,
    }
}

fn get_jolt_statistics_v2<P>(
    analyze: fn([u8; 32], u32) -> ProgramSummary,
    program: Program,
    preprocessing: JoltPreprocessing<F, G>,
    prove: P,
    a: [u8; 32],
    b: u32,
) -> ProveAndVerifyResult
where
    P: Fn(Program, JoltPreprocessing<F, G>) -> Proof,
{
    // Get the cycle count of the program.
    let summary = analyze(a, b);
    let instruction_counts = summary.analyze::<F>();
    let total_cycles = instruction_counts.iter().map(|(_, count)| count).sum::<usize>();
    println!("Total cycles: {}", total_cycles);

    // Generate the proof.
    let prove_start = Instant::now();
    let proof = prove(program, preprocessing.clone());
    let prove_duration = prove_start.elapsed().as_secs_f64();

    // Get the proof size.
    let proof_size = proof.size().expect("Failed to get proof size");

    // Verify the proof.
    let verify_start = Instant::now();
    let _ = RV32IJoltVM::verify(preprocessing, proof.proof, proof.commitments);
    let verify_duration = verify_start.elapsed().as_secs_f64();

    ProveAndVerifyResult {
        total_cycles: total_cycles as u64,
        prove_duration,
        verify_duration,
        proof_size,
    }
}

fn get_jolt_statistics_for_program(program: ProgramId) -> ProveAndVerifyResult {
    match program {
        ProgramId::Fibonacci => {
            println!("Preprocessing fibonacci");
            // Preprocess to separate compilation and trace generation separately from proving.
            let (program, preprocessing) = preprocess_fibonacci();
            println!("Starting proving");
            // Wrap the prove function in a closure that ignores the output.
            let prove_wrapper = |program: Program, preprocessing: JoltPreprocessing<F, G>| {
                let (_, proof) = prove_fibonacci(program, preprocessing);
                proof
            };
            get_jolt_statistics(analyze_fibonacci, program, preprocessing, prove_wrapper)
        }
        ProgramId::Loop => {
            println!("Preprocessing loop");
            // Preprocess to separate compilation and trace generation separately from proving.
            let (program, preprocessing) = preprocess_loop_jolt();
            println!("Starting proving");
            // Wrap the prove function in a closure that ignores the output.
            let prove_wrapper = |program: Program, preprocessing: JoltPreprocessing<F, G>| {
                let (_, proof) = prove_loop_jolt(program, preprocessing);
                proof
            };
            get_jolt_statistics(analyze_loop_jolt, program, preprocessing, prove_wrapper)
        }
        ProgramId::Tendermint => {
            println!("Preprocessing tendermint");
            // Preprocess to separate compilation and trace generation separately from proving.
            let (program, preprocessing) = preprocess_tendermint();
            println!("Starting proving");
            // Wrap the prove function in a closure that ignores the output.
            let prove_wrapper = |program: Program, preprocessing: JoltPreprocessing<F, G>| {
                let (_, proof) = prove_tendermint(program, preprocessing);
                proof
            };
            get_jolt_statistics(analyze_tendermint, program, preprocessing, prove_wrapper)
        }
        ProgramId::Sha2Chain => {
            println!("Preprocessing sha2-chain");
            // Preprocess to separate compilation and trace generation separately from proving.
            let (program, preprocessing) = preprocess_sha2_chain();
            println!("Starting proving");
            // Wrap the prove function in a closure that ignores the output.
            let input = [5u8; 32];
            let num_iters: u32 = 2500;
            let prove_wrapper = |program: Program, preprocessing: JoltPreprocessing<F, G>| {
                let (_, proof) = prove_sha2_chain(program, preprocessing, input, num_iters);
                proof
            };
            get_jolt_statistics_v2(
                analyze_sha2_chain,
                program,
                preprocessing,
                prove_wrapper,
                input,
                num_iters,
            )
        }
        _ => {
            todo!();
        }
    }
}

impl PerformanceReportGenerator for JoltPerformanceReportGenerator {
    fn get_report(args: &EvalArgs) -> PerformanceReport {
        let res = get_jolt_statistics_for_program(args.program.clone());

        // Create the performance report.
        PerformanceReport {
            program: args.program.to_string(),
            prover: args.prover.to_string(),
            hashfn: args.hashfn.to_string(),
            shard_size: 0,
            shards: 0,
            cycles: res.total_cycles,
            speed: (res.total_cycles as f64 / res.prove_duration) as f64,
            execution_duration: 0.0,
            prove_duration: 0.0,
            core_prove_duration: res.prove_duration,
            core_verify_duration: res.verify_duration,
            core_proof_size: res.proof_size,
            recursive_prove_duration: 0.0,
            recursive_verify_duration: 0.0,
            recursive_proof_size: 0,
            compressed_proof_size: None,
            compressed_proof_duration: None,
            bn254_compress_duration: 0.0,
            bn254_compress_proof_size: 0,
            groth16_compress_duration: 0.0,
        }
    }
}
