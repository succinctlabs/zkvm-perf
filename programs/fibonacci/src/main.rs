#![no_main]

use std::hint::black_box;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let sum = (a + b) % 7919; // Mod to avoid overflow
        a = b;
        b = sum;
    }
    b
}

pub fn main() {
    #[cfg(feature = "risc0")]
    let n: u32 = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let n: u32 = sp1_zkvm::io::read();
    let result = black_box(fibonacci(black_box(n)));
    println!("result: {}", result);
}
