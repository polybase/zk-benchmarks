#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn bench_blake3_n(n_thousands: usize) {
    let arr = [123u8; 1000];
    for _ in 0..n_thousands {
        blake3::hash(&arr);
    }
}

pub fn main() {
    let n = env::read();
    bench_blake3_n(n);
}
