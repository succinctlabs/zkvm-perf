use std::fs;

use crate::{
    utils::{gas_amount, get_elf, get_reth_input, hash_bytes_per_second, hashes_per_second, rand_ecdsa_signature, rand_eddsa_signature, time_operation},
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
            ProgramId::Loop300m => {
                stdin.write::<usize>(&75000000);
            }
            ProgramId::Fibonacci20k => {
                stdin.write::<u32>(&1500);
            }
            ProgramId::Fibonacci200k => {
                stdin.write::<u32>(&15000);
            }
            ProgramId::Fibonacci2m => {
                stdin.write::<u32>(&150000);
            }
            ProgramId::Fibonacci4m => {
                stdin.write::<u32>(&300000);
            }
            ProgramId::Fibonacci20m => {
                stdin.write::<u32>(&1500000);
            }
            ProgramId::Fibonacci40m => {
                stdin.write::<u32>(&3000000);
            }
            ProgramId::Fibonacci200m => {
                stdin.write::<u32>(&15000000);
            }
            ProgramId::Fibonacci400m => {
                stdin.write::<u32>(&30000000);
            }
            ProgramId::Fibonacci1b => {
                stdin.write::<u32>(&75000000);
            }
            ProgramId::Fibonacci2b => {
                stdin.write::<u32>(&150000000);
            }
            ProgramId::Fibonacci4b => {
                stdin.write::<u32>(&300000000);
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
            ProgramId::Sha25610mb => {
                stdin.write(&vec![0u8; 1048576 * 10]);
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
            ProgramId::Keccak25610mb => {
                stdin.write(&vec![0u8; 1048576 * 10]);
            }
            ProgramId::Reth => {
                let input = get_reth_input(args);
                stdin.write(&input);
            }
            ProgramId::Rsp20526626 => {
                let input = include_bytes!("../../fixtures/20526626.bin");
                stdin.write_vec(input.to_vec());
            }
            ProgramId::Rsp20526627 => {
                let input = include_bytes!("../../fixtures/20526627.bin");
                stdin.write_vec(input.to_vec());
            }
            ProgramId::Rsp20526628 => {
                let input = include_bytes!("../../fixtures/20526628.bin");
                stdin.write_vec(input.to_vec());
            }
            ProgramId::Rsp20526629 => {
                let input = include_bytes!("../../fixtures/20526629.bin");
                stdin.write_vec(input.to_vec());
            }
            ProgramId::Rsp20526630 => {
                let input = include_bytes!("../../fixtures/20526630.bin");
                stdin.write_vec(input.to_vec());
            }
            ProgramId::Rsp20528708 => {
                let input = include_bytes!("../../fixtures/20528708.bin");
                stdin.write_vec(input.to_vec());
            }
            ProgramId::Rsp20528709 => {
                let input = include_bytes!("../../fixtures/20528709.bin");
                stdin.write_vec(input.to_vec());
            }
            ProgramId::Rsp20528710 => {
                let input = include_bytes!("../../fixtures/20528710.bin");
                stdin.write_vec(input.to_vec());
            }
            ProgramId::Rsp20528711 => {
                let input = include_bytes!("../../fixtures/20528711.bin");
                stdin.write_vec(input.to_vec());
            }
            ProgramId::Rsp20528712 => {
                let input = include_bytes!("../../fixtures/20528712.bin");
                stdin.write_vec(input.to_vec());
            },
            ProgramId::ECDSAVerify => {
                stdin.write(&rand_ecdsa_signature());
            },
            ProgramId::EDDSAVerify => {
                stdin.write(&rand_eddsa_signature());
            },
            _ => {}
        }

        let elf_path = get_elf(args);
        let elf = fs::read(elf_path).unwrap();
        if std::env::var("SAVE").unwrap_or_default() == "1" {
            let stdin_bytes = bincode::serialize(&stdin).unwrap();
            fs::write("stdin.bin", &stdin_bytes).unwrap();
            std::process::Command::new("aws")
                .args(&[
                    "s3",
                    "cp",
                    &format!("stdin.bin"),
                    &format!("s3://sp1-testing-suite/{}/stdin.bin", args.program.to_string())
                ])
                .status()
                .expect("Failed to upload stdin.bin to S3");

            fs::write("program.bin", &elf).unwrap();
            std::process::Command::new("aws")
                .args(&[
                    "s3", 
                    "cp",
                    &format!("program.bin"),
                    &format!("s3://sp1-testing-suite/{}/program.bin", args.program.to_string())
                ])
                .status()
                .expect("Failed to upload program.bin to S3");
            std::process::exit(0);
        }

        // Get the elf. 
        let cycles = get_cycles(&elf, &stdin); 

        let mut prover = SP1Prover::<DefaultProverComponents>::new();

        #[cfg(feature = "cuda")]
        {
            prover.single_shard_programs = None;
        }

        #[cfg(feature = "cuda")]
        let server = SP1CudaProver::new().expect("Failed to initialize CUDA prover");

        // Setup the program.
        #[cfg(not(feature = "cuda"))]
        let (_, pk_d, program, vk) = prover.setup(&elf);

        #[cfg(feature = "cuda")]
        let (pk, vk) = server.setup(&elf).unwrap();

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
            time_operation(|| prover.prove_core(&pk_d, program, &stdin, opts, context).unwrap());

        // Generate the core proof (CUDA).
        #[cfg(feature = "cuda")]
        let (core_proof, prove_core_duration) =
            time_operation(|| server.prove_core(&stdin).unwrap());

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
            priority: args.program.priority(),
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
            gas: gas_amount(&args.program),
            hashes_per_second: hashes_per_second(&args.program, prove_duration),
            hash_bytes_per_second: hash_bytes_per_second(&args.program, prove_duration),
        }
    }
}

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
