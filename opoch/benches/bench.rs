use benchy::{benchmark, BenchmarkRun};
use opoch_poc_sha::{
    segment::{SegmentProver, SegmentConfig},
    aggregation::{AggregationProver, AggregationConfig},
    proof::{OpochProof, ProofHeader, compute_params_hash},
    fri::{FriConfig, FriVerifier},
    sha256::Sha256,
    transcript::Transcript,
};

/// FRI configuration for benchmarks (128-bit security)
fn benchmark_fri_config() -> FriConfig {
    FriConfig {
        num_queries: 68,
        blowup_factor: 8,
        max_degree: 65536,
    }
}

/// Generate a SHA-256 hash chain proof
fn generate_proof_for_n(n: u64) -> Vec<u8> {
    let input = b"OPOCH zkbench benchmark input";
    let d0 = Sha256::hash(input);

    // Determine segment parameters
    let segment_length = if n <= 64 { n as usize } else { 64 };
    let num_segments = ((n as usize) + segment_length - 1) / segment_length;

    let fri_config = benchmark_fri_config();

    let segment_config = SegmentConfig {
        segment_length,
        fri_config: fri_config.clone(),
    };

    let segment_prover = SegmentProver::new(segment_config);

    // Generate segment proofs
    let mut segment_proofs = Vec::with_capacity(num_segments);
    let mut current_hash = d0;

    for i in 0..num_segments {
        let proof = segment_prover.prove(i as u32, &current_hash);
        current_hash = proof.end_hash;
        segment_proofs.push(proof);
    }

    let y = current_hash;

    // Aggregate proofs
    let agg_config = AggregationConfig {
        max_children: num_segments + 1,
        fri_config: fri_config.clone(),
    };

    let agg_prover = AggregationProver::new(agg_config);
    let l1_proof = agg_prover.aggregate_segments(&segment_proofs);
    let final_proof = agg_prover.aggregate_level1(&[l1_proof]);

    // Build header
    let actual_n = (num_segments * segment_length) as u64;
    let params_hash = compute_params_hash(actual_n, segment_length as u64);
    let header = ProofHeader::new(actual_n, segment_length as u64, d0, y, params_hash);

    let proof = OpochProof {
        header,
        final_proof,
    };

    proof.serialize()
}

/// Verify a proof
fn verify_proof(input: &[u8], proof_bytes: &[u8]) -> bool {
    let proof = match OpochProof::deserialize(proof_bytes) {
        Some(p) => p,
        None => return false,
    };

    // Verify magic and version
    if &proof.header.magic != b"OPSH" || proof.header.version != 1 {
        return false;
    }

    // Verify d0 matches input
    let computed_d0 = Sha256::hash(input);
    if proof.header.d0 != computed_d0 {
        return false;
    }

    // Verify chain linkage
    if proof.final_proof.chain_start != proof.header.d0 {
        return false;
    }
    if proof.final_proof.chain_end != proof.header.y {
        return false;
    }

    // Verify aggregation level
    if proof.final_proof.level != 2 {
        return false;
    }

    // Reconstruct transcript and verify FRI proof
    let mut transcript = Transcript::new();
    transcript.append_commitment(&proof.final_proof.children_root);
    transcript.append(&proof.final_proof.chain_start);
    transcript.append(&proof.final_proof.chain_end);
    let _alpha = transcript.challenge_aggregation();

    let fri_config = benchmark_fri_config();
    let fri_verifier = FriVerifier::new(fri_config);
    fri_verifier.verify(&proof.final_proof.fri_proof, &mut transcript)
}

/// SHA-256 Hash Chain Proving (matches other frameworks' pattern)
/// OPOCH proves N iterations of: h_{i+1} = SHA256(h_i)
#[benchmark("SHA256 Hash Chain", [
    ("N=64", 64u64),
    ("N=256", 256u64),
    ("N=1024", 1024u64),
    ("N=4096", 4096u64),
])]
fn sha256_hash_chain(b: &mut BenchmarkRun, n: u64) {
    // Benchmark proving (same pattern as other frameworks)
    let proof_bytes = b.run(|| {
        generate_proof_for_n(n)
    });

    b.log("proof_size_bytes", proof_bytes.len());
    b.log(
        "compressed_proof_size_bytes",
        zstd::encode_all(&proof_bytes[..], 21).unwrap().len(),
    );
    b.log("n_iterations", n as usize);
}

/// Verification benchmark (OPOCH-specific - shows our fast verification)
#[benchmark("SHA256 Chain Verify", [
    ("N=64", 64u64),
    ("N=1024", 1024u64),
    ("N=4096", 4096u64),
])]
fn sha256_chain_verify(b: &mut BenchmarkRun, n: u64) {
    // Setup: generate proof (not timed)
    let proof_bytes = generate_proof_for_n(n);
    let input = b"OPOCH zkbench benchmark input";

    // Benchmark verification only
    let _valid = b.run(|| {
        verify_proof(input, &proof_bytes)
    });

    b.log("proof_size_bytes", proof_bytes.len());
    b.log("n_iterations", n as usize);
}

benchy::main!(
    "opoch",
    sha256_hash_chain,
    sha256_chain_verify,
);
