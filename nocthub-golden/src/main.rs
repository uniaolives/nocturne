use nocturne_core::crypto::bls::{BlsPublicKey, BlsSignature};
use nocturne_core::serialization::canonical::to_canonical_json;
use nocturne_core::types::aletheia::{AletheiaHeader, AletheiaWitness};
use nocturne_core::types::entropy::EntropyProof;
use nocturne_core::types::proofs::AletheiaProof;
use std::fs::File;
use std::io::Write;
use chrono::{Utc, TimeZone};

fn main() {
    println!("Generating golden vectors...");

    // --- VALID VECTORS ---

    let witness = AletheiaWitness {
        public_key: BlsPublicKey([1u8; 48]),
        signature: BlsSignature([2u8; 96]),
    };

    let header = AletheiaHeader {
        witness: witness.clone(),
        root_hash: [3u8; 32],
    };

    let valid_entropy_proof = EntropyProof {
        p_before: 500_000,
        q_after: 250_000,
        energy_investment: 1_000,
        timestamp: Utc.with_ymd_and_hms(2023, 10, 27, 10, 0, 0).unwrap(),
    };

    let valid_aletheia_proof = AletheiaProof {
        header: header.clone(),
        entropy_proofs: vec![valid_entropy_proof.clone(), valid_entropy_proof.clone()],
    };

    save_golden_vector("aletheia_witness.json", &witness);
    save_golden_vector("aletheia_header.json", &header);
    save_golden_vector("entropy_proof_valid.json", &valid_entropy_proof);
    save_golden_vector("aletheia_proof_valid.json", &valid_aletheia_proof);

    println!("\nGenerating edge case vectors...");

    // --- INVALID VECTORS ---

    // Edge Case 1: Q > P (Invalid Entropy Gain)
    let invalid_entropy_q_gt_p = EntropyProof {
        p_before: 250_000,
        q_after: 500_000, // q > p
        energy_investment: 1_000,
        timestamp: Utc.with_ymd_and_hms(2023, 10, 27, 10, 0, 0).unwrap(),
    };
    save_golden_vector("entropy_proof_invalid_q_gt_p.json", &invalid_entropy_q_gt_p);

    // Edge Case 2: E < 0 (Negative Energy)
    let invalid_entropy_neg_e = EntropyProof {
        p_before: 500_000,
        q_after: 250_000,
        energy_investment: -1000, // Negative E
        timestamp: Utc.with_ymd_and_hms(2023, 10, 27, 10, 0, 0).unwrap(),
    };
    save_golden_vector("entropy_proof_invalid_negative_energy.json", &invalid_entropy_neg_e);

    // Edge Case 3: P = 0 (Potential Division by Zero)
    let invalid_entropy_zero_p = EntropyProof {
        p_before: 0, // P = 0
        q_after: 250_000,
        energy_investment: 1_000,
        timestamp: Utc.with_ymd_and_hms(2023, 10, 27, 10, 0, 0).unwrap(),
    };
    save_golden_vector("entropy_proof_invalid_zero_p.json", &invalid_entropy_zero_p);

    println!("\nGolden vectors generated successfully.");
}

fn save_golden_vector<T: serde::Serialize>(filename: &str, data: &T) {
    let path = format!("tests/golden_vectors/{}", filename);
    let mut file = File::create(&path).expect("Failed to create file");
    let json = to_canonical_json(data).expect("Failed to serialize to canonical JSON");
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");
    println!("  -> Saved {}", path);
}
