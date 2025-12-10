# TASK 036: Trim git_worktree_add

**Tool**: `git_worktree_add`
**Complexity**: 2 (Simple)
**Current size**: 167 lines (4 scenarios)
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_add/prompts.rs`

---

## Current State Analysis

The file contains 4 scenario functions that need evaluation:

### Current Scenarios (167 lines total)

1. **`prompt_basic()`** (lines 31-57, ~27 lines) ← **KEEP**
   - User question: "What is a Git worktree and how do I create one?"
   - Explains fundamental concept + benefits
   - Shows basic JSON usage example
   - Core feature instruction (required)

2. **`prompt_parallel()`** (lines 59-90, ~32 lines) ← **DELETE**
   - User question: "How do I work on multiple features at once with worktrees?"
   - Shows workflow pattern with multiple worktrees
   - Classification: **USE-CASE SCENARIO** (not a core feature)
   - Does not teach new tool parameters or capabilities
   - Violates Complexity 2 standard (no use-case workflows)

3. **`prompt_branch()`** (lines 92-112, ~21 lines) ← **KEEP**
   - User question: "Can I create a new branch when adding a worktree?"
   - Teaches the `new_branch` parameter (core feature)
   - Shows both scenarios: creating new branch vs using existing
   - Essential tool capability

4. **`prompt_comprehensive()`** (lines 114-167, ~54 lines) ← **DELETE**
   - User question: "How do I use git_worktree_add?"
   - Duplicates all content from basic + branch scenarios
   - Contains no unique information
   - Classification: **COMPREHENSIVE DUPLICATION** (pure redundancy)
   - Violates Complexity 2 standard (no comprehensive scenario)

### Routing Structure (lines 14-23, ~10 lines)

Current match statement:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("parallel") => prompt_parallel(),
    Some("branch") => prompt_branch(),
    _ => prompt_comprehensive(),
}
```

This handles all 4 scenarios and defaults to comprehensive. Must be simplified to 2 scenarios.

---

## Step 1: Read and Analyze ✓ COMPLETE

- Total lines: 167 ✓
- Scenarios counted: 4 ✓
- Use-case scenarios identified: prompt_parallel() ✓
- Comprehensive scenario identified: prompt_comprehensive() ✓

---

## Step 2: Trim to Target (KEEP 2 SCENARIOS)

### KEEP & ENHANCE: `prompt_basic()` (Target: 85-95 lines)

**Current state**: 27 lines of basic concept explanation

**Enhanced structure**:
- Tool description with core capability (5 lines)
- Basic usage with clear JSON example (10 lines)
- What worktrees enable (bullet points, 8 lines)
- Benefits summary (8 lines)
- Common patterns reference (10 lines)
- When to use this scenario (5 lines)

**Expansion guidance**: The current prompt_basic is too brief. Expand with:
- More detailed explanation of worktree vs branch behavior
- 2-3 JSON examples showing different paths/branches
- Explicit mention of the tool's core parameters
- Clear separation of concepts from use-cases

**Keep every word of**:
- "Separate working directory" concept
- "Shares same repository (.git)" 
- Benefits list (work simultaneously, no stash/switch, separate builds, isolation)

### KEEP & ENHANCE: `prompt_branch()` (Target: 85-95 lines)

**Current state**: 21 lines showing new_branch parameter

**Enhanced structure**:
- Parameter explanation (10 lines)
- Creating new branch (20 lines with examples)
- Using existing branch (15 lines with examples)
- When new_branch=true vs false (10 lines)
- Common patterns (15 lines)
- Quick reference (10 lines)

**Expansion guidance**: The current prompt_branch is incomplete. Expand with:
- Detailed explanation of what new_branch does
- Behavior differences: new vs existing branches
- Multiple JSON examples showing both patterns
- Error conditions (branch already exists, etc.)
- Practical workflow demonstration

**Keep every word of**:
- "Creates new branch" explanation
- "Creates worktree at path"
- "Checks out the new branch there"

### DELETE ENTIRELY

**`prompt_parallel()`** (32 lines, lines 59-90):
- This is a WORKFLOW EXAMPLE, not a tool feature
- Demonstrates "parallel development" which is a use-case, not a tool capability
- Adding multiple worktrees requires calling git_worktree_add multiple times
- No unique tool parameters or behaviors
- Users learn this naturally from basic + branch scenarios
- Violates Complexity 2 standard

**`prompt_comprehensive()`** (54 lines, lines 114-167):
- Pure duplication of basic + branch content
- Repeats parameter explanations already in scenarios
- Repeats examples already shown
- Repeats use-case descriptions (workflows, builds, etc.)
- Adds no new information or examples
- Violates Complexity 2 standard

---

## Step 3: Update Routing and Exports

### Update generate_prompts() Match Statement

