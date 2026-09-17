//! Capability token primitive for warden-core / wardenkernel.

/// Maximum delegation depth permitted for a capability token chain.
pub const MAX_DELEGATION_DEPTH: u8 = 255;

/// Represents a capability token granting permissions on a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    /// Identifier or hash for the resource (e.g. tool ID, database table, endpoint hash).
    pub resource: u64,
    /// Permission bitmask (e.g. Read=1, Write=2, Execute=4, Delete=8).
    pub permissions: u32,
    /// Provenance tracking depth for delegation chains.
    pub depth: u8,
}

/// Backwards compatibility alias for `CapabilityToken`.
pub type Capability = CapabilityToken;

impl CapabilityToken {
    /// Creates a new root capability token with initial permissions and depth 0.
    pub fn new(resource: u64, permissions: u32) -> Self {
        Self {
            resource,
            permissions,
            depth: 0,
        }
    }

    /// Delegates a capability token to a child context with a requested permission mask.
    ///
    /// # Invariant
    /// Capability tokens can **only narrow** permissions when delegated from a parent to a child context.
    /// If `requested_permissions` attempts to grant any bit not held by `self`,
    /// delegation fails with `"PermissionEscalationDenied: requested permissions exceed parent permissions"`.
    /// If `self.depth` reaches `MAX_DELEGATION_DEPTH`, delegation fails with `"MaxDelegationDepthExceeded"`.
    pub fn delegate(&self, requested_permissions: u32) -> Result<CapabilityToken, &'static str> {
        if (requested_permissions & !self.permissions) != 0 {
            return Err("PermissionEscalationDenied: requested permissions exceed parent permissions");
        }

        if self.depth >= MAX_DELEGATION_DEPTH {
            return Err("MaxDelegationDepthExceeded: cannot delegate beyond maximum depth");
        }

        Ok(CapabilityToken {
            resource: self.resource,
            permissions: requested_permissions,
            depth: self.depth.saturating_add(1),
        })
    }
}

pub fn run_tests() -> bool {
    const READ: u32 = 1 << 0;
    const WRITE: u32 = 1 << 1;
    const EXEC: u32 = 1 << 2;

    // Test 1: Valid narrowing
    let parent = CapabilityToken::new(101, READ | WRITE | EXEC);
    let child = match parent.delegate(READ | WRITE) {
        Ok(c) => c,
        Err(_) => return false,
    };
    if child.resource != 101 || child.permissions != (READ | WRITE) || child.depth != 1 {
        return false;
    }

    let grandchild = match child.delegate(READ) {
        Ok(gc) => gc,
        Err(_) => return false,
    };
    if grandchild.permissions != READ || grandchild.depth != 2 {
        return false;
    }

    // Test 2: Identity delegation
    let parent2 = CapabilityToken::new(102, READ | WRITE);
    let child2 = match parent2.delegate(READ | WRITE) {
        Ok(c) => c,
        Err(_) => return false,
    };
    if child2.permissions != (READ | WRITE) || child2.depth != 1 {
        return false;
    }

    // Test 3: Empty permission delegation
    let parent3 = CapabilityToken::new(103, READ);
    let child3 = match parent3.delegate(0) {
        Ok(c) => c,
        Err(_) => return false,
    };
    if child3.permissions != 0 || child3.depth != 1 {
        return false;
    }

    // Test 4: Prevent permission escalation
    let parent4 = CapabilityToken::new(104, READ);
    if parent4.delegate(READ | WRITE).is_ok() {
        return false;
    }

    // Test 5: Prevent escalation from narrowed child
    let parent5 = CapabilityToken::new(105, READ | WRITE | EXEC);
    let child5 = match parent5.delegate(READ) {
        Ok(c) => c,
        Err(_) => return false,
    };
    if child5.delegate(READ | EXEC).is_ok() || child5.delegate(WRITE).is_ok() {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const READ: u32 = 1 << 0;
    const WRITE: u32 = 1 << 1;
    const EXEC: u32 = 1 << 2;
    const DELETE: u32 = 1 << 3;

    #[test]
    fn test_valid_narrowing_delegation() {
        let parent = CapabilityToken::new(1, READ | WRITE | EXEC);
        let child = parent.delegate(READ | WRITE).unwrap();

        assert_eq!(child.resource, 1);
        assert_eq!(child.permissions, READ | WRITE);
        assert_eq!(child.depth, 1);

        let grandchild = child.delegate(READ).unwrap();
        assert_eq!(grandchild.permissions, READ);
        assert_eq!(grandchild.depth, 2);
    }

    #[test]
    fn test_identity_delegation() {
        let parent = CapabilityToken::new(2, READ | WRITE);
        let child = parent.delegate(READ | WRITE).unwrap();

        assert_eq!(child.permissions, READ | WRITE);
        assert_eq!(child.depth, 1);
    }

    #[test]
    fn test_empty_permissions_delegation() {
        let parent = CapabilityToken::new(3, READ);
        let child = parent.delegate(0).unwrap();

        assert_eq!(child.permissions, 0);
        assert_eq!(child.depth, 1);
    }

    #[test]
    fn test_adversarial_escalation_rejection() {
        let parent = CapabilityToken::new(4, READ);
        let err = parent.delegate(READ | WRITE).unwrap_err();
        assert!(err.contains("PermissionEscalationDenied"));

        let child = parent.delegate(READ).unwrap();
        assert!(child.delegate(READ | EXEC).is_err());
        assert!(child.delegate(DELETE).is_err());
    }

    #[test]
    fn test_max_delegation_depth() {
        let mut token = CapabilityToken::new(5, READ);
        token.depth = MAX_DELEGATION_DEPTH;

        let err = token.delegate(READ).unwrap_err();
        assert!(err.contains("MaxDelegationDepthExceeded"));
    }
}
