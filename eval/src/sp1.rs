use std::fs;

use crate::{
    utils::{get_elf, get_reth_input, time_operation},
    EvalArgs, PerformanceReport, ProgramId,
};

use sp1_core_executor::SP1Context;

use sp1_core_machine::io::SP1Stdin;
use sp1_prover::{components::DefaultProverComponents, utils::get_cycles, SP1Prover};

#[cfg(feature = "cuda")]
use sp1_cuda::SP1CudaProver;

#[cfg(not(feature = "cuda"))]
use sp1_stark::SP1ProverOpts;

pub struct SP1Evaluator;

impl SP1Evaluator {
    pub fn eval(args: &EvalArgs) -> PerformanceReport {
        // Setup the logger.
        sp1_core_machine::utils::setup_logger();

        // Set enviroment variables to configure the prover.
        std::env::set_var("SHARD_SIZE", format!("{}", 1 << args.shard_size));
        if args.program == ProgramId::Reth {
            std::env::set_var("SHARD_CHUNKING_MULTIPLIER", "4");
        }

        // Get stdin.
        let mut stdin = SP1Stdin::new();
        match args.program {
            ProgramId::Reth => {
                let input = get_reth_input(args);
                stdin.write(&input);
            }
            ProgramId::Loop10k => {
                stdin.write::<usize>(&2500);
            }
            ProgramId::Loop100k => {
                stdin.write::<usize>(&25000);
            }
            ProgramId::Loop1m => {
                stdin.write::<usize>(&250000);
            }
            ProgramId::Loop3m => {
                stdin.write::<usize>(&750000);
            }
            ProgramId::Loop10m => {
                stdin.write::<usize>(&2500000);
            }
            ProgramId::Loop30m => {
                stdin.write::<usize>(&7500000);
            }
            ProgramId::Loop100m => {
                stdin.write::<usize>(&25000000);
            }
            ProgramId::Sha256100kb => {
                stdin.write(&vec![0u8; 102400]);
            }
            ProgramId::Sha256300kb => {
                stdin.write(&vec![0u8; 102400 * 3]);
            }
            ProgramId::Sha2561mb => {
                stdin.write(&vec![0u8; 1048576]);
            }
            ProgramId::Sha2563mb => {
                stdin.write(&vec![0u8; 1048576 * 3]);
            }
            ProgramId::Keccak256100kb => {
                stdin.write(&vec![0u8; 102400]);
            }
            ProgramId::Keccak256300kb => {
                stdin.write(&vec![0u8; 102400 * 3]);
            }
            ProgramId::Keccak2561mb => {
                stdin.write(&vec![0u8; 1048576]);
            }
            ProgramId::Keccak2563mb => {
                stdin.write(&vec![0u8; 1048576 * 3]);
            }
            _ => {}
        }

        // Get the elf.
        let elf_path = get_elf(args);
        let elf = fs::read(elf_path).unwrap();
        let cycles = get_cycles(&elf, &stdin);

        // let stdin_bytes = bincode::serialize(&stdin).unwrap();
        // let stdin_path = format!("{}/stdin.bin", args.program.to_string());
        // let elf_path = format!("{}/elf.bin", args.program.to_string());
        // fs::create_dir_all(args.program.to_string()).unwrap();
        // fs::write(format!("{}/stdin.bin", args.program.to_string()), &stdin_bytes).unwrap();
        // fs::write(format!("{}/program.bin", args.program.to_string()), &elf).unwrap();
        // let command = format!(
        //     "aws s3 cp --recursive {} s3://sp1-testing-suite/{}",
        //     args.program.to_string(),
        //     args.program.to_string()
        // );
        // Command::new("bash")
        //     .arg("-c")
        //     .arg(&command)
        //     .status()
        //     .expect("Failed to execute command");
        // exit(0);

        let prover = SP1Prover::<DefaultProverComponents>::new();

        #[cfg(feature = "cuda")]
        let server = SP1CudaProver::new().expect("Failed to initialize CUDA prover");

        // Setup the program.
        let (pk, vk) = prover.setup(&elf);

        // Execute the program.
        let context = SP1Context::default();
        let (_, execution_duration) =
            time_operation(|| prover.execute(&elf, &stdin, context.clone()));

        // Setup the prover opionts.
        #[cfg(not(feature = "cuda"))]
        let opts = SP1ProverOpts::default();

        // Generate the core proof (CPU).
        #[cfg(not(feature = "cuda"))]
        let (core_proof, prove_core_duration) =
            time_operation(|| prover.prove_core(&pk, &stdin, opts, context).unwrap());

        // Generate the core proof (CUDA).
        #[cfg(feature = "cuda")]
        let (core_proof, prove_core_duration) =
            time_operation(|| server.prove_core(&pk.elf, &stdin).unwrap());

        let num_shards = core_proof.proof.0.len();

        // Verify the proof.
        let core_bytes = bincode::serialize(&core_proof).unwrap();
        let (_, verify_core_duration) = time_operation(|| {
            prover.verify(&core_proof.proof, &vk).expect("Proof verification failed")
        });

        #[cfg(not(feature = "cuda"))]
        let (compress_proof, compress_duration) =
            time_operation(|| prover.compress(&vk, core_proof, vec![], opts).unwrap());

        #[cfg(feature = "cuda")]
        let (compress_proof, compress_duration) =
            time_operation(|| server.compress(&vk, core_proof, vec![]).unwrap());

        let compress_bytes = bincode::serialize(&compress_proof).unwrap();
        println!("recursive proof size: {}", compress_bytes.len());

        let (_, verify_compress_duration) = time_operation(|| {
            prover.verify_compressed(&compress_proof, &vk).expect("Proof verification failed")
        });

        let prove_duration = prove_core_duration + compress_duration;

        let core_khz = cycles as f64 / prove_core_duration.as_secs_f64() / 1_000.0;
        let overall_khz = cycles as f64 / prove_duration.as_secs_f64() / 1_000.0;

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
            core_khz,
            compress_prove_duration: compress_duration.as_secs_f64(),
            compress_verify_duration: verify_compress_duration.as_secs_f64(),
            compress_proof_size: compress_bytes.len(),
            overall_khz,
        }
    }
}
