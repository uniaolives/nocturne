use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlsSignature(#[serde(with = "BigArray")] pub [u8; 96]);

impl BlsSignature {
    pub fn to_bytes(&self) -> [u8; 96] {
        self.0
    }

    pub fn from_bytes(bytes: &[u8; 96]) -> Self {
        Self(*bytes)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlsPublicKey(#[serde(with = "BigArray")] pub [u8; 48]);

impl BlsPublicKey {
    pub fn to_bytes(&self) -> [u8; 48] {
        self.0
    }
}
