#![no_std]

extern crate alloc;

use alloc::{boxed::Box, string::String, vec::Vec};
use hash::HashFn;
use serde::{Deserialize, Serialize};

use core::iter::once;

pub mod hash;

#[cfg(feature = "std")]
pub use fastrand;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(
    serialize = "H::Digest: Serialize",
    deserialize = "H::Digest: Deserialize<'de>",
))]
pub enum Tree<H: HashFn> {
    Leaf(H::Digest),
    Node {
        left: Box<Tree<H>>,
        right: Box<Tree<H>>,
        digest: H::Digest,
    },
}

impl<H: HashFn> Tree<H> {
    fn new_node(left: Self, right: Option<Self>) -> Self {
        let right = right.unwrap_or_else(|| Self::null(left.depth()));
        let digest = H::merge(left.digest(), right.digest());
        let left = Box::new(left);
        let right = Box::new(right);

        Self::Node {
            left,
            right,
            digest,
        }
    }

    fn null(depth: usize) -> Self {
        match depth {
            0 => panic!(),
            1 => Self::Leaf(H::null()),
            _ => {
                let left = Self::null(depth - 1);
                let right = left.clone();
                Self::new_node(left, Some(right))
            }
        }
    }

    pub fn new(hashes: impl IntoIterator<Item = H::Digest>) -> Self {
        let mut trees: Vec<_> = hashes.into_iter().map(Self::Leaf).collect();

        while trees.len() > 1 {
            trees = trees
                .chunks(2)
                .map(|i| match i {
                    [left] => Self::new_node(left.clone(), None),
                    [left, right] => Self::new_node(left.clone(), Some(right.clone())),
                    _ => panic!(),
                })
                .collect()
        }

        trees.remove(0)
    }

    pub fn digest(&self) -> H::Digest {
        match self {
            Tree::Leaf(digest) => *digest,
            Tree::Node { digest, .. } => *digest,
        }
    }

    fn depth(&self) -> usize {
        match self {
            Self::Leaf(_) => 1,
            Self::Node { left, .. } => left.depth() + 1,
        }
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Tree<H>> + 'a> {
        match self {
            Tree::Leaf(_) => Box::new(once(self)),
            Tree::Node { left, right, .. } => {
                let iter = once(self).chain(left.iter()).chain(right.iter());
                Box::new(iter)
            }
        }
    }

    pub fn leaves(&self) -> Box<dyn Iterator<Item = H::Digest>> {
        match self {
            Tree::Leaf(hash) => Box::new(once(*hash)),
            Tree::Node { left, right, .. } => Box::new(left.leaves().chain(right.leaves())),
        }
    }

    pub fn to_json(&self) -> String
    where
        H::Digest: Serialize,
    {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_json(s: &str) -> Option<Self>
    where
        H::Digest: for<'de> Deserialize<'de>,
    {
        serde_json::from_str(s).ok()
    }

    pub fn insert(&mut self, hash: H::Digest) -> bool {
        match self {
            Self::Leaf(leaf) if *leaf == H::null() => {
                *leaf = hash;
                true
            }
            Self::Leaf(_) => false,
            Self::Node {
                left,
                right,
                digest,
            } => {
                let result = if left.insert(hash) {
                    false
                } else {
                    right.insert(hash)
                };

                *digest = H::merge(left.digest(), right.digest());

                result
            }
        }
    }
}

/// actually 2^n
pub fn tree_size_n<H: HashFn>(n: usize) -> Tree<H> {
    match n {
        0 => Tree::Leaf(H::random()),
        _ => {
            let left = Box::new(tree_size_n(n - 1));
            let right = Box::new(tree_size_n(n - 1));
            let digest = H::merge(left.digest(), right.digest());

            Tree::Node {
                left,
                right,
                digest,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::Sha;

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

    #[test]
    fn can_merge_trees_in_rust() {
        let tree1 = tree_size_n::<Sha>(10);
        let tree2 = tree_size_n::<Sha>(10);

        merge_trees(tree1, tree2);
    }
}
