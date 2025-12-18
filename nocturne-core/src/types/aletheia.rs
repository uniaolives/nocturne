use serde::{Deserialize, Serialize};
use crate::crypto::bls::{BlsPublicKey, BlsSignature};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AletheiaWitness {
    pub public_key: BlsPublicKey,
    pub signature: BlsSignature,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AletheiaHeader {
    pub witness: AletheiaWitness,
    pub root_hash: [u8; 32],
}
