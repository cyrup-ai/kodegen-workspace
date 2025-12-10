# TASK 038: Trim git_worktree_remove

**Tool**: `git_worktree_remove`
**Complexity**: 2 (Simple)
**Current size**: 114 lines (3 scenarios)
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_remove/prompts.rs`

---

## Current State Analysis

### File Structure (114 lines total)

```
Lines 1-5:       Module comments and imports
Lines 7-30:      PromptProvider trait implementation (24 lines)
Lines 32-46:     prompt_basic() function (15 lines) ← KEEP & EXPAND
Lines 48-68:     prompt_force() function (21 lines) ← KEEP & EXPAND
Lines 70-114:    prompt_comprehensive() function (45 lines) ← DELETE
```

### Current Scenarios

1. **`prompt_basic()`** (lines 32-46, 15 lines):
   - User: "How do I remove a worktree?"
   - Assistant: Shows basic removal with path and worktree_path parameters
   - Content: 2 messages, explains what the tool does
   - Assessment: Complete but minimal - needs expansion to ~60-70 lines

2. **`prompt_force()`** (lines 48-68, 21 lines):
   - User: "What if the worktree has uncommitted changes?"
   - Assistant: Shows force removal example with warning
   - Content: 2 messages with safety warnings and safer alternative
   - Assessment: Good foundation - expand to ~60-70 lines with more context

3. **`prompt_comprehensive()`** (lines 70-114, 45 lines):
   - User: "How do I use git_worktree_remove?"
   - Assistant: Lists parameters, examples, and warnings
   - Content: 2 messages covering all aspects
   - Assessment: REDUNDANT - completely duplicates basic + force scenarios combined. Must delete.

### Routing Logic (lines 13-18)

```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("force") => prompt_force(),
    _ => prompt_comprehensive(),  // ← Will change default to prompt_basic()
}
```

---

## Trimming Instructions

### STEP 1: Delete prompt_comprehensive() Function

**Action**: Remove the entire `prompt_comprehensive()` function (lines 70-114, 45 lines).

**Reason**: The comprehensive scenario is pure duplication. It combines basic + force into one larger message without adding unique value. The basic and force scenarios together already teach the user everything about the tool.

**Verification**:
- grep "BEFORE REMOVING" prompts.rs should return 0 results after deletion
- grep "^fn prompt_" should return exactly 2 functions (basic, force)

### STEP 2: Expand prompt_basic() Function

**Current**: 15 lines (lines 32-46)

**Target**: 65-75 lines

**Current content**:
```rust
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I remove a worktree?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Remove a worktree:\\n\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/repo\\\", \\\"worktree_path\\\": \\\"/repo-feature\\\"}\\n\\\
                 ```\\n\\n\\\
                 This removes both the directory and Git's administrative files."
            ),
        },
    ]
}
```

**Expansion strategy**:

Replace the Assistant message with more comprehensive guidance:

1. **What it does** (current 2 lines - keep):
   - Removes worktree directory
   - Removes Git administrative files

2. **Parameters** (ADD 8-10 lines):
   - path (required): Main repository root
   - worktree_path (required): Worktree to remove
   - force (optional): Skip safety checks

3. **Basic example** (current 3-4 lines - keep as core):
   - Simple worktree removal

4. **When to use** (ADD 10 lines):
   - After finishing work in worktree
   - When ready to clean up temporary branches
   - Worktree status first with git_worktree_list
   - When changes are committed or merged

5. **Common pattern** (ADD 12-15 lines):
   - Workflow: create → work → commit → remove
   - Safety check with git_worktree_list before removing
   - Example verification steps

6. **Response format** (ADD 8 lines):
   - Successful removal feedback
   - What happens to files/branches

**New basic scenario will**:
- Be 65-75 lines
- Show path+worktree_path parameters clearly
- Explain when to use (standard removal scenario)
- Mention force parameter exists but primary use is without it
- Provide context about safety/workflow

### STEP 3: Expand prompt_force() Function

**Current**: 21 lines (lines 48-68)

**Target**: 65-75 lines

**Current content**:
```rust
fn prompt_force() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("What if the worktree has uncommitted changes?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Force remove worktree with uncommitted changes:\\n\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\", \\\"worktree_path\\\": \\\"./repo-temp\\\", \\\"force\\\": true}\\n\\\
                 ```\\n\\n\\\
                 ⚠️  WARNING:\\n\\\
                 - Uncommitted changes will be lost\\n\\\
                 - No recovery possible\\n\\\
                 - Use only when certain\\n\\n\\\
                 SAFER ALTERNATIVE:\\n\\\
                 1. Commit changes first\\n\\\
                 2. Then remove without force"
            ),
        },
    ]
}
```

**Expansion strategy**:

Replace the Assistant message with comprehensive guidance on force removal:

1. **When to use force** (ADD 10 lines):
   - Worktree locked due to uncommitted changes
   - Temporary worktrees that shouldn't be committed
   - Cleanup of abandoned development branches
   - Emergency removal scenarios

2. **Force parameter explanation** (ADD 8 lines):
   - What force: true does
   - Skips safety checks
   - Deletes worktree regardless of state
   - Cannot be undone

3. **Danger warnings** (current 4 lines - expand to 12 lines):
   - Uncommitted changes lost permanently
   - No recovery possible
   - Stashed changes not affected
   - Merged commits preserved in main branch but worktree copy deleted

4. **Current example** (keep 4 lines):
   - Shows force: true in JSON

5. **Safer alternatives** (ADD 15 lines):
   - Step 1: Check status with git_worktree_list
   - Step 2: Commit or stash changes first
   - Step 3: Remove without force flag
   - When force is actually necessary (locked worktrees)
   - Recovery steps if accidentally forced

6. **Decision tree** (ADD 10 lines):
   - Is work committed elsewhere? → Safe to use force
   - Uncommitted changes? → Commit/stash first, then remove
   - Worktree locked? → Check lock status, consider force only after confirmation

**New force scenario will**:
- Be 65-75 lines
- Address edge case of uncommitted changes
- Provide safety warnings without being preachy
- Show safer alternatives prominently
- Help user decide when force is appropriate
- Teach recovery knowledge

### STEP 4: Update Routing Logic

**Current** (lines 13-18):
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("force") => prompt_force(),
    _ => prompt_comprehensive(),
}
```

