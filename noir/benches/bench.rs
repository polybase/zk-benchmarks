extern crate noir;
extern crate rand;

use bench::Benchmark;
use noir::{InputMap, InputValue, Proof};
use rand::Rng;

fn main() {
    let mut bench = Benchmark::from_env("noir");

    let backend = noir::backends::ConcreteBackend::default();
    let dir = std::env::current_dir().expect("current dir to exist");

    bench.benchmark_with(
        "SHA256",
        &[
            ("1 byte", 1),
            ("10 bytes", 10),
            ("100 bytes", 100),
            ("1000 bytes", 1000),
        ],
        |b, p| {
            let mut inputs = InputMap::new();

            // Generate random bytes
            let bytes = generate_random_u8_slice(*p)
                .iter()
                .map(|b| InputValue::Field((*b as u128).into()))
                .collect::<Vec<_>>();

            inputs.insert("x".to_string(), InputValue::Vec(bytes));

            let proof = Proof::new(
                &backend,
                "sha256",
                dir.join(format!("pkgs/sha256/{}", p)),
                &inputs,
            );
            b.run(|| proof.prove());
        },
    );

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

fn generate_random_u8_slice(len: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(rng.gen::<u8>());
    }
    vec
}
