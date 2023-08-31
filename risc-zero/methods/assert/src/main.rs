#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let a: u32 = env::read();
    let b: u32 = env::read();

    assert!(a != b);
}
