#![no_main]

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

use std::str::FromStr;

use base64::prelude::*;

#[cfg(feature = "risc0")]
use rsa::{BigUint, Pkcs1v15Sign, RsaPublicKey};
#[cfg(feature = "sp1")]
use sp1_rsa::{BigUint, Pkcs1v15Sign, RsaPublicKey};

use serde::{Deserialize, Serialize};

#[cfg(feature = "risc0")]
use sha2_risc0::{Digest, Sha256};

#[cfg(feature = "sp1")]
use sha2_sp1::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EmailInputs {
    public_key: String,
    signature: String,
    headers: String,
    body: String,
    body_hash: String,
}

pub fn main() {
    #[cfg(feature = "sp1")]
    let email_inputs: EmailInputs = sp1_zkvm::io::read();

    #[cfg(feature = "risc0")]
    let email_inputs: EmailInputs = risc0_zkvm::guest::env::read();

    let signature_verified = verify_signature(&email_inputs);
    let body_verified = verify_body(&email_inputs);
    
    #[cfg(feature = "sp1")]
    {
        sp1_zkvm::io::commit(&signature_verified);
        sp1_zkvm::io::commit(&body_verified);
    }

    #[cfg(feature = "risc0")]
    {
        risc0_zkvm::guest::env::commit(&signature_verified);
        risc0_zkvm::guest::env::commit(&body_verified);
    }
}

fn verify_body(email_inputs: &EmailInputs) -> bool {
    // get sha256 hash of body
    let mut hasher = Sha256::new();
    hasher.update(email_inputs.body.as_bytes());
    let hash = hasher.finalize();

    // encode hash to base64
    let base64_hash = BASE64_STANDARD.encode(hash);

    // compare computed body hash with signed body hash & print if fails
    base64_hash == email_inputs.body_hash
}

fn verify_signature(email_inputs: &EmailInputs) -> bool {
    // signature scheme: rsa-sha256
    // 1. get sha256 hash of header
    let mut hasher = Sha256::new();
    hasher.update(email_inputs.headers.as_bytes());
    let hash = hasher.finalize();

    // 2. decode the public key from PEM format
    let public_key = RsaPublicKey::new(
        BigUint::from_bytes_be(&BASE64_STANDARD.decode(&email_inputs.public_key).unwrap()),
        // BigUint::from_str(&email_inputs.public_key).unwrap(),
        BigUint::from(65537u64),
    )
    .expect("error decoding public key into PEM format");

    // 3. decode the signature from base64 into binary
    let signature = BASE64_STANDARD
        .decode(&email_inputs.signature)
        .expect("error decoding signature into binary");

    // 4. verify the signature
    // RSASSA-PKCS1-V1_5 magic padding bytes
    // https://crypto.stackexchange.com/questions/86385/initial-value-for-rsa-and-sha-256-signature-encoding
    let prefix: Box<[u8]> = Box::new([
        0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01,
        0x05, 0x00, 0x04, 0x20,
    ]);
    // SHA-256 produces hash output of 32 bytes
    let hash_len = Some(32);
    let padding = Pkcs1v15Sign { hash_len, prefix };
    let result = public_key.verify(padding, &hash, &signature);

    // Print if signature is invalid
    result.is_ok()
}
