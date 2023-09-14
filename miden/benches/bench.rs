extern crate miden_bench;

use bench::{benchmark, BenchmarkRun};
use miden_bench::{blake3::blake3, fib::fib, rpo::rpo, sha::sha};

#[benchmark]
fn assert(b: &mut BenchmarkRun) {
    let (setup, vm) = miden_bench::assert::assert(1, 2);
    let last_vm_state = vm.last().unwrap().unwrap();
    let proof = b.run(setup);
    b.log("cycles", last_vm_state.clk as usize);

    let proof = &proof.to_bytes();
    b.log("proof_size_bytes", proof.len());
    b.log(
        "compressed_proof_size_bytes",
        zstd::encode_all(&proof[..], 21).unwrap().len(),
    );
}

#[benchmark("multiple assert proof compression")]
fn multiple_assert_proof_compression(b: &mut BenchmarkRun) {
    let mut proofs = Vec::new();
    for x in 0..10 {
        let (setup, _) = miden_bench::assert::assert(x, x + 1);
        let proof = setup();
        proofs.push(proof);
    }

    let proof_bytes = proofs
        .into_iter()
        .map(|p| p.to_bytes())
        .collect::<Vec<_>>()
        .concat();

    b.log("proof_size_bytes", proof_bytes.len());
    b.log(
        "compressed_proof_size_bytes",
        zstd::encode_all(&proof_bytes[..], 21).unwrap().len(),
    );
}

#[benchmark("multiple sha256 proof compression")]
fn multiple_sha256_proof_comperssion(b: &mut BenchmarkRun) {
    let mut proofs = Vec::new();
    for x in 0..10 {
        let (setup, _) = sha(x + 1);
        let proof = setup();
        proofs.push(proof);
    }

    let proof_bytes = proofs
        .into_iter()
        .map(|p| p.to_bytes())
        .collect::<Vec<_>>()
        .concat();

    b.log("proof_size_bytes", proof_bytes.len());
    b.log(
        "compressed_proof_size_bytes",
        zstd::encode_all(&proof_bytes[..], 21).unwrap().len(),
    );
}

#[benchmark("Fibonacci", [
    ("1", 1),
    ("10", 10),
    ("100", 100),
    ("1000", 1000),
    ("10000", 10000),
    ("100000", 100000),
])]
fn fibonacci(b: &mut BenchmarkRun, p: u32) {
    let (setup, vm) = fib(p);
    let last_vm_state = vm.last().unwrap().unwrap();
    b.run(setup);
    b.log("cycles", last_vm_state.clk as usize);
}

// Averages 464.654 cycles per byte
#[benchmark("SHA256", [
    ("1k bytes", 1000),
    ("10k bytes", 10000),
    // ("100k bytes", 100000),
])]
fn sha256(b: &mut BenchmarkRun, p: usize) {
    let (setup, vm) = sha(p);
    let last_vm_state = vm.last().unwrap().unwrap();
    let proof = b.run(setup);
    b.log("cycles", last_vm_state.clk as usize);

    let proof = proof.to_bytes();
    b.log("proof_size_bytes", proof.len());
    b.log(
        "compressed_proof_size_bytes",
        zstd::encode_all(&proof[..], 21).unwrap().len(),
    );
}

// Averages 153.854 cycles per byte
#[benchmark("Blake3", [
    ("1k bytes", 1000),
    ("10k bytes", 10000),
    // ("100k bytes", 100000),
])]
fn blake3_bench(b: &mut BenchmarkRun, p: usize) {
    let (setup, vm) = blake3(p);
    let last_vm_state = vm.last().unwrap().unwrap();
    let proof = b.run(setup);
    b.log("cycles", last_vm_state.clk as usize);

    let proof = &proof.to_bytes();
    b.log("proof_size_bytes", proof.len());
    b.log(
        "compressed_proof_size_bytes",
        zstd::encode_all(&proof[..], 21).unwrap().len(),
    );
}

// Averages 0.869 cycles per byte
#[benchmark("RPO", [
    ("1k bytes", 1000),
    ("10k bytes", 10000),
    // ("100k bytes", 100000),
])]
fn rpo_bench(b: &mut BenchmarkRun, p: usize) {
    let (setup, vm) = rpo(p);
    let last_vm_state = vm.last().unwrap().unwrap();
    let proof = b.run(setup);
    b.log("cycles", last_vm_state.clk as usize);

    let proof = &proof.to_bytes();
    b.log("proof_size_bytes", proof.len());
    b.log(
        "compressed_proof_size_bytes",
        zstd::encode_all(&proof[..], 21).unwrap().len(),
    );
}

bench::main!(
    "miden",
    assert,
    multiple_assert_proof_compression,
    multiple_sha256_proof_comperssion,
    fibonacci,
    sha256,
    blake3_bench,
    rpo_bench
);
