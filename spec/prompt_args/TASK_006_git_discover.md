# TASK 006: Trim git_discover Prompts

**Tool**: `git_discover`  
**Complexity**: 1 (Trivial)  
**Current size**: 221 lines (4 scenarios)  
**Target size**: 100-110 lines (1 scenario)  
**Category**: MCP Schema Prompt Refactoring  
**Pattern**: Follows PRECURSOR_01 template for Complexity 1 tools  

---

## Problem Statement

The `git_discover` prompts file contains excessive redundancy with 4 different scenario functions that repeat similar concepts:

- `prompt_basic()` (~55 lines): Basic repository discovery from any path
- `prompt_nested()` (~55 lines): Deep directory traversal (overlaps with basic)
- `prompt_monorepo()` (~60 lines): Monorepo handling (edge case, not core functionality)
- `prompt_comprehensive()` (~50 lines): Verbose overview

The tool's core purpose is simple: **find the Git repository root from any path**. This does not require 4 scenario explanations. The existing `prompt_basic()` already covers all essential information with clear examples and workflow patterns.

### Current File Structure
```
File: /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/discover/prompts.rs
Lines: 221 total
Scenarios: 4 (basic, nested, monorepo, comprehensive)
```

---

## Root Cause

The prompts follow an over-engineered pattern where each scenario adds alternative explanations of the same functionality:

- **prompt_basic** and **prompt_nested**: Both explain upward directory traversal (same concept, different framing)
- **prompt_monorepo** and **comprehensive**: Both provide edge case details that the basic scenario already covers

This violates the principle: **each concept should be explained exactly once**.

---

## Refactoring Objective

Reduce the prompts to a single, focused scenario that clearly explains:

1. **What it does**: Finds Git repository root from any interior path
2. **How to use it**: JSON input with path parameter
3. **Response structure**: success, searched_from, repo_root, message fields
4. **When to use**: 3-4 practical bullet points
5. **Common pattern**: Complete workflow example
6. **Quick reference**: Command syntax and related tools

---

## Implementation Requirements

### STEP 1: Delete Redundant Functions

Remove these function definitions entirely from `prompts.rs`:
- `fn prompt_nested()` (entire function, lines ~52-99)
- `fn prompt_monorepo()` (entire function, lines ~101-170)
- `fn prompt_comprehensive()` (entire function, lines ~172-221)

Keep only `fn prompt_basic()` (lines ~30-51 in original file).

### STEP 2: Simplify Scenario Routing

**Update `generate_prompts()` method** in the `PromptProvider` impl:

```rust
// BEFORE (complex routing)
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("nested") => prompt_nested(),
        Some("monorepo") => prompt_monorepo(),
        _ => prompt_comprehensive(),
    }
}

// AFTER (single scenario)
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

Note: The underscore prefix on `_args` indicates the parameter is intentionally unused (standard Rust pattern for ignoring parameters).

### STEP 3: Deprecate Scenario Arguments

**Update `prompt_arguments()` method** to return an empty vector:

```rust
// BEFORE
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario: basic, nested, monorepo".to_string()),
            required: Some(false),
        }
    ]
}

// AFTER (no arguments)
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![]  // No scenario selection available for trivial tool
}
```

### STEP 4: Optimize prompt_basic() Content

The existing `prompt_basic()` function already contains the essential content structure. Keep it as-is:

- Tool description with JSON example (10 lines)
- Explanation of how it works - stepwise upward traversal (10 lines)
- Use cases with 3 practical examples (10 lines)
- Related tools context (workflow reference) (5 lines)

**Do NOT add** decorative headers, ASCII art, or extended examples.

---

## Code Patterns Reference

### Pattern 1: File Structure After Trim

```rust
//! Prompt messages for git_discover tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitDiscoverPromptArgs;

/// Prompt provider for git_discover tool
pub struct DiscoverPrompts;

