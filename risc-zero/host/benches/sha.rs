use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};
use risc0_zkvm::{prove::get_prover, ExecutorEnv};

fn main() {
    run_sha(10000 / 1000); // 10k bytes
    run_sha(100000 / 1000); // 100k bytes
    run_sha(1000000 / 1000); // 1M bytes
}

fn run_sha(n_thousands: usize) {
    let env = ExecutorEnv::builder()
        .add_input(&[n_thousands])
        .build()
        .unwrap();

    let prover = get_prover("cpu");

    let start = std::time::Instant::now();
    let receipt = prover.prove_elf(env, METHOD_NAME_ELF).unwrap();
    let end = std::time::Instant::now();
    println!(
        "sha, {} bytes, took {}ms",
        n_thousands * 1000,
        (end - start).as_millis()
    );
}
