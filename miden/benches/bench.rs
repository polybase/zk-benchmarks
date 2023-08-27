extern crate miden_bench;

use bench::Benchmark;
use miden_bench::rpo::rpo;

fn main() {
    let mut bench = Benchmark::new("miden");

    bench.benchmark_with("RPO", &[10000, 100000, 1000000], |b, p| {
        let (setup, vm) = rpo(*p);
        let last_vm_state = vm.last().unwrap().unwrap();
        b.run(setup);
        b.log("cycles", last_vm_state.clk as usize);
    });

    bench.output();
}
