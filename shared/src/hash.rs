use miden_crypto::{
    hash::rpo::{Rpo256, RpoDigest},
    Felt, FieldElement, StarkField,
};
use risc0_zkvm::sha::{self, Sha256};
use serde::{Deserialize, Serialize};

pub trait HashFn: Clone {
    type Digest: Copy;

    fn merge(a: Self::Digest, b: Self::Digest) -> Self::Digest;

    fn random() -> Self::Digest;

    fn null() -> Self::Digest;
}

#[derive(Debug, Clone)]
pub struct Rpo;

impl HashFn for Rpo {
    type Digest = RpoDigest;

    fn merge(a: Self::Digest, b: Self::Digest) -> Self::Digest {
        Rpo256::merge(&[a, b])
    }

    fn random() -> Self::Digest {
        let max = Felt::ZERO - Felt::ONE;
        let elements = [
            Felt::new(fastrand::u64(0..(max.as_int()))),
            Felt::new(fastrand::u64(0..(max.as_int()))),
            Felt::new(fastrand::u64(0..(max.as_int()))),
            Felt::new(fastrand::u64(0..(max.as_int()))),
        ];
        RpoDigest::new(elements)
    }

    fn null() -> Self::Digest {
        RpoDigest::new([Felt::ZERO; 4])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sha;

impl HashFn for Sha {
    type Digest = sha::Digest;

    fn merge(a: Self::Digest, b: Self::Digest) -> Self::Digest {
        *sha::Impl::hash_pair(&a, &b)
    }

    fn random() -> Self::Digest {
        unimplemented!("we never do this here, we only use this for generating a tree in the host");
    }

    fn null() -> Self::Digest {
        sha::Digest::new([0; 8])
    }
}