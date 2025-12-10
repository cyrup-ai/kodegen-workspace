# TASK 057: Trim github_search_users Prompts

**Tool**: `github_search_users`
**Complexity**: 2 (Simple)
**Current size**: 1153 lines
**Target size**: 170-220 lines (1 scenario)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/search_users/prompts.rs`

---

## Current State Analysis

### File Location
`/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/search_users/prompts.rs`

### Current Content Breakdown
- **Lines 1-42**: Module header, imports, struct definition, `PromptProvider` trait implementation
- **Lines 43-142**: `prompt_basic()` function - 100 lines - KEEP
- **Lines 143-331**: `prompt_syntax()` function - 189 lines - DELETE
- **Lines 332-560**: `prompt_workflows()` function - 229 lines - DELETE (use-case scenario)
- **Lines 561-823**: `prompt_advanced()` function - 263 lines - DELETE (use-case scenario)
- **Lines 824-1153**: `prompt_comprehensive()` function - 330 lines - DELETE (comprehensive scenario)

### Current Routing Logic
The match statement at lines 18-24 currently routes to 5 scenarios:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("syntax") => prompt_syntax(),
        Some("workflows") => prompt_workflows(),
        Some("advanced") => prompt_advanced(),
        _ => prompt_comprehensive(),
    }
}
```

### Current Scenarios Evaluation
1. **prompt_basic()**: Core functionality - covers basic user search operations, response structure, key fields, common patterns, authentication, rate limits, error handling, best practices. THIS IS ESSENTIAL.
2. **prompt_syntax()**: Detailed query syntax reference - 189 lines covering all GitHub search qualifiers. NOT ESSENTIAL for basic usage.
3. **prompt_workflows()**: Use-case driven scenarios (find experts, discover locals, find contributors, discover organizations, community analysis, talent recruitment). MUST DELETE - these are use-case focused.
4. **prompt_advanced()**: Advanced patterns with pagination, sorting, filtering strategies, rate limit optimization. MUST DELETE - this is use-case focused and too long.
5. **prompt_comprehensive()**: Complete guide combining all features. MUST DELETE - this is comprehensive scenario per requirements.

---

## Implementation Instructions

### Step 1: Trim the prompts.rs File

Delete all functions except `prompt_basic()`. This means removing lines 143-1153 (1011 lines total).

### Step 2: Update the PromptProvider Implementation

The `generate_prompts` method (lines 18-24) must be simplified. Replace the current match statement:

**BEFORE:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("syntax") => prompt_syntax(),
        Some("workflows") => prompt_workflows(),
        Some("advanced") => prompt_advanced(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER:**
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

**Rationale**: With only one scenario remaining, the match statement is unnecessary. The underscore prefix on `_args` indicates the parameter is intentionally unused (suppresses compiler warnings). The function always returns the basic prompt.

### Step 3: Update prompt_arguments()

The `prompt_arguments()` method at lines 26-34 currently documents the scenario parameter with reference to deleted scenarios. Replace:

**BEFORE:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, syntax, workflows, advanced)".to_string()),
            required: Some(false),
        }
    ]
}
```

**AFTER:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![]
}
```

**Rationale**: Since the scenario parameter is no longer used (no routing logic), there are no prompt arguments to document. Return an empty vector.

### Step 4: Clean Up Module Documentation

The comment at lines 37-39 is decorative and references deleted helper functions:

**BEFORE:**
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO SEARCH GITHUB USERS
// ============================================================================
```

**AFTER:**
Delete these lines entirely (they become unnecessary with only one scenario).

### Step 5: Verify Final Structure

After all deletions and modifications, the file should contain:

1. Module documentation and imports (lines 1-2)
2. Imports for required types (lines 4-6)
3. `SearchUsersPrompts` struct definition (lines 8-10)
4. `PromptProvider` trait implementation with simplified `generate_prompts()` (lines 12-18)
5. `prompt_arguments()` returning empty vector (lines 20-22)
6. `prompt_basic()` function (100 lines)

**Total expected lines**: ~130-145 lines (well within 170-220 target)

---

## Code Changes Summary

### File: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/search_users/prompts.rs`

