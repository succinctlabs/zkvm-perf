#![no_main]
risc0_zkvm::guest::entry!(main);

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use risc0_zkvm::guest::env;

fn main() {
    let (signer, message, signature): (VerifyingKey, Vec<u8>, Signature) = env::read();

    signer.verify(&message, &signature).expect("Ed25519 signature verification failed");

    env::commit(&(signer, message));
}
