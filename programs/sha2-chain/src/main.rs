// Code taken from JOLT's benchmark repository.

#![no_main]

use sha2::{Digest, Sha256};
use std::hint::black_box;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let input = [5u8; 32];
    let num_iters: u32 = 2500;
    let mut hash = input;
    for _ in 0..num_iters {
        let mut hasher = Sha256::new();
        hasher.update(hash);
        let res = &hasher.finalize();
        hash = Into::<[u8; 32]>::into(*res);
    }
}
