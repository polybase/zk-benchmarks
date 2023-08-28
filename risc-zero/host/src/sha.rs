use methods::METHOD_NAME_ELF;
use risc0_zkvm::{prove::get_prover, Executor, ExecutorEnv, Session, VerifierContext};

pub fn sha(n_thousands: usize) -> impl FnMut() -> Session {
    let env = ExecutorEnv::builder()
        .add_input(&[n_thousands])
        .build()
        .unwrap();

    let prover = get_prover("cpu");

    let mut exec = Executor::from_elf(env, METHOD_NAME_ELF).unwrap();

    move || {
        let session = exec.run().unwrap();
        let _receipt = prover
            .prove_session(&VerifierContext::default(), &session)
            .unwrap();
        session
    }
}
