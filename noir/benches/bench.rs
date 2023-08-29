extern crate noir;

use bench::Benchmark;
use noir::{InputMap, InputValue, Proof};

fn main() {
    let mut bench = Benchmark::from_env("noir");

    let backend = noir::backends::ConcreteBackend::default();
    let dir = std::env::current_dir().expect("current dir to exist");

    bench.benchmark("assert", |b| {
        let mut inputs = InputMap::new();

        inputs.insert("x".to_string(), InputValue::Field((1_u128).into()));
        inputs.insert("y".to_string(), InputValue::Field((2_u128).into()));

        let proof = Proof::new(&backend, "assert", dir.join("pkgs/assert"), &inputs);
        b.run(|| proof.prove());
        // b.log("cycles", last_vm_state.clk as usize);
    });

    bench.output();
}
