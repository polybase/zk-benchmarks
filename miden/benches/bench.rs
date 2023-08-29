extern crate miden_bench;

use bench::Benchmark;
use miden_bench::{rpo::rpo, sha::sha};

fn main() {
    #[allow(unused_variables)]
    let bench_name = "miden-single-cpu";
    #[cfg(feature = "multi-cpu")]
    let bench_name = "miden-multi-cpu";
    #[cfg(feature = "metal")]
    let bench_name = "miden-metal";
    let mut bench = Benchmark::from_env(bench_name);

    bench.benchmark_with("SHA256", &[1, 10, 100, 1000], |b, p| {
        let (setup, vm) = sha(*p);
        let last_vm_state = vm.last().unwrap().unwrap();
        b.run(setup);
        b.log("cycles", last_vm_state.clk as usize);
    });

    bench.benchmark_with("RPO", &[10000, 100000, 1000000], |b, p| {
        let (setup, vm) = rpo(*p);
        let last_vm_state = vm.last().unwrap().unwrap();
        b.run(setup);
        b.log("cycles", last_vm_state.clk as usize);
    });

    bench.output();
}
