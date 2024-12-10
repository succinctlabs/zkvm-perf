
#![no_main]
risc0_zkvm::guest::entry!(main);

use risc0_groth16::{Fr, Seal, Verifier, VerifyingKey};
use risc0_zkvm::{guest::env, sha::Digestible};

pub fn main() {
    let (seal, public_inputs, verifying_key): (Seal, Vec<Fr>, VerifyingKey) = env::read();

    Verifier::new(&seal, &public_inputs, &verifying_key)
        .unwrap()
        .verify()
        .unwrap();

    env::commit(&(verifying_key.digest(), public_inputs.digest()));
}
