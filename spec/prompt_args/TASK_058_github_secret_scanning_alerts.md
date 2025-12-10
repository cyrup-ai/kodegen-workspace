# TASK 058: Trim github_secret_scanning_alerts Prompts

**Tool**: `github_secret_scanning_alerts`
**Complexity**: 2 (Simple)
**Current size**: 1254 lines (6 scenarios total)
**Target size**: 170-200 lines (1 scenario)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/secret_scanning_alerts/prompts.rs`

---

## Current State Analysis

### File Location
- **Absolute path**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/secret_scanning_alerts/prompts.rs`
- **Current line count**: 1254 lines
- **Module purpose**: Provides prompt templates for the github_secret_scanning_alerts MCP tool

### Current Scenario Inventory
The prompts.rs file contains 6 scenarios:
1. **prompt_basic()** (lines 49-310, ~262 lines): Basic alert retrieval, filtering, understanding parameters and response structure. Teaches how to use the tool to retrieve and interpret alerts.
2. **prompt_remediation()** (lines 312-700, ~388 lines): Step-by-step remediation guide for exposed secrets. Covers revocation, investigation, code cleanup, and git history removal.
3. **prompt_workflows()** (lines 702-850, ~148 lines): Automation patterns including continuous monitoring, triage, remediation automation, compliance reporting, and tool integration.
4. **prompt_audit()** (lines 852-975, ~123 lines): Security audit procedures including organization-wide assessment, compliance-specific audits (SOC 2, PCI DSS, ISO 27001), and monthly reviews.
5. **prompt_prevention()** (lines 977-1100, ~123 lines): Prevention best practices including developer education, pre-commit hooks, gitignore configuration, secret management, code review practices, and continuous monitoring.
6. **prompt_comprehensive()** (lines 1102-1254, ~152 lines): Complete guide covering all aspects in condensed form (default fallback scenario).

### Current Routing Logic
Located in `impl PromptProvider for SecretScanningAlertsPrompts`:

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("remediation") => prompt_remediation(),
        Some("workflows") => prompt_workflows(),
        Some("audit") => prompt_audit(),
        Some("prevention") => prompt_prevention(),
        _ => prompt_comprehensive(),
    }
}
```

**Current behavior**: 6-way match statement that routes to appropriate scenario function, defaulting to comprehensive.

### Prompt Arguments Configuration
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, remediation, workflows, audit, prevention)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Current state**: Documents all 5 available named scenarios plus default.

---

## Implementation Instructions

### Step 1: Simplify Routing Logic (Lines 19-32)

**What to do**: Replace the 6-way match statement with a simplified routing that only acknowledges the "basic" scenario.

**Before** (current code at lines 19-32):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("remediation") => prompt_remediation(),
        Some("workflows") => prompt_workflows(),
        Some("audit") => prompt_audit(),
        Some("prevention") => prompt_prevention(),
        _ => prompt_comprehensive(),
    }
}
```

**After** (replace with):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") | None => prompt_basic(),
        _ => prompt_basic(),
    }
}
```

**Alternative simpler form**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    // All scenarios route to the single basic scenario
    let _ = args; // Suppress unused warning
    prompt_basic()
}
```

**Rationale**: Complexity 2 tools have only one essential scenario. All requests default to prompt_basic() which covers core usage.

---

### Step 2: Update Prompt Arguments (Lines 34-42)

**What to do**: Simplify the scenario argument description to reflect that only "basic" is available.

**Before** (current code):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, remediation, workflows, audit, prevention)".to_string()),
            required: Some(false),
        }
    ]
}
```

**After** (replace with):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario type (basic only - shows alert retrieval and filtering)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Rationale**: Description now accurately reflects available options, preventing user confusion.

---

### Step 3: Remove Decorative Header Comment Block (Lines 43-48)

**What to do**: Delete the decorative section comment.

**Content to delete**:
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE SECRET SCANNING ALERTS
// ============================================================================
```

**Rationale**: Decorative headers are removed per task requirements.

---

### Step 4: Keep prompt_basic() Function (Lines 49-310)

**Action**: KEEP this entire function unchanged. This is the only scenario function to retain.

**Function signature**:
```rust
/// Basic alert listing and understanding alert data
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_secret_scanning_alerts to find exposed secrets in my repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                // ... extensive content covering:
                // - Basic alert retrieval with examples
                // - Required and optional parameters
                // - Understanding response structure
                // - Alert object fields and types
                // - Common secret types
                // - Pagination handling
                // - Authentication requirements
                // - Error scenarios
                // - Rate limiting
                // - Best practices
            ),
        },
    ]
}
```

**Content coverage**:
- Basic alert retrieval (4 examples with different filters)
- Required parameters: owner, repo
- Optional parameters: state, secret_type, resolution, page, per_page
- Response structure (success, owner, repo, total_count, alerts array)
- Alert object structure with 15+ fields
- Common secret types (10+ types listed)
- Pagination implementation
- Authentication requirements and scopes
- Error scenarios with solutions
- Rate limiting information
- Best practices for listing