**BEFORE** (lines 14-23):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("parallel") => prompt_parallel(),
        Some("branch") => prompt_branch(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER** (simplified routing):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("branch") => prompt_branch(),
        _ => prompt_basic(),
    }
}
```

This implementation:
- Routes "branch" scenario explicitly
- Defaults to "basic" for any other input (including None)
- Matches Complexity 2 two-scenario pattern

### Update prompt_arguments() Description

**BEFORE** (lines 25-31):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario: basic, parallel, branch".to_string()),
            required: Some(false),
        }
    ]
}
```

**AFTER** (updated options):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario: basic, branch".to_string()),
            required: Some(false),
        }
    ]
}
```

This shows only the two remaining scenarios.

### Delete Unused Scenario Functions

Remove these function definitions entirely:
- Line 59-90: `fn prompt_parallel() -> Vec<PromptMessage> { ... }`
- Line 114-167: `fn prompt_comprehensive() -> Vec<PromptMessage> { ... }`

---

## Detailed Trimming Actions

### Action 1: Delete prompt_parallel() function

**Location**: Lines 59-90 (entire function)

**Commands**:
1. Delete lines 59-91 (from `fn prompt_parallel()` through closing brace)
2. This removes 32 lines and the blank line after

### Action 2: Delete prompt_comprehensive() function

**Location**: Lines 114-167 (entire function)

**Commands**:
1. Delete lines 114-167 (from `fn prompt_comprehensive()` through closing brace)
2. This removes 54 lines

### Action 3: Update prompt_arguments() description

**Location**: Line 28

**Replace**:
```rust
description: Some("Scenario: basic, parallel, branch".to_string()),
```

**With**:
```rust
description: Some("Scenario: basic, branch".to_string()),
```

### Action 4: Update generate_prompts() routing

**Location**: Lines 14-23

**Replace entire match statement**:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("parallel") => prompt_parallel(),
    Some("branch") => prompt_branch(),
    _ => prompt_comprehensive(),
}
```

**With**:
```rust
match args.scenario.as_deref() {
    Some("branch") => prompt_branch(),
    _ => prompt_basic(),
}
```

---

## Expected Result

After all actions:

**File structure**:
```
Lines 1-8:      File header comment
Lines 9-12:     Imports
Lines 13-24:    WorktreeAddPrompts struct + impl block start
Lines 25-32:    generate_prompts() with updated routing
Lines 33-41:    prompt_arguments() with updated description
Lines 42-43:    impl block end
Lines 44-128:   prompt_basic() - enhanced (~85 lines)
Lines 129-214:  prompt_branch() - enhanced (~85 lines)
```

**Total**: 214 lines (within target 170-220 range)

**Scenarios remaining**: 2 (basic, branch)
**Use-case scenarios**: 0
**Comprehensive scenario**: 0
**Redundancy**: Minimal (no duplicated content)

---

## Success Criteria

- ✓ File is **170-220 lines total** (target: ~214 lines after enhancement)
- ✓ **Exactly 2 scenario functions**: prompt_basic() and prompt_branch()
- ✓ **No use-case scenarios** (prompt_parallel deleted)
- ✓ **No comprehensive scenario** (prompt_comprehensive deleted)
- ✓ **Routing simplified** to handle 2 scenarios with simple default
- ✓ **prompt_arguments() updated** to list only "basic, branch"
- ✓ **No decorative headers** or redundant sections
- ✓ **Each parameter demonstrated** clearly in its relevant scenario
- ✓ **Tool concepts explained once** (not repeated 4 times)
- ✓ **Read through in 2 minutes**: User understands how to use git_worktree_add

---

## Validation Checklist

After completing all actions, verify:

1. **Line count check**:
   ```bash
   wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_add/prompts.rs
   # Expected output: 214 lines (or 170-220 range)
   ```

2. **Scenario count check**:
   ```bash
   grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_add/prompts.rs
   # Expected output: exactly 2 functions
   # fn prompt_basic() -> Vec<PromptMessage> {
   # fn prompt_branch() -> Vec<PromptMessage> {
   ```

3. **Routing completeness**:
   - Match statement has exactly 2 arms: Some("branch") and _
   - Default arm uses prompt_basic()

4. **No deleted content remains**:
   ```bash
   grep -c "parallel\|comprehensive" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_add/prompts.rs
   # Expected output: 0 (no references to deleted scenarios)
   ```

5. **Prompt arguments updated**:
   - description field shows "Scenario: basic, branch"
   - Does not mention "parallel" or others

---

## Complexity 2 Standard Reference

This task follows the **Complexity 2 Standard** established by PRECURSOR_02_fs_read_file.md:

- **1 basic scenario** (~80-90 lines): Core usage, fundamental concepts
- **1 optional advanced scenario** (~80-90 lines): Special parameters or edge cases
- **0 use-case scenarios**: No workflow examples (parallel, integration patterns)
- **0 comprehensive scenarios**: No duplication or redundancy
- **Total**: 150-220 lines of focused, non-redundant content

The git_worktree_add tool's two core features:
1. **Basic**: Understanding and creating worktrees (fundamental concept)
2. **Branch**: Creating new branches during worktree creation (special parameter: new_branch)
