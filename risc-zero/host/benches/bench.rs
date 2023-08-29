extern crate host;

use std::rc::Rc;

use bench::{Benchmark, BenchmarkRun};
use host::sha::sha;
use risc0_zkvm::{prove::get_prover, Session};

fn main() {
    let (bench_name, prover) = match std::env::args().nth(1) {
        Some(prover) if prover == "multi-cpu" => ("risc_zero-multi-cpu", get_prover("cpu")),
        Some(prover) if prover == "metal" => ("risc_zero-metal", get_prover("metal")),
        Some(prover) if prover == "cuda" => ("risc_zero-cuda", get_prover("cuda")),
        Some(_) | None => {
            println!("Usage: bench <multi-cpu, metal or cuda>");
            std::process::exit(1);
        }
    };

    let mut bench = Benchmark::from_env(bench_name);

    bench.benchmark_with("SHA256", &[1, 10, 100, 1000], |b, n| {
        let prove = sha(Rc::clone(&prover), *n);
        log_session(&b.run(prove), b);
    });

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
    b.log("instructions", insn_cycles);
}
