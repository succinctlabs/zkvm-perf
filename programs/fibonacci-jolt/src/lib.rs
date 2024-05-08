#![no_main]

#[jolt::provable]
fn fibonacci() -> u32 {
    let n = 300000;
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let sum = (a + b) % 7919; // Mod to avoid overflow
        a = b;
        b = sum;
    }
    b
}
