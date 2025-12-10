# TASK 001: Trim config_get Prompts to Single Scenario

**Tool**: `config_get`
**Complexity**: 1 (Trivial)
**Current size**: 614 lines (5 scenarios)
**Target size**: 90-110 lines (1 scenario)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/config/config_get/prompts.rs`

---

## Context

The `config_get` tool retrieves the complete server configuration including security settings, shell preferences, resource limits, and system information. The prompt file is responsible for teaching AI agents how to use this tool effectively.

### Current Implementation

The file contains 5 different scenarios:
1. `prompt_basic()` - Basic config retrieval (lines ~45-105)
2. `prompt_security()` - Understanding security constraints (lines ~107-211)
3. `prompt_troubleshooting()` - Debugging permission issues (lines ~213-367)
4. `prompt_inspection()` - Reviewing config before changes (lines ~369-526)
5. `prompt_comprehensive()` - Complete guide to config management (lines ~528-614, default fallback)

The routing logic in `generate_prompts()` uses a match statement to select which scenario to return based on the `scenario` argument in `ConfigGetPromptArgs`.

### Problem Statement

The current implementation contains redundant content across multiple scenarios. Each scenario re-explains similar concepts from different angles. This causes:
- Excessive prompt file size (614 lines)
- Cognitive load for the AI agent (too many prompt options)
- Maintenance burden (changes must be coordinated across multiple scenarios)
- Unnecessary token consumption

The solution is to consolidate all essential knowledge into a **single, lean "basic" scenario** that covers the most important use case: understanding how to retrieve and use the config_get response.

---

## Architecture Analysis

### File Structure

```
packages/kodegen-mcp-schema/src/config/config_get/
├── mod.rs              # Module re-exports
├── prompt_args.rs      # PromptArgs struct with scenario option
├── prompts.rs          # (THIS FILE - needs trimming)
└── schema.rs           # Tool input/output schema
```

### Current Routing Pattern

```rust
// In PromptProvider impl
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("security") => prompt_security(),
        Some("troubleshooting") => prompt_troubleshooting(),
        Some("inspection") => prompt_inspection(),
        _ => prompt_comprehensive(),  // DEFAULT - THIS MUST CHANGE
    }
}
```

**Key Issue**: The default case returns `prompt_comprehensive()`, which is the longest (87 lines). This should instead default to `prompt_basic()`.

### Prompt Arguments Structure

Current `ConfigGetPromptArgs` accepts an optional `scenario` parameter. When no scenario is provided, it defaults to `prompt_comprehensive()`. This can remain as-is, but the default routing will change.

---

## Implementation Instructions

### Step 1: Simplify the Routing Logic

**CHANGE**: The `generate_prompts()` method currently has a match statement. Simplify it to always return `prompt_basic()`:

```rust
impl PromptProvider for ConfigGetPrompts {
    type PromptArgs = ConfigGetPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()  // Always return the single scenario
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]  // No arguments needed - single scenario only
    }
}
```

**Rationale**: By removing the match statement and scenario selection, we:
- Eliminate unused code paths
- Simplify the API - there's now one clear way to use config_get prompts
- Reduce decision overhead for the AI agent

### Step 2: Keep Only prompt_basic()

**DELETE** these functions entirely:
- `prompt_security()` (lines ~107-211)
- `prompt_troubleshooting()` (lines ~213-367)  
- `prompt_inspection()` (lines ~369-526)
- `prompt_comprehensive()` (lines ~528-614)

**KEEP** `prompt_basic()` with the following structure:

#### Content to Preserve from prompt_basic()

The prompt_basic() function should teach these core concepts:

1. **What is config_get**: Returns complete server configuration
2. **Response Structure**: Full JSON example showing all sections
3. **When to Check Config**: 5 key decision points
4. **Common Use Cases**: 4 practical examples
5. **Key Constraint**: No parameters required

#### Clean Up Decorative Elements

Remove these elements to save lines:
- Decorative header lines with `=` characters
- Repetitive section introductions
- Examples that can be consolidated
- Verbose best practices lists that can be condensed to bullets

#### Target Length for prompt_basic()

Target: ~60-70 lines for the actual prompt content, plus function definition = ~70 lines total for the function.

### Step 3: Final Code Structure

The final `prompts.rs` should have this minimal structure:

```rust
//! Prompt messages for config_get tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::ConfigGetPromptArgs;

/// Prompt provider for config_get tool
pub struct ConfigGetPrompts;