#### Change 1: Simplify generate_prompts() method
- **Location**: Lines 18-24
- **Action**: Replace 7-line match statement with single function call
- **Effect**: Removes routing logic since only one scenario exists

#### Change 2: Simplify prompt_arguments() method
- **Location**: Lines 26-34
- **Action**: Return empty vector instead of PromptArgument with scenario parameter
- **Effect**: Removes now-unused scenario parameter documentation

#### Change 3: Delete decorative header comment
- **Location**: Lines 37-39
- **Action**: Delete lines entirely
- **Effect**: Removes reference to deleted helper functions

#### Change 4: Delete all scenario functions except basic
- **Location**: Lines 143-1153
- **Action**: Delete entire range
- **Functions deleted**:
  - `prompt_syntax()` (lines 143-331)
  - `prompt_workflows()` (lines 332-560)
  - `prompt_advanced()` (lines 561-823)
  - `prompt_comprehensive()` (lines 824-1153)
- **Effect**: Reduces file from 1153 to ~140 lines

---

## Implementation Example

Here's what the trimmed file structure will look like:

```rust
//! Prompt messages for github_user_search tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::SearchUsersPromptArgs;

/// Prompt provider for search_users tool
pub struct SearchUsersPrompts;

impl PromptProvider for SearchUsersPrompts {
    type PromptArgs = SearchUsersPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

/// Basic user search operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        // ... existing basic scenario content (100 lines) ...
    ]
}
```

---

## Success Criteria

All of the following must be true:

- **Line count**: Final file is between 130-145 lines (within 170-220 target)
- **Scenario count**: Only 1 scenario remains (`prompt_basic`)
- **No comprehensive scenario**: `prompt_comprehensive()` function completely deleted
- **No use-case scenarios**: `prompt_workflows()` and `prompt_advanced()` functions completely deleted
- **No syntax scenario**: `prompt_syntax()` function completely deleted
- **Routing simplified**: `generate_prompts()` method contains only a single function call, no match statement
- **No orphaned decorative headers**: All "HELPER FUNCTIONS" style comments removed
- **Compilation successful**: File compiles without errors or warnings
- **Function signatures unchanged**: `SearchUsersPrompts` struct and `PromptProvider` impl remain compatible with existing code

---

## Additional Notes

### Why Keep Basic Only?
The `prompt_basic()` scenario contains essential information for using the `github_search_users` tool:
- How to perform basic searches
- Response structure explanation
- Key fields documentation
- Common usage patterns
- Authentication requirements
- Rate limit awareness
- Error handling basics
- Best practices

This is sufficient for agents to effectively use the tool. Detailed syntax reference, advanced pagination patterns, and use-case workflows are not necessary for core functionality.

### Why Delete the Others?
- **prompt_syntax()**: At 189 lines, it's a comprehensive reference that duplicates information already in basic scenario
- **prompt_workflows()**: Use-case scenarios are explicitly marked for deletion per requirements
- **prompt_advanced()**: Advanced patterns and pagination details fall under use-case category
- **prompt_comprehensive()**: Explicitly listed in requirements as something to delete

### File Retention
All supporting files remain unchanged:
- `mod.rs` - module exports
- `prompt_args.rs` - argument types
- `schema.rs` - tool schema

Only `prompts.rs` is modified.

---

## Execution Checklist

When implementing this task:

1. Read the current prompts.rs file to understand structure
2. Delete lines 143-1153 (all scenario functions except basic)
3. Update `generate_prompts()` to remove match statement
4. Update `prompt_arguments()` to return empty vector
5. Delete decorative header comment (lines 37-39)
6. Verify final line count is 130-145 lines
7. Compile with `cargo check` to verify no errors
8. Run `cargo test` to ensure no broken tests
