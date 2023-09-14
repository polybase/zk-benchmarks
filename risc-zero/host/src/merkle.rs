use std::rc::Rc;

use methods::{MERKLE_ELF, MERKLE_MEMBERSHIP_ELF};
use risc0_zkvm::{prove::Prover, ExecutorEnv, Receipt, Session};
use shared::hash::{HashFn, Sha};

pub fn merkle(
    prover: Rc<dyn Prover>,
    tree_1_size_log: usize,
    tree_2_size_log: usize,
) -> impl FnMut() -> (Receipt, Session) {
    let tree1 = shared::tree_size_n::<Sha>(tree_1_size_log);
    let tree2 = shared::tree_size_n::<Sha>(tree_2_size_log);
    let string = format!("{};;{}", tree1.to_json(), tree2.to_json());

    let env = ExecutorEnv::builder()
        .add_input(string.as_bytes())
        .build()
        .unwrap();

    let mut exec = Executor::from_elf(env, MERKLE_ELF).unwrap();

    move || {
        let session = exec.run().unwrap();
        let receipt = prover
            .prove_session(&VerifierContext::default(), &session)
            .unwrap();

        (receipt, session)
    }
}

pub fn merkle_membership(
    prover: Rc<dyn Prover>,
    path_size: usize,
) -> impl FnMut() -> (Receipt, Session) {
    let path = core::iter::from_fn(|| Some(Sha::random()))
        .take(path_size + 1)
        .flat_map(|sha| sha.as_bytes())
        .collect::<Vec<_>>();

    let env = ExecutorEnv::builder().add_input(&path).build().unwrap();

    let mut exec = Executor::from_elf(env, MERKLE_MEMBERSHIP_ELF).unwrap();

    move || {
        let session = exec.run().unwrap();
        let receipt = prover
            .prove_session(&VerifierContext::default(), &session)
            .unwrap();

        (receipt, session)
    }
}
