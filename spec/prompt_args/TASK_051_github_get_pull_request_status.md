# TASK 051: Restructure github_get_pull_request_status Prompts

**Tool**: `github_get_pull_request_status`
**Complexity**: 2 (Simple)
**Current size**: 111 lines
**Target size**: 200-220 lines
**Scenarios**: Restructure to 1-2 focused scenarios
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_pull_request_status/prompts.rs`
**Related**: `prompt_args.rs`, `schema.rs`

---

## Reference

See **PRECURSOR_02_fs_read_file.md** for Complexity 2 template approach.

---

## Current State Analysis

### File Structure (111 lines total)

**Location**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_pull_request_status/prompts.rs`

**Current Contents**:
- Lines 1-11: Imports and struct definition
- Lines 13-17: PromptProvider trait implementation header
- Lines 19-108: Single monolithic `generate_prompts()` method with ONE User/Assistant message pair
- Lines 110-111: Empty `prompt_arguments()` method

**Current Prompt Content Structure** (Lines 24-107):
1. Intro paragraph (~3 lines) - Tool description
2. BASIC USAGE section (~8 lines) - 4 examples covering:
   - Check PR status
   - Verify merge readiness
   - Check CI status
   - Monitor PR state
3. PARAMETERS section (~6 lines) - owner, repo, pull_number definitions
4. AUTHENTICATION section (~4 lines) - GITHUB_TOKEN requirements
5. RESPONSE section (~18 lines) - 17 response fields listed
6. COMMON WORKFLOWS section (~18 lines) - 3 detailed workflows:
   - Auto-merge eligibility (6 steps)
   - CI/CD monitoring (4 steps)
   - Branch status check (3 steps)
7. RATE LIMITING section (~4 lines) - Request limits
8. ERROR SCENARIOS section (~9 lines) - 3 error cases with fixes
9. BEST PRACTICES section (~14 lines) - 10 best practices bullets

### Prompt Arguments Structure

**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_pull_request_status/prompt_args.rs`

**Current state**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPullRequestStatusPromptArgs {}
```

The struct is **EMPTY** - no scenario variants. Unlike similar tools like `get_pull_request_files` (which has scenario: Option<String>), this tool has no parameterization.

### Routing Pattern (Lines 16-18)

Currently, `generate_prompts()` always returns the same single prompt regardless of input:
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![/* single prompt hardcoded */]
}
```

---

## Implementation Strategy

This task has TWO execution paths depending on architectural preference:

### PATH A: Single Expanded Scenario (Simpler)
Keep as ONE scenario but restructure and expand to ~200-220 lines total.

**Rationale**: No scenario variants exist yet; adding them requires changing prompt_args.rs.

**Advantages**: Minimal file changes, focused single use case.

**Disadvantages**: Less flexibility for different usage patterns.

### PATH B: Two Focused Scenarios (Recommended - Following Template)
Add scenario support to prompt_args.rs and split into 2 functions:
1. `prompt_basic()` - Basic PR status retrieval
2. `prompt_merge_decision()` - Decision workflows for auto-merge

**Rationale**: Aligns with Complexity 2 template (fs_read_file pattern has 2 focused scenarios).

**Advantages**: Clearer separation of concerns, easier for agents to choose usage pattern.

**Disadvantages**: Requires changes to prompt_args.rs and routing logic.

---

## IMPLEMENTATION: Path B (Recommended)

### Step 1: Update prompt_args.rs

Add scenario enum to prompt arguments:

**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_pull_request_status/prompt_args.rs`

**Before** (7 lines):
```rust
//! Prompt argument types for github_get_pull_request_status tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPullRequestStatusPromptArgs {}
```

**After** (18 lines):
```rust
//! Prompt argument types for github_get_pull_request_status tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_get_pull_request_status tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPullRequestStatusPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Retrieving pull request status and checks
    /// - "merge_decision": Evaluating merge readiness and workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
```

### Step 2: Restructure prompts.rs

**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_pull_request_status/prompts.rs`

#### Update generate_prompts() Method

Replace the single monolithic prompt with scenario routing (Lines 19-108):

**Change Pattern**:

**Before** (lines 19-108):
```rust
    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_get_pull_request_status to check PR status?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_pull_request_status tool retrieves comprehensive status information about a pull request including state, checks, and mergeability.\n\n\
                 BASIC USAGE:\n\
                 ...[entire 70+ line content]...\n\
                 - Implement exponential backoff for status polling",
            ),
        },    ]
    }
```

**After** (target: ~25 lines for routing):
```rust
    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("merge_decision") => Self::prompt_merge_decision(),
            _ => Self::prompt_basic(),
        }
    }
