use std::fs;

use risc0_zkvm::{
    compute_image_id, get_prover_server, ExecutorEnv, ExecutorImpl, ProverOpts, VerifierContext,
};

use crate::{
    utils::{get_elf, get_reth_input, time_operation},
    EvalArgs, HashFnId, PerformanceReport, ProgramId,
};

pub struct Risc0Evaluator;

impl Risc0Evaluator {
    pub fn eval(args: &EvalArgs) -> PerformanceReport {
        if args.hashfn != HashFnId::Poseidon {
            panic!("Only Poseidon hash function is supported for Risc0.");
        }

        let elf_path = get_elf(args);
        let elf = fs::read(&elf_path).unwrap();
        let image_id = compute_image_id(elf.as_slice()).unwrap();

        // If the program is Reth, read the block and set it as input. Otherwise, we assume other
        // benchmarking programs don't have input.
        let env = if args.program == ProgramId::Reth {
            let input = get_reth_input(args);
            ExecutorEnv::builder()
                .segment_limit_po2(args.shard_size as u32)
                .write(&input)
                .expect("Failed to write input to executor")
                .build()
                .unwrap()
        } else {
            ExecutorEnv::builder().segment_limit_po2(args.shard_size as u32).build().unwrap()
        };

        // Compute some statistics.
        let mut exec = ExecutorImpl::from_elf(env, &elf).unwrap();
        let session = exec.run().unwrap();
        let cycles = session.user_cycles;

        // Setup the prover.
        let env = if args.program == ProgramId::Reth {
            let input = get_reth_input(args);
            ExecutorEnv::builder()
                .segment_limit_po2(args.shard_size as u32)
                .write(&input)
                .expect("Failed to write input to executor")
                .build()
                .unwrap()
        } else {
            ExecutorEnv::builder().segment_limit_po2(args.shard_size as u32).build().unwrap()
        };
        let opts = ProverOpts::default();
        let prover = get_prover_server(&opts).unwrap();

        // Generate the session.
        let mut exec = ExecutorImpl::from_elf(env, &elf).unwrap();
        let (session, execution_duration) = time_operation(|| exec.run().unwrap());

        // Generate the proof.
        let ctx = VerifierContext::default();
        let (info, core_prove_duration) =
            time_operation(|| prover.prove_session(&ctx, &session).unwrap());

        let receipt = info.receipt;

        let composite_receipt = receipt.inner.composite().unwrap();
        let num_segments = composite_receipt.segments.len();

        // Get the core proof size by summing across all segments.
        let mut core_proof_size = 0;
        for segment in composite_receipt.segments.iter() {
            core_proof_size += segment.seal.len() * 4;
        }

        // Verify the core proof.
        let ((), core_verify_duration) = time_operation(|| receipt.verify(image_id).unwrap());

        // Now compress the proof with recursion.
        // let composite_receipt = receipt.inner.composite().unwrap();
        let (compressed_proof, compress_duration) =
            time_operation(|| prover.compress(&ProverOpts::succinct(), &receipt).unwrap());

        // Verify the recursive proof
        let ((), recursive_verify_duration) =
            time_operation(|| compressed_proof.verify(image_id).unwrap());

        let succinct_receipt = compressed_proof.inner.succinct().unwrap();

        // Get the recursive proof size.
        let recursive_proof_size = succinct_receipt.seal.len() * 4;
        let prove_duration = core_prove_duration + compress_duration;

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
            compress_prove_duration: compress_duration.as_secs_f64(),
            compress_verify_duration: recursive_verify_duration.as_secs_f64(),
            compress_proof_size: recursive_proof_size,
        }
    }
}
