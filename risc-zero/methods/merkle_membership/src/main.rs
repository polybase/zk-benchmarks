#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::hint::black_box;
use risc0_zkvm::{
    guest::{env, sha::Impl},
    sha::{Digest, Sha256},
};

risc0_zkvm::guest::entry!(main);

fn main() {
    let hash_bytes: Vec<u8> = env::read();
    let hash_words: Vec<u32> = hash_bytes
        .chunks(4)
        .map(|slice| u32::from_be_bytes(slice.try_into().unwrap()))
        .collect();
    let hash = hash_words
        .chunks(8)
        .map(|words| Digest::new(words.try_into().unwrap()))
        .reduce(|a, b| *<Impl as Sha256>::hash_pair(&a, &b))
        .unwrap();

    black_box(hash);
}
