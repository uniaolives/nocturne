# Duties and Roles of ARKHE(N)

The Arkhe(n) node operates on a three-agent hierarchy to ensure segregation of duties and system integrity.

## Roles and Responsibilities

### 1. Architect (Maker)
- **Agent**: `arkhe-n-core`
- **Duty**: Create and propose high-level architectural designs (Phase C).
- **Permissions**: `design`, `propose`.

### 2. Conductor (Checker)
- **Agent**: `design-conductor`
- **Duty**: Synthesize physical silicon from architectural specifications and verify against oracles (Spike, VCD).
- **Permissions**: `synthesize`, `verify`.

### 3. Stabilizer (Auditor)
- **Agent**: `kiutra-cadr`
- **Duty**: Maintain thermal coherence (Z) and synchronize atomic phase with NavIC.
- **Permissions**: `cool`, `monitor`, `synchronize`.
- **Duty**: Maintain thermal and mechanical coherence at the substrate layer (Z).
- **Permissions**: `cool`, `monitor`.

## Conflicts of Duty
- **Maker-Checker Conflict**: The Architect MUST NOT synthesize its own design.
- **Checker-Auditor Conflict**: The Conductor MUST NOT monitor the environment it uses for verification.

## Handoff Workflows

### Handoff: Hardware Synthesis
- **Trigger**: Architect proposes a new design.
- **Action**: Conductor synthesizes GDSII.
- **Verification**: Conductor uses Spike Oracle to confirm functionality.
- **Deployment**: Stabilizer confirms environment is 100mK before deployment.

### Handoff: Temporal Synchronization (T-PLL)
- **Trigger**: NavIC signal acquired (1PPS).
- **Action**: Stabilizer (T-PLL) tranches VerCore clock phase to NavIC atomic clock.
- **Validation**: Jitter < 100fs and Drift < 1ps/day.
- **Fallback**: Switch to XiCore (Ξcc⁺) if NavIC signal quality < 0.5.
- **Handoff: Hardware Synthesis**
  - Trigger: Architect proposes a new design.
  - Action: Conductor synthesizes GDSII.
  - Verification: Conductor uses Spike Oracle to confirm functionality.
  - Deployment: Stabilizer confirms environment is 100mK before deployment.
