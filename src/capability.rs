//! Capability token primitive and delegation mechanics.

/// Represents access permissions on a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capability {
    /// Identifier for the underlying resource (e.g., memory range, channel, object ID).
    pub resource_id: u64,
    /// Bitmask of granted permissions (e.g., READ = 1, WRITE = 2, EXECUTE = 4).
    pub permissions: u32,
    /// Number of delegations performed from the root capability (0 = root).
    pub delegation_depth: u32,
}

impl Capability {
    /// Creates a new root capability with full initial permissions and depth 0.
    pub fn new(resource_id: u64, permissions: u32) -> Self {
        Self {
            resource_id,
            permissions,
            delegation_depth: 0,
        }
    }

    /// Delegates a capability to a child context with a requested permission mask.
    ///
    /// # Invariant
    /// Capabilities can **only narrow** permissions when delegated from a parent to a child context.
    /// If `requested_mask` attempts to grant any permission bit not held by `self`,
    /// delegation fails with an error (`"PermissionEscalationDenied"`).
    pub fn delegate(&self, requested_mask: u32) -> Result<Capability, &'static str> {
        if (requested_mask & !self.permissions) != 0 {
            return Err("PermissionEscalationDenied: requested rights exceed parent permissions");
        }

        Ok(Capability {
            resource_id: self.resource_id,
            permissions: requested_mask,
            delegation_depth: self.delegation_depth.saturating_add(1),
        })
    }
}

pub fn run_tests() -> bool {
    const READ: u32 = 1 << 0;
    const WRITE: u32 = 1 << 1;
    const EXEC: u32 = 1 << 2;

    // Test 1: Valid narrowing
    let parent = Capability::new(101, READ | WRITE | EXEC);
    let child = match parent.delegate(READ | WRITE) {
        Ok(c) => c,
        Err(_) => return false,
    };
    if child.resource_id != 101 || child.permissions != (READ | WRITE) || child.delegation_depth != 1 {
        return false;
    }

    let grandchild = match child.delegate(READ) {
        Ok(gc) => gc,
        Err(_) => return false,
    };
    if grandchild.permissions != READ || grandchild.delegation_depth != 2 {
        return false;
    }

    // Test 2: Identity delegation
    let parent2 = Capability::new(102, READ | WRITE);
    let child2 = match parent2.delegate(READ | WRITE) {
        Ok(c) => c,
        Err(_) => return false,
    };
    if child2.permissions != (READ | WRITE) || child2.delegation_depth != 1 {
        return false;
    }

    // Test 3: Empty permission delegation
    let parent3 = Capability::new(103, READ);
    let child3 = match parent3.delegate(0) {
        Ok(c) => c,
        Err(_) => return false,
    };
    if child3.permissions != 0 || child3.delegation_depth != 1 {
        return false;
    }

    // Test 4: Prevent permission escalation
    let parent4 = Capability::new(104, READ);
    if parent4.delegate(READ | WRITE).is_ok() {
        return false;
    }

    // Test 5: Prevent escalation from narrowed child
    let parent5 = Capability::new(105, READ | WRITE | EXEC);
    let child5 = match parent5.delegate(READ) {
        Ok(c) => c,
        Err(_) => return false,
    };
    if child5.delegate(READ | EXEC).is_ok() || child5.delegate(WRITE).is_ok() {
        return false;
    }

    true
}
