# TASK 039: Trim git_worktree_unlock

**Tool**: `git_worktree_unlock`
**Complexity**: 2 (Simple)
**Current size**: 104 lines (3 scenarios)
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_unlock/prompts.rs`

---

## Reference

Based on **PRECURSOR_02_fs_read_file.md** Complexity 2 template pattern. This tool is a simple two-parameter Git operation: unlock a worktree (path, worktree_path). Following the template: keep core scenarios, expand them to proper depth, delete the comprehensive duplication.

---

## Context

The `git_worktree_unlock` tool manages Git worktree locks. When a Git worktree is locked, it cannot be automatically pruned. This tool removes that lock, allowing the worktree to be cleaned up via `git worktree prune`. The tool takes two required parameters:
- `path`: Main repository directory
- `worktree_path`: Path to the worktree to unlock

Current scenarios over-represent comprehensive overview at the expense of depth in core use cases.

---

## Current State Analysis

**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_unlock/prompts.rs`
**Total lines**: 104 lines
**Scenarios**: 3 functions

### Scenario Breakdown

1. **`prompt_basic()`** (lines 28-41, 14 lines) ← KEEP, EXPAND to 90-110 lines
   - Current: Minimal user question ("How do I unlock a worktree?")
   - Current response: Single JSON example with 1-line explanation
   - Issue: TOO SHORT - lacks parameter descriptions, common patterns, when to use

2. **`prompt_cleanup()`** (lines 43-61, 19 lines) ← KEEP, EXPAND to 80-100 lines
   - Current: Workflow explanation with cleanup steps
   - Current response: Shows multi-step workflow with bullet points
   - Issue: TOO SHORT - needs more detail on relationship to git prune, safety considerations

3. **`prompt_comprehensive()`** (lines 63-104, 42 lines) ← DELETE (PURE DUPLICATION)
   - This scenario combines basic + cleanup concepts
   - Repeats parameter descriptions already in basic
   - Adds no unique value over keeping both basic and cleanup
   - Pure overhead following the PRECURSOR_02 pattern

### Module Structure (lines 1-26)

```rust
// Lines 1-5: Module header and imports
use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitWorktreeUnlockPromptArgs;

// Lines 7-9: Struct definition
pub struct WorktreeUnlockPrompts;

// Lines 11-26: PromptProvider implementation
impl PromptProvider for WorktreeUnlockPrompts {
    type PromptArgs = GitWorktreeUnlockPromptArgs;
    
    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("cleanup") => prompt_cleanup(),
            _ => prompt_comprehensive(),  // ← CHANGE THIS TO prompt_basic()
        }
    }
    
    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario: basic, cleanup".to_string()),
                required: Some(false),
            }
        ]
    }
}
```

Keep this structure intact. Only change line 17 from `_ => prompt_comprehensive()` to `_ => prompt_basic()`.

---

## Step 1: Read and Analyze

**Already completed**. Current file has 3 scenarios: basic (14 lines), cleanup (19 lines), comprehensive (42 lines).

---

## Step 2: Expand `prompt_basic()` to 90-110 lines

Current content (14 lines):
```rust
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I unlock a worktree?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Unlock a worktree:\n\n\
                 ```json\n\
                 {\"path\": \"/repo\", \"worktree_path\": \"/repo-feature\"}\n\
                 ```\n\n\
                 This allows the worktree to be pruned if missing."
            ),
        },
    ]
}
```

**Expand to include** (target: 100-110 lines):
- Tool description (5 lines): What git_worktree_unlock does
- Parameter explanation (15 lines):
  - `path`: What it means, examples (absolute vs relative)
  - `worktree_path`: Which worktree to unlock, how to find it
- Examples (30 lines):
  - Basic unlock: /repo with /repo-feature
  - Relative paths: ./repo with ./repo-old
  - Cleanup workflow: unlock then prune
- Common patterns (20 lines):
  - When to unlock (after deleting worktree content)
  - Before pruning operations
  - In cleanup scripts
- Quick reference (10 lines): "Always unlock before pruning" (mention ONCE total in file)

**Pattern for expansion**:
```rust
"Unlock a worktree to enable automatic cleanup:\n\n\
 What it does:\n\
 Removes the lock flag from a Git worktree, allowing it to be\n\
 automatically pruned via 'git worktree prune' command.\n\n\
 Parameters:\n\
 - path (string, required): Main repository directory\n\
   Example: /home/user/project or ./project\n\
 - worktree_path (string, required): Path to the worktree to unlock\n\
   Example: /home/user/project-feature or ./project-old\n\n\
 Examples:\n\n\
 UNLOCK WITH ABSOLUTE PATHS:\n\
 ```json\n\
 {\"path\": \"/home/user/repo\", \"worktree_path\": \"/home/user/repo-feature\"}\n\
 ```\n\n\
 UNLOCK WITH RELATIVE PATHS:\n\
 ```json\n\
 {\"path\": \"./repo\", \"worktree_path\": \"./repo-temp\"}\n\
 ```\n\n\
 BEFORE CLEANUP:\n\
 1. Finish work in worktree: /repo-old\n\
 2. Unlock it: use this tool\n\
 3. Later: git worktree prune (safely removes)\n\n\
 When to use:\n\
 - Before running 'git worktree prune'\n\
 - When you no longer need a worktree\n\
 - After manually deleting worktree directory\n\
 - In cleanup scripts or automation\n\n\
 Remember: Always unlock before pruning to avoid leaving stale entries."
