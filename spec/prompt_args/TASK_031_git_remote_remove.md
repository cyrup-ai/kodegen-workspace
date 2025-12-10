# TASK 031: Trim git_remote_remove Prompts

**Tool**: `git_remote_remove`
**Complexity**: 2 (Simple)
**Current size**: 624 lines (4 scenarios + comprehensive guide)
**Target size**: 170-220 lines (2 scenarios only)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_remove/prompts.rs`

---

## Current State Analysis

### File Structure
The prompts.rs file is located at:
`/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_remove/prompts.rs`

### Current Content Breakdown
- **Lines 1-33**: Module imports and RemoteRemovePrompts struct definition
- **Lines 14-25**: PromptProvider implementation with match statement routing to 4 scenarios
- **Lines 38-140**: `prompt_basic()` function (~102 lines) - basic remote removal scenario
- **Lines 142-255**: `prompt_cleanup()` function (~114 lines) - cleanup patterns and best practices
- **Lines 257-490**: `prompt_replacement()` function (~234 lines) - remove and replace workflow (DELETE)
- **Lines 492-624**: `prompt_comprehensive()` function (~133 lines) - complete decorative guide (DELETE)

### Scenario Functions Currently Present
1. **prompt_basic()**: Teaches basic single remote removal workflow
2. **prompt_cleanup()**: Teaches when to cleanup remotes and cleanup patterns
3. **prompt_replacement()**: Teaches remove-and-replace workflows (USE-CASE - DELETE)
4. **prompt_comprehensive()**: Large decorative guide with "=====" headers (DELETE)

### Routing Logic (Lines 17-25)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("cleanup") => prompt_cleanup(),
        Some("replacement") => prompt_replacement(),  // DELETE this branch
        _ => prompt_comprehensive(),  // CHANGE to _ => prompt_basic()
    }
}
```

---

## Implementation Instructions

### STEP 1: Update Routing Logic (Lines 17-25)
**Action**: Modify the match statement in `generate_prompts()` to only route to 2 scenarios.

**Current code**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("cleanup") => prompt_cleanup(),
        Some("replacement") => prompt_replacement(),
        _ => prompt_comprehensive(),
    }
}
```

**New code**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("cleanup") => prompt_cleanup(),
        _ => prompt_basic(),
    }
}
```

**Why**: Eliminates routing to deleted functions and defaults to basic scenario for unknown inputs.

---

### STEP 2: Trim prompt_basic() Function
**Action**: Reduce from ~102 lines to ~65-75 lines by removing redundant examples.

**Areas to trim**:
- COMMON USE CASES section: Keep only 2-3 examples instead of all 4
  - Keep: "Remove unused upstream" and "Remove old fork reference"
  - Delete: "Remove test remote" (too similar to fork)
- Consolidate WHAT GETS REMOVED and WHAT DOES NOT CHANGE into more concise bullet lists
- Keep PARAMETERS section
- Keep SAFETY NOTES section (important for tool usage)
- Keep VERIFICATION section at end

**Target structure**:
- User prompt (5 lines)
- Opening statement (2 lines)
- REMOVING A REMOTE section (15 lines)
- RESPONSE format (5 lines)
- WHAT GETS/DOESN'T CHANGE (10 lines consolidated)
- COMMON USE CASES (10 lines - 2 examples)
- PARAMETERS (3 lines)
- SAFETY NOTES (10 lines)
- VERIFICATION (3 lines)
- Total: ~65 lines

---

### STEP 3: Trim prompt_cleanup() Function
**Action**: Reduce from ~114 lines to ~95-110 lines by consolidating patterns and removing decorative sections.

**Areas to trim**:
- Remove "Testing Cleanup" category from "WHEN TO REMOVE REMOTES" (4 lines)
  - Keep: URL Changes, Service Migrated, Unused Remotes (3 categories)
- CLEANUP WORKFLOW: Condense steps to be more concise (10 lines instead of 20)
- COMMON CLEANUP PATTERNS: Keep 2 key patterns instead of 3
  - Keep: "Remove multiple old remotes" and "Clean up after organization rename"
  - Delete: "Remove fork after upstream merge" (covered by cleanup basics)
- BEST PRACTICES: Keep only essential practices (6 lines instead of 10)
- Keep SAFETY CONSIDERATIONS section
- Keep ERROR SCENARIOS section

