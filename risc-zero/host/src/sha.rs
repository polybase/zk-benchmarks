use std::rc::Rc;

use methods::SHA_ELF;
use risc0_zkvm::{prove::Prover, Executor, ExecutorEnv, Session, VerifierContext};

pub fn sha(prover: Rc<dyn Prover>, n_thousands: usize) -> impl FnMut() -> Session {
    let env = ExecutorEnv::builder()
        .add_input(&[n_thousands])
        .build()
        .unwrap();

    let mut exec = Executor::from_elf(env, SHA_ELF).unwrap();

    move || {
        let session = exec.run().unwrap();
        let _receipt = prover
            .prove_session(&VerifierContext::default(), &session)
            .unwrap();
        session
    }
}
