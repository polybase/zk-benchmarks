extern crate host;

use bench::{Benchmark, BenchmarkRun};
use host::sha::sha;
use risc0_zkvm::Session;

fn main() {
    let mut bench = Benchmark::new("risc_zero");

    bench.benchmark_with("SHA256", &[1, 100, 1000], |b, n| {
        let prove = sha(*n);
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
