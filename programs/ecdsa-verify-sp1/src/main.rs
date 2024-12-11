#![no_main]

use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    EncodedPoint,
};

sp1_zkvm::entrypoint!(main);

fn main() {
    let (encoded_verifying_key, message, signature): (EncodedPoint, Vec<u8>, Signature) = sp1_zkvm::io::read();

    let verifying_key = VerifyingKey::from_encoded_point(&encoded_verifying_key).unwrap();

    verifying_key
        .verify(&message, &signature)
        .expect("ECDSA signature verification failed");

    sp1_zkvm::io::commit(&(encoded_verifying_key, message));
}