```

---

## Step 3: Expand `prompt_cleanup()` to 80-100 lines

Current content (19 lines):
```rust
fn prompt_cleanup() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("Why would I unlock a worktree?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Unlock to enable cleanup:\n\n\
                 ```json\n\
                 {\"path\": \"./repo\", \"worktree_path\": \"./repo-old\"}\n\
                 ```\n\n\
                 After unlocking:\n\
                 - Worktree can be auto-pruned\n\
                 - Safe to remove directory\n\
                 - No longer protected\n\n\
                 TYPICAL WORKFLOW:\n\
                 1. Finish work in worktree\n\
                 2. Unlock it\n\
                 3. Remove directory\n\
                 4. Run prune to clean up"
            ),
        },
    ]
}
```

**Expand to include** (target: 90-100 lines):
- Cleanup context (10 lines): Why worktrees get locked and need unlocking
- Workflow explanation (25 lines):
  - Step-by-step: finish work → unlock → cleanup → prune
  - Safety: what happens if you don't unlock
  - State transitions: locked vs unlocked
- Examples (30 lines):
  - Cleanup abandoned worktree
  - Automated cleanup in CI/CD
  - Batch unlock multiple worktrees
- Safety considerations (15 lines):
  - Lock purpose (prevent accidental pruning)
  - Why unlock is safe
  - Integration with git prune
- Relationship to prune (10 lines):
  - Without unlock: prune skips locked worktrees
  - With unlock: prune can remove everything

**Pattern for expansion**:
```rust
"Unlock a worktree to enable cleanup and pruning:\n\n\
 Why unlock?\n\
 By default, Git locks worktrees to protect them from being accidentally\n\
 deleted during cleanup. Unlocking removes this protection, allowing\n\
 'git worktree prune' to safely remove the worktree if its directory is gone.\n\n\
 Cleanup workflow:\n\
 1. FINISH WORK: Complete and commit changes in the worktree\n\
    $ cd /repo-feature && git commit -am 'Final changes'\n\
 2. UNLOCK: Remove the lock (this tool)\n\
    path: /repo, worktree_path: /repo-feature\n\
 3. VERIFY: Manually delete worktree directory if desired\n\
    $ rm -rf /repo-feature\n\
 4. PRUNE: Clean up stale entries\n\
    $ cd /repo && git worktree prune\n\n\
 Example - Cleanup abandoned worktree:\n\
 ```json\n\
 {\"path\": \"./myproject\", \"worktree_path\": \"./myproject-old-branch\"}\n\
 ```\n\n\
 Example - Cleanup multiple worktrees:\n\
 First unlock each:\n\
 ```json\n\
 {\"path\": \"/repo\", \"worktree_path\": \"/repo-feature1\"}\n\
 {\"path\": \"/repo\", \"worktree_path\": \"/repo-feature2\"}\n\
 ```\n\
 Then: git worktree prune\n\n\
 Safety:\n\
 - Locked worktrees are protected from pruning\n\
 - Unlocking doesn't delete the directory\n\
 - Directory must be manually deleted first\n\
 - Safe to unlock old/unused worktrees\n\n\
 Relationship to git prune:\n\
 - git prune without unlock: Skips locked worktrees\n\
 - git prune after unlock: Can remove if directory is missing\n\
 - Stale entries cleaned up automatically"
