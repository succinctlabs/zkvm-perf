#[cfg(feature = "risc0")]
use std::fs;

#[cfg(feature = "risc0")]
use crate::{
    utils::{
        gas_amount, get_elf, hash_bytes_per_second, hashes_per_second,
        rand_ecdsa_signature, rand_eddsa_signature, time_operation,
    },
    HashFnId, ProgramId,
};
#[cfg(feature = "risc0")]
use risc0_zkvm::{
    compute_image_id, get_prover_server, ExecutorEnv, ExecutorImpl, ProverOpts, VerifierContext,
};
#[cfg(feature = "risc0")]
use risc0_groth16::{
    Fr, ProofJson, PublicInputsJson, Seal, Verifier, VerifyingKey, VerifyingKeyJson,
};

use serde::{Deserialize, Serialize};

use crate::{EvalArgs, PerformanceReport};
use std::time::Duration;

pub struct Risc0Evaluator;

impl Risc0Evaluator {
    #[cfg(feature = "risc0")]
    pub fn eval(args: &EvalArgs) -> PerformanceReport {
        use crate::ProgramId;

        if args.hashfn != HashFnId::Poseidon {
            panic!("Only Poseidon hash function is supported for Risc0.");
        }

        let elf_path = get_elf(args);
        let elf = fs::read(&elf_path).unwrap();
        let image_id = compute_image_id(elf.as_slice()).unwrap();

        let mut builder = ExecutorEnv::builder();
        builder.segment_limit_po2(args.shard_size as u32);
        match args.program {
            ProgramId::Loop10k => {
                builder.write::<usize>(&2500);
            }
            ProgramId::Loop100k => {
                builder.write::<usize>(&25000);
            }
            ProgramId::Loop1m => {
                builder.write::<usize>(&250000);
            }
            ProgramId::Loop3m => {
                builder.write::<usize>(&750000);
            }
            ProgramId::Loop10m => {
                builder.write::<usize>(&2500000);
            }
            ProgramId::Loop30m => {
                builder.write::<usize>(&7500000);
            }
            ProgramId::Loop100m => {
                builder.write::<usize>(&25000000);
            }
            ProgramId::Loop300m => {
                builder.write::<usize>(&75000000);
            }
            ProgramId::Fibonacci20k => {
                builder.write::<u32>(&1500);
            }
            ProgramId::Fibonacci200k => {
                builder.write::<u32>(&15000);
            }
            ProgramId::Fibonacci2m => {
                builder.write::<u32>(&150000);
            }
            ProgramId::Fibonacci4m => {
                builder.write::<u32>(&300000);
            }
            ProgramId::Fibonacci20m => {
                builder.write::<u32>(&1500000);
            }
            ProgramId::Fibonacci40m => {
                builder.write::<u32>(&3000000);
            }
            ProgramId::Fibonacci200m => {
                builder.write::<u32>(&15000000);
            }
            ProgramId::Fibonacci400m => {
                builder.write::<u32>(&30000000);
            }
            ProgramId::Sha256100kb => {
                builder.write(&vec![0u8; 102400]);
            }
            ProgramId::Sha256300kb => {
                builder.write(&vec![0u8; 102400 * 3]);
            }
            ProgramId::Sha2561mb => {
                builder.write(&vec![0u8; 1048576]);
            }
            ProgramId::Sha2563mb => {
                builder.write(&vec![0u8; 1048576 * 3]);
            }
            ProgramId::Keccak256100kb => {
                builder.write(&vec![0u8; 102400]);
            }
            ProgramId::Keccak256300kb => {
                builder.write(&vec![0u8; 102400 * 3]);
            }
            ProgramId::Keccak2561mb => {
                builder.write(&vec![0u8; 1048576]);
            }
            ProgramId::Keccak2563mb => {
                builder.write(&vec![0u8; 1048576 * 3]);
            }
            ProgramId::Keccak25610mb => {
                builder.write(&vec![0u8; 1048576 * 10]);
            }
            ProgramId::Rsp20526626 => {
                let input = include_bytes!("../../fixtures/20526626.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20526627 => {
                let input = include_bytes!("../../fixtures/20526627.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20526628 => {
                let input = include_bytes!("../../fixtures/20526628.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20526629 => {
                let input = include_bytes!("../../fixtures/20526629.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20526630 => {
                let input = include_bytes!("../../fixtures/20526630.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20528708 => {
                let input = include_bytes!("../../fixtures/20528708.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20528709 => {
                let input = include_bytes!("../../fixtures/20528709.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20528710 => {
                let input = include_bytes!("../../fixtures/20528710.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20528711 => {
                let input = include_bytes!("../../fixtures/20528711.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20528712 => {
                let input = include_bytes!("../../fixtures/20528712.bin");
                builder.write(&input.to_vec());
            },
            ProgramId::ECDSAVerify => {
                builder.write(&rand_ecdsa_signature());
            },
            ProgramId::EDDSAVerify => {
                let times: u8 = 100;
                builder.write(&times);
                
                for _ in 0..times {
                    builder.write(&rand_eddsa_signature());
                }
            },
            ProgramId::Helios => {
                let input = include_bytes!("../../fixtures/helios/proof_inputs.cbor");
                builder.write(&input.to_vec());
            },
            ProgramId::Groth16ProofVerify => {
                const PROOF: &str = include_str!("../../fixtures/risc0/proof.json");
                const PUBLIC_INPUTS: &str = include_str!("../../fixtures/risc0/public.json");
                const VERIFICATION_KEY: &str = include_str!("../../fixtures/risc0/verification_key.json");

                // Verification_key, proof and public witness generated by SnarkJS using Groth16 over BN254
                // (https://docs.circom.io/getting-started/proving-circuits/)
                let proof_json: ProofJson = serde_json::from_str(PROOF).unwrap();
                let public_inputs_json = PublicInputsJson {
                    values: serde_json::from_str(PUBLIC_INPUTS).unwrap(),
                };

                let verifying_key_json: VerifyingKeyJson = serde_json::from_str(VERIFICATION_KEY).unwrap();
                // Convert from the JSON data structure, with string encoded values.
                let seal: Seal = proof_json.try_into().unwrap();
                let public_inputs: Vec<Fr> = public_inputs_json.to_scalar().unwrap();
                let verifying_key: VerifyingKey = verifying_key_json.verifying_key().unwrap();
                
                builder.write(&(seal, public_inputs, verifying_key)).unwrap();
            },
            ProgramId::ZKEmail => {
                #[derive(Serialize, Deserialize, Debug, Clone)]
                #[serde(rename_all = "camelCase")]
                struct EmailInputs {
                    public_key: String,
                    signature: String,
                    headers: String,
                    body: String,
                    body_hash: String,
                }

                const EMAIL_JSON: &[u8] = include_bytes!("../../fixtures/zk-email/email.json");
                let email_input = serde_json::from_slice::<EmailInputs>(EMAIL_JSON).unwrap();

                builder.write(&email_input).unwrap();
            },
            _ => {}
        }
        let env = builder.build().unwrap();

        // Compute some statistics.
        let mut exec = ExecutorImpl::from_elf(env, &elf).unwrap();
        let session = exec.run().unwrap();
        let cycles = session.user_cycles;
        println!("cycles: {}", cycles);

        // Setup the prover.
        let mut builder = ExecutorEnv::builder();
        builder.segment_limit_po2(args.shard_size as u32);
        match args.program {
            ProgramId::Loop10k => {
                builder.write::<usize>(&2500);
            }
            ProgramId::Loop100k => {
                builder.write::<usize>(&25000);
            }
            ProgramId::Loop1m => {
                builder.write::<usize>(&250000);
            }
            ProgramId::Loop3m => {
                builder.write::<usize>(&750000);
            }
            ProgramId::Loop10m => {
                builder.write::<usize>(&2500000);
            }
            ProgramId::Loop30m => {
                builder.write::<usize>(&7500000);
            }
            ProgramId::Loop100m => {
                builder.write::<usize>(&25000000);
            }
            ProgramId::Loop300m => {
                builder.write::<usize>(&75000000);
            }
            ProgramId::Fibonacci20k => {
                builder.write::<u32>(&1500);
            }
            ProgramId::Fibonacci200k => {
                builder.write::<u32>(&15000);
            }
            ProgramId::Fibonacci2m => {
                builder.write::<u32>(&150000);
            }
            ProgramId::Fibonacci4m => {
                builder.write::<u32>(&300000);
            }
            ProgramId::Fibonacci20m => {
                builder.write::<u32>(&1500000);
            }
            ProgramId::Fibonacci40m => {
                builder.write::<u32>(&3000000);
            }
            ProgramId::Fibonacci200m => {
                builder.write::<u32>(&15000000);
            }
            ProgramId::Fibonacci400m => {
                builder.write::<u32>(&30000000);
            }
            ProgramId::Sha256100kb => {
                builder.write(&vec![0u8; 102400]);
            }
            ProgramId::Sha256300kb => {
                builder.write(&vec![0u8; 102400 * 3]);
            }
            ProgramId::Sha2561mb => {
                builder.write(&vec![0u8; 1048576]);
            }
            ProgramId::Sha2563mb => {
                builder.write(&vec![0u8; 1048576 * 3]);
            }
            ProgramId::Keccak256100kb => {
                builder.write(&vec![0u8; 102400]);
            }
            ProgramId::Keccak256300kb => {
                builder.write(&vec![0u8; 102400 * 3]);
            }
            ProgramId::Keccak2561mb => {
                builder.write(&vec![0u8; 1048576]);
            }
            ProgramId::Keccak2563mb => {
                builder.write(&vec![0u8; 1048576 * 3]);
            },
            ProgramId::Rsp20526626 => {
                let input = include_bytes!("../../fixtures/20526626.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20526627 => {
                let input = include_bytes!("../../fixtures/20526627.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20526628 => {
                let input = include_bytes!("../../fixtures/20526628.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20526629 => {
                let input = include_bytes!("../../fixtures/20526629.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20526630 => {
                let input = include_bytes!("../../fixtures/20526630.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20528708 => {
                let input = include_bytes!("../../fixtures/20528708.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20528709 => {
                let input = include_bytes!("../../fixtures/20528709.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20528710 => {
                let input = include_bytes!("../../fixtures/20528710.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20528711 => {
                let input = include_bytes!("../../fixtures/20528711.bin");
                builder.write(&input.to_vec());
            }
            ProgramId::Rsp20528712 => {
                let input = include_bytes!("../../fixtures/20528712.bin");
                builder.write(&input.to_vec());
            },
            ProgramId::ECDSAVerify => {
                builder.write(&rand_ecdsa_signature());
            },
            ProgramId::EDDSAVerify => {
                let times: u8 = 100;
                builder.write(&times);

                for _ in 0..times {
                    builder.write(&rand_eddsa_signature());
                }
            },
            ProgramId::Helios => {
                let input = include_bytes!("../../fixtures/helios/proof_inputs.cbor");
                builder.write(&input.to_vec());
            },
            ProgramId::Groth16ProofVerify => { 
                const PROOF: &str = include_str!("../../fixtures/risc0/proof.json");
                const PUBLIC_INPUTS: &str = include_str!("../../fixtures/risc0/public.json");
                const VERIFICATION_KEY: &str = include_str!("../../fixtures/risc0/verification_key.json");

                // Verification_key, proof and public witness generated by SnarkJS using Groth16 over BN254
                // (https://docs.circom.io/getting-started/proving-circuits/)
                let proof_json: ProofJson = serde_json::from_str(PROOF).unwrap();
                let public_inputs_json = PublicInputsJson {
                    values: serde_json::from_str(PUBLIC_INPUTS).unwrap(),
                };

                let verifying_key_json: VerifyingKeyJson = serde_json::from_str(VERIFICATION_KEY).unwrap();
                // Convert from the JSON data structure, with string encoded values.
                let seal: Seal = proof_json.try_into().unwrap();
                let public_inputs: Vec<Fr> = public_inputs_json.to_scalar().unwrap();
                let verifying_key: VerifyingKey = verifying_key_json.verifying_key().unwrap();
                
                builder.write(&(seal, public_inputs, verifying_key)).unwrap();
            },
            ProgramId::ZKEmail => {
                #[derive(Serialize, Deserialize, Debug, Clone)]
                #[serde(rename_all = "camelCase")]
                struct EmailInputs {
                    public_key: String,
                    signature: String,
                    headers: String,
                    body: String,
                    body_hash: String,
                }

                const EMAIL_JSON: &[u8] = include_bytes!("../../fixtures/zk-email/email.json");
                let email_input = serde_json::from_slice::<EmailInputs>(EMAIL_JSON).unwrap();

                builder.write(&email_input).unwrap();
            },
            _ => {}
        }
        let env = builder.build().unwrap();
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
        let (compressed_proof, compress_duration) =
            time_operation(|| prover.compress(&ProverOpts::succinct(), &receipt).unwrap());

        // Verify the recursive proof
        let ((), recursive_verify_duration) =
            time_operation(|| compressed_proof.verify(image_id).unwrap());

        let succinct_receipt = compressed_proof.inner.succinct().unwrap();

        let mut shrink_prove_duration = std::time::Duration::from_secs(0);
        let mut wrap_prove_duration = std::time::Duration::from_secs(0);
        let mut groth16_prove_duration = std::time::Duration::from_secs(0);

        if args.groth16 {
            // Bn254 wrapping duration
            let (bn254_proof, tmp_bn254_compress_duration) = time_operation(|| {
                prover.identity_p254(&compressed_proof.inner.succinct().unwrap()).unwrap()
            });
            let seal_bytes = bn254_proof.get_seal_bytes();
            println!("Running groth16 wrapper");
            let (_groth16_proof, tmp_groth16_duration) =
                time_operation(|| risc0_zkvm::stark_to_snark(&seal_bytes).unwrap());

            println!("Done running groth16");
            wrap_prove_duration = tmp_bn254_compress_duration;
            groth16_prove_duration = tmp_groth16_duration;
        }

        // Get the recursive proof size.
        let recursive_proof_size = succinct_receipt.seal.len() * 4;
        let prove_duration = core_prove_duration + compress_duration;

        let core_khz = cycles as f64 / core_prove_duration.as_secs_f64() / 1_000.0;
        let overall_khz = cycles as f64 / prove_duration.as_secs_f64() / 1_000.0;

        // Create the performance report.
        let report = PerformanceReport {
            priority: args.program.priority(),
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
            core_khz,
            compress_prove_duration: compress_duration.as_secs_f64(),
            compress_verify_duration: recursive_verify_duration.as_secs_f64(),
            compress_proof_size: recursive_proof_size,
            overall_khz,
            gas: gas_amount(&args.program),
            hashes_per_second: hashes_per_second(&args.program, prove_duration),
            hash_bytes_per_second: hash_bytes_per_second(&args.program, prove_duration),
            shrink_prove_duration: shrink_prove_duration.as_secs_f64(),
            wrap_prove_duration: wrap_prove_duration.as_secs_f64(),
            groth16_prove_duration: groth16_prove_duration.as_secs_f64(),
            plonk_prove_duration: 0.0,
        };

        println!("report: {:#?}", report);

        report
    }

    #[cfg(not(feature = "risc0"))]
    pub fn eval(_args: &EvalArgs) -> PerformanceReport {
        panic!("RISC0 feature is not enabled. Please compile with --features risc0");
    }
}
