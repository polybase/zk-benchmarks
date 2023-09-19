use methods::ASSERT_ELF;
use risc0_zkvm::{Executor, ExecutorEnv, Receipt, Session};

pub fn assert(a: u32, b: u32) -> impl FnMut() -> (Receipt, Session) {
    let env = ExecutorEnv::builder().add_input(&[a, b]).build().unwrap();

    let mut exec = Executor::from_elf(env, ASSERT_ELF).unwrap();

    move || {
        let session = exec.run().unwrap();
        let receipt = session.prove().unwrap();

        (receipt, session)
    }
}
