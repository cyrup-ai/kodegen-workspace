# TASK 037: Trim git_worktree_lock

**Tool**: `git_worktree_lock`
**Complexity**: 2 (Simple)
**Current size**: 103 lines
**Target size**: 170-220 lines (2 expanded scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_lock/prompts.rs`

---

## Current State Analysis

### File Overview
- **Total lines**: 103
- **Current scenario count**: 3 scenarios
- **Location**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_lock/prompts.rs`

### Existing Scenarios

**1. prompt_basic() — Lines 33-43 (~11 lines)**
- User query: "How do I lock a worktree?"
- Status: KEEP and EXPAND to ~100 lines
- Current content: Shows single basic example with path, worktree_path, reason parameters
- Expansion needed: Add multiple examples (removable drive, network mount, deployment), parameter explanations, behavior description

**2. prompt_prevent() — Lines 45-61 (~17 lines)**
- User query: "Why would I lock a worktree?"
- Status: KEEP and EXPAND to ~90 lines
- Current content: Lists 4 locking scenarios (removable drive, network mount, temporary filesystem, critical deployment)
- Expansion needed: Add detailed example for each scenario showing what happens if locked vs. not locked, git behavior explanation, CI/CD and backup scenarios

**3. prompt_comprehensive() — Lines 63-82 (~20 lines)**
- User query: "How do I use git_worktree_lock?"
- Status: DELETE ENTIRELY
- Reason: Comprehensive scenario combines basic and prevent content; trimming requires keeping only focused, single-scenario prompts

### Routing Structure (Lines 13-19)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("prevent") => prompt_prevent(),
        _ => prompt_comprehensive(),  // <-- WILL BE CHANGED TO => prompt_basic()
    }
}
```

### Prompt Arguments (Lines 21-29)
- Current description: "Scenario: basic, prevent"
- Will remain unchanged (both scenarios still listed)
- Default scenario: None (handled by wildcard match arm)

---

## Step-by-Step Implementation

### Step 1: Expand prompt_basic() Function (Lines 33-43 → ~100 lines)

**Current structure**: 11 lines total
- 1 PromptMessage with User role
- 1 PromptMessage with Assistant role (brief example)

**Required expansion**:
1. Keep the User message unchanged: "How do I lock a worktree?"
2. Expand Assistant response to include:
   - Brief intro explaining purpose (2 lines)
   - Example 1: Basic local usage (8 lines)
   - Example 2: Worktree on removable drive (8 lines)
   - Example 3: Network mount worktree (8 lines)
   - Example 4: Deployment scenario (8 lines)
   - Parameter explanation section (10 lines)
   - Behavior explanation (5 lines)

**Detailed code pattern - Assistant response content**:

```rust
"Lock a worktree to prevent automatic removal during git maintenance operations.\\n\\n\

BASIC EXAMPLE:\\n\
```json\\n\
{\\\"path\\\": \\\"/repo\\\", \\\"worktree_path\\\": \\\"/repo-feature\\\", \\\"reason\\\": \\\"In use on server\\\"}\\n\
```\\n\
Locked worktrees are protected from git worktree prune commands.\\n\\n\

REMOVABLE DRIVE:\\n\
```json\\n\
{\\\"path\\\": \\\"/media/usb/repo\\\", \\\"worktree_path\\\": \\\"/media/usb/repo-work\\\", \\\"reason\\\": \\\"USB drive may be disconnected\\\"}\\n\
```\\n\
Lock prevents git from removing stale references when drive is absent.\\n\\n\

NETWORK MOUNT:\\n\
```json\\n\
{\\\"path\\\": \\\"/mnt/nfs/repo\\\", \\\"worktree_path\\\": \\\"/mnt/nfs/repo-share\\\", \\\"reason\\\": \\\"Network storage\\\"}\\n\
```\\n\
Network latency or intermittent disconnections won't trigger auto-pruning.\\n\\n\

DEPLOYMENT:\\n\
```json\\n\
{\\\"path\\\": \\\"/var/repo\\\", \\\"worktree_path\\\": \\\"/var/live-deploy\\\", \\\"reason\\\": \\\"Active production deployment\\\"}\\n\
```\\n\
Production worktrees remain safe from accidental cleanup operations.\\n\\n\

PARAMETERS:\\n\
- path: Main repository directory (required)\\n\
- worktree_path: Absolute path to the worktree to lock (required)\\n\
- reason: Human-readable reason for lock, logged in .git/worktrees/<name>/locked (optional)\\n\\n\

LOCKING MECHANISM:\\n\
Creates .git/worktrees/<worktree-name>/locked file. Git skips this worktree during prune operations and reports it as locked when listing."
```

### Step 2: Expand prompt_prevent() Function (Lines 45-61 → ~90 lines)

**Current structure**: 17 lines total
- 1 PromptMessage with User role
- 1 PromptMessage with Assistant role (4 scenarios listed)

**Required expansion**:
1. Keep User message unchanged: "Why would I lock a worktree?"
2. Expand Assistant response to include:
   - Intro: Why locking matters (3 lines)
   - Scenario 1: Removable storage with impact explanation (8 lines)
   - Scenario 2: Network filesystem with impact explanation (8 lines)
   - Scenario 3: Temporary/ephemeral filesystem with impact (8 lines)
   - Scenario 4: Critical deployment directory (8 lines)
   - Scenario 5: CI/CD pipeline worktree (8 lines)
   - Git behavior explanation (10 lines)
   - Summary of when to lock (7 lines)

**Detailed code pattern - Assistant response content**:

```rust
"Lock worktrees to prevent git from automatically removing them during maintenance operations.\\n\\n\

