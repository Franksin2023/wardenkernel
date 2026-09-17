# 5. Agent Gateway Pattern

## Context
Large Language Model (LLM) agents and autonomous multi-agent systems frequently construct and execute tool requests dynamically. In multi-agent delegation chains (where a root agent spawns sub-agents or tool runners), sub-agents may attempt to invoke ungranted tools or escalate permissions due to prompt injection, hallucination, or compromise.

`ToolGateway` (`warden-gateway`) acts as an inline, zero-trust execution firewall that evaluates every tool invocation against the agent's active `CapabilityToken`.

## Decision
- Defined `ToolRequest` with `resource_id` (u64), `required_permission` (u32), and `payload` (`String`).
- Implemented `ToolGateway` pairing the execution context with an active `CapabilityToken`.
- Implemented `ToolGateway::execute_request(&mut self, request: ToolRequest) -> Result<String, &'static str>` enforcing:
  1. `active_token.resource == request.resource_id`.
  2. `(request.required_permission & !active_token.permissions) == 0`.
- Integrated `ToolGateway` with the `CapabilityToken::delegate` narrowing mechanism so that sub-agent contexts inherit strictly narrowed tokens before making gateway requests.

## Consequences
- Guarantees that prompt injection or sub-agent compromise cannot execute ungranted tool operations at the gateway level.
- Provides a clean, lightweight interception layer for LLM tool-calling loops without introducing external runtime dependencies or latency.
- Validated via `src/gateway_tests.rs` proving that unauthorized tool operations by restricted sub-agents are reliably intercepted and blocked.
