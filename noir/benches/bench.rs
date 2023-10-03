extern crate noir;
extern crate rand;

use acvm::FieldElement;
use benchy::{benchmark, BenchmarkRun};
use noir::{InputMap, InputValue, Proof};
use rand::Rng;
use shared::{
    hash::{random_hashes, HashFn, Sha},
    tree_size_n, Tree,
};

#[benchmark]
fn assert(b: &mut BenchmarkRun) {
    let backend = noir::backends::ConcreteBackend::default();
    let dir = std::env::current_dir().expect("current dir to exist");

    let mut inputs = InputMap::new();

    inputs.insert("x".to_string(), InputValue::Field((1_u128).into()));
    inputs.insert("y".to_string(), InputValue::Field((2_u128).into()));

    let proof = Proof::new(&backend, "assert", dir.join("pkgs/assert"));
    let proof_bytes = b.run(|| proof.run_and_prove(&inputs));
    b.log("proof_size_bytes", proof_bytes.len());
    b.log(
        "compressed_proof_size_bytes",
        zstd::encode_all(&proof_bytes[..], 21).unwrap().len(),
    );
    // b.log("cycles", last_vm_state.clk as usize);
}

#[benchmark("Fibonacci", [
    ("1", 1),
    ("10", 10),
    ("100", 100),
    ("1000", 1000),
    ("10000", 10000),
    ("100000", 100000),
    ("1000000", 1000000),
])]
fn fibonacci(b: &mut BenchmarkRun, p: usize) {
    let backend = noir::backends::ConcreteBackend::default();
    let dir = std::env::current_dir().expect("current dir to exist");

    let mut inputs = InputMap::new();

    inputs.insert("a_start".to_string(), InputValue::Field((0_u128).into()));
    inputs.insert("b_start".to_string(), InputValue::Field((1_u128).into()));

    let proof = Proof::new(&backend, "fib", dir.join(format!("pkgs/fib/{}", p)));
    let proof_bytes = b.run(|| proof.run_and_prove(&inputs));
    b.log("proof_size_bytes", proof_bytes.len());
    b.log(
        "compressed_proof_size_bytes",
        zstd::encode_all(&proof_bytes[..], 21).unwrap().len(),
    );
}

#[benchmark("Merkle Membership")]
fn merkle_membership(b: &mut BenchmarkRun) {
    let backend = noir::backends::ConcreteBackend::default();
    let dir = std::env::current_dir().expect("current dir to exist");

    let mut inputs = InputMap::new();

    let path = generate_random_u8_slice(320)
        .iter()
        .map(|b| InputValue::Field((*b as u128).into()))
        .collect::<Vec<_>>();
    let hash = generate_random_u8_slice(32)
        .iter()
        .map(|b| InputValue::Field((*b as u128).into()))
        .collect::<Vec<_>>();

    inputs.insert("hash".to_string(), InputValue::Vec(hash));
    inputs.insert("path".to_string(), InputValue::Vec(path));

    let proof = Proof::new(
        &backend,
        "merkle_membership",
        dir.join("pkgs/merkle_membership"),
    );
    let proof_bytes = b.run(|| proof.run_and_prove(&inputs));
    b.log("proof_size_bytes", proof_bytes.len());
    b.log(
        "compressed_proof_size_bytes",
        zstd::encode_all(&proof_bytes[..], 21).unwrap().len(),
    );
}

#[benchmark("Merkle Insert", [
   ("1k tree", ( tree_size_n(9), tree_size_n(9) ))
])]
fn merkle_insert(b: &mut BenchmarkRun, (tree1, tree2): (Tree<Sha>, Tree<Sha>)) {
    let backend = noir::backends::ConcreteBackend::default();
    let dir = std::env::current_dir().expect("current dir to exist");

    let proofs = b.run(|| {
        let mut proofs = vec![];

        let mut tree_before = tree1;

        for node in tree2.leaves() {
            let mut leaves = vec![Sha::null(); 1024];
            for (index, hash) in tree_before.leaves().enumerate() {
                leaves[index] = hash;
            }

            let leaves = leaves
                .into_iter()
                .map(|hash| slice_to_input(hash.as_bytes()))
                .collect();
            let leaves = InputValue::Vec(leaves);

            let root_hash = tree_before.digest();

            let mut inputs = InputMap::new();
            inputs.insert("node".to_string(), slice_to_input(node.as_bytes()));
            inputs.insert("leaves".to_string(), leaves);
            inputs.insert(
                "root_hash".to_string(),
                slice_to_input(root_hash.as_bytes()),
            );

            let proof = Proof::new(&backend, "sha256", dir.join("pkgs/merkle_insert"));
            let proof_bytes = proof.run_and_prove(&inputs);

            proofs.push(proof_bytes);
            tree_before.insert(node);
        }

        proofs
    });

    // this isn't a real encoding, but it's probably close enough w.r.t. proof size/compressed size
    let proof_bytes: Vec<_> = proofs.into_iter().flatten().collect();

    b.log("proof_size_bytes", proof_bytes.len());
    b.log(
        "compressed_proof_size_bytes",
        zstd::encode_all(&proof_bytes[..], 21).unwrap().len(),
    );
}

#[benchmark("SHA256", [
    ("1k bytes", 1000),
    ("10k bytes", 10000),
    // ("100k bytes", 100000),
])]
fn sha256(b: &mut BenchmarkRun, p: usize) {
    let backend = noir::backends::ConcreteBackend::default();
    let dir = std::env::current_dir().expect("current dir to exist");

    let mut inputs = InputMap::new();

    // Generate random bytes
    let bytes = generate_random_u8_slice(p)
        .iter()
        .map(|b| InputValue::Field((*b as u128).into()))
        .collect::<Vec<_>>();

    inputs.insert("x".to_string(), InputValue::Vec(bytes));

    let proof = Proof::new(&backend, "sha256", dir.join(format!("pkgs/sha256/{}", p)));
    let proof_bytes = b.run(|| proof.run_and_prove(&inputs));
    b.log("proof_size_bytes", proof_bytes.len());
    b.log(
        "compressed_proof_size_bytes",
        zstd::encode_all(&proof_bytes[..], 21).unwrap().len(),
    );
}

fn generate_random_u8_slice(len: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(rng.gen::<u8>());
    }
    vec
}

fn slice_to_input(slice: &[u8]) -> InputValue {
    let vec = slice
        .iter()
        .map(|i| {
            let i = u128::from(*i);
            InputValue::Field(i.into())
        })
        .collect();

    InputValue::Vec(vec)
}

benchy::main!(
    "noir",
    assert,
    fibonacci,
    sha256,
    merkle_membership,
    merkle_insert
);
