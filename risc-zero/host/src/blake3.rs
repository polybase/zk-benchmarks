
use methods::BLAKE3_ELF;
use risc0_zkvm::{Executor, ExecutorEnv, Receipt, Session};

pub fn blake3(n_thousands: usize) -> impl FnMut() -> (Receipt, Session) {
    let env = ExecutorEnv::builder()
        .add_input(&[n_thousands])
        .build()
        .unwrap();

    let mut exec = Executor::from_elf(env, BLAKE3_ELF).unwrap();

    move || {
        let session = exec.run().unwrap();
        let receipt = session.prove().unwrap();
        (receipt, session)
    }
}
