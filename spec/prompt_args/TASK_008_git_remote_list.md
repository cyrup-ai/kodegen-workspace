# TASK 008: Trim git_remote_list Prompts (Complexity 1)

**Tool**: `git_remote_list`
**Current size**: 519 lines (4 scenarios)
**Target size**: 90-110 lines (1 scenario)
**Complexity**: 1 (Trivial)
**Reference**: [PRECURSOR_01_memory_list_libraries.md](PRECURSOR_01_memory_list_libraries.md)

---

## Overview

The `git_remote_list` tool lists all configured Git remotes in a repository. This is a simple, focused operation that lists remote repositories and their URLs. The current 519-line prompt file is 5x too verbose for such a straightforward tool.

The file is located at:
`/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_list/prompts.rs`

---

## Current File Analysis

### Current Scenarios (ALL TO BE DELETED EXCEPT ONE)

The prompts.rs file currently implements **4 separate scenarios**:

1. **prompt_basic()** (~100 lines) - KEEP THIS ONE
   - Covers: Basic remote listing, response structure, common remotes
   - Most essential and concise
   - Shows simple usage and typical workflows

2. **prompt_verbose()** (~110 lines) - DELETE
   - Covers: Detailed URL information, fetch/push distinction
   - Redundant with basic scenario
   - Advanced feature not needed for core understanding

3. **prompt_verification()** (~140 lines) - DELETE
   - Covers: Verification patterns and troubleshooting
   - Contains 5 specific scenarios for verification
   - Too much depth for Complexity 1 tool

4. **prompt_comprehensive()** (~160 lines) - DELETE
   - Covers: Complete guide with all features, workflows, troubleshooting
   - Massive overkill for a trivial tool
   - Repeats information from other scenarios

### Routing Logic (MUST UPDATE)

Current `generate_prompts()` method:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("verbose") => prompt_verbose(),
        Some("verification") => prompt_verification(),
        _ => prompt_comprehensive(),  // DEFAULT TO COMPREHENSIVE
    }
}
```

This will be simplified to:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()  // ALWAYS RETURN BASIC
}
```

---

## Trimming Instructions

### WHAT TO KEEP (Target: ~100 lines total)

**Keep ONLY the `prompt_basic()` function** with these sections:

1. **Tool description** (8-10 lines):
   - "The git_remote_list tool shows all configured remote repositories"
   - Takes one required parameter: path
   - Returns array of remote names and URLs

2. **Basic usage** (10-15 lines):
   - Example: `git_remote_list({"path": "/project"})`
   - Shows typical response with origin and upstream remotes
   - Shows what empty response looks like

3. **Response structure** (8-10 lines):
   - Explain remotes array structure
   - Describe name field (e.g., "origin", "upstream")
   - Describe url field (e.g., "https://github.com/user/repo.git")

4. **When to use** (12-15 lines):
   - Verify remote after cloning
   - Check which remotes are available before push/pull
   - Confirm fork workflow setup (origin + upstream)
   - Troubleshoot remote-related issues

5. **Common remote names** (10-15 lines):
   - origin: Main remote (from git clone)
   - upstream: Original repo (fork workflows)
   - personal/production: Custom remotes if applicable

6. **Common pattern** (25-30 lines):
   - Single complete workflow:
     1. Clone a repository
     2. List remotes to verify origin is correct
     3. Add upstream if working with fork
     4. List again to confirm both are configured

7. **Quick reference** (8-10 lines):
   - `git_remote_list({"path": "/project"})` → lists all remotes
   - Related tools: git_remote_add, git_remote_remove
   - No complex parameters needed

### WHAT TO DELETE ENTIRELY

**DELETE these functions completely:**
- `prompt_verbose()` - All 110+ lines
- `prompt_verification()` - All 140+ lines  
- `prompt_comprehensive()` - All 160+ lines

**DELETE these helper elements:**
- Decorative header line: `// ============================================================================`
- Comment: `// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO LIST GIT REMOTES`
- Any multi-section explanations
- Any extended workflow walkthroughs (>30 lines)
- Any "troubleshooting guide" sections
- Any "best practices" sections
- Any "integration with other tools" sections

### Update prompt_arguments()

Current method includes description for "basic, verbose, verification":
```rust
description: Some("Scenario to show (basic, verbose, verification)".to_string()),
```

