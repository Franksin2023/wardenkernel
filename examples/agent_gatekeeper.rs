//! Usage example demonstrating warden-core CapabilityToken in an autonomous agent tool-gatekeeper pipeline.

use wardenkernel::{CapabilityToken, MAX_DELEGATION_DEPTH};

// Define permission bit flags for agent tool access
const TOOL_READ_DATA: u32 = 1 << 0;
const TOOL_WRITE_DATA: u32 = 1 << 1;
const TOOL_EXECUTE_CODE: u32 = 1 << 2;
const TOOL_DELETE_RECORD: u32 = 1 << 3;

/// Simulated Gatekeeper representing an agent tool runner
struct AgentToolRunner {
    resource_id: u64,
}

impl AgentToolRunner {
    fn new(resource_id: u64) -> Self {
        Self { resource_id }
    }

    fn execute_action(&self, token: &CapabilityToken, required_perm: u32) -> Result<&'static str, &'static str> {
        if token.resource != self.resource_id {
            return Err("ResourceMismatch: token resource ID does not match target tool runner");
        }

        if (required_perm & !token.permissions) != 0 {
            return Err("PermissionDenied: token lacks required permission for this tool operation");
        }

        Ok("Action executed successfully")
    }
}

fn main() {
    let database_tool_id: u64 = 0x4442_5f54_4f4f_4c31; // "DB_TOOL1"
    let tool_runner = AgentToolRunner::new(database_tool_id);

    // Root Agent creates full root capability token
    let root_token = CapabilityToken::new(
        database_tool_id,
        TOOL_READ_DATA | TOOL_WRITE_DATA | TOOL_EXECUTE_CODE | TOOL_DELETE_RECORD,
    );

    println!("Root Agent initialized root token: {:?}", root_token);

    // Root Agent delegates read+write token to Analysis Sub-Agent
    let analysis_subagent_token = root_token
        .delegate(TOOL_READ_DATA | TOOL_WRITE_DATA)
        .expect("Root agent delegation to analysis subagent failed");

    println!("Analysis Sub-Agent received token: {:?}", analysis_subagent_token);

    // Analysis Sub-Agent executes READ action
    assert!(tool_runner
        .execute_action(&analysis_subagent_token, TOOL_READ_DATA)
        .is_ok());

    // Analysis Sub-Agent further delegates READ-ONLY token to Untrusted Tool Runner
    let untrusted_tool_token = analysis_subagent_token
        .delegate(TOOL_READ_DATA)
        .expect("Subagent delegation to untrusted tool runner failed");

    println!("Untrusted Tool Runner received token: {:?}", untrusted_tool_token);

    // Untrusted Tool Runner attempts unauthorized WRITE action -> BLOCKED BY GATEKEEPER
    let write_result = tool_runner.execute_action(&untrusted_tool_token, TOOL_WRITE_DATA);
    assert!(write_result.is_err());
    println!("Gatekeeper blocked unauthorized write: {:?}", write_result.unwrap_err());

    // Untrusted Tool Runner attempts adversarial delegation to gain DELETE permissions -> REJECTED BY TOKEN
    let escalation_attempt = untrusted_tool_token.delegate(TOOL_READ_DATA | TOOL_DELETE_RECORD);
    assert!(escalation_attempt.is_err());
    println!(
        "Token primitive rejected privilege escalation delegation: {:?}",
        escalation_attempt.unwrap_err()
    );

    println!("\nAgent Tool-Gatekeeper example completed successfully!");
}