```

#### Add prompt_basic() Function (Lines after routing - target: ~105 lines)

Focus on retrieval, parameters, and response structure:

```rust
    fn prompt_basic() -> Vec<PromptMessage> {
        vec![
            PromptMessage {
                role: PromptMessageRole::User,
                content: PromptMessageContent::text(
                    "How do I use github_get_pull_request_status to get PR information?",
                ),
            },
            PromptMessage {
                role: PromptMessageRole::Assistant,
                content: PromptMessageContent::text(
                    "The github_get_pull_request_status tool retrieves pull request status including state, checks, \
                     and mergeability information.\n\n\
                     BASIC USAGE:\n\
                     1. Check PR status:\n\
                        github_get_pull_request_status({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"pull_number\": 5678})\n\n\
                     2. Verify PR is open and check CI:\n\
                        github_get_pull_request_status({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"pull_number\": 123})\n\n\
                     3. Monitor PR state changes:\n\
                        github_get_pull_request_status({\"owner\": \"actix\", \"repo\": \"actix-web\", \"pull_number\": 999})\n\n\
                     PARAMETERS:\n\
                     - owner (required): Repository owner (username or organization)\n\
                     - repo (required): Repository name\n\
                     - pull_number (required): Pull request number\n\n\
                     REQUIRED: GITHUB_TOKEN environment variable with scopes:\n\
                     - repo (for private repositories)\n\
                     - public_repo (for public repositories only)\n\n\
                     RESPONSE FIELDS:\n\
                     - success: true/false - Operation succeeded\n\
                     - state: \"open\", \"closed\", or \"merged\" - PR state\n\
                     - merged: true/false - If PR was merged\n\
                     - mergeable: true/false/null - Can merge (null = calculating)\n\
                     - mergeable_state: \"clean\", \"dirty\", \"blocked\", \"unstable\", \"behind\" - Merge status\n\
                     - draft: true/false - Is draft PR\n\
                     - checks_status: \"success\", \"pending\", \"failure\" - Combined CI status\n\
                     - review_decision: \"APPROVED\", \"CHANGES_REQUESTED\", \"REVIEW_REQUIRED\"\n\
                     - title: Pull request title\n\
                     - head: Source branch SHA and ref\n\
                     - base: Target branch SHA and ref\n\
                     - created_at, updated_at: ISO timestamps\n\
                     - html_url: Link to PR on GitHub\n\n\
                     WHEN TO USE:\n\
                     - Always read PR status BEFORE attempting any merge operations\n\
                     - Check mergeable_state to diagnose why PR can't merge\n\
                     - Monitor checks_status while waiting for CI to complete\n\
                     - Verify review_decision to ensure approval requirements met\n\
                     - Compare head/base SHAs to confirm PR not stale\n\n\
                     COMMON PATTERNS:\n\
                     - Get fresh PR state: Call with owner, repo, pull_number\n\
                     - Check if ready to merge: mergeable=true AND checks_status=\"success\" AND review_decision=\"APPROVED\"\n\
                     - Handle mergeable=null: GitHub calculating; retry after 1-2 seconds\n\
                     - Interpret mergeable_state: \"behind\"=update branch, \"dirty\"=resolve conflicts, \"blocked\"=check requirements\n\n\
                     RATE LIMITS:\n\
                     - Authenticated: 5,000 requests/hour\n\
                     - Unauthenticated: 60 requests/hour\n\
                     - Monitor X-RateLimit-Remaining response header\n\n\
                     COMMON ERRORS:\n\
                     - 404 Not Found: PR or repository doesn't exist - verify owner/repo/pull_number\n\
                     - 403 Forbidden: No access to private repository - verify GITHUB_TOKEN has repo scope\n\
                     - mergeable=null: GitHub still calculating - normal for new PRs, wait and retry",
                ),
            },
        ]
    }
