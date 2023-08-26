use methods::METHOD_NAME_ELF;
use risc0_zkvm::{prove::get_prover, Executor, ExecutorEnv, VerifierContext};

pub fn run_sha(n_thousands: usize) {
    let env = ExecutorEnv::builder()
        .add_input(&[n_thousands])
        .build()
        .unwrap();

    let prover = get_prover("cpu");

    let mut exec = Executor::from_elf(env, METHOD_NAME_ELF).unwrap();

    let start = std::time::Instant::now();
    let session = exec.run().unwrap();
    let _receipt = prover
        .prove_session(&VerifierContext::default(), &session)
        .unwrap();
    let end = std::time::Instant::now();

    let segments = session.resolve().unwrap();
    let (cycles, insn_cycles) = segments
        .iter()
        .fold((0, 0), |(cycles, insn_cycles), segment| {
            (
                cycles + (1 << segment.po2),
                insn_cycles + segment.insn_cycles,
            )
        });

    println!(
        "SHA256 of {} bytes took {:?} for {cycles} cycles ({insn_cycles} instruction cycles)",
        n_thousands * 1000,
        end - start,
    );
}
