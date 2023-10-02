#![no_main]

use core::hint::black_box;
use nalgebra::Matrix2;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let n: u32 = env::read();
    // Prevent the compiler from optimizing away the computation.
    black_box(fibonacci(n));
}

fn fibonacci(n: u32) -> u64 {
    let mut mat = Matrix2::new(1, 1, 1, 0);
    mat = mat.pow(n - 1);
    mat[(0, 0)]
}
