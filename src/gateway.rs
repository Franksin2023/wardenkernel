//! Agent tool-execution gateway evaluating requests against active capability tokens.

extern crate alloc;

use alloc::format;
use alloc::string::String;
use crate::capability::CapabilityToken;

/// Represents a request by an agent to execute a tool or access a resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolRequest {
    /// Target resource or tool identifier.
    pub resource_id: u64,
    /// Permission required for this operation (e.g. Read = 1, Write = 2, Execute = 4, Delete = 8).
    pub required_permission: u32,
    /// Request payload or parameters.
    pub payload: String,
}

impl ToolRequest {
    /// Creates a new `ToolRequest`.
    pub fn new(resource_id: u64, required_permission: u32, payload: impl Into<String>) -> Self {
        Self {
            resource_id,
            required_permission,
            payload: payload.into(),
        }
    }
}

/// Agent tool-execution gateway enforcing active capability token permissions.
#[derive(Debug, Clone)]
pub struct ToolGateway {
    /// Currently active capability token bound to the gateway execution context.
    pub active_token: CapabilityToken,
}

impl ToolGateway {
    /// Creates a new `ToolGateway` bound to an active capability token.
    pub fn new(active_token: CapabilityToken) -> Self {
        Self { active_token }
    }

    /// Updates the active token on the gateway.
    pub fn set_token(&mut self, token: CapabilityToken) {
        self.active_token = token;
    }

    /// Evaluates and executes a `ToolRequest` against the gateway's active `CapabilityToken`.
    ///
    /// # Interception Rules
    /// 1. Token resource ID must match the request `resource_id`.
    /// 2. Token permissions must satisfy `(required_permission & !token.permissions) == 0`.
    ///
    /// If permissions are insufficient, execution is intercepted and rejected.
    pub fn execute_request(&mut self, request: ToolRequest) -> Result<String, &'static str> {
        if self.active_token.resource != request.resource_id {
            return Err("ResourceMismatch: active token does not match target request resource ID");
        }

        if (request.required_permission & !self.active_token.permissions) != 0 {
            return Err("PermissionDenied: active token lacks required permissions for this action");
        }

        Ok(format!(
            "ExecutionPermitted: action on resource {:#x} succeeded with payload '{}'",
            request.resource_id, request.payload
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::CapabilityToken;

    const READ: u32 = 1 << 0;
    const WRITE: u32 = 1 << 1;

    #[test]
    fn test_gateway_permitted_execution() {
        let token = CapabilityToken::new(0x100, READ | WRITE);
        let mut gateway = ToolGateway::new(token);

        let req = ToolRequest::new(0x100, READ, "fetch_records");
        let result = gateway.execute_request(req);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("ExecutionPermitted"));
    }

    #[test]
    fn test_gateway_denied_execution() {
        let token = CapabilityToken::new(0x100, READ);
        let mut gateway = ToolGateway::new(token);

        let req = ToolRequest::new(0x100, WRITE, "update_records");
        let result = gateway.execute_request(req);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "PermissionDenied: active token lacks required permissions for this action"
        );
    }
}
