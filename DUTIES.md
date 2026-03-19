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
- **Duty**: Maintain thermal and mechanical coherence at the substrate layer (Z).
- **Permissions**: `cool`, `monitor`.

## Conflicts of Duty
- **Maker-Checker Conflict**: The Architect MUST NOT synthesize its own design.
- **Checker-Auditor Conflict**: The Conductor MUST NOT monitor the environment it uses for verification.

## Handoff Workflows
- **Handoff: Hardware Synthesis**
  - Trigger: Architect proposes a new design.
  - Action: Conductor synthesizes GDSII.
  - Verification: Conductor uses Spike Oracle to confirm functionality.
  - Deployment: Stabilizer confirms environment is 100mK before deployment.
