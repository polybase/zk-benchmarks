extern crate miden_bench;

use bench::Benchmark;
use miden_bench::{blake3::blake3, rpo::rpo, sha::sha};

fn main() {
    #[allow(unused_variables)]
    let bench_name = "miden-single-cpu";
    #[cfg(feature = "multi-cpu")]
    let bench_name = "miden-multi-cpu";
    #[cfg(feature = "metal")]
    let bench_name = "miden-metal";
    let mut bench = Benchmark::from_env(bench_name);

    bench.benchmark("assert", |b| {
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
    });

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

    // Averages 464.654 cycles per byte
    bench.benchmark_with(
        "SHA256",
        &[
            ("1 byte", 1),
            ("10 bytes", 10),
            ("100 bytes", 100),
            ("1000 bytes", 1000),
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
            ("1 byte", 1),
            ("10 bytes", 10),
            ("100 bytes", 100),
            ("1000 bytes", 1000),
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
            ("10000 bytes", 10000),
            ("100000 bytes", 100000),
            ("1000000 bytes", 1000000),
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

    bench.output();
}
