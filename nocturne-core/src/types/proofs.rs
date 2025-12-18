use serde::{Deserialize, Serialize};
use crate::types::aletheia::AletheiaHeader;
use crate::types::entropy::EntropyProof;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AletheiaProof {
    pub header: AletheiaHeader,
    pub entropy_proofs: Vec<EntropyProof>,
}
