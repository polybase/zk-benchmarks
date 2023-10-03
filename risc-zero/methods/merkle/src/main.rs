#![no_std]
#![no_main]

extern crate alloc;

use core::hint::black_box;

use risc0_zkvm::guest::env;
use shared::hash::Sha;
use shared::Tree;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let tree1: Tree<Sha> = env::read();
    let tree2: Tree<Sha> = env::read();
    let tree = merge_trees(tree1, tree2);
    black_box(tree);
}

fn merge_trees(tree1: Tree<Sha>, tree2: Tree<Sha>) -> Tree<Sha> {
    let hashes = tree1
        .iter()
        .chain(tree2.iter())
        .filter_map(|node| match node {
            Tree::Leaf(digest) => Some(*digest),
            Tree::Node { .. } => None,
        });

    Tree::new(hashes)
}