**New** (lines 13-17):
```rust
match args.scenario.as_deref() {
    Some("force") => prompt_force(),
    _ => prompt_basic(),
}
```

**Rationale**:
- Change default from comprehensive to basic
- Most users will want basic scenario (standard workflow)
- Force is advanced/optional scenario
- Reduces match arms from 3 to 2

### STEP 5: Update PromptArgument Description (if applicable)

**Current** (lines 22-27):
```rust
PromptArgument {
    name: "scenario".to_string(),
    title: None,
    description: Some("Scenario: basic, force".to_string()),
    required: Some(false),
}
```

**Check**: Verify this line already correctly lists only "basic, force" (not comprehensive). If it says "basic, force, comprehensive", remove "comprehensive" from the description string.

**New** (if changes needed):
```rust
description: Some("Scenario: basic (default), force".to_string()),
```

---

## Before and After Code Comparison

### Routing Change

**BEFORE**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("force") => prompt_force(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("force") => prompt_force(),
        _ => prompt_basic(),
    }
}
```

### Function Deletion

**BEFORE** (45 lines to delete):
```rust
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I use git_worktree_remove?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Remove a linked worktree:\\n\\n\\\
                 BASIC USAGE:\\n\\\
                 ...
                 BEFORE REMOVING:\\n\\\
                 ...
            ),
        },
    ]
}
```

**AFTER**: Function completely removed (0 lines)

---

## Line Count Progression

| Stage | Lines | Change |
|-------|-------|--------|
| Current state | 114 | Baseline |
| After deleting comprehensive | ~69 | -45 lines |
| After expanding basic (add 50 lines) | ~119 | +50 lines |
| After expanding force (add 50 lines) | ~169 | +50 lines |
| **Final target** | **170-220** | **+56-106 lines net** |

The final line count should be 170-220 lines, achieved by:
1. Removing 45 lines (comprehensive scenario)
2. Adding ~100 lines total (expanded basic: +50, expanded force: +50)
3. Net: +55 lines from current 114 = ~169-179 lines

---

## Execution Checklist

Follow these steps IN ORDER:

1. **Read current file**: `cat packages/kodegen-mcp-schema/src/git/worktree_remove/prompts.rs`

2. **Delete comprehensive function**: Remove lines 70-114 entirely (the `fn prompt_comprehensive() -> Vec<PromptMessage>` function and its closing brace)

3. **Expand prompt_basic() to 65-75 lines**:
   - Keep the User message: "How do I remove a worktree?"
   - Expand the Assistant message with:
     * What it does (2 lines)
     * Parameters explanation (10 lines): path, worktree_path, force
     * Basic example (5 lines): the JSON example
     * When to use (10 lines): common scenarios
     * Workflow (12 lines): typical usage pattern
     * Response format (8 lines): what success looks like

4. **Expand prompt_force() to 65-75 lines**:
   - Keep the User message: "What if the worktree has uncommitted changes?"
   - Expand the Assistant message with:
     * When force is needed (10 lines)
     * Force parameter explanation (8 lines)
     * Danger warnings (12 lines): losses and implications
     * Force example (4 lines): JSON with force: true
     * Safer alternatives (15 lines): commit, stash, or check status first
     * Decision tree (10 lines): when to use vs. when to avoid

5. **Update routing logic**: Change the match statement to have "force" specific arm and basic as default

6. **Verify completeness**:
   - `wc -l prompts.rs` should be 170-220
   - `grep "^fn prompt_" | wc -l` should be 2
   - `grep "comprehensive" prompts.rs` should be 0 results
   - File should compile: `cd packages/kodegen-mcp-schema && cargo check`

---

## Success Criteria

The task is DONE when ALL of these are true:

- File size: `wc -l prompts.rs` outputs 170-220 lines ✓
- Scenario count: Exactly 2 functions exist: `prompt_basic()` and `prompt_force()` ✓
- No comprehensive scenario: `grep -i comprehensive prompts.rs` returns 0 results ✓
- No use-case scenarios: Only basic + force scenarios exist ✓
- Routing updated: Match statement has 2 arms (force specific, default to basic) ✓
- Each parameter explained: path, worktree_path, force all described in prompts ✓
- Safety guidance clear: Force scenario explains dangers and alternatives ✓
- No decorative headers: No `═══` or similar visual separators ✓
- Compiles successfully: `cargo check` passes in kodegen-mcp-schema ✓
- Code quality: `cargo clippy` passes with no warnings ✓

---

## Related Files to Check (Not to Edit)

These files may reference or depend on the prompts, but should NOT be edited in this task:

- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_remove/lib.rs` - Tool implementation
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_remove/prompt_args.rs` - Scenario argument definitions
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-tools-git/src/tools/worktree_remove.rs` - Tool server

If those files define scenario enums or routing, they may need updates, but that's a separate task. This task ONLY modifies prompts.rs.

---

## Template Reference

This task follows the **Complexity 2 (Simple)** template established by PRECURSOR_02_fs_read_file.md:
- 1 basic scenario covering standard usage (~70 lines)
- 1 advanced/edge-case scenario (~70 lines)
- No use-case scenarios
- No comprehensive redundant scenario
- Total ~170-220 lines
