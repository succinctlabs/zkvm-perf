use sp1_core::utils;
use sp1_prover::{utils::get_cycles, SP1Prover, SP1Stdin};
use std::env;
use std::fs;

use crate::{
    get_elf, time_operation, EvalArgs, PerformanceReport, PerformanceReportGenerator, ProgramId,
};

pub struct SP1PerformanceReportGenerator {}

impl PerformanceReportGenerator for SP1PerformanceReportGenerator {
    fn get_report(args: &EvalArgs) -> PerformanceReport {
        utils::setup_logger();
        let elf_path = get_elf(args);

        if args.program == ProgramId::Reth {
            // This is needed to limit how many shards we try to prove in parallel for Reth.
            std::env::set_var("SHARD_CHUNKING_MULTIPLIER", "4");
        }

        // Setup the prover.
        std::env::set_var("SHARD_SIZE", format!("{}", 1 << args.shard_size));
        //
        // Read the program from the file system.
        let elf = fs::read(elf_path).unwrap();

        // We assume that all benchmarking programs don't have stdin.
        let stdin = SP1Stdin::new();
        let cycles = get_cycles(&elf, &stdin);

        // Setup the prover.
        let prover = SP1Prover::new();
        let (pk, vk) = prover.setup(&elf);

        let stdin = SP1Stdin::new();
        // Execute the program.
        let (_, execution_duration) = time_operation(|| SP1Prover::execute(&elf, &stdin));

        // Generate the core proof ("leaf" stage).
        println!("Proving core");
        let (proof, prove_core_duration) = time_operation(|| prover.prove_core(&pk, &stdin));
        let core_bytes = bincode::serialize(&proof).unwrap();

        let (_, verify_core_duration) = time_operation(|| {
            prover
                .verify(&proof.proof, &vk)
                .expect("Proof verification failed")
        });
        let num_shards = proof.proof.0.len();
        println!(
            "Core proof time {} number of shards {}, proof size: {}",
            prove_core_duration.as_secs_f64(),
            num_shards,
            core_bytes.len()
        );

        println!("Generating reduce proofs (recursive stage)");
        let (reduce_proof, reduce_duration) =
            time_operation(|| prover.compress(&vk, proof, vec![]));
        let reduce_proof_size = bincode::serialize(&reduce_proof).unwrap();
        println!("Recursive proof size: {}", reduce_proof_size.len());

        let compress_start = std::time::Instant::now();
        let compressed_proof = prover.shrink(&vk, reduce_proof);
        let compress_duration = compress_start.elapsed();
        let compressed_proof_size = bincode::serialize(&compressed_proof).unwrap();
        println!("Done compressing proof before bn254 wrapping");

        let wrapped_bn_254_start = std::time::Instant::now();
        let wrapped_bn_254_proof = prover.wrap_bn254(&vk, compressed_proof);
        let wrapped_bn_254_duration = wrapped_bn_254_start.elapsed();
        let wrapped_bn_254_proof_size = bincode::serialize(&wrapped_bn_254_proof).unwrap();

        // We use this flag when benchmarking with JOLT, since they don't have groth16.
        let no_groth16 = match env::var("NO_GROTH16") {
            Ok(val) => val == "true",
            Err(_err) => false, // Default to running groth16
        };
        let groth16_start = std::time::Instant::now();
        if !no_groth16 {
            let groth16_build_dir = sp1_prover::build::get_groth16_artifacts_dir();
            let _groth16_proof =
                prover.wrap_groth16(wrapped_bn_254_proof, groth16_build_dir.into());
        }
        let groth16_duration = groth16_start.elapsed();

        let prove_duration = prove_core_duration
            + reduce_duration
            + compress_duration
            + wrapped_bn_254_duration
            + groth16_duration;

        // Create the performance report.
        PerformanceReport {
            program: args.program.to_string(),
            prover: args.prover.to_string(),
            hashfn: args.hashfn.to_string(),
            shard_size: args.shard_size,
            shards: num_shards,
            cycles: cycles as u64,
            speed: (cycles as f64) / prove_core_duration.as_secs_f64(),
            execution_duration: execution_duration.as_secs_f64(),
            prove_duration: prove_duration.as_secs_f64(),
            core_prove_duration: prove_core_duration.as_secs_f64(),
            core_verify_duration: verify_core_duration.as_secs_f64(),
            core_proof_size: core_bytes.len(),
            recursive_prove_duration: reduce_duration.as_secs_f64(),
            recursive_verify_duration: 0.0, // TODO: fill this in.
            recursive_proof_size: reduce_proof_size.len(),
            compressed_proof_size: Some(compressed_proof_size.len()),
            compressed_proof_duration: Some(compress_duration.as_secs_f64()),
            bn254_compress_duration: wrapped_bn_254_duration.as_secs_f64(),
            bn254_compress_proof_size: wrapped_bn_254_proof_size.len(),
            groth16_compress_duration: groth16_duration.as_secs_f64(),
        }
    }
}
