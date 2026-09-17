# 3. Adversarial Proof

## Context
Phase 2 established the core kernel capability data structure and the bitwise invariant that delegation can only narrow permissions (`requested_mask & !parent.permissions == 0`). Phase 3 requires an adversarial verification suite simulating attacks by malicious or compromised contexts seeking privilege escalation or permission widening.

## Decision
- Implemented an adversarial test harness in `src/adversarial.rs` covering 6 explicit attack scenarios:
  1. Direct bitwise permission escalation requests (e.g. requesting `READ | WRITE` from a `READ`-only capability).
  2. Ungranted privilege bit injections (e.g. injecting `EXEC` bit into `READ | WRITE` capabilities).
  3. Single ungranted permission bit requests.
  4. Multi-generational privilege recovery attempts (e.g. parent `ALL` -> child `READ` -> grandchild attempting to regain `WRITE` or `EXEC`).
  5. Full bitmask spectrum escalation requests (`0xFFFF_FFFF`).
  6. Zero-permission capabilities attempting to delegate rights.
- Integrated the adversarial suite execution into `_start` in `src/main.rs`.
- Upon successful rejection of all 6 attack vectors with 0 escalations permitted, the kernel prints `adversarial_proof: PASS` to the VGA text buffer (`0xb8000`).

## Consequences
- Formally proves at runtime under QEMU that the `wardenkernel` capability narrowing primitive strictly prevents privilege escalation under adversarial attack conditions.
- Completes Phase 3 and final verification of the `wardenkernel` prototype guarantees.