```

---

## Step 4: Delete `prompt_comprehensive()`

Remove entire function at lines 63-104 (42 lines).

Current content to DELETE:
```rust
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I use git_worktree_unlock?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Unlock a worktree:\n\n\
                 BASIC USAGE:\n\
                 ```json\n\
                 {\"path\": \"/repo\", \"worktree_path\": \"/repo-worktree\"}\n\
                 ```\n\n\
                 PARAMETERS:\n\
                 - path (required): Main repository path\n\
                 - worktree_path (required): Worktree to unlock\n\n\
                 Example:\n\
                 ```json\n\
                 {\"path\": \"./repo\", \"worktree_path\": \"./repo-temp\"}\n\
                 ```\n\n\
                 Unlocked worktrees can be automatically pruned."
            ),
        },
    ]
}
```

**Reason for deletion**: This scenario is pure duplication combining basic + cleanup. It adds nothing new and follows the PRECURSOR pattern of deleting comprehensive scenarios that don't add unique value.

---

## Step 5: Update Match Statement

**Location**: Line 17 in `impl PromptProvider for WorktreeUnlockPrompts`

**Current**:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("cleanup") => prompt_cleanup(),
    _ => prompt_comprehensive(),
}
```

**After**:
```rust
match args.scenario.as_deref() {
    Some("cleanup") => prompt_cleanup(),
    _ => prompt_basic(),
}
```

**Reasoning**: 
- Both basic and cleanup are now comprehensive enough to stand alone
- Basic scenario is the default (most common use case)
- Cleanup is optional specialized scenario
- This matches PRECURSOR pattern: one default, one specialized

---

## Step 6: Verify No Changes Needed to Imports or Struct

The module header (lines 1-9) stays completely unchanged:
- Imports are correct
- WorktreeUnlockPrompts struct definition is correct
- prompt_arguments() remains the same

---

## Success Criteria

After completion, the file MUST meet ALL these criteria:

- **✓ Line count**: 170-220 lines total (measure with `wc -l`)
  - Current: 104 lines
  - Basic expanded: ~100-110 lines
  - Cleanup expanded: ~90-100 lines
  - Comprehensive deleted: -42 lines
  - Module/impl: 26 lines (unchanged)
  - Expected total: 100-110 + 90-100 + 26 = 216-236 lines (within range)

- **✓ Scenario functions**: Exactly 2 functions
  - `prompt_basic()` function exists (90-110 lines)
  - `prompt_cleanup()` function exists (80-100 lines)
  - `prompt_comprehensive()` function DELETED
  - Verify with: `grep "^fn prompt_" prompts.rs` → exactly 2 matches

- **✓ No comprehensive scenario**
  - Verify: `grep -c "comprehensive" prompts.rs` → 0 results

- **✓ Match statement updated**
  - Line 15-17 should be:
    ```rust
    match args.scenario.as_deref() {
        Some("cleanup") => prompt_cleanup(),
        _ => prompt_basic(),
    }
    ```
  - No reference to `prompt_comprehensive()`

- **✓ Redundancy check**
  - "Always unlock before pruning" appears 1-2 times total (not repeated 5 times)
  - Verify: `grep -i "always unlock" prompts.rs` → 1-2 matches

- **✓ Parameters explained**
  - `path` parameter described with examples
  - `worktree_path` parameter described with examples
  - Both in prompt_basic()

- **✓ Common patterns included**
  - When to unlock documented
  - Relationship to git prune documented
  - Cleanup workflow explained in cleanup scenario
  - Basic usage in basic scenario

- **✓ No decorative headers**
  - No `════`, `─────`, `═══` decorative lines
  - Clean text-based structure

- **✓ Response structure shown**
  - JSON examples present in both scenarios
  - Clear before/after examples

---

## Validation Checklist

After writing the augmented code, run these checks:

```bash
# Line count
wc -l packages/kodegen-mcp-schema/src/git/worktree_unlock/prompts.rs
# Expected: 170-220

# Scenario functions
grep "^fn prompt_" packages/kodegen-mcp-schema/src/git/worktree_unlock/prompts.rs
# Expected: 2 matches (basic, cleanup)

# No comprehensive
grep -i "comprehensive" packages/kodegen-mcp-schema/src/git/worktree_unlock/prompts.rs
# Expected: 0 results

# No decorative headers
grep "═══" packages/kodegen-mcp-schema/src/git/worktree_unlock/prompts.rs
# Expected: 0 results

# Syntax check
cd packages/kodegen-mcp-schema && cargo check
# Expected: No errors
```

---

## Summary

This task standardizes `git_worktree_unlock` to Complexity 2 template:
1. DELETE the comprehensive scenario (pure duplication, 42 lines)
2. EXPAND prompt_basic from 14 to ~100 lines with full parameter documentation and common patterns
3. EXPAND prompt_cleanup from 19 to ~95 lines with detailed workflow and safety information
4. UPDATE match statement to make basic the default
5. Result: 170-220 lines with 2 focused scenarios teaching tool usage without duplication