**Keep this function because**:
- Covers essential tool usage for Complexity 2
- Teaches AI agents core retrieval patterns
- Includes practical filtering examples
- Explains parameter meanings
- Documents response structure comprehensively

---

### Step 5: Delete All Other Scenario Functions

**Delete COMPLETELY**:
- Lines 312-700: `fn prompt_remediation()` (388 lines)
- Lines 702-850: `fn prompt_workflows()` (148 lines)
- Lines 852-975: `fn prompt_audit()` (123 lines)
- Lines 977-1100: `fn prompt_prevention()` (123 lines)
- Lines 1102-1254: `fn prompt_comprehensive()` (152 lines)

**Total lines to delete**: 934 lines

**Rationale**: 
- "Use-case scenarios" (remediation, workflows, audit, prevention) teach advanced patterns beyond core tool usage
- Comprehensive scenario duplicates basic information in condensed form
- Complexity 2 tools maintain only essential scenarios
- Trimming reduces cognitive load, making prompts more focused

---

## Execution Sequence

Execute these operations IN ORDER:

1. **Edit routing logic** (lines 19-32): Replace match statement with simplified single-call routing
2. **Edit prompt arguments** (lines 34-42): Update description string
3. **Delete decorator comment** (lines 43-48): Remove header block
4. **Verify prompt_basic keeps** (lines 49-310): Confirm no edits made
5. **Delete remediation function** (lines 312-700): Remove entire function definition
6. **Delete workflows function** (lines 702-850): Remove entire function definition
7. **Delete audit function** (lines 852-975): Remove entire function definition
8. **Delete prevention function** (lines 977-1100): Remove entire function definition
9. **Delete comprehensive function** (lines 1102-1254): Remove entire function definition

---

## Success Criteria (MUST ALL PASS)

1. **File size**: Final file is 170-200 lines
   - Measure: `wc -l prompts.rs` returns value in range [170, 200]

2. **Scenario count**: Exactly 1 scenario function exists
   - Measure: grep -c "^fn prompt_" returns `1`
   - Only `fn prompt_basic()` should exist

3. **Routing simplification**: Match statement fully simplified
   - Measure: `generate_prompts()` function has no conditional logic OR single match arm
   - All code paths route to `prompt_basic()`

4. **Compilation**: Code compiles without warnings or errors
   - Measure: `cargo check` in kodegen-mcp-schema package passes
   - No unused import or function warnings

5. **Prompt arguments**: Description reflects "basic only" status
   - Measure: "basic" or "basic only" appears in scenario description string
   - No references to remediation, workflows, audit, prevention in description

6. **No decorative headers**: All "====" and "HELPER FUNCTIONS" comments removed
   - Measure: grep "HELPER FUNCTIONS" returns no results
   - grep "=====" returns no results

7. **No duplicate content**: Comprehensive function completely removed
   - Measure: grep -c "QUICK START" returns `0` (that pattern only in comprehensive)
   - grep -c "Give me a complete guide" returns `0`

---

## Code Patterns Reference

### Pattern 1: Simplified Routing (Pick ONE approach)

**Approach A - Single match arm** (most explicit):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    // All scenarios consolidated to basic
    match args.scenario.as_deref() {
        _ => prompt_basic(),
    }
}
```

**Approach B - Direct call** (simplest):
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

**Approach C - Accept basic or default** (most compatible):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") | None => prompt_basic(),
        _ => prompt_basic(),
    }
}
```

**Recommendation**: Use Approach B for simplicity and clarity. The function signature doesn't need the args parameter.

---

## Expected Outcome

### Before Trim
- 1254 total lines
- 6 scenario functions: basic, remediation, workflows, audit, prevention, comprehensive
- Match statement with 6 branches
- Complex routing logic

### After Trim
- 170-200 total lines
- 1 scenario function: basic only
- Simplified/removed routing (or single match arm)
- Clear, focused prompts

### File Structure After Completion
```rust
//! Prompt messages for github_secret_scanning_alerts tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::SecretScanningAlertsPromptArgs;

/// Prompt provider for secret_scanning_alerts tool
pub struct SecretScanningAlertsPrompts;

impl PromptProvider for SecretScanningAlertsPrompts {
    type PromptArgs = SecretScanningAlertsPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario type (basic only - shows alert retrieval and filtering)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic alert listing and understanding alert data
fn prompt_basic() -> Vec<PromptMessage> {
    // ... ~150 lines of content ...
}
```

---

## Testing and Verification

After completion, verify with these commands:

```bash
# Check file size
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/secret_scanning_alerts/prompts.rs

# Count scenario functions
grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/secret_scanning_alerts/prompts.rs | wc -l

# Verify no decorative headers
grep "=====" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/secret_scanning_alerts/prompts.rs | wc -l

# Check compilation
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
cargo check 2>&1 | grep -E "error|warning"
```

---

## Notes

- This is a Complexity 2 task requiring straightforward deletions and simple edits
- No new functionality added, only streamlining
- All prompts remain functionally correct after trim
- The basic scenario is sufficient for Complexity 2 tool documentation
- Trimming reduces maintenance burden and improves clarity
