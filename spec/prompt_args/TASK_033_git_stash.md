# TASK 033: Trim git_stash_save Prompts

**Tool**: `git_stash_save`  
**Complexity**: 2 (Simple)  
**Current size**: 959 lines  
**Target size**: 170-220 lines  
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/stash/prompts.rs`

---

## Current State Analysis

### File Structure
The prompts.rs file is located at:
```
/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/stash/prompts.rs
```

### Current Scenarios (5 total)
1. **prompt_basic()** (lines ~45-125): Core stashing operations with apply/pop/list examples
2. **prompt_messages()** (lines ~127-265): Stash message best practices (DELETE)
3. **prompt_options()** (lines ~267-450): Stash parameter options (RENAME to prompt_operations, TRIM)
4. **prompt_workflows()** (lines ~452-820): 8 use-case workflows (DELETE)
5. **prompt_comprehensive()** (lines ~822-959): Complete guide (DELETE)

### Routing Logic
The routing is defined in `impl PromptProvider for StashPrompts`:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("messages") => prompt_messages(),
        Some("options") => prompt_options(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

Location: Lines 18-26

The `prompt_arguments()` function describes available scenarios at lines 28-35.

---

## Implementation Instructions

### Step 1: Delete Unused Scenario Functions

Delete these three functions entirely:

**1. DELETE prompt_messages() function**
- Start: Line ~127 (`fn prompt_messages() -> Vec<PromptMessage> {`)
- End: Line ~265 (closing brace)
- Content: Stash message best practices (NOT essential for basic agent usage)
- Reason: Use-case scenario, not core functionality

**2. DELETE prompt_workflows() function**
- Start: Line ~452 (`fn prompt_workflows() -> Vec<PromptMessage> {`)
- End: Line ~820 (closing brace)
- Content: 8 workflow examples (switch branches, pull, hotfix, testing, partial commit, rebase, multiple stashes, risky operations)
- Reason: These are use-case scenarios; agents need basic understanding, not workflow guides

**3. DELETE prompt_comprehensive() function**
- Start: Line ~822 (`fn prompt_comprehensive() -> Vec<PromptMessage> {`)
- End: Line ~959 (file end, closing brace)
- Content: Complete end-to-end guide with all sections combined
- Reason: Comprehensive scenario should not exist; kept scenarios must be focused

### Step 2: Rename and Trim prompt_options()

Rename function from `fn prompt_options()` to `fn prompt_operations()` (line ~267).

This section covers the three stash options: `include_untracked`, `keep_index`, and `all`.

**KEEP these sections within prompt_operations():**
- OPTION 1: include_untracked explanation (~15 lines)
- OPTION 2: keep_index explanation (~15 lines)
- OPTION 3: all explanation (~15 lines)
- COMBINING OPTIONS section (~10 lines)
- DECISION TREE - WHICH OPTIONS TO USE (~20 lines)
- OPTION COMPARISON table (~20 lines)
- BEST PRACTICES section (~8 lines)

**DELETE these sections within prompt_operations():**
- "PRACTICAL EXAMPLES" section (lines with "Example 1:", "Example 2:", "Example 3:") - DELETE entirely (~40 lines)
- Decorative header lines with excessive equal signs (simplify to inline text)

**Resulting prompt_operations() size:** ~100-110 lines

### Step 3: Update Routing Logic

Replace the entire match statement in `generate_prompts()`:

**OLD (lines 20-26):**
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("messages") => prompt_messages(),
    Some("options") => prompt_options(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

**NEW:**
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("operations") => prompt_operations(),
    _ => prompt_basic(),
}
```

The default case now returns `prompt_basic()` instead of `prompt_comprehensive()`.

### Step 4: Update prompt_arguments()

Replace the `prompt_arguments()` function (lines 28-35):

**OLD:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, messages, options, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

**NEW:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, operations)".to_string()),
            required: Some(false),
        }
    ]
}
```

Update only the description string to list "basic, operations" (removed "messages, workflows").

### Step 5: Keep prompt_basic() Unchanged

The `prompt_basic()` function (lines ~45-125) is essential and should remain exactly as-is.

Content includes:
- Basic stashing operations (stash all changes, stash with message, verify)
- What gets stashed (tracked files, staged changes, what doesn't)
- Parameters explanation (path, message, include_untracked, keep_index, all)
- When to stash (branch switch, pull, testing, backup, incomplete work)
- Common patterns with code examples
- Verification and retrieval procedures
- Error handling overview
- Best practices summary

This is the core guidance all agents need for basic stashing.

### Step 6: Simplify Helper Comment Section

Update the decorative comment header above prompt_basic() (line ~38):

**OLD:**
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT STASH SAVE
// ============================================================================
```

