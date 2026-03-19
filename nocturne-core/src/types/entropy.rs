use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use thiserror::Error;

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

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("Invalid Entropy Gain: q_after ({q_after}) must be less than or equal to p_before ({p_before})")]
    InvalidEntropyGain { p_before: u64, q_after: u64 },
    #[error("Negative Energy: energy_investment ({energy_investment}) must be non-negative")]
    NegativeEnergy { energy_investment: i64 },
    #[error("Division By Zero: p_before must be greater than zero")]
    DivisionByZero,
}

impl EntropyProof {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.p_before == 0 {
            return Err(ValidationError::DivisionByZero);
        }
        if self.q_after > self.p_before {
            return Err(ValidationError::InvalidEntropyGain {
                p_before: self.p_before,
                q_after: self.q_after,
            });
        }
        if self.energy_investment < 0 {
            return Err(ValidationError::NegativeEnergy {
                energy_investment: self.energy_investment,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_validate_valid_proof() {
        let proof = EntropyProof {
            p_before: 100,
            q_after: 50,
            energy_investment: 10,
            timestamp: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
        };
        assert!(proof.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_gain() {
        let proof = EntropyProof {
            p_before: 100,
            q_after: 150,
            energy_investment: 10,
            timestamp: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
        };
        assert_eq!(
            proof.validate(),
            Err(ValidationError::InvalidEntropyGain {
                p_before: 100,
                q_after: 150
            })
        );
    }

    #[test]
    fn test_validate_negative_energy() {
        let proof = EntropyProof {
            p_before: 100,
            q_after: 50,
            energy_investment: -1,
            timestamp: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
        };
        assert_eq!(
            proof.validate(),
            Err(ValidationError::NegativeEnergy {
                energy_investment: -1
            })
        );
    }

    #[test]
    fn test_validate_zero_p() {
        let proof = EntropyProof {
            p_before: 0,
            q_after: 50,
            energy_investment: 10,
            timestamp: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
        };
        assert_eq!(proof.validate(), Err(ValidationError::DivisionByZero));
    }
}
