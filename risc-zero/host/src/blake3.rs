use std::rc::Rc;

use methods::BLAKE3_ELF;
use risc0_zkvm::{prove::Prover, Executor, ExecutorEnv, Session, VerifierContext};

pub fn blake3(prover: Rc<dyn Prover>, n_thousands: usize) -> impl FnMut() -> Session {
    let env = ExecutorEnv::builder()
        .add_input(&[n_thousands])
        .build()
        .unwrap();

    let mut exec = Executor::from_elf(env, BLAKE3_ELF).unwrap();

    move || {
        let session = exec.run().unwrap();
        let _receipt = prover
            .prove_session(&VerifierContext::default(), &session)
            .unwrap();
        session
    }
}