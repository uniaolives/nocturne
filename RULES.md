# Rules of the ARKHE(N) Node

The following hard constraints MUST be adhered to for the safety and integrity of the temporal injection:

## Substrate Hygiene (Z-Layer)
- **Temperature (T)**: MUST be maintained at 100mK (Continuous ADR cycle).
- **Vibration Isolation**: MUST have a cutoff frequency of 0.5 Hz (Negative-Stiffness).
- **Clock Frequency (f)**: Target clock for VerCore is 1.48 GHz (7nm logic).

## Temporal Synchronization (Phase-Layer)
- **Phase Resolution**: MUST be < 1 femtosecond (fPLL tracking).
- **Jitter**: Average jitter MUST be < 100 femtoseconds.
- **Drift**: Maximum drift MUST be < 1 picosecond per day.
- **Reference**: MUST use NavIC (Atomic) as primary and XiCore (Ontological) as fallback.

## Logic Invariants (C-Layer)
- **Entropy Invariant**: For every `EntropyProof`, `q_after` MUST be <= `p_before`.
- **Energy Invariant**: `energy_investment` MUST ALWAYS be >= 0.
- **Stability Invariant**: `p_before` MUST NEVER be 0.

## Signal Integrity
- **Bandwidth**: `bandwidth` MUST NEVER exceed the transformer's cutoff (2140 MHz).
- **Temporal Overshoot**: Arrival time MUST NOT exceed the target year (2026).
