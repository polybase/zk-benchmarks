extern crate host;

use std::rc::Rc;

use benchy::{benchmark, BenchmarkRun};
use host::{blake3::blake3, fib::fib, sha::sha};
use risc0_zkvm::{prove::get_prover, Receipt, Session};

fn prover() -> Rc<dyn risc0_zkvm::prove::Prover> {
    match std::env::args().nth(1) {
        Some(prover) if prover == "multi-cpu" => get_prover("cpu"),
        Some(prover) if prover == "metal" => get_prover("metal"),
        Some(prover) if prover == "cuda" => get_prover("cuda"),
        Some(_) | None => {
            panic!("Usage: bench <multi-cpu, metal or cuda>");
        }
    }
}

#[benchmark]
fn assert(b: &mut BenchmarkRun) {
    let prover = prover();

    let prove = host::assert::assert(Rc::clone(&prover), 1, 2);
    log_session(&b.run(prove), b);
}

#[benchmark("Fibonacci", [
    ("1", 1),
    ("10", 10),
    ("100", 100),
    ("1000", 1000),
    ("10000", 10000),
    ("100000", 100000),
])]
fn fibonacci(b: &mut BenchmarkRun, n: u32) {
    let prover = prover();

    let prove = fib(Rc::clone(&prover), n);
    log_session(&b.run(prove), b);
}

#[benchmark("SHA256", [
    ("1k bytes", 1),
    ("10k bytes", 10),
    ("100k bytes", 100),
])]
fn sha256(b: &mut BenchmarkRun, n: usize) {
    let prover = prover();

    let prove = sha(Rc::clone(&prover), n);
    log_session(&b.run(prove), b);
}

#[benchmark("Blake3", [
    ("1k bytes", 1),
    ("10k bytes", 10),
    ("100k bytes", 100),
])]
fn blake3_bench(b: &mut BenchmarkRun, n: usize) {
    let prover = prover();

    let prove = blake3(Rc::clone(&prover), n);
    log_session(&b.run(prove), b);
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

benchy::main!(assert, fibonacci, sha256, blake3_bench);
