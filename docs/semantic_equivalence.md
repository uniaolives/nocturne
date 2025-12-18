# Semantic Equivalence: Rust vs. ZK Circuit

This document outlines the key behavioral differences between the high-level Rust implementation and the low-level ZK circuit implementation when handling edge cases and invalid inputs. These differences are by design and reflect the distinct philosophies of each environment.

- **Rust Implementation:** Aims to be fault-tolerant, often normalizing or clamping invalid inputs to produce a predictable (if neutral) result.
- **ZK Circuit Implementation:** Adheres to a "fail-fast" security posture. Any input that violates a constraint will cause the entire proof to be invalid and rejected.

This distinction is critical for developers to understand, as behavior that might be gracefully handled in a Rust-native context will lead to a hard failure within the ZK proving system.

## Equivalence Matrix

| Event / Input Condition | Rust Behavior (v1.1) | ZK Circuit Behavior (v1.1 Hardened) | Security Implication |
|-------------------------|------------------------------------|------------------------------------------|----------------------|
| `Q > P` (Invalid Entropy) | Returns `Entropy = 0` (Innocuous) | **Proof Invalid** (Rejection) | The ZK circuit correctly prevents the creation of a proof that violates thermodynamic principles. |
| `E < -1.0` (Negative Energy) | Clamps value to `0` | **Proof Invalid** (Range Violation) | The ZK circuit prevents "semantic inversion" attacks where a disaster could be framed as a positive contribution. |
| `P = 0` (Division by Zero) | Panics / Runtime Error | **Constraint Unmet** (Impossible to prove) | The ZK circuit's constraints make it impossible to construct a valid proof with this input, preventing division-by-zero failures. |
