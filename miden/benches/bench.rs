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
        b.run(setup);
        b.log("cycles", last_vm_state.clk as usize);
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
            b.run(setup);
            b.log("cycles", last_vm_state.clk as usize);
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
            b.run(setup);
            b.log("cycles", last_vm_state.clk as usize);
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
            b.run(setup);
            b.log("cycles", last_vm_state.clk as usize);
        },
    );

    bench.output();
}
