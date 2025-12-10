# TASK 084: Trim git_push Prompts

**Tool**: `git_push`
**Complexity**: 3 (Medium)
**Current size**: 928 lines
**Target size**: 280-360 lines (2-3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/push/prompts.rs`

---

## Current State Analysis

### File Structure Overview
The prompts.rs file is currently 928 lines with the following organization:

| Section | Lines | Content |
|---------|-------|---------|
| Module imports & struct | 1-43 | Use statements, PushPrompts struct, impl PromptProvider |
| prompt_basic() | 44-143 | Basic push operations (100 lines) |
| prompt_upstream() | 144-245 | Upstream tracking setup (102 lines) |
| prompt_tags() | 246-383 | Tag pushing scenarios (138 lines) - **DELETE** |
| prompt_force() | 384-533 | Force push warnings (150 lines) - **TRIM to 80-90** |
| prompt_workflows() | 534-745 | Workflow examples (212 lines) - **DELETE** |
| prompt_comprehensive() | 746-928 | Comprehensive guide (183 lines) - **DELETE** |

### Current Match Statement (Lines 20-26)
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("upstream") => prompt_upstream(),
    Some("tags") => prompt_tags(),
    Some("force") => prompt_force(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

Currently handles 5 scenarios + 1 default. Must be reduced to 3 scenarios.

### Current Prompt Arguments (Lines 28-36)
```rust
PromptArgument {
    name: "scenario".to_string(),
    title: None,
    description: Some("Scenario to show (basic, upstream, tags, force, workflows)".to_string()),
    required: Some(false),
}
```

Must be updated to only mention the 3 kept scenarios.

---

## Scenario Breakdown

### KEEP: prompt_basic() (100 lines, Lines 44-143)
**Purpose**: How to push commits to remote repositories

**Content includes**:
- Push current branch with no arguments
- Push to specific remote
- Push specific branch
- Response format example
- Push requirements (local commits, write access, authentication)
- All parameters documented
- Common patterns (push after commit, multiple branches, different remotes)
- Authentication methods (SSH, HTTPS)
- Common errors (non-fast-forward, auth failed, protected branch)
- Post-push state
- Best practices

**Action**: Keep as-is. This function is perfectly sized and comprehensive.

### KEEP: prompt_upstream() (102 lines, Lines 144-245)
**Purpose**: How to set up upstream tracking when pushing new branches

**Content includes**:
- First push with upstream setup
- New branch first push with tracking
- Push to different remote name
- Upstream benefits (simpler commands, pull knows source, status info)
- Typical 4-step workflow (create branch, commit, first push, subsequent pushes)
- Checking upstream status with git_status
- Multiple remotes for fork workflows
- Branch naming conventions
- Upstream configuration stored in .git/config
- Best practices

**Action**: Keep as-is. This function covers a specific, important use case.

### TRIM: prompt_force() (150 lines → 80-90 lines, Lines 384-533)
**Purpose**: When to use force push and critical safety information

**Current content** includes:
- Basic force push examples (lines 384-397) - KEEP
- When force is needed after rebase/amend/reset (lines 399-425) - KEEP (condense slightly)
- Force risks (lines 427-446) - KEEP
- Safe practices (lines 448-468) - KEEP
- Safer alternatives (lines 470-487) - KEEP (concise)
- Force push checklist (lines 489-496) - KEEP (essential)
- Recovery from force push (lines 498-521) - **DELETE** (too detailed, not primary use case)
- Example safe force push (lines 523-536) - **DELETE** (too verbose)
- Remember section (lines 538-543) - **KEEP** (safety-critical)

**Action**: Remove the "RECOVERY FROM FORCE PUSH" subsection entirely (~24 lines). Remove the detailed "EXAMPLE - SAFE FORCE PUSH" section (~14 lines). Keep all safety warnings and the checklist. Result: ~80-90 lines.

**Lines to delete within force function**:
- Lines 498-521: "RECOVERY FROM FORCE PUSH" section (24 lines)
- Lines 523-536: "EXAMPLE - SAFE FORCE PUSH" section (14 lines)

### DELETE: prompt_tags() Function (138 lines, Lines 246-383)
Tags are a less essential use case and covered implicitly in other scenarios. Delete entirely.

### DELETE: prompt_workflows() Function (212 lines, Lines 534-745)
Workflows are not core to push functionality and create redundancy with basic/upstream scenarios. Delete entirely.

### DELETE: prompt_comprehensive() Function (183 lines, Lines 746-928)
This is the "catch-all" default scenario. Must be eliminated and replaced with a simpler default (prompt_basic).

---

## Step-by-Step Implementation

### Step 1: Update Prompt Arguments Description (Lines 28-36)
**What**: Update the description to only mention the 3 kept scenarios

**Current**:
```rust
description: Some("Scenario to show (basic, upstream, tags, force, workflows)".to_string()),
```

**New**:
```rust
description: Some("Scenario to show (basic, upstream, force)".to_string()),
```

**Why**: Users should only see the available scenarios.

---

### Step 2: Update Match Statement (Lines 20-26)
**What**: Remove branches for "tags" and "workflows", change default

**Current**:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("upstream") => prompt_upstream(),
    Some("tags") => prompt_tags(),
    Some("force") => prompt_force(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

**New**:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("upstream") => prompt_upstream(),
    Some("force") => prompt_force(),
    _ => prompt_basic(),
}
```

**Why**: Only keep 3 scenarios. Default to basic when unrecognized scenario is passed.

---

### Step 3: Keep prompt_basic() Unchanged (Lines 44-143)
**Action**: Do not modify. Already optimal at 100 lines.

---

### Step 4: Keep prompt_upstream() Unchanged (Lines 144-245)
**Action**: Do not modify. Already optimal at 102 lines.

---

### Step 5: Trim prompt_force() Function
**What**: Delete the recovery and example sections to reduce from 150 to ~90 lines

**Sections to delete**:
1. Delete lines 498-521 (the entire "RECOVERY FROM FORCE PUSH" subsection with its heading and examples)
2. Delete lines 523-536 (the entire "EXAMPLE - SAFE FORCE PUSH" subsection with detailed workflow)

**Keep**:
- Force push basic examples
- When force is needed (rebase, amend, reset scenarios)
- Force risks (all 4 risk categories)
- Safe practices (only on personal branches)
- Safer alternatives (merge, revert, create new branch)
- Force push checklist (7-item checklist)
- Remember section (final safety reminder)

**Expected result**: ~90 lines after trimming

---

### Step 6: Delete prompt_tags() Function Entirely (Lines 246-383)
**Action**: Remove all 138 lines of this function.

**Delete from**: Line 246 (starting with `/// Pushing tags`)
**Delete to**: Line 383 (closing brace of prompt_tags function)

This includes:
- Doc comment for the function
- Function signature
- Entire vec! with 2 PromptMessage blocks
- All closing braces

---

### Step 7: Delete prompt_workflows() Function Entirely (Lines 534-745)
**Action**: Remove all 212 lines of this function.

**Delete from**: Line 534 (starting with `/// Complete push workflows`)
**Delete to**: Line 745 (closing brace of prompt_workflows function)

This includes:
- Doc comment for the function
- Function signature
- Entire vec! with 2 PromptMessage blocks containing 10 workflow examples
- All closing braces

---

### Step 8: Delete prompt_comprehensive() Function Entirely (Lines 746-928)
**Action**: Remove all 183 lines of this function.

**Delete from**: Line 746 (starting with `/// Comprehensive guide covering all scenarios`)
**Delete to**: Line 928 (end of file, closing brace of prompt_comprehensive function)

This includes:
- Doc comment for the function
- Function signature
- Entire vec! with 2 PromptMessage blocks
- All closing braces

---

## Execution Checklist

Execute in this exact order to avoid line number shifts:

- [ ] Step 1: Update prompt_arguments description (lines 28-36) - 1 line change
- [ ] Step 2: Update match statement (lines 20-26) - 4 lines removed, 1 line changed
- [ ] Step 3: Verify prompt_basic() unchanged
- [ ] Step 4: Verify prompt_upstream() unchanged
- [ ] Step 5: Trim prompt_force() - delete lines 498-521 and 523-536
- [ ] Step 6: Delete prompt_tags() - delete lines 246-383 (138 lines)
- [ ] Step 7: Delete prompt_workflows() - delete remaining lines for this function
- [ ] Step 8: Delete prompt_comprehensive() - delete remaining lines for this function

**IMPORTANT**: After each deletion of a large function, be aware that line numbers shift. Consider doing larger deletions last, or recalculating line numbers after each change.

---

## Expected Final Result

After all changes:
- **Total lines**: ~335 lines (within 280-360 target)
- **Scenarios**: 3 (basic, upstream, force)
- **Functions**: 3 prompt_* functions
- **Overhead**: ~43 lines (imports, struct, trait impl)
- **Content**: ~292 lines (100 + 102 + 90)

Structure:
```
Lines 1-43:   Imports and struct definition
Lines 44-143: prompt_basic() - 100 lines
Lines 144-245: prompt_upstream() - 102 lines
Lines 246-335: prompt_force() - ~90 lines (trimmed)
```

---

## Success Criteria (Measurable)

- ✓ File is exactly between 280-360 lines (verify with `wc -l`)
- ✓ Match statement handles exactly 3 scenarios (basic, upstream, force)
- ✓ Default case in match statement routes to `prompt_basic()`
- ✓ Prompt arguments description lists exactly 3 scenarios
- ✓ No prompt_tags() function exists
- ✓ No prompt_workflows() function exists
- ✓ No prompt_comprehensive() function exists
- ✓ prompt_force() is between 80-100 lines
- ✓ File compiles without errors (`cargo check` succeeds)
- ✓ No clippy warnings (`cargo clippy` passes)
- ✓ All force push safety warnings are preserved
- ✓ Force push checklist remains intact

---

## Definition of Done

Task is complete when:

1. File edited: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/push/prompts.rs`
2. Verified line count: 280-360 lines (run `wc -l prompts.rs`)
3. File compiles: `cd packages/kodegen-mcp-schema && cargo check` passes
4. No warnings: `cargo clippy` shows no errors
5. Confirm exactly 3 functions remain: prompt_basic, prompt_upstream, prompt_force
6. Confirm match statement has exactly 3 Some branches + 1 default
7. Confirm comprehensive scenario is completely gone
8. Confirm force scenario covers 80-100 lines
9. All safety content preserved (checklist, risk warnings, best practices)
