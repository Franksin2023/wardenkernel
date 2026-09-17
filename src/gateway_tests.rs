//! Gateway integration tests executed inside the bare-metal kernel environment.

use crate::capability::CapabilityToken;
use crate::gateway::{ToolGateway, ToolRequest};

const READ: u32 = 1 << 0;
const WRITE: u32 = 1 << 1;
const EXECUTE: u32 = 1 << 2;

pub fn run_gateway_tests() -> bool {
    let api_endpoint_id: u64 = 0x1A2B_3C4D_5E6F;

    // 1. Root agent initializes with broad access (Read + Write)
    let root_token = CapabilityToken::new(api_endpoint_id, READ | WRITE);
    let mut gateway = ToolGateway::new(root_token);

    // Verify Root agent can perform Read operation
    let read_req = ToolRequest::new(api_endpoint_id, READ, "query=analytics");
    if gateway.execute_request(read_req).is_err() {
        return false;
    }

    // Verify Root agent can perform Write operation
    let write_req = ToolRequest::new(api_endpoint_id, WRITE, "action=update_config");
    if gateway.execute_request(write_req).is_err() {
        return false;
    }

    // 2. Root agent spawns a restricted sub-agent token (Read-only)
    let subagent_token = match root_token.delegate(READ) {
        Ok(t) => t,
        Err(_) => return false,
    };

    // Switch gateway active context to restricted sub-agent
    gateway.set_token(subagent_token);

    // 3. Sub-agent attempts Read operation through gateway -> SUCCEEDS
    let sub_read_req = ToolRequest::new(api_endpoint_id, READ, "query=user_profile");
    if gateway.execute_request(sub_read_req).is_err() {
        return false;
    }

    // 4. Sub-agent attempts Write operation through gateway -> INTERCEPTED AND BLOCKED
    let sub_write_req = ToolRequest::new(api_endpoint_id, WRITE, "action=delete_logs");
    let sub_write_res = gateway.execute_request(sub_write_req);
    if sub_write_res.is_ok() {
        return false;
    }

    // 5. Sub-agent attempts privilege escalation delegation to gain EXECUTE -> REJECTED
    if subagent_token.delegate(READ | EXECUTE).is_ok() {
        return false;
    }

    true
}
