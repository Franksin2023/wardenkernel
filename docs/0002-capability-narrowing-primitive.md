# 2. Capability Narrowing Primitive

## Context
In `wardenkernel`, capability tokens represent granular rights to access kernel or hardware resources (e.g. memory ranges, channels, or objects). A fundamental requirement of capability-based microkernel architectures is that capability delegation from a parent context to a child context must strictly enforce rights narrowing: a child context can only receive a subset of permissions held by the parent, never a superset.

## Decision
- Implemented the `Capability` struct in `src/capability.rs` with `resource_id` (u64), `permissions` (u32 bitmask), and `delegation_depth` (u32).
- Implemented `Capability::delegate(&self, requested_mask: u32) -> Result<Capability, &'static str>` enforcing the invariant `(requested_mask & !self.permissions) == 0`.
- If a delegation attempts to request permission bits not held by the parent capability, the kernel rejects the delegation with `"PermissionEscalationDenied: requested rights exceed parent permissions"`.
- If valid, a new child `Capability` is created with `permissions = requested_mask` and incremented `delegation_depth`.

## Consequences
- Guarantees at the kernel primitive level that capability tokens can only narrow or maintain permission masks during delegation, preventing privilege escalation.
- Provides test coverage running inside the bare-metal kernel environment verifying valid narrowing, identity delegation, empty delegation, and rejection of privilege escalation.
- Lays the groundwork for future object management, process isolation, and syscall dispatching in subsequent phases.
