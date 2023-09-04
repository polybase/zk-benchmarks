use std::rc::Rc;

use methods::ASSERT_ELF;
use risc0_zkvm::{prove::Prover, Executor, ExecutorEnv, Receipt, Session, VerifierContext};

pub fn assert(prover: Rc<dyn Prover>, a: u32, b: u32) -> impl FnMut() -> (Receipt, Session) {
    let env = ExecutorEnv::builder().add_input(&[a, b]).build().unwrap();

    let mut exec = Executor::from_elf(env, ASSERT_ELF).unwrap();

    move || {
        let session = exec.run().unwrap();
        let receipt = prover
            .prove_session(&VerifierContext::default(), &session)
            .unwrap();

        (receipt, session)
    }
}
