#![no_std]
#![no_main]

extern crate alloc;

use core::hint::black_box;

use alloc::string::String;
use risc0_zkvm::guest::env;
use shared::hash::Sha;
use shared::Tree;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let string: String = env::read();
    let (tree1, tree2) = parse_trees(string);
    let tree = merge_trees(tree1, tree2);
    black_box(tree);
}

fn parse_trees(string: String) -> (Tree<Sha>, Tree<Sha>) {
    let (s1, s2) = string.split_once(";;").unwrap();
    (Tree::from_json(s1).unwrap(), Tree::from_json(s2).unwrap())
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
