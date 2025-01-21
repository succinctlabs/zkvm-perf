#![no_main]
sp1_zkvm::entrypoint!(main);

use ed25519_dalek::{Signature, Verifier, VerifyingKey};

fn main() {
    let times: u8 = sp1_zkvm::io::read();
    for _ in 0..times {
        verify_inner();
    }
}

fn verify_inner() {
    let (signer, message, signature): (VerifyingKey, Vec<u8>, Signature) = sp1_zkvm::io::read();

    signer.verify(&message, &signature).expect("Ed25519 signature verification failed");

    sp1_zkvm::io::commit(&(signer, message));
}