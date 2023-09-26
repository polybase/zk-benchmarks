use benchy::{benchmark, BenchmarkRun};
use snarkvm::{
    prelude::{Execution, Testnet3},
    utilities::CanonicalSerialize,
};

#[benchmark]
fn assert(b: &mut BenchmarkRun) {
    let run = leo::assert();

    let execution = b.run(|| run());
    report(b, execution);
}

#[benchmark("Fibonacci", [
    ("1", 1),
    ("10", 10),
    ("100", 100),
    ("1000", 1000),
    ("10000", 10000),
    // ("100000", 100000), // Failes to compile with VerboseError { errors: [("\n", Nom(MapRes)) ... <code> ... <expanded for loop> add.w r99997 r99998 into r99999;\n", Nom(Many1))] }'
])]
fn fibonacci(b: &mut BenchmarkRun, n: u32) {
    let run = leo::fib(n);

    let execution = b.run(|| run());
    report(b, execution);
}

#[benchmark("SHA-3-256", [
    ("1k bytes", 1000),
    ("10k bytes", 10000),
])]
fn sha_3_256(b: &mut BenchmarkRun, n_bytes: u32) {
    let run = leo::sha_3_256(n_bytes);

    let execution = b.run(|| run());
    report(b, execution);
}

#[benchmark("Pedersen", [
    ("1k bytes", 1000),
    ("10k bytes", 10000),
    ("100k bytes", 100000),
])]
fn pedersen_128(b: &mut BenchmarkRun, n_bytes: u32) {
    let run = leo::pedersen_128(n_bytes);

    let execution = b.run(|| run());
    report(b, execution);
}

fn report(b: &mut BenchmarkRun, execution: Execution<Testnet3>) {
    b.log(
        "proof_size_bytes",
        execution.proof().unwrap().uncompressed_size(),
    );
    b.log(
        "compressed_proof_size_bytes",
        execution.proof().unwrap().compressed_size(),
    );
}

benchy::main!("leo", assert, fibonacci, sha_3_256, pedersen_128);