impl PromptProvider for DiscoverPrompts {
    type PromptArgs = GitDiscoverPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

fn prompt_basic() -> Vec<PromptMessage> {
    // ~90 lines of well-structured prompt content
    // (keep the existing prompt_basic() body as-is)
}
```

### Pattern 2: Prompt Message Structure

Each prompt consists of exactly 2 messages (User question → Assistant response):

```rust
vec![
    PromptMessage {
        role: PromptMessageRole::User,
        content: PromptMessageContent::text("Single focused question about the tool")
    },
    PromptMessage {
        role: PromptMessageRole::Assistant,
        content: PromptMessageContent::text("Comprehensive answer covering all aspects")
    }
]
```

### Pattern 3: Content Organization

The assistant message body should follow this sequence:
1. **Tool Description** (what it does, parameters, return values) - 10 lines
2. **Basic Usage** (simple JSON example) - 10 lines
3. **How It Works** (step-by-step process) - 10 lines
4. **Use Cases** (3-4 bullet points with examples) - 15 lines
5. **Workflow Pattern** (complete example flow) - 20 lines
6. **Key Points** (summary, related tools) - 10 lines

---

## Related Files (No Changes Required)

### `prompt_args.rs`
Location: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/discover/prompt_args.rs`

The `GitDiscoverPromptArgs` struct with `scenario` field can remain as-is for backward compatibility, though the scenario values (basic, nested, monorepo) will be ignored. Alternatively, you may simplify it to remove the scenario field entirely if API compatibility is not a concern.

### `mod.rs` and `schema.rs`
No changes needed. These files simply export the prompts module and define the tool schema, which are unaffected by prompt trimming.

---

## File Locations and Path References

| File | Location | Action |
|------|----------|--------|
| Prompts (primary edit) | `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/discover/prompts.rs` | Delete 3 functions, simplify routing |
| Reference template | `/Volumes/samsung_t9/kodegen-workspace/task/PRECURSOR_01_memory_list_libraries.md` | Use as structure guide |
| Prompt args (no change) | `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/discover/prompt_args.rs` | Keep as-is |

---

## Definition of Done

The task is complete when:

1. **Line count verified**: `wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/discover/prompts.rs` shows 100-110 lines
2. **Single scenario**: Only `prompt_basic()` function exists in file
3. **Simple routing**: `generate_prompts()` unconditionally returns `prompt_basic()`
4. **No arguments**: `prompt_arguments()` returns `vec![]`
5. **Content quality**: File is readable and understandable in ~60 seconds
6. **No redundancy**: Each concept explained exactly once, no decorative headers
7. **Structural consistency**: Follows PRECURSOR_01 pattern (tool description → usage → response → when to use → workflow → quick reference)
8. **Clean diff**: Git diff shows only deletions of redundant functions and simplification of routing logic

---

## Validation Checklist

After implementing changes:

- [ ] Verify file is 100-110 lines: `wc -l prompts.rs`
- [ ] Verify only one function body: `grep -c "fn prompt_" prompts.rs` should return 1
- [ ] Check imports are still valid: Ensure all `use` statements are still needed
- [ ] Read through file: Can you understand the tool in under 60 seconds?
- [ ] Compare with template: Structure matches PRECURSOR_01 example?
- [ ] No trailing whitespace or formatting issues

---

## Example Diff Summary

Expected changes:

```
-221 lines (original)
+110 lines (trimmed)
-130 lines net reduction

Deletions:
- prompt_nested() function
- prompt_monorepo() function  
- prompt_comprehensive() function
- Complex match statement in generate_prompts()
- scenario argument in prompt_arguments()

Modifications:
+ Simplify generate_prompts() to unconditionally call prompt_basic()
+ Simplify prompt_arguments() to return empty vec
+ Keep prompt_basic() content as-is (already optimal)
```

---

## Notes

- **No backward compatibility concerns**: The tool's behavior doesn't change; prompts are documentation/guidance for users, not functional code
- **Scenario field deprecated**: Users may still pass scenario values, but they will be ignored (no harm)
- **Other git tools unaffected**: This change is isolated to the git_discover schema/prompts package
- **Performance**: No performance impact; prompts are loaded at runtime, not performance-critical
