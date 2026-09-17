#![no_std]

pub mod capability;
pub mod adversarial;

pub use capability::{Capability, CapabilityToken, MAX_DELEGATION_DEPTH};
