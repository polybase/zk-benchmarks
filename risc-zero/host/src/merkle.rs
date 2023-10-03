use methods::{MERKLE_ELF, MERKLE_MEMBERSHIP_ELF};
use risc0_zkvm::{serde::to_vec, Executor, ExecutorEnv, Receipt, Session};
use shared::{
    hash::{HashFn, Sha},
    Tree,
};

pub fn merkle(tree1: Tree<Sha>, tree2: Tree<Sha>) -> impl FnMut() -> (Receipt, Session) {
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&tree1).unwrap())
        .add_input(&to_vec(&tree2).unwrap())
        .build()
        .unwrap();

    let mut exec = Executor::from_elf(env, MERKLE_ELF).unwrap();

    move || {
        let session = exec.run().unwrap();
        let receipt = session.prove().unwrap();

        (receipt, session)
    }
}

pub fn merkle_membership(path_size: usize) -> impl FnMut() -> (Receipt, Session) {
    let path = core::iter::from_fn(|| Some(Sha::random()))
        .take(path_size + 1)
        .flat_map(|sha| sha.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&path).unwrap())
        .build()
        .unwrap();

    let mut exec = Executor::from_elf(env, MERKLE_MEMBERSHIP_ELF).unwrap();

    move || {
        let session = exec.run().unwrap();
        let receipt = session.prove().unwrap();

        (receipt, session)
    }
}
