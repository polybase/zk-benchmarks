extern crate miden_bench;

use bench::{benchmark, BenchmarkRun};
use miden_bench::{
    blake3::blake3,
    fib::fib,
    merkle::{membership, merge_trees},
    rpo::rpo,
    sha::sha,
};
use shared::{
    hash::{rpo::Rpo, HashFn},
    tree_size_n, Tree,
};

#[benchmark]
fn assert(b: &mut BenchmarkRun) {
    let (setup, vm) = miden_bench::assert::assert(1, 2);
    let last_vm_state = vm.last().unwrap().unwrap();
    let _proof = b.run(setup);
    b.log("cycles", last_vm_state.clk as usize);
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

#[benchmark("Merkle Tree Merge", [
    ("1 + 1", (tree_size_n(0), tree_size_n(0))),
    ("2^10 + 2^10", (tree_size_n(10), tree_size_n(10))),
    ("2^10 + 2^20", (tree_size_n(10), tree_size_n(20))),
    ("2^20 + 2^20", (tree_size_n(20), tree_size_n(20))),
])]
fn merkle_tree_merge(b: &mut BenchmarkRun, (tree1, tree2): (Tree<Rpo>, Tree<Rpo>)) {
    let (prove, iter) = merge_trees(&tree1, &tree2);

    let proof = b.run(prove);
    let proof_bytes = proof.to_bytes();
    let proof_bytes_zstd = zstd::encode_all(&*proof_bytes, 21).unwrap();

    b.log("proof_size_bytes", proof_bytes.len());
    b.log("compressed_proof_size_bytes", proof_bytes_zstd.len());
    let last_vm_state = iter.last().unwrap().unwrap();

    b.log("cycles", last_vm_state.clk as usize);
}

#[benchmark("Merkle Membership")]
fn merkle_membership(b: &mut BenchmarkRun) {
    let vec = core::iter::from_fn(|| Some(Rpo::random()))
        .take(10)
        .collect();
    let (prove, iter) = membership(vec, Rpo::random());

    let proof = b.run(prove);
    let proof_bytes = proof.to_bytes();
    let proof_bytes_zstd = zstd::encode_all(&*proof_bytes, 21).unwrap();

    b.log("proof_size_bytes", proof_bytes.len());
    b.log("compressed_proof_size_bytes", proof_bytes_zstd.len());
    let last_vm_state = iter.last().unwrap().unwrap();

    b.log("cycles", last_vm_state.clk as usize);
}

bench::main!(
    "miden",
    assert,
    multiple_assert_proof_compression,
    multiple_sha256_proof_comperssion,
    fibonacci,
    sha256,
    blake3_bench,
    rpo_bench,
    merkle_tree_merge,
    merkle_membership,
);
