# TASK 009: Trim git_worktree_list Prompts

**Tool**: `git_worktree_list`
**Complexity**: 1 (Trivial)
**Current file**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_list/prompts.rs`
**Current size**: 114 lines
**Target size**: 90-110 lines
**Target scenarios**: 1 (prompt_basic only)

---

## Reference Template

See **[PRECURSOR_01_memory_list_libraries.md](../PRECURSOR_01_memory_list_libraries.md)** for the established Complexity 1 gold standard pattern.

---

## Objective

The `git_worktree_list` tool lists all git worktrees in a repository. It's a trivial operation with minimal parameters (`path` is required, `verbose` is optional boolean). The current 114-line prompt file contains THREE separate scenario functions, which creates redundancy and makes the tool harder to discover quickly.

**Goal**: Consolidate to a single, comprehensive `prompt_basic()` scenario that explains the tool completely in ~100 lines, removing all scenario branching and optional routing.

---

## Current State Analysis

### Scenario Breakdown
The current prompts.rs contains:

1. **prompt_basic()** (~18 lines)
   - User asks: "How do I see all my worktrees?"
   - Basic list output format
   - Shows: paths, branches, main vs linked, lock status

2. **prompt_status()** (~18 lines)
   - User asks: "What information does worktree list show?"
   - Verbose output with detailed fields
   - Explains: Path, Branch, HEAD, Locked, Prunable status

3. **prompt_comprehensive()** (~23 lines)
   - User asks: "How do I use git_worktree_list?"
   - Shows both simple and detailed examples
   - Lists parameters and use cases

4. **Routing logic** (~25 lines)
   - PromptProvider struct
   - generate_prompts() match statement with 3 branches
   - prompt_arguments() defining scenario parameter

**Problem**: Three overlapping scenarios explaining the same simple tool. The "status" and "comprehensive" scenarios are redundant with basic.

---

## Target State

### Single Scenario Structure

Replace the 3-scenario approach with ONE expanded `prompt_basic()` function that follows the PRECURSOR_01 template:

```
File structure (~100 lines total):
├── Header + imports (8 lines)
├── WorktreeListPrompts struct (1 line)
├── PromptProvider impl (12 lines)
│   ├── generate_prompts() → returns prompt_basic()
│   └── prompt_arguments() → returns empty vec[]
├── fn prompt_basic() (79 lines)
│   └── Single PromptMessage with text containing:
│       ├── Tool description (10-15 lines)
│       ├── Basic usage example (10-15 lines)
│       ├── Response structure explanation (10-15 lines)
│       ├── When to use (15-20 lines)
│       ├── Common pattern/workflow (20-30 lines)
│       └── Quick reference (10-15 lines)
```

---

## Exact Changes Required

### Step 1: Simplify PromptProvider Implementation

**BEFORE:**
```rust
impl PromptProvider for WorktreeListPrompts {
    type PromptArgs = GitWorktreeListPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("status") => prompt_status(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario: basic, status".to_string()),
                required: Some(false),
            }
        ]
    }
}
```

**AFTER:**
```rust
impl PromptProvider for WorktreeListPrompts {
    type PromptArgs = GitWorktreeListPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}
```

**Rationale**: With only one scenario, we don't need the match statement or scenario parameter. The underscore prefix (`_args`) indicates the parameter is intentionally unused.

### Step 2: Delete prompt_status() Function

Remove the entire `fn prompt_status()` function (lines ~54-67). This function is redundant - its content is covered in the comprehensive scenario and will be integrated into expanded prompt_basic().

### Step 3: Delete prompt_comprehensive() Function

Remove the entire `fn prompt_comprehensive()` function (lines ~69-98). This function will be replaced by an expanded prompt_basic() that contains all the information users need.

### Step 4: Expand prompt_basic() Function

Replace the current ~18-line `fn prompt_basic()` with an expanded version following PRECURSOR_01 structure. The new function should be ~75-85 lines with the following content structure:

```rust
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I list and manage worktrees in a git repository?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "List git worktrees in your repository.

TOOL DESCRIPTION:
git_worktree_list displays all worktrees attached to a git repository, including the main 
worktree and all linked worktrees. Each worktree can have a different branch checked out,
allowing you to work on multiple branches simultaneously without constant switching.

REQUIRED PARAMETERS:
- path: Repository path (can be absolute or relative)

OPTIONAL PARAMETERS:
- verbose: Boolean (default: false). Shows detailed information when true.

BASIC USAGE:
```json
{\"path\": \"/path/to/repo\"}
```

RESPONSE STRUCTURE:
Shows for each worktree:
- path: Filesystem location of the worktree
- branch: Branch name checked out (or detached state)
- commit: Current HEAD commit (if needed via verbose)
- locked: Boolean - whether worktree is locked
- prunable: Boolean - whether worktree can be cleaned up

