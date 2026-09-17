# wardenkernel / warden-core

`warden-core` is a zero-dependency Rust capability token library (`#![no_std]` compatible) designed to enforce strict privilege narrowing in autonomous agent tool-calling pipelines, microkernels, and sandboxed runtimes.

## Core Guarantee

> **Capability tokens can only narrow when delegated between contexts, never widen.**

When a parent context delegates a capability token to a child context or tool runner, the child context can only receive a subset of permissions held by the parent. Any attempt to escalate privileges or request unheld permission bits is strictly rejected.

## API Usage Example

```rust
use wardenkernel::CapabilityToken;

const READ: u32 = 1 << 0;
const WRITE: u32 = 1 << 1;
const DELETE: u32 = 1 << 2;

// Root Agent creates full capability token for a database tool
let root_token = CapabilityToken::new(0xDB_01, READ | WRITE | DELETE);

// Root Agent delegates READ | WRITE token to Sub-Agent
let subagent_token = root_token.delegate(READ | WRITE).unwrap();

// Sub-Agent delegates READ-ONLY token to Untrusted Tool Runner
let tool_token = subagent_token.delegate(READ).unwrap();

// Untrusted Tool Runner attempts to escalate to DELETE -> REJECTED
assert!(tool_token.delegate(READ | DELETE).is_err());
```

## Running Examples & Verification

```bash
# Build bootable kernel image
cargo bootimage

# Run agent gatekeeper example
cargo run --example agent_gatekeeper --target x86_64-unknown-linux-gnu
```