```

**Size Target**: 105-115 lines for this function (includes docstring and all examples)

#### Add prompt_merge_decision() Function (Lines after basic - target: ~105 lines)

Focus on workflows, error handling, and best practices for merge decisions:

```rust
    fn prompt_merge_decision() -> Vec<PromptMessage> {
        vec![
            PromptMessage {
                role: PromptMessageRole::User,
                content: PromptMessageContent::text(
                    "How do I use github_get_pull_request_status to determine if a PR is ready to merge?",
                ),
            },
            PromptMessage {
                role: PromptMessageRole::Assistant,
                content: PromptMessageContent::text(
                    "Use github_get_pull_request_status to evaluate merge readiness through workflows and \
                     decision trees.\n\n\
                     AUTO-MERGE ELIGIBILITY CHECK:\n\
                     1. Get PR status: Call github_get_pull_request_status({\"owner\": \"...\", \"repo\": \"...\", \"pull_number\": ...})\n\
                     2. Verify state open: Check response.state == \"open\" (not \"closed\" or \"merged\")\n\
                     3. Verify mergeable: Check response.mergeable == true (false = conflicts, null = wait)\n\
                     4. Verify CI passed: Check response.checks_status == \"success\"\n\
                     5. Verify approved: Check response.review_decision == \"APPROVED\"\n\
                     6. Skip drafts: Check response.draft == false (don't merge WIPs)\n\
                     7. Proceed: If all pass, PR is safe to merge\n\n\
                     CI/CD MONITORING PATTERN:\n\
                     - Poll PR status periodically during CI runs\n\
                     - Wait for checks_status: \"pending\" → \"success\" or \"failure\"\n\
                     - Alert team on \"failure\" - CI broken\n\
                     - Track time from creation to green status\n\
                     - Cancel polling when state != \"open\"\n\n\
                     BRANCH CONFLICT RESOLUTION:\n\
                     - If mergeable_state == \"behind\": Update base branch from remote\n\
                     - If mergeable_state == \"dirty\": Resolve merge conflicts in source branch\n\
                     - If mergeable_state == \"blocked\": Check branch protection rules required\n\
                     - Retry after fixes - mergeable_state recalculates\n\n\
                     BEST PRACTICES:\n\
                     - Always check mergeable BEFORE merge operations\n\
                     - Handle mergeable=null by retrying after 1-2 second delay\n\
                     - Use mergeable_state for detailed diagnosis (not just boolean)\n\
                     - Verify checks_status before auto-merge\n\
                     - Prevent merging draft PRs (check draft field)\n\
                     - Require approval (review_decision == \"APPROVED\")\n\
                     - Compare head/base SHAs - if different, PR has new commits\n\
                     - Use state field to ignore closed/merged PRs\n\
                     - Implement exponential backoff (1s, 2s, 4s) when polling\n\
                     - Set timeout: If mergeable null after 30s, fail safe\n\n\
                     DECISION TREE:\n\
                     Is state \"open\"? → NO: Already merged or closed, skip merge attempt\n\
                     Is draft true? → YES: WIP PR, stop, notify author\n\
                     Is mergeable null? → YES: Still calculating, wait 2 seconds and retry\n\
                     Is mergeable true? → NO: Has conflicts, notify to fix\n\
                     Is checks_status \"success\"? → NO: CI failed, alert team\n\
                     Is review_decision \"APPROVED\"? → NO: Missing approval, add to queue\n\
                     All passed? → YES: Safe to merge",
                ),
            },
        ]
    }
```

**Size Target**: 105-115 lines for this function

### Step 3: Verify Structure

After changes, the file should have:

**Lines 1-11**: Unchanged imports and struct definition
**Lines 13-17**: Unchanged trait header
**Lines 19-30**: NEW routing in generate_prompts() (~12 lines)
**Lines 32-138**: NEW prompt_basic() function (~107 lines)
**Lines 140-245**: NEW prompt_merge_decision() function (~106 lines)
**Lines 247-248**: Unchanged prompt_arguments() method

**Total target**: 248 lines ± 10

### Step 4: Verify No Other Changes Needed

The routing logic in `generate_prompts()` uses `args.scenario.as_deref()` which works with the Option<String> field added to prompt_args. No changes needed to schema.rs or mod.rs files.

---

## Success Criteria

### File Metrics
- Total lines: 240-260 (increased from 111 to support 2 scenarios)
- Scenarios: Exactly 2 functions (prompt_basic, prompt_merge_decision)
- Routing: Single match statement with default case

### Content Requirements
- prompt_basic() is 100-120 lines covering:
  - 3+ usage examples
  - All 3 parameters clearly explained
  - All 13+ response fields documented
  - Authentication requirements stated
  - Common patterns (5+ examples)
- prompt_merge_decision() is 100-120 lines covering:
  - 4 detailed workflows (auto-merge, CI monitoring, conflict resolution, decision tree)
  - 10+ best practices
  - Rate limiting mentioned once
  - Error handling for mergeable=null

### No Redundancy
- Each section appears in ONLY ONE scenario (no duplication between functions)
- Each parameter explained ONE time
- Response structure documented ONE time
- "mergeable=null" handling explained ONCE (in both scenarios is acceptable as it's critical)

### Code Quality
- No decorative headers (═══, ---, etc.)
- All examples use realistic GitHub repos (tokio-rs, rust-lang, actix, etc.)
- All code examples are valid JSON
- Line length < 120 characters (natural Rust string formatting)

---

## Validation Checklist

After implementation:

1. Line count: `wc -l prompts.rs` → Output should be 240-260 lines
2. Compile: `cd packages/kodegen-mcp-schema && cargo check` → No errors
3. Scenarios exist: `grep "fn prompt_" prompts.rs` → 2 matches (basic, merge_decision)
4. Routing works: `grep "scenario.as_deref" prompts.rs` → 1 match
5. Read test: Can understand both scenarios in 3 minutes
6. No duplication: `grep "always read" prompts.rs` → appears 1-2 times max
7. Parameter coverage: Both "mergeable", "checks_status", "review_decision" explained

---

## Reference: Similar Completed Tools

The `get_pull_request_files` tool (4 scenarios) and `fs_read_file` tool (trimmed to 2 scenarios) follow this pattern. This restructuring makes `github_get_pull_request_status` consistent with codebase standards.

---

## Architectural Notes

This is a **Complexity 2: Simple CRUD Operation**. The tool has:
- 3 required parameters (no optional parameters)
- 1 GET operation (no mutations)
- Single endpoint call (no sequential calls)
- Well-defined response structure

Two scenarios suffice because scenarios should differentiate between:
- Basic operation (retrieval, structure, parameters)
- Advanced operation (workflows, decision logic, error handling)

Not by use-case (code files vs config files) or duplicative scenarios.