REMOVABLE STORAGE:\\n\
```json\\n\
{\\\"path\\\": \\\"/media/usb/code/repo\\\", \\\"worktree_path\\\": \\\"/media/usb/code/temp-work\\\", \\\"reason\\\": \\\"USB drive may be disconnected\\\"}\\n\
```\\n\
Without lock: Worktree reference deleted when drive offline for extended time.\\n\
With lock: Reference preserved; you can reconnect and resume work.\\n\\n\

NETWORK MOUNT:\\n\
```json\\n\
{\\\"path\\\": \\\"/mnt/shared-repo\\\", \\\"worktree_path\\\": \\\"/mnt/shared-repo-team\\\", \\\"reason\\\": \\\"Team worktree on NFS share\\\"}\\n\
```\\n\
Without lock: Network latency or brief disconnection triggers removal of worktree index.\\n\
With lock: Worktree survives temporary network issues.\\n\\n\

EPHEMERAL FILESYSTEM:\\n\
```json\\n\
{\\\"path\\\": \\\"/tmp/build-repo\\\", \\\"worktree_path\\\": \\\"/tmp/build-work\\\", \\\"reason\\\": \\\"Temporary build directory\\\"}\\n\
```\\n\
Without lock: System cleanup (tmpwatch, systemd-tmpfiles) may remove directory; git loses worktree metadata.\\n\
With lock: Git won't prune the reference even if directory is missing.\\n\\n\

CRITICAL DEPLOYMENT:\\n\
```json\\n\
{\\\"path\\\": \\\"/var/www/repo\\\", \\\"worktree_path\\\": \\\"/var/www/live-app\\\", \\\"reason\\\": \\\"Production application serving traffic\\\"}\\n\
```\\n\
Without lock: Maintenance script that prunes unused worktrees could remove active deployment.\\n\
With lock: Production worktree is protected from accidental removal.\\n\\n\

CI/CD PIPELINE:\\n\
```json\\n\
{\\\"path\\\": \\\"/opt/ci/repo\\\", \\\"worktree_path\\\": \\\"/opt/ci/build-worker-1\\\", \\\"reason\\\": \\\"Active CI build process\\\"}\\n\
```\\n\
Without lock: Long-running builds could be interrupted by repository cleanup.\\n\
With lock: Worktree remains stable even during scheduled maintenance windows.\\n\\n\

GIT BEHAVIOR:\\n\
- git worktree prune: Skips locked worktrees entirely\\n\
- git worktree list: Shows locked status with reason\\n\
- git worktree unlock <path>: Required to remove lock before pruning\\n\
- Lock file: .git/worktrees/<name>/locked (exists as marker; content is reason or empty)\\n\\n\

WHEN TO LOCK:\\n\
Lock when worktree is on storage that might disconnect, when directory uses ephemeral filesystem, when worktree serves active production or CI process, when multiple users share worktree access, or when maintenance scripts run frequently that could trigger pruning."
```

### Step 3: Update generate_prompts() Match Statement (Line 14-18)

**Current**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("prevent") => prompt_prevent(),
        _ => prompt_comprehensive(),
    }
}
```

**After**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("prevent") => prompt_prevent(),
        _ => prompt_basic(),
    }
}
```

**Change**: Replace default case `_ => prompt_comprehensive()` with `_ => prompt_basic()`

### Step 4: Delete prompt_comprehensive() Function

**Lines to delete**: Entire function from line 63 to line 82

```rust
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I use git_worktree_lock?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Lock a worktree to prevent automatic removal:\\n\\n\
                 BASIC USAGE:\\n\
                 ```json\\n\
                 {\\\"path\\\": \\\"/repo\\\", \\\"worktree_path\\\": \\\"/repo-worktree\\\", \\\"reason\\\": \\\"Lock reason\\\"}\\n\
                 ```\\n\\n\
                 PARAMETERS:\\n\
                 - path (required): Main repository path\\n\
                 - worktree_path (required): Worktree to lock\\n\
                 - reason (optional): Why it's locked\\n\\n\
                 Example:\\n\
                 ```json\\n\
                 {\\\"path\\\": \\\"./repo\\\", \\\"worktree_path\\\": \\\"./repo-deploy\\\", \\\"reason\\\": \\\"Active deployment\\\"}\\n\
                 ```\\n\\n\
                 Locked worktrees are protected from git_worktree_prune."
            ),
        },
    ]
}
```

**Replacement**: Nothing (delete entirely)

### Step 5: Verify Imports and Structure

The following imports remain unchanged and are sufficient:
```rust
use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitWorktreeLockPromptArgs;
```

No new imports needed.

---

## Before and After Structure

### BEFORE (103 lines)
```
Lines 1-11:   Imports and WorktreeLockPrompts struct declaration
Lines 13-19:  generate_prompts() with 3 match arms
Lines 21-29:  prompt_arguments() returning description with "basic, prevent"
Lines 33-43:  prompt_basic() function (~11 lines)
Lines 45-61:  prompt_prevent() function (~17 lines)
Lines 63-82:  prompt_comprehensive() function (~20 lines)
Lines 83-103: Closing braces
```

### AFTER (195-210 lines estimated)
```
Lines 1-11:   Imports and WorktreeLockPrompts struct declaration (unchanged)
Lines 13-19:  generate_prompts() with 2 match arms + default to basic (1 line change)
Lines 21-29:  prompt_arguments() returning description (unchanged)
Lines 33-130: prompt_basic() function (~100 lines, expanded)
Lines 132-215: prompt_prevent() function (~90 lines, expanded)
Lines 217+:   Closing braces (unchanged)
```

---

## Detailed Execution Checklist

Execute these steps in order:

1. **Locate and open file**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_lock/prompts.rs`

