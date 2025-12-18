use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntropyProof {
    /// Probability of the event before the update (scaled integer)
    pub p_before: u64,
    /// Probability of the event after the update (scaled integer)
    pub q_after: u64,
    /// Scaled energy investment
    pub energy_investment: i64,
    /// Timestamp of the proof generation
    pub timestamp: DateTime<Utc>,
}