**Target structure**:
- User prompt (3 lines)
- Opening statement (2 lines)
- CLEANING UP REMOTES section (12 lines)
- WHEN TO REMOVE REMOTES (12 lines - 3 categories)
- CLEANUP WORKFLOW (10 lines)
- COMMON CLEANUP PATTERNS (20 lines - 2 examples)
- BEST PRACTICES (6 lines)
- SAFETY CONSIDERATIONS (10 lines)
- ERROR SCENARIOS (8 lines)
- Total: ~95 lines

---

### STEP 4: Delete prompt_replacement() Function Entirely
**Action**: Remove lines 257-490 (the entire function definition).

**Function signature to delete**:
```rust
/// Replacement workflow - removing and replacing remotes
fn prompt_replacement() -> Vec<PromptMessage> {
    vec![
        // ... all content through closing brace
    ]
}
```

**Why**: This is a use-case scenario beyond the scope of basic tool education. The basic scenario covers all necessary functionality.

---

### STEP 5: Delete prompt_comprehensive() Function Entirely
**Action**: Remove lines 492-624 (the entire function definition).

**Function signature to delete**:
```rust
/// Comprehensive guide covering all aspects of removing remotes
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        // ... all content with decorative "=====" headers through closing brace
    ]
}
```

**Why**: This function uses decorative headers and duplicates content from basic/cleanup scenarios. It exceeds the educational purpose of prompt scenarios.

---

### STEP 6: Remove All Decorative Header Lines
**Action**: While trimming basic() and cleanup(), remove decorative separator lines.

**Pattern to remove**:
- Any lines that are purely "=" characters (e.g., `=============================================================================`)
- Any lines that are purely "-" characters used as visual separators
- These add no educational value and bloat the file

**Where they appear**:
- prompt_basic(): Remove if present (~2-3 lines)
- prompt_cleanup(): Remove if present (~2-3 lines)

---

## Success Criteria

**File Size**: 170-220 lines total (measured in file editor)

**Scenario Count**: Exactly 2 functions remaining
- `prompt_basic()` function present and complete
- `prompt_cleanup()` function present and complete
- No `prompt_replacement()` function
- No `prompt_comprehensive()` function

**Routing Logic**: Match statement has exactly 3 branches
```rust
Some("basic") => prompt_basic(),
Some("cleanup") => prompt_cleanup(),
_ => prompt_basic(),
```

**Code Quality**:
- No references to deleted functions (no compilation errors)
- prompt_args.rs unchanged (still supports "basic", "cleanup" scenarios)
- No decorative "=" or "-" separator lines in prompt content

**Verification Steps**:
1. Open the file and count total lines (should be 170-220)
2. Run `cargo check` in packages/kodegen-mcp-schema to verify compilation
3. Verify no "=====" decorative headers remain in the file
4. Verify prompt_replacement and prompt_comprehensive are completely removed
5. Verify match statement has exactly 3 branches

---

## Related Files

These files remain unchanged:
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_remove/prompt_args.rs` - Prompt argument types (17 lines)
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_remove/mod.rs` - Module exports

The prompt_args.rs file defines:
```rust
pub struct GitRemoteRemovePromptArgs {
    pub scenario: Option<String>,  // Supports "basic", "cleanup"
}
```

This will continue to work with trimmed scenarios.

---

## Code Pattern Reference

### Before (4 scenarios + comprehensive)
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("cleanup") => prompt_cleanup(),
    Some("replacement") => prompt_replacement(),
    _ => prompt_comprehensive(),
}
// Then 4 functions defined below
```

### After (2 scenarios only)
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("cleanup") => prompt_cleanup(),
    _ => prompt_basic(),
}
// Then 2 functions defined below
```

---

## Execution Order

1. Update routing logic in `generate_prompts()` to remove replacement/comprehensive branches
2. Trim `prompt_basic()` by removing redundant examples and consolidating sections
3. Trim `prompt_cleanup()` by removing Testing category and consolidating patterns
4. Delete entire `prompt_replacement()` function
5. Delete entire `prompt_comprehensive()` function
6. Verify file compiles with `cargo check`
7. Count total lines - should be 170-220

---

## Notes for Executor

- This is a Complexity 2 (Simple) task following the pattern from PRECURSOR_02_fs_read_file.md
- The core functionality of the tool is not changing - only the educational prompt scenarios
- Keep safety notes and verification steps as they are critical for tool usage
- The tool itself (git_remote_remove implementation) is NOT being modified, only the prompts
- Focus on reducing examples rather than removing entire sections
- Ensure the remaining 2 scenarios cover: basic operation and cleanup patterns
