use miden::Assembler;
use miden_crypto::merkle::InnerNodeInfo;
use miden_processor::{AdviceInputs, MemAdviceProvider, StackInputs, VmStateIterator};
use miden_prover::{ExecutionProof, ProofOptions};
use shared::{
    hash::{HashFn, rpo::Rpo},
    Tree,
};

pub fn membership(
    path: Vec<<Rpo as HashFn>::Digest>,
    digest: <Rpo as HashFn>::Digest,
) -> (impl Fn() -> ExecutionProof, VmStateIterator) {
    assert_eq!(path.len(), 10);
    let mut advice = AdviceInputs::default();
    advice.extend_stack(path.iter().flat_map(|digest| digest.as_elements()).copied());
    advice.extend_stack(digest.as_elements().iter().copied());
    let advice = MemAdviceProvider::from(advice);

    let program = Assembler::default()
        .compile(include_str!("./asm/membership.masm"))
        .unwrap();

    let stack = StackInputs::default();
    let opts = ProofOptions::default();

    let vm_iter = miden_processor::execute_iter(&program, stack.clone(), advice.clone());

    let prove = move || {
        miden_prover::prove(&program, stack.clone(), advice.clone(), opts.clone())
            .unwrap()
            .1
    };

    (prove, vm_iter)
}

pub fn merge_trees(
    tree1: &Tree<Rpo>,
    tree2: &Tree<Rpo>,
) -> (impl Fn() -> ExecutionProof, VmStateIterator) {
    let mut advice = AdviceInputs::default();
    advice.extend_merkle_store(tree1.iter().map(inner_node_info));
    advice.extend_merkle_store(tree2.iter().map(inner_node_info));

    let advice = MemAdviceProvider::from(advice);
    let program = Assembler::default()
        .compile("begin mtree_merge end")
        .unwrap();
    let stack = [tree1, tree2]
        .into_iter()
        .flat_map(|tree| tree.digest().as_elements().to_owned())
        .collect();

    let stack = StackInputs::new(stack);
    let opts = ProofOptions::default();

    let vm_iter = miden_processor::execute_iter(&program, stack.clone(), advice.clone());

    let prove = move || {
        let (_stack, proof) =
            miden_prover::prove(&program, stack.clone(), advice.clone(), opts.clone()).unwrap();
        proof
    };

    (prove, vm_iter)
}

fn inner_node_info(tree: &Tree<Rpo>) -> InnerNodeInfo {
    match tree {
        Tree::Leaf(digest) => InnerNodeInfo {
            value: *digest,
            left: Rpo::null(),
            right: Rpo::null(),
        },
        Tree::Node {
            left,
            right,
            digest,
        } => InnerNodeInfo {
            value: *digest,
            left: left.digest(),
            right: right.digest(),
        },
    }
}
