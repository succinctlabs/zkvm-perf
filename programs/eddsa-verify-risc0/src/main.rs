#![no_main]
risc0_zkvm::guest::entry!(main);

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use risc0_zkvm::guest::env;

fn main() {
    let times: u8 = env::read();

    for _ in 0..times {
        verify_inner();
    }
}

fn verify_inner() {
    let (signer, message, signature): (VerifyingKey, Vec<u8>, Signature) = env::read();

    signer.verify(&message, &signature).expect("Ed25519 signature verification failed");

    env::commit(&(signer, message));
}