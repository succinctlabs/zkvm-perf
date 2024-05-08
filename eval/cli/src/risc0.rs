use std::fs;

use crate::HashFnId;
use crate::{get_elf, time_operation, EvalArgs, PerformanceReport, PerformanceReportGenerator};

pub struct Risc0PerformanceReportGenerator {}

impl PerformanceReportGenerator for Risc0PerformanceReportGenerator {
    fn get_report(args: &EvalArgs) -> PerformanceReport {
        use risc0_zkvm::{
            compute_image_id, get_prover_server, ExecutorEnv, ExecutorImpl, ProverOpts,
            VerifierContext,
        };

        if args.hashfn != HashFnId::Poseidon {
            panic!("Only Poseidon hash function is supported for Risc0.");
        }

        let elf_path = get_elf(args);
        // Read the program from the file system.
        let elf = fs::read(&elf_path).unwrap();
        let image_id = compute_image_id(elf.as_slice()).unwrap();

        // Compute some statistics.
        let env = ExecutorEnv::builder().build().unwrap();
        let mut exec = ExecutorImpl::from_elf(env, &elf).unwrap();
        let session = exec.run().unwrap();
        let cycles = session.user_cycles;

        println!("risc0 cycles: {}", cycles);

        // Setup the prover.
        let env = ExecutorEnv::builder().build().unwrap();
        let opts = ProverOpts::default();
        let prover = get_prover_server(&opts).unwrap();

        // Generate the session.
        let mut exec = ExecutorImpl::from_elf(env, &elf).unwrap();
        let (session, execution_duration) = time_operation(|| exec.run().unwrap());

        // Generate the proof.
        let ctx = VerifierContext::default();
        let (receipt, core_prove_duration) =
            time_operation(|| prover.prove_session(&ctx, &session).unwrap());

        let composite_receipt = receipt.inner.composite().unwrap();
        let num_segments = composite_receipt.segments.len();
        println!("Generated the core proof with {} segments", num_segments);

        // Get the core proof size by summing across all segments.
        let mut core_proof_size = 0;
        for segment in composite_receipt.segments.iter() {
            core_proof_size += segment.seal.len() * 4;
        }

        // Verify the core proof.
        let ((), core_verify_duration) = time_operation(|| receipt.verify(image_id).unwrap());

        println!("Generated and verified the core proof");

        // Now compress the proof with recursion.
        let composite_receipt = receipt.inner.composite().unwrap();
        let (compressed_proof, compress_duration) =
            time_operation(|| prover.compress(composite_receipt).unwrap());

        // Verify the recursive proof
        let ((), recursive_verify_duration) =
            time_operation(|| compressed_proof.verify_integrity().unwrap());

        // Get the recursive proof size.
        let recursive_proof_size = compressed_proof.seal.len() * 4;

        // Bn254 wrapping duration
        let (bn254_proof, bn254_compress_duration) =
            time_operation(|| prover.identity_p254(&compressed_proof).unwrap());

        let seal_bytes = bn254_proof.get_seal_bytes();
        println!("Running groth16 wrapper");
        let (_groth16_proof, groth16_duration) =
            time_operation(|| risc0_zkvm::stark_to_snark(&seal_bytes).unwrap());
        println!("Done running groth16");

        let prove_duration =
            core_prove_duration + compress_duration + bn254_compress_duration + groth16_duration;

        // Create the performance report.
        PerformanceReport {
            program: args.program.to_string(),
            prover: args.prover.to_string(),
            hashfn: args.hashfn.to_string(),
            shard_size: args.shard_size,
            shards: num_segments,
            cycles: cycles as u64,
            speed: (cycles as f64) / prove_duration.as_secs_f64(),
            execution_duration: execution_duration.as_secs_f64(),
            prove_duration: prove_duration.as_secs_f64(),
            core_prove_duration: core_prove_duration.as_secs_f64(),
            core_verify_duration: core_verify_duration.as_secs_f64(),
            core_proof_size,
            recursive_prove_duration: compress_duration.as_secs_f64(),
            recursive_verify_duration: recursive_verify_duration.as_secs_f64(),
            recursive_proof_size,
            compressed_proof_size: None,
            compressed_proof_duration: None,
            bn254_compress_duration: bn254_compress_duration.as_secs_f64(),
            bn254_compress_proof_size: bn254_proof.seal.len() * 4,
            groth16_compress_duration: groth16_duration.as_secs_f64(),
        }
    }
}
