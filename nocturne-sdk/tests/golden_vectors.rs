use nocturne_core::types::aletheia::{AletheiaHeader, AletheiaWitness};
use nocturne_core::types::entropy::EntropyProof;
use nocturne_core::types::proofs::AletheiaProof;
use std::fs;
use std::path::PathBuf;

fn load_vector(name: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path.push("tests");
    path.push("golden_vectors");
    path.push(name);
    fs::read_to_string(&path)
        .expect(&format!("Failed to load golden vector: {}", path.display()))
}

#[test]
fn test_deserialize_aletheia_witness() {
    let json = load_vector("aletheia_witness.json");
    let witness: AletheiaWitness = serde_json::from_str(&json).expect("Failed to deserialize witness");
    assert_eq!(witness.public_key.0, [1u8; 48]);
    assert_eq!(witness.signature.0, [2u8; 96]);
}

#[test]
fn test_deserialize_aletheia_header() {
    let json = load_vector("aletheia_header.json");
    let header: AletheiaHeader = serde_json::from_str(&json).expect("Failed to deserialize header");
    assert_eq!(header.root_hash, [3u8; 32]);
}

#[test]
fn test_deserialize_entropy_proof_valid() {
    let json = load_vector("entropy_proof_valid.json");
    let proof: EntropyProof = serde_json::from_str(&json).expect("Failed to deserialize valid entropy proof");
    assert_eq!(proof.p_before, 500_000);
    assert_eq!(proof.q_after, 250_000);
}

#[test]
fn test_deserialize_entropy_proof_invalid_q_gt_p() {
    let json = load_vector("entropy_proof_invalid_q_gt_p.json");
    let proof: EntropyProof = serde_json::from_str(&json).expect("Failed to deserialize q > p entropy proof");
    assert!(proof.q_after > proof.p_before);
}

#[test]
fn test_deserialize_entropy_proof_invalid_negative_energy() {
    let json = load_vector("entropy_proof_invalid_negative_energy.json");
    let proof: EntropyProof = serde_json::from_str(&json).expect("Failed to deserialize negative energy entropy proof");
    assert!(proof.energy_investment < 0);
}

#[test]
fn test_deserialize_entropy_proof_invalid_zero_p() {
    let json = load_vector("entropy_proof_invalid_zero_p.json");
    let proof: EntropyProof = serde_json::from_str(&json).expect("Failed to deserialize zero p entropy proof");
    assert_eq!(proof.p_before, 0);
}

#[test]
fn test_deserialize_aletheia_proof_valid() {
    let json = load_vector("aletheia_proof_valid.json");
    let proof: AletheiaProof = serde_json::from_str(&json).expect("Failed to deserialize valid aletheia proof");
    assert_eq!(proof.entropy_proofs.len(), 2);
}