RESPONSE EXAMPLE:
```json
{
  \"worktrees\": [
    {
      \"path\": \"/home/user/repo\",
      \"branch\": \"main\",
      \"commit\": \"abc123def456\",
      \"locked\": false,
      \"prunable\": false
    },
    {
      \"path\": \"/home/user/repo-feature\",
      \"branch\": \"feature/new-api\",
      \"commit\": \"xyz789uvw012\",
      \"locked\": false,
      \"prunable\": false
    }
  ]
}
```

WHEN TO USE THIS TOOL:
- Before adding a new worktree: Verify existing worktrees and plan branch assignments
- Managing active work: See which branches are in use across all worktrees
- Before removing a worktree: Check which worktrees exist and their status
- Cleanup and maintenance: Identify prunable worktrees (incomplete operations)
- Debugging branch state: Verify which branch is checked out in each worktree
- Checking lock status: See if any worktrees are locked (protected from removal)

COMMON PATTERN - List and Check Before Creating New Worktree:
```
1. List current worktrees: {\"path\": \"/path/to/repo\"}
   → See all existing worktrees and their branches

2. Check if feature branch exists: 
   → Review which branches are already in use

3. If branch not in use, create new worktree:
   {\"path\": \"/path/to/repo\", \"branch\": \"feature/new-work\", \"track\": true}

4. Verify it was created: {\"path\": \"/path/to/repo\"}
   → Confirm new worktree appears in list
```

QUICK REFERENCE:
- Command: git_worktree_list
- Parameters: path (required), verbose (optional bool)
- Returns: Array of worktree objects with path, branch, commit, locked, prunable
- Related tools: git_worktree_add, git_worktree_remove, git_worktree_lock, git_worktree_unlock, git_worktree_prune"
            ),
        },
    ]
}
```

**Content Guidelines**:
- Keep the User question concise (1 line)
- Assistant response should be comprehensive but scannable
- Use section headers (TOOL DESCRIPTION, PARAMETERS, etc.) for clarity
- Include one realistic JSON example showing tool usage
- Include one realistic JSON response example showing output structure
- List 4-5 "when to use" bullets with concrete scenarios
- Show one complete workflow pattern (as numbered steps or code blocks)
- End with quick reference summary

---

## Validation Checklist

After making changes, verify:

1. **File size**: Run `wc -l prompts.rs` should output: **90-110 lines**

2. **Function count**: Verify only ONE prompt function exists
   ```bash
   grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_list/prompts.rs
   ```
   Should return only: `fn prompt_basic() -> Vec<PromptMessage> {`

3. **Routing simplicity**: Verify generate_prompts() has no match statement
   ```rust
   fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
       prompt_basic()
   }
   ```

4. **No scenario parameter**: Verify prompt_arguments() returns empty vec
   ```rust
   fn prompt_arguments() -> Vec<PromptArgument> {
       vec![]
   }
   ```

5. **Code compiles**: Run from the package directory
   ```bash
   cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
   cargo check
   ```

6. **No clippy warnings**: Verify no lint issues
   ```bash
   cargo clippy -- -D warnings
   ```

---

## Definition of Done

This task is complete when:

✅ The prompts.rs file is exactly 90-110 lines (verified with `wc -l`)

✅ Only ONE `prompt_basic()` function exists (all others deleted)

✅ The `generate_prompts()` method directly calls `prompt_basic()` without match statement

✅ The `prompt_arguments()` method returns an empty vector `vec![]`

✅ The `prompt_basic()` function contains all required sections:
   - Tool description (what the tool does)
   - Required and optional parameters
   - Basic usage example with JSON
   - Response structure explanation
   - Response example with realistic JSON
   - 4-5 "when to use" scenarios
   - One complete workflow pattern
   - Quick reference summary

✅ The code compiles without errors: `cargo check` in the package directory passes

✅ No clippy warnings: `cargo clippy -- -D warnings` passes

✅ The file is readable and the tool usage is understandable in under 2 minutes of reading

---

## File Location Reference

**File to modify**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_list/prompts.rs`

**Related files** (read-only for context):
- [`prompt_args.rs`](../packages/kodegen-mcp-schema/src/git/worktree_list/prompt_args.rs) - Defines GitWorktreeListPromptArgs struct
- [`schema.rs`](../packages/kodegen-mcp-schema/src/git/worktree_list/schema.rs) - Defines tool input/output schema
- [`mod.rs`](../packages/kodegen-mcp-schema/src/git/worktree_list/mod.rs) - Module exports

**Reference task**: [PRECURSOR_01_memory_list_libraries.md](../PRECURSOR_01_memory_list_libraries.md) - Template for Complexity 1 tools

---

## Key Principles

1. **One scenario = simpler tool discovery**: Users don't need to choose between "basic" and "status" views
2. **Comprehensive in one place**: All information about the tool is in prompt_basic()
3. **No redundancy**: Each concept is explained exactly once
4. **Scannable**: Headers and structure make it quick to find answers
5. **Code correctness**: Clean Rust with no compiler or clippy warnings
