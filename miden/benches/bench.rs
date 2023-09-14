extern crate miden_bench;

use bench::Benchmark;
use miden_bench::{
    blake3::blake3,
    fib::fib,
    merkle::{membership, merge_trees},
    rpo::rpo,
    sha::sha,
};
use shared::{
    hash::{HashFn, rpo::Rpo},
    tree_size_n,
};

fn main() {
    let mut bench = Benchmark::from_env("miden");

    bench.benchmark("multiple assert proof compression", |b| {
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
    });

    bench.benchmark("multiple sha256 proof compression", |b| {
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
    });

    bench.benchmark_with(
        "Fibonacci",
        &[
            ("1", 1),
            ("10", 10),
            ("100", 100),
            ("1000", 1000),
            ("10000", 10000),
            ("100000", 100000),
        ],
        |b, p| {
            let (setup, vm) = fib(*p);
            let last_vm_state = vm.last().unwrap().unwrap();
            b.run(setup);
            b.log("cycles", last_vm_state.clk as usize);
        },
    );

    // Averages 464.654 cycles per byte
    bench.benchmark_with(
        "SHA256",
        &[
            ("1k bytes", 1000),
            ("10k bytes", 10000),
            // ("100k bytes", 100000),
        ],
        |b, p| {
            let (setup, vm) = sha(*p);
            let last_vm_state = vm.last().unwrap().unwrap();
            let proof = b.run(setup);
            b.log("cycles", last_vm_state.clk as usize);

            let proof = proof.to_bytes();
            b.log("proof_size_bytes", proof.len());
            b.log(
                "compressed_proof_size_bytes",
                zstd::encode_all(&proof[..], 21).unwrap().len(),
            );
        },
    );

    // Averages 153.854 cycles per byte
    bench.benchmark_with(
        "Blake3",
        &[
            ("1k bytes", 1000),
            ("10k bytes", 10000),
            // ("100k bytes", 100000),
        ],
        |b, p| {
            let (setup, vm) = blake3(*p);
            let last_vm_state = vm.last().unwrap().unwrap();
            let proof = b.run(setup);
            b.log("cycles", last_vm_state.clk as usize);

            let proof = &proof.to_bytes();
            b.log("proof_size_bytes", proof.len());
            b.log(
                "compressed_proof_size_bytes",
                zstd::encode_all(&proof[..], 21).unwrap().len(),
            );
        },
    );

    // Averages 0.869 cycles per byte
    bench.benchmark_with(
        "RPO",
        &[
            ("1k bytes", 1000),
            ("10k bytes", 10000),
            // ("100k bytes", 100000),
        ],
        |b, p| {
            let (setup, vm) = rpo(*p);
            let last_vm_state = vm.last().unwrap().unwrap();
            let proof = b.run(setup);
            b.log("cycles", last_vm_state.clk as usize);

            let proof = &proof.to_bytes();
            b.log("proof_size_bytes", proof.len());
            b.log(
                "compressed_proof_size_bytes",
                zstd::encode_all(&proof[..], 21).unwrap().len(),
            );
        },
    );

    bench.benchmark_with(
        "Merkle Tree Merge",
        &[
            ("2^10 + 2^10", (tree_size_n(10), tree_size_n(10))),
            ("2^10 + 2^20", (tree_size_n(10), tree_size_n(20))),
            ("2^20 + 2^20", (tree_size_n(20), tree_size_n(20))),
        ],
        |b, (tree1, tree2)| {
            let (prove, iter) = merge_trees(tree1, tree2);

            let proof = b.run(prove);
            let proof_bytes = proof.to_bytes();
            let proof_bytes_zstd = zstd::encode_all(&*proof_bytes, 21).unwrap();

            b.log("proof_size_bytes", proof_bytes.len());
            b.log("compressed_proof_size_bytes", proof_bytes_zstd.len());
            let last_vm_state = iter.last().unwrap().unwrap();

            b.log("cycles", last_vm_state.clk as usize);
        },
    );

    bench.benchmark("Merkle Membership", |b| {
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
    });

    bench.output();
}
