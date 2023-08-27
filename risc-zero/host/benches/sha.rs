extern crate host;

use bench::Benchmark;
use host::sha::run_sha;

fn main() {
    let mut bench = Benchmark::new("risc_zero");

    bench.benchmark_with("SHA256", &[1, 100, 1000], |b, n| {
        b.run(|| {
            run_sha(*n);
        })
    });
}
