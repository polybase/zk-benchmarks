extern crate host;

use std::rc::Rc;

use bench::{Benchmark, BenchmarkRun};
use host::{blake3::blake3, fib::fib, sha::sha};
use risc0_zkvm::{prove::get_prover, Receipt, Session};

fn main() {
    let prover_getter = |name: &'static str| || get_prover(name);
    let (bench_name, prover) = match std::env::args().nth(1) {
        Some(prover) if prover == "multi-cpu" => ("risc_zero-multi-cpu", prover_getter("cpu")),
        Some(prover) if prover == "metal" => ("risc_zero-metal", prover_getter("metal")),
        Some(prover) if prover == "cuda" => ("risc_zero-cuda", prover_getter("cuda")),
        Some(_) | None => {
            println!("Usage: bench <multi-cpu, metal or cuda>");
            std::process::exit(1);
        }
    };

    let mut bench = Benchmark::from_env("risc_zero");

    bench.benchmark("assert", |b| {
        let prover = prover();

        let prove = host::assert::assert(Rc::clone(&prover), 1, 2);
        log_session(&b.run(prove), b);
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
        |b, n| {
            let prover = prover();

            let prove = fib(Rc::clone(&prover), *n);
            log_session(&b.run(prove), b);
        },
    );

    bench.benchmark_with(
        "SHA256",
        &[("1k bytes", 1), ("10k bytes", 10), ("100k bytes", 100)],
        |b, n| {
            let prover = prover();

            let prove = sha(Rc::clone(&prover), *n);
            log_session(&b.run(prove), b);
        },
    );

    bench.benchmark_with(
        "Blake3",
        &[("1k bytes", 1), ("10k bytes", 10), ("100k bytes", 100)],
        |b, n| {
            let prover = prover();

            let prove = blake3(Rc::clone(&prover), *n);
            log_session(&b.run(prove), b);
        },
    );

    bench.output();
}

fn log_session((receipt, session): &(Receipt, Session), b: &mut BenchmarkRun) {
    let segments = session.resolve().unwrap();
    let (cycles, insn_cycles) = segments
        .iter()
        .fold((0, 0), |(cycles, insn_cycles), segment| {
            (
                cycles + (1 << segment.po2),
                insn_cycles + segment.insn_cycles,
            )
        });
    b.log("cycles", cycles);
    b.log("instruction_cycles", insn_cycles);

    let proof = bincode::serialize(receipt).unwrap();
    b.log("proof_size_bytes", proof.len());
    b.log(
        "compressed_proof_size_bytes",
        zstd::encode_all(&proof[..], 21).unwrap().len(),
    );
}
