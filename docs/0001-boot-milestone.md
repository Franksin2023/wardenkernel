# 1. Boot Milestone

## Context
The goal of `wardenkernel` is to build a minimal bare-metal x86_64 kernel in Rust proving one core guarantee: capability tokens can only narrow when delegated between processes, never widen, as enforced by the kernel.

Phase 1 requires establishing a minimal bare-metal boot environment using Rust `#![no_std]` and the `bootloader` crate (v0.9.x), building a bootable disk image, and running it in QEMU to display a boot confirmation message on screen output.

## Decision
- Standardized the build environment using nightly Rust (`rust-src`, `llvm-tools-preview`) and target `x86_64-unknown-none`.
- Set `-C relocation-model=static` in `.cargo/config.toml` to satisfy the ELF loading requirements of `bootloader` v0.9.x.
- Implemented a `#![no_std]` bare-metal entry point `_start` in `src/main.rs` that writes `agent-kernel: boot ok` directly to the VGA text buffer (`0xb8000`).
- Configured `cargo bootimage` to package the ELF executable and bootloader into a bootable disk image (`bootimage-wardenkernel.bin`).

## Consequences
- The kernel successfully boots in QEMU (via `qemu-system-x86_64`) and displays `agent-kernel: boot ok` on screen output (VGA text buffer at `0xb8000`).
- No additional OS features (memory management, multitasking, process isolation, capability checks) are implemented yet.
- Sets the foundation for Phase 2 (memory management, capabilities, and processes).