impl PromptProvider for ConfigGetPrompts {
    type PromptArgs = ConfigGetPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

/// Basic config retrieval - teaches AI how to use config_get
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I retrieve and understand the server configuration?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The config_get tool retrieves complete server configuration with no parameters required.\n\n\
                 BASIC USAGE:\n\
                 config_get({})\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"config\": {\n\
                     \"security\": { /* blocked_commands, allowed_directories, max_file_size_bytes */ },\n\
                     \"shell\": { /* default_shell, timeout_ms */ },\n\
                     \"resources\": { /* file limits, memory, timeouts */ },\n\
                     \"system_info\": { /* platform, arch, cpu_count, memory */ }\n\
                   }\n\
                 }\n\n\
                 WHEN TO CHECK CONFIG:\n\
                 - Before file operations in specific directories\n\
                 - Before executing shell commands\n\
                 - When operations fail with permission errors\n\
                 - When planning resource-intensive operations\n\n\
                 COMMON PATTERNS:\n\
                 1. Check allowed directories: config = config_get({}); dirs = config.config.security.allowed_directories\n\
                 2. Check blocked commands: config = config_get({}); blocked = config.config.security.blocked_commands\n\
                 3. Verify file size limit: config = config_get({}); max_bytes = config.config.security.max_file_size_bytes\n\
                 4. Check timeouts: config = config_get({}); timeout = config.config.shell.timeout_ms\n\n\
                 INTERPRETATION:\n\
                 - allowed_directories [] = no restrictions (full access)\n\
                 - allowed_directories [paths] = only listed paths accessible\n\
                 - blocked_commands [] = no command restrictions\n\
                 - blocked_commands [cmds] = these specific commands are blocked\n\n\
                 Configuration is live and may change between calls. Always verify assumptions with config_get before critical operations."
            ),
        },
    ]
}
```

**Note**: The above is an *example template*. The actual content should be refined from the existing `prompt_basic()` function, keeping the most essential teaching points while removing verbose explanations.

---

## Implementation Plan

### Phase 1: Prepare the New Implementation

1. Copy the current `prompt_basic()` function
2. Remove all decorative headers (==== lines)
3. Consolidate redundant explanations
4. Trim verbose examples to 1-2 line pseudocode
5. Keep JSON response structure example (it's critical)
6. Keep the 4 common use cases (distilled)

### Phase 2: Update PromptProvider Implementation

1. Replace the entire match statement with direct `prompt_basic()` call
2. Change `prompt_arguments()` to return `vec![]`
3. Ignore the scenario parameter (use `_args` to signal it's unused)

### Phase 3: Delete Other Functions

1. Delete `prompt_security()` function entirely
2. Delete `prompt_troubleshooting()` function entirely
3. Delete `prompt_inspection()` function entirely
4. Delete `prompt_comprehensive()` function entirely
5. Delete the section comment `// ============================================================================`

### Phase 4: Verify

1. Ensure file is 90-110 lines total
2. Ensure it still compiles: `cargo check` in `kodegen-mcp-schema` package
3. Verify no references to deleted functions exist

---

## Code Patterns: What to Keep

### Essential Teaching Content

The prompt MUST teach:
1. **No parameters**: config_get() takes an empty object `{}`
2. **Complete response**: Always returns full config, never partial
3. **Response structure**: All 5 main sections (security, shell, resources, system_info, client_info)
4. **Common decision points**: When to call config_get (5 scenarios)
5. **Practical patterns**: 4 concrete usage examples

### Essential Response Structure

This JSON example is CRITICAL - keep it:
```json
{
  "success": true,
  "config": {
    "security": {
      "blocked_commands": [...],
      "allowed_directories": [...],
      "max_file_size_bytes": 10485760
    },
    "shell": {
      "default_shell": "/bin/bash",
      "timeout_ms": 300000
    },
    "resources": {
      "max_concurrent_terminals": 10,
      "file_read_line_limit": 1000,
      "file_write_line_limit": 50
    },
    "system_info": {
      "platform": "linux",
      "arch": "x86_64",
      "cpu_count": 8,
      "memory": { "total_mb": "16384" }
    }
  }
}
```

---

## Definition of Done

The task is complete when:

1. File `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/config/config_get/prompts.rs` has been modified to:
   - Line count: 90-110 lines (verify with `wc -l`)
   - Contains exactly ONE scenario function: `prompt_basic()`
   - Has NO decorative header lines (==== or similar)
   - Has simplified `generate_prompts()` that always calls `prompt_basic()`
   - Has empty `prompt_arguments()` returning `vec![]`
   - No references to security, troubleshooting, inspection, or comprehensive scenarios

2. File compiles successfully:
   ```bash
   cd packages/kodegen-mcp-schema
   cargo check
   ```

3. The MCP schema test (if exists) passes:
   ```bash
   cargo test
   ```

---

## Testing the Change

After making changes, verify:

1. **Compilation**: `cargo check` in the kodegen-mcp-schema package
2. **Line count**: `wc -l prompts.rs` should show 90-110
3. **No orphaned code**: Search for references to deleted functions
4. **Routing works**: The simplified `generate_prompts()` is called correctly

---

## Source References

- **Current implementation**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/config/config_get/prompts.rs`
- **Related files**:
  - `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/config/config_get/prompt_args.rs` (may need updates if scenario param is removed)
  - `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/config/config_get/mod.rs` (no changes needed)
  - `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/config/config_get/schema.rs` (no changes needed)

---

## Key Principles

1. **Single Responsibility**: One scenario teaches one essential thing - how to use config_get and interpret responses
2. **No Options**: Don't present "basic vs comprehensive". There's only one way to do this now.
3. **Lean Content**: Every line must teach something essential. Remove verbose explanations and redundant examples.
4. **Clean Routing**: No match statements, no branching logic. Always return the same prompt.
5. **Future Proof**: Future tools with multiple scenarios can follow this pattern, but config_get is intentionally simple.

---

## Notes for Developer

- The `ConfigGetPromptArgs` struct in `prompt_args.rs` still has the `scenario` field. You can leave it as-is (it will be ignored), or remove it if you want to completely eliminate the unused parameter. **Recommendation**: Leave it for now to minimize changes.
- The simplification makes the tool's AI teaching more focused and reduces prompt token consumption.
- This is a first step toward general prompt optimization. Other tools may follow similar trimming patterns.