2. **Replace prompt_basic() function** (lines 33-43):
   - Delete existing 11-line function
   - Paste expanded version from Step 1 above (~100 lines)
   - Result: Includes basic intro, 4 examples (local, removable drive, network, deployment), parameter section, and behavior description

3. **Replace prompt_prevent() function** (lines 45-61):
   - Delete existing 17-line function
   - Paste expanded version from Step 2 above (~90 lines)
   - Result: Includes 5 detailed scenarios (removable, network, ephemeral, deployment, CI/CD), each with before/after comparison, git behavior explanation, and when-to-lock guidance

4. **Update generate_prompts() match statement** (line 14-18):
   - Locate the default match arm: `_ => prompt_comprehensive(),`
   - Replace with: `_ => prompt_basic(),`
   - Keep the two specific match arms unchanged: `Some("basic")` and `Some("prevent")`

5. **Delete prompt_comprehensive() function**:
   - Locate lines 63-82
   - Delete entire function definition
   - Verify no orphaned code remains

6. **Verify no compilation errors**:
   - File should have exactly 2 function definitions: prompt_basic() and prompt_prevent()
   - WorktreeLockPrompts struct unchanged
   - PromptProvider impl trait unchanged except for match statement default case

---

## Success Criteria

Verify all of the following after completing implementation:

- **Line count**: File is 190-215 lines total (within 170-220 target)
  - Can verify: `wc -l prompts.rs`

- **Scenario count**: Exactly 2 scenarios remain
  - Can verify: `grep -c "fn prompt_" prompts.rs` returns 2

- **No orphaned code**: prompt_comprehensive function is completely removed
  - Can verify: `grep -c "comprehensive" prompts.rs` returns 0

- **Match statement updated**: Default case routes to prompt_basic
  - Can verify: `grep "_ =>" prompts.rs` shows `_ => prompt_basic(),`

- **Basic scenario expanded**: prompt_basic() is ~95-105 lines
  - Can verify: Line count from prompt_basic() declaration to its closing brace

- **Prevent scenario expanded**: prompt_prevent() is ~85-95 lines
  - Can verify: Line count from prompt_prevent() declaration to its closing brace

- **Code compiles**: Run `cargo check` in kodegen-mcp-schema package
  - Command: `cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check`
  - Expected result: No errors or warnings

- **Argument description valid**: Both "basic" and "prevent" still listed in prompt_arguments()
  - Can verify: Description string contains both scenario names

- **JSON examples valid**: All code blocks are properly escaped and valid JSON
  - Can verify: Manually review \\\" escaping and JSON structure in both expanded functions

---

## Key Implementation Notes

1. **String escaping**: All JSON examples inside Rust string literals must use `\\\"` for quotes and `\\n\\` for newlines
2. **Line wrapping**: Assistant response is single multi-line string with `\\n\\` separating logical sections
3. **No whitespace changes**: Maintain existing code style (4-space indentation, existing formatting)
4. **Comment preservation**: Keep existing doc comment `//! Prompt messages for git_worktree_lock tool` unchanged
5. **Imports**: No new imports required; all types already in scope
6. **Default scenario behavior**: With `_ => prompt_basic()`, any unknown scenario defaults to basic usage

---

## Definition of Done

This task is complete when:

1. File has been modified at `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_lock/prompts.rs`

2. Exactly 2 scenario functions exist: `prompt_basic()` and `prompt_prevent()`

3. `prompt_comprehensive()` function is completely removed

4. Match statement in `generate_prompts()` has default case `_ => prompt_basic(),`

5. File line count is 190-215 lines (verified with `wc -l`)

6. `cargo check` passes with zero errors and zero warnings

7. Each scenario function is between 85-105 lines in length

8. All JSON examples are properly escaped and syntactically valid

9. Both scenario names appear in prompt_arguments() description

10. No references to "comprehensive" scenario remain in the file