Update to reflect only basic scenario is available. You can either:
- Option A: Keep scenario parameter but note it's ignored
  ```rust
  description: Some("Scenario parameter (currently basic only)".to_string()),
  ```
- Option B: Remove the required field entirely since there's only one scenario
  ```rust
  // Return empty vec! if no scenarios are supported
  vec![]
  ```

**Recommendation**: Go with Option A to maintain backward compatibility.

---

## Implementation Strategy

### Step 1: Keep prompt_basic() Function

Extract the current `prompt_basic()` function (lines ~42-115). This already contains approximately 100 lines and is the most concise scenario. Verify it includes:
- User question: "How do I list all configured remotes in a Git repository?"
- All 7 sections listed above
- Clear, concise explanations
- Practical code examples

### Step 2: Delete Other Scenario Functions

Remove completely:
- Function: `prompt_verbose()` (starts around line 117)
- Function: `prompt_verification()` (starts around line 240)
- Function: `prompt_comprehensive()` (starts around line 400)

### Step 3: Simplify generate_prompts()

Replace the match statement with:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

The `args` parameter is no longer used, so you can rename it to `_args` to suppress unused warnings.

### Step 4: Update prompt_arguments()

Simplify the description to reflect single scenario:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario parameter (basic only, parameter is ignored)".to_string()),
            required: Some(false),
        }
    ]
}
```

### Step 5: Validation

After making changes:
1. Run: `wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_list/prompts.rs`
2. Verify: 90-110 lines total
3. Verify: Only one function `fn prompt_basic()`
4. Verify: No other `fn prompt_*()` functions exist
5. Verify: No decorative separator lines (===, etc.)
6. Verify: File is clean and readable

---

## Core Rust Pattern

After trimming, the file structure will be:

```rust
//! Prompt messages for git_remote_list tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitRemoteListPromptArgs;

/// Prompt provider for git_remote_list tool
pub struct RemoteListPrompts;

impl PromptProvider for RemoteListPrompts {
    type PromptArgs = GitRemoteListPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario parameter (basic only, parameter is ignored)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic remote listing - THE ONLY scenario
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        // ... existing basic scenario content (~100 lines)
    ]
}
```

---

## Files Modified

**Only one file changes:**
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_list/prompts.rs`

**Related files (read-only, for context):**
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_list/mod.rs` - Module exports (no changes needed)
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_list/prompt_args.rs` - Argument types (no changes needed)
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-tools-git/src/tools/remote_list.rs` - Tool implementation (no changes needed)

---

## Success Criteria

All of these must be true after completing the task:

1. ✓ **Line count**: File is between 90-110 lines (verify with `wc -l`)
2. ✓ **One scenario only**: Only `prompt_basic()` function exists
3. ✓ **No comprehensive**: `prompt_comprehensive()` deleted entirely
4. ✓ **No verbose**: `prompt_verbose()` deleted entirely
5. ✓ **No verification**: `prompt_verification()` deleted entirely
6. ✓ **Clean routing**: `generate_prompts()` always returns `prompt_basic()`
7. ✓ **No decorative headers**: No `====`, `----`, or `====` separator lines
8. ✓ **Readable in 60 seconds**: Anyone can understand the tool in under 1 minute
9. ✓ **No duplication**: Each concept explained exactly once
10. ✓ **Clear struct**: Follows pattern from PRECURSOR_01

---

## Why This Matters

The `git_remote_list` tool is trivial:
- **Input**: One required parameter (path)
- **Output**: Array of remotes with names and URLs
- **Complexity**: List and return

A trivial tool needs a trivial prompt. 519 lines teaches nothing that 100 lines doesn't cover better. The current prompts spend 400+ lines on scenarios most users will never read.

By trimming to the essential `prompt_basic()` scenario, we:
1. Reduce cognitive load for AI agents
2. Make the prompt easier to maintain
3. Establish the pattern for 15+ other Complexity 1 tools
4. Demonstrate focus on essential functionality
5. Save context window tokens in LLM interactions

---

## Definition of Done

This task is complete when:
1. File has 90-110 lines total
2. Only `prompt_basic()` function remains
3. `generate_prompts()` unconditionally returns `prompt_basic()`
4. `prompt_arguments()` reflects single scenario
5. File can be read and understood in under 60 seconds
6. All 4 success criteria above are verified true
