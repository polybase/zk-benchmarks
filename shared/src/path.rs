use alloc::vec::Vec;

use crate::hash::HashFn;

pub struct MerklePath<H: HashFn>(pub Vec<H::Digest>);


