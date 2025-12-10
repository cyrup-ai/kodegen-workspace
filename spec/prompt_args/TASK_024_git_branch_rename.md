# TASK 024: Trim git_branch_rename

**Tool**: `git_branch_rename`
**Complexity**: 2 (Simple)
**Current size**: 250 lines (4 scenarios)
**Target size**: 180-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/branch_rename/prompts.rs`
**Companion files**:
  - `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/branch_rename/prompt_args.rs`

---

## Reference

This task follows the **PRECURSOR_02_fs_read_file.md** template for Complexity 2 trimming. The pattern:
- Keep 1-2 core scenarios that teach the tool's parameters and usage
- Delete use-case and comprehensive scenarios
- Remove decorative headers and redundancy
- Target 170-220 lines total

---

## Current State Analysis

**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/branch_rename/prompts.rs`
**Total lines**: 250
**Scenario count**: 4

### Current Scenario Breakdown

1. **`prompt_basic()`** (lines 30-61, 32 lines)
   - Core tool usage: renaming a branch
   - Shows: path, old_name, new_name parameters
   - Shows: safe rename behavior (fails if new_name exists)
   - Status: KEEP, expand to ~95 lines

2. **`prompt_current()`** (lines 63-110, 48 lines)
   - Workflow: renaming the current branch
   - Shows: three-step workflow (check status → rename → verify)
   - Shows: how to stay on same branch after rename
   - Shows: renaming other branches works same way
   - Status: KEEP, expand to ~80 lines

3. **`prompt_conventions()`** (lines 112-181, 70 lines)
   - Pure USE CASE content: when and why to rename
   - Examples: fix typos, add prefix, improve description, standardize, add issue number, change category
   - "WHEN TO RENAME" and "WHEN NOT TO RENAME" guidance
   - Status: DELETE (teaches "when to use", not tool features)

4. **`prompt_comprehensive()`** (lines 183-250, 68 lines)
   - Duplicates basic scenario
   - Duplicates current scenario workflow
   - Duplicates conventions scenarios guidance
   - Lists force parameter but no unique content
   - Status: DELETE (pure duplication)

### Routing Logic (lines 13-19)

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("current") => prompt_current(),
        Some("conventions") => prompt_conventions(),
        _ => prompt_comprehensive(),
    }
}
```

After trimming, this becomes:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("current") => prompt_current(),
        _ => prompt_basic(),
    }
}
```

### Prompt Arguments (prompt_args.rs)

```rust
pub struct GitBranchRenamePromptArgs {
    pub scenario: Option<String>,
    // Docs list: "basic", "current", "conventions"
    // Default: comprehensive overview
}
```

Update documentation to list only: "basic" and "current" scenarios.

---

## Implementation Steps

### Step 1: Understand What to Keep

**`prompt_basic()`** teaches tool usage:
- Path, old_name, new_name parameters (required)
- Safe rename behavior (fails if new_name exists)
- Commit preservation
- Local-only operation

**`prompt_current()`** teaches a distinct workflow:
- How to check which branch you're on (git_status tool)
- Three-step workflow: check → rename → verify
- What happens when you rename current branch
- That you stay on the same branch post-rename
- That renaming other branches uses same tool

### Step 2: Identify What to Delete

**`prompt_conventions()`** is USE CASE content:
- NOT about tool features or parameters
- NOT about how git_branch_rename works
- IS about when humans should rename branches
- Duplicates nothing unique from basic scenario
- Belongs in user workflow docs, not tool prompts

**`prompt_comprehensive()`** is pure duplication:
- Merges basic + current + conventions
- No unique teaching not in other scenarios
- Only advantage is all-in-one (but tool can call basic as default)

### Step 3: Expand Kept Scenarios

#### Expand `prompt_basic()` to ~95 lines

Keep existing content:
- User question: "How do I rename a branch?"
- Example: path, old_name, new_name

ADD:
- More detailed response explaining each parameter
- Example of renaming a feature branch
- Explanation of SAFE RENAME: "Fails if new_name already exists, protects against accidental overwrites"
- Add the force parameter example (1 call with force:true)
- Response structure: what the tool returns (success, old_name, new_name, message)
- Common patterns section:
  - Fix typos in branch names
  - More descriptive naming
  - Adopt team conventions
