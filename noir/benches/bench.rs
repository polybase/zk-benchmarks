extern crate noir;
extern crate rand;

use bench::Benchmark;
use noir::{InputMap, InputValue, Proof};
use rand::Rng;

fn main() {
    let mut bench = Benchmark::from_env("noir");

    bench.benchmark_with(
        "SHA256",
        &[
            ("1k bytes", 1000),
            ("10k bytes", 10000),
            // ("100k bytes", 100000),
        ],
        |b, p| {
            let backend = noir::backends::ConcreteBackend::default();
            let dir = std::env::current_dir().expect("current dir to exist");

            let mut inputs = InputMap::new();

            // Generate random bytes
            let bytes = generate_random_u8_slice(*p)
                .iter()
                .map(|b| InputValue::Field((*b as u128).into()))
                .collect::<Vec<_>>();

            inputs.insert("x".to_string(), InputValue::Vec(bytes));

            let proof = Proof::new(&backend, "sha256", dir.join(format!("pkgs/sha256/{}", p)));
            let proof_bytes = b.run(|| proof.run_and_prove(&inputs));
            b.log("proof_size_bytes", proof_bytes.len());
            b.log(
                "compressed_proof_size_bytes",
                zstd::encode_all(&proof_bytes[..], 21).unwrap().len(),
            );
        },
    );

    bench.benchmark("assert", |b| {
        let backend = noir::backends::ConcreteBackend::default();
        let dir = std::env::current_dir().expect("current dir to exist");

        let mut inputs = InputMap::new();

        inputs.insert("x".to_string(), InputValue::Field((1_u128).into()));
        inputs.insert("y".to_string(), InputValue::Field((2_u128).into()));

        let proof = Proof::new(&backend, "assert", dir.join("pkgs/assert"));
        let proof_bytes = b.run(|| proof.run_and_prove(&inputs));
        b.log("proof_size_bytes", proof_bytes.len());
        b.log(
            "compressed_proof_size_bytes",
            zstd::encode_all(&proof_bytes[..], 21).unwrap().len(),
        );
        // b.log("cycles", last_vm_state.clk as usize);
    });

    bench.benchmark_with(
        "Fibonacci",
        &[
            ("1", 1),
            ("10", 10),
            ("100", 100),
            ("1000", 1000),
            ("10000", 10000),
            ("100000", 100000),
            ("1000000", 1000000),
        ],
        |b, p| {
            let backend = noir::backends::ConcreteBackend::default();
            let dir = std::env::current_dir().expect("current dir to exist");

            let mut inputs = InputMap::new();

            inputs.insert("a_start".to_string(), InputValue::Field((0_u128).into()));
            inputs.insert("b_start".to_string(), InputValue::Field((1_u128).into()));

            let proof = Proof::new(&backend, "fib", dir.join(format!("pkgs/fib/{}", p)));
            let proof_bytes = b.run(|| proof.run_and_prove(&inputs));
            b.log("proof_size_bytes", proof_bytes.len());
            b.log(
                "compressed_proof_size_bytes",
                zstd::encode_all(&proof_bytes[..], 21).unwrap().len(),
            );
        },
    );

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
