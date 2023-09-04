use std::rc::Rc;

use methods::FIB_ELF;
use risc0_zkvm::{prove::Prover, Executor, ExecutorEnv, Session, VerifierContext};

pub fn fib(prover: Rc<dyn Prover>, n: u32) -> impl FnMut() -> Session {
    let env = ExecutorEnv::builder().add_input(&[n]).build().unwrap();

    let mut exec = Executor::from_elf(env, FIB_ELF).unwrap();

    move || {
        let session = exec.run().unwrap();
        let _receipt = prover
            .prove_session(&VerifierContext::default(), &session)
            .unwrap();
        session
    }
}
