# 4. Warden Core Agent Gatekeeper

## Context
Autonomous AI agents and sub-agent pipelines delegate access rights to tools, APIs, and sandboxed runners. Without strict enforcement, a compromised sub-agent or tool runner could escalate privileges by requesting ungranted rights or forging capability tokens.

`warden-core` extracts the bare-metal kernel capability narrowing invariant proven in `wardenkernel` into a zero-dependency `#![no_std]` Rust library usable across application layer agent frameworks, WebAssembly runtimes, and embedded microkernels.

## Decision
- Defined `CapabilityToken` with fields `resource` (u64), `permissions` (u32 bitmask), and `depth` (u8 delegation provenance depth).
- Implemented `CapabilityToken::delegate(&self, requested_permissions: u32) -> Result<CapabilityToken, &'static str>` enforcing `(requested_permissions & !self.permissions) == 0` and `self.depth < MAX_DELEGATION_DEPTH`.
- Provided `#![no_std]` compatibility for zero-dependency execution across embedded, WASM, and standard server environments.
- Added an agent tool gatekeeper example in `examples/agent_gatekeeper.rs` demonstrating root agent delegation to sub-agents and tool runners with gatekeeper enforcement.

## Consequences
- Guarantees that sub-agents and tool runners in autonomous pipelines cannot escalate privileges beyond parent token rights.
- Provides a unified capability primitive shared between the `wardenkernel` bare-metal kernel and application-layer agent frameworks.
