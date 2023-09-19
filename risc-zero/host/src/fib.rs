use methods::FIB_ELF;
use risc0_zkvm::{Executor, ExecutorEnv, Receipt, Session};

pub fn fib(n: u32) -> impl FnMut() -> (Receipt, Session) {
    let env = ExecutorEnv::builder().add_input(&[n]).build().unwrap();

    let mut exec = Executor::from_elf(env, FIB_ELF).unwrap();

    move || {
        let session = exec.run().unwrap();
        let receipt = session.prove().unwrap();

        (receipt, session)
    }
}