- When to use this tool (early in branch lifecycle, before push, before PR)
- What this tool does NOT do (doesn't update remote, doesn't update pull requests)

Result: Comprehensive overview of basic functionality, ~95 lines.

#### Expand `prompt_current()` to ~80 lines

Keep existing content:
- User question: "Can I rename the branch I'm currently on?"
- Three-step workflow (check, rename, verify)
- What happens (stay on same branch, branch just renamed)
- Renaming other branches works same way

ADD:
- Expand each workflow step with more detail
- Add error handling (what if old_name doesn't exist)
- Add behavior note: working directory unchanged, no checkout needed
- Add interaction with other tools: mention git_status for discovery
- Add gotcha: if branch was pushed to remote, old name remains there
- Example: typical workflow for local-only rename vs shared branch
- When to use this scenario (have a branch you want to improve naming for)

Result: Complete workflow understanding, ~80 lines.

### Step 4: Update Routing and Documentation

1. **Update match statement** (lines 13-19):
   - Remove Some("conventions") match arm
   - Remove Some("conventions") case in match
   - Keep Some("current") and _ defaults
   - Remove prompt_comprehensive() call from default case

2. **Update prompt_arguments()** (lines 21-27):
   ```rust
   fn prompt_arguments() -> Vec<PromptArgument> {
       vec![
           PromptArgument {
               name: "scenario".to_string(),
               title: None,
               description: Some("Scenario: basic, current".to_string()),  // CHANGED from "basic, current, conventions"
               required: Some(false),
           }
       ]
   }
   ```

3. **Delete function definitions**:
   - Delete entire `prompt_conventions()` function (lines 112-181)
   - Delete entire `prompt_comprehensive()` function (lines 183-250)

### Step 5: Line Count Target

**Structure after trimming**:
- Lines 1-8: file header comment
- Lines 10-27: imports and struct definition (~18 lines)
- Lines 29-31: PromptProvider impl start (~3 lines)
- Lines 32-40: generate_prompts function with 2 match arms (~9 lines)
- Lines 41-47: prompt_arguments function updated (~7 lines)
- Lines 48-50: closing brace (~3 lines)
- Lines 51-145: prompt_basic() function (~95 lines)
- Lines 146-225: prompt_current() function (~80 lines)

**Expected total**: ~225 lines (within 170-220 target with some variance)

---

## Detailed Scenario Content After Trimming

### prompt_basic() (~95 lines total)

This scenario covers the core tool functionality. Structure:

**Keep existing** (from current basic scenario):
- User: "How do I rename a branch?"
- Initial JSON example with path, old_name, new_name
- Statement: "This renames the branch while preserving all commits and history"

**Add/Expand**:
- Second example showing renaming a different type of branch
- Parameter explanation section:
  * `path` (required): Repository location
  * `old_name` (required): Current branch name to rename
  * `new_name` (required): New branch name
  * `force` (optional, default false): Overwrite if new_name exists
- SAFE RENAME explanation:
  * Fails if new_name already exists (without force:true)
  * Protects against accidental overwrites
  * Recommended for normal use
- Response structure:
  * What the tool returns (success boolean, old_name, new_name, message)
- Important behaviors:
  * Preserves all commits
  * Preserves branch history
  * Can rename current or other branches
  * Local operation only
  * Does not affect remote automatically
- Example use cases:
  * Fix typo in branch name
  * Make branch name more descriptive
  * Follow team naming conventions
- When to use:
  * Early in branch lifecycle
  * Before pushing to remote (ideally)
  * Before creating pull request
  * To improve code clarity

**Remove**:
- Decorative headers (═══)
- Verbose repetition

**Target**: ~95 lines maintaining clear, actionable guidance.

### prompt_current() (~80 lines total)

This scenario covers the specific workflow of renaming the current checked-out branch. Structure:

**Keep existing** (from current current scenario):
- User: "Can I rename the branch I'm currently on?"
- Opening: "Yes! You can rename the current branch"
- Three-step workflow section
- "WHAT HAPPENS" section explaining you stay on same branch
- "RENAMING OTHER BRANCHES" section showing it works the same way

**Add/Expand**:
- More detail in workflow steps:
  * Step 1 expanded: why check first, what git_status returns
  * Step 2 expanded: the actual rename call details
  * Step 3 expanded: verify the new name is active
- Behavior guarantees:
  * Working directory remains unchanged
  * No checkout needed after rename
  * You stay on the same branch (branch just has new name)
  * All commits preserved
- Error cases:
  * What happens if old_name doesn't exist (fails with error)
  * What happens if new_name already exists (fails unless force:true)
  * How to handle these errors (check branch list first)
- Remote coordination:
  * If branch was pushed, old name remains on remote
  * May need separate operations to clean up remote
  * Important for team coordination
- Common workflow patterns:
  * Local-only: rename and continue working
  * Before push: rename, then push with new name
  * Shared branch: coordinate with team before renaming
- When to use this scenario:
  * Actively on a branch you want to rename
  * Need to understand workflow implications
  * Want to know what happens to your current state

**Remove**:
- Decorative headers
- Excessive repetition of workflow steps

**Target**: ~80 lines focusing on practical workflow understanding.

---

## Success Criteria (Definition of Done)

The task is complete when:

- ✓ **File size**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/branch_rename/prompts.rs` is 170-220 lines
- ✓ **Scenario count**: Exactly 2 scenario functions exist (`prompt_basic` and `prompt_current`)
- ✓ **Deletions complete**: 
  - `prompt_conventions()` function entirely removed (was 70 lines)
  - `prompt_comprehensive()` function entirely removed (was 68 lines)
- ✓ **Routing updated**:
  - `generate_prompts()` match statement has exactly 2 arms: Some("current") and _ (default to basic)
  - No Some("conventions") arm
  - No Some("comprehensive") arm
- ✓ **Prompts argument updated**:
  - Description field shows only "Scenario: basic, current" (removed conventions reference)
- ✓ **No decorative headers**: 
  - Grep "═══" returns 0 results
  - No ASCII art or decorative elements
- ✓ **Content quality**:
  - prompt_basic teaches: path, old_name, new_name, force parameters; safe rename behavior; commit preservation; local-only operation
  - prompt_current teaches: workflow for renaming current branch; three-step process; what happens during rename; remote considerations
- ✓ **No redundancy**: 
  - Each scenario teaches distinct concepts
  - "Preserves commits" mentioned in context, not repeated 5 times
  - Response structure shown in basic only
- ✓ **Validation checklist**:
  ```bash
  # Line count
  wc -l packages/kodegen-mcp-schema/src/git/branch_rename/prompts.rs
  # Should output: 170-220 lines
  
  # Scenario function count
  grep "^fn prompt_" packages/kodegen-mcp-schema/src/git/branch_rename/prompts.rs
  # Should output exactly 2 lines: prompt_basic and prompt_current
  
  # Verify deletions
  grep "prompt_conventions\|prompt_comprehensive" packages/kodegen-mcp-schema/src/git/branch_rename/prompts.rs
  # Should output 0 results
  
  # Verify no decorative headers
  grep "═══" packages/kodegen-mcp-schema/src/git/branch_rename/prompts.rs
  # Should output 0 results
  
  # Verify routing is correct
  grep -A 5 "fn generate_prompts" packages/kodegen-mcp-schema/src/git/branch_rename/prompts.rs
  # Should show match with 2 arms only
  ```

---

## Notes for Executor

1. **Preserve Rust syntax**: Maintain all `vec![]`, `PromptMessage {}`, and quote escaping
2. **Keep imports intact**: Don't modify the imports at the top
3. **Maintain JSON formatting**: Examples in prompts use literal JSON - keep backticks and escaping
4. **Line ending**: File should end with closing brace of impl block, then newline
5. **No new files**: Only modify the one prompts.rs file mentioned
6. **Test after**: After trimming, try `cargo check` in the kodegen-mcp-schema package to verify syntax

---

## Output File Path

When complete, verify the augmented task is saved at:
`/Volumes/samsung_t9/kodegen-workspace/task/TASK_024_git_branch_rename.md`