**NEW:**
```rust
// Helper functions for prompt scenarios
```

Reduce decorative headers throughout remaining code.

---

## Detailed Code Patterns

### Before: Full routing (4 active scenarios)
```rust
impl PromptProvider for StashPrompts {
    type PromptArgs = GitStashSavePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("messages") => prompt_messages(),
            Some("options") => prompt_options(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, messages, options, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// 4 scenario functions + 1 default = 5 functions total
fn prompt_basic() -> Vec<PromptMessage> { ... }      // ~80 lines
fn prompt_messages() -> Vec<PromptMessage> { ... }   // ~150 lines - DELETE
fn prompt_options() -> Vec<PromptMessage> { ... }    // ~180 lines - TRIM to operations
fn prompt_workflows() -> Vec<PromptMessage> { ... }  // ~370 lines - DELETE
fn prompt_comprehensive() -> Vec<PromptMessage> { ... } // ~140 lines - DELETE
```

### After: Minimal routing (2 active scenarios)
```rust
impl PromptProvider for StashPrompts {
    type PromptArgs = GitStashSavePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("operations") => prompt_operations(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, operations)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// 2 scenario functions = 2 functions total
fn prompt_basic() -> Vec<PromptMessage> { ... }       // ~80 lines - KEEP unchanged
fn prompt_operations() -> Vec<PromptMessage> { ... }  // ~100 lines - RENAMED from options, TRIMMED
```

---

## Success Criteria

All success criteria are mandatory (not optional):

- **Total line count**: 170-220 lines (measured from start of file to final closing brace)
- **Scenario count**: Exactly 2 active scenarios (basic, operations)
- **Removed functions**: prompt_messages(), prompt_workflows(), prompt_comprehensive() completely deleted
- **Routing updated**: match statement handles exactly 2 scenarios, default returns prompt_basic()
- **Arguments updated**: prompt_arguments() description lists "basic, operations" only
- **No comprehensive scenario**: The default case does NOT return a comprehensive scenario
- **prompt_basic() unchanged**: The basic scenario must not be modified in content
- **prompt_operations() size**: Approximately 100-110 lines after trimming practical examples
- **File compiles**: Run `cargo check` in the package directory to verify no syntax errors
- **File is valid Rust**: No broken function signatures or unmatched braces

---

## Definition of Done

This task is complete when:

1. The file `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/stash/prompts.rs` has been modified
2. Total line count is within 170-220 range
3. Only 2 prompt functions exist: prompt_basic() and prompt_operations()
4. The match statement in generate_prompts() handles "basic" and "operations" with basic as default
5. The prompt_arguments() description string reads "Scenario to show (basic, operations)"
6. The file passes `cargo check` without errors
7. All deleted functions have been completely removed with no leftover code

---

## Execution Notes

- Do NOT delete the StashPrompts struct or PromptProvider trait implementation
- Do NOT modify imports or use statements
- Do NOT change the prompt_basic() content
- Do modify only the function names and content within prompt_options (rename to prompt_operations)
- Work from top to bottom: update routing first, then delete unwanted functions, then trim prompt_operations
- After editing, verify the file structure with cargo check to catch any syntax errors immediately
