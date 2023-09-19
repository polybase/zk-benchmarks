use methods::SHA_ELF;
use risc0_zkvm::{Executor, ExecutorEnv, Receipt, Session};

pub fn sha(n_thousands: usize) -> impl FnMut() -> (Receipt, Session) {
    let env = ExecutorEnv::builder()
        .add_input(&[n_thousands])
        .build()
        .unwrap();

    let mut exec = Executor::from_elf(env, SHA_ELF).unwrap();

    move || {
        let session = exec.run().unwrap();
        let receipt = session.prove().unwrap();

        (receipt, session)
    }
}
