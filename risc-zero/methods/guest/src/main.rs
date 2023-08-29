#![no_main]
#![no_std]

use crypto::hash::rpo::Rpo256;
use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

risc0_zkvm::guest::entry!(main);

#[allow(dead_code)]
fn bench_rpo() {
    let arr = [123u8; 10];
    Rpo256::hash(&arr);
}

fn bench_sha_n(n_thousands: usize) {
    let arr = [123u8; 1000];
    for _ in 0..n_thousands {
        Impl::hash_bytes(&arr);
    }
}

fn bench_blake3_n(n_thousands: usize) {
    let arr = [123u8; 1000];
    for _ in 0..n_thousands {
        blake3::hash(&arr);
    }
}

pub fn main() {
    let program = env::read::<u8>();
    match program {
        // sha256
        0 => {
            let n = env::read();
            bench_sha_n(n);
        }
        // blake3
        1 => {
            let n = env::read();
            bench_blake3_n(n);
        }
        x => panic!("unknown program {x:?}"),
    }
}
