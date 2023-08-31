extern crate host;

use std::rc::Rc;

use bench::{Benchmark, BenchmarkRun};
use host::{blake3::blake3, sha::sha};
use risc0_zkvm::{prove::get_prover, Session};

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

    let mut bench = Benchmark::from_env(bench_name);

    bench.benchmark("assert", |b| {
        let prover = prover();

        let prove = host::assert::assert(Rc::clone(&prover), 1, 2);
        log_session(&b.run(prove), b);
    });

    bench.benchmark_with(
        "SHA256",
        &[
            ("1000 bytes", 1),
            ("10000 bytes", 10),
            ("100000 bytes", 100),
        ],
        |b, n| {
            let prover = prover();

            let prove = sha(Rc::clone(&prover), *n);
            log_session(&b.run(prove), b);
        },
    );

    bench.benchmark_with(
        "Blake3",
        &[
            ("1000 bytes", 1),
            ("10000 bytes", 10),
            ("100000 bytes", 100),
        ],
        |b, n| {
            let prover = prover();

            let prove = blake3(Rc::clone(&prover), *n);
            log_session(&b.run(prove), b);
        },
    );

    bench.output();
}

fn log_session(session: &Session, b: &mut BenchmarkRun) {
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
}
