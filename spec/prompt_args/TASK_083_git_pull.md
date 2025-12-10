# TASK 083: Trim git_pull

**Tool**: `git_pull`
**Complexity**: 3 (Medium)
**Current size**: 633 lines (5 scenarios)
**Target size**: 310 lines (3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/pull/prompts.rs`

---

## Reference

See **PRECURSOR_03_git_branch_create.md** for Complexity 3 template and methodology.

---

## Current State Analysis

The `prompts.rs` file contains 633 lines with 5 scenario functions plus a comprehensive fallback. The tool has 5 parameters: `path` (required), `remote` (optional), `branch` (optional), `rebase` (optional), `ff_only` (optional).

**Current scenario breakdown** (633 lines total):

```
1. prompt_basic() - lines 46-156 (111 lines)
   - Covers: path, remote, branch parameters
   - Content: Simple pull patterns, merge types, before/after pulling
   - Status: KEEP but consolidate remotes into this scenario

2. prompt_rebase() - lines 159-315 (157 lines)
   - Covers: rebase parameter and merge vs rebase strategy
   - Content: Detailed comparison, when to use each, warnings, multiple workflows
   - Status: KEEP but TRIM to ~110 lines

3. prompt_remotes() - lines 318-538 (221 lines)
   - Covers: remote parameter variations (origin, upstream, colleague, etc.)
   - Content: Multiple remote patterns, fork sync workflow, collaboration
   - Status: DELETE - consolidate essential 30 lines into prompt_basic()

4. prompt_conflicts() - lines 541-724 (184 lines)
   - Covers: Conflict handling and resolution workflows
   - Content: Conflict markers, resolution strategies, prevention
   - Status: KEEP but TRIM to ~100 lines

5. prompt_comprehensive() - lines 727-1282 (556 lines)
   - Covers: Repetitive overview of all parameters and scenarios
   - Content: Basic usage, parameters, workflows, best practices (all duplicated)
   - Status: DELETE ENTIRELY - pure redundancy with other scenarios
```

**Current routing** (lines 17-23):
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("rebase") => prompt_rebase(),
    Some("remotes") => prompt_remotes(),       // DELETE
    Some("conflicts") => prompt_conflicts(),
    _ => prompt_comprehensive(),               // DELETE
}
```

---

## Trimming Instructions

### KEEP & CONSOLIDATE: `prompt_basic()` (Target: ~120 lines)

**Current**: 111 lines covering path, remote, branch parameters and simple pull patterns

**What to keep:**
- Basic pull patterns (30 lines):
  - Pull from tracking branch: `git_pull({"path": "/project"})`
  - Pull specific remote/branch: `git_pull({"path": "/project", "remote": "origin", "branch": "main"})`
  - Pull develop: `git_pull({"path": "/project", "remote": "origin", "branch": "develop"})`
- How pull works (15 lines):
  - PULL = FETCH + MERGE explanation
  - Downloads new commits from remote
  - Merges into current branch
- Before pulling (15 lines):
  - Commit or stash local changes
  - Check git_status for clean state
  - Uncommitted changes cause conflicts
- After pulling (15 lines):
  - Local branch includes remote changes
  - Working directory is updated
  - May need to resolve conflicts
  - Build/test to ensure everything works
- **Remote examples** (30 lines) ← ADD FROM prompt_remotes():
  - Pull from origin: `git_pull({"path": "/project", "remote": "origin"})`
  - Pull from upstream: `git_pull({"path": "/project", "remote": "upstream", "branch": "main"})`
  - Colleague's branch: `git_pull({"path": "/project", "remote": "origin", "branch": "colleague-feature"})`
  - Fork sync workflow (one example)
- Parameters documentation (15 lines):
  - path (required): Repository path
  - remote (optional): Remote name (default: origin)
  - branch (optional): Branch to pull (default: current upstream)
  - rebase (optional): Use rebase instead of merge (default: false)
  - ff_only (optional): Only allow fast-forward (default: false)

**What to remove from current prompt_basic():**
- None - all content should be kept (but may trim slightly)

**What to add from prompt_remotes():**
- One fork sync workflow (~20 lines):
  - Step 1: Fetch from upstream
  - Step 2: Checkout main branch
  - Step 3: Pull upstream changes
  - Step 4: Push to your fork
- Three remote examples (~15 lines):
  - origin, upstream, colleague patterns
- Remote setup explanation (~5 lines):
  - What origin/upstream/colleague mean

**What to DELETE from prompt_remotes():**
- Collaboration workflow (lines 390-411): too detailed
- Open source contribution section (lines 413-426): too lengthy
- Best practices list (lines 428-432): redundant

### KEEP & TRIM: `prompt_rebase()` (Target: ~110 lines)

**Current**: 157 lines covering the rebase parameter and merge vs rebase strategy

**Keep:**
- Merge vs Rebase comparison (50 lines):
  - MERGE: Creates merge commit, preserves history, safe for shared branches
  - REBASE: Linear history, no merge commits, cleaner log, rewrites history
  - Side-by-side comparison with code examples
- When to use each (30 lines):
  - REBASE ✓: Personal feature branches, before PR, clean linear history, unpushed commits
  - REBASE ✗: Shared branches, pushed commits, shared branches
  - MERGE ✓: Main/master, shared public branches, preserve history
- Rebase warning (15 lines):
  - Rewrites commit history
  - Don't rebase already-pushed commits
  - Never rebase main/master
  - Can cause issues for collaborators
- One workflow example (15 lines):
  - Feature branch workflow: rebase to stay current

**Remove from current prompt_rebase():**
- Multiple workflow examples (lines 259-281): keep only ONE
- Extended benefits list (lines 195-210): too verbose, summarize in comparison
- Multiple "when to use" sections: consolidate into single decision section
- Rebase conflicts subsection (lines 315): too detailed for this scenario

### KEEP & TRIM: `prompt_conflicts()` (Target: ~100 lines)

**Current**: 184 lines covering merge conflict handling and resolution

**Keep:**
- Conflict markers explanation (20 lines):
  - <<<<<<< HEAD shows local code
  - ======= separator
  - >>>>>>> origin/main shows remote code
  - One example showing markers
- Conflict resolution workflow (50 lines):
  - Step 1: Identify conflicts with git_status
  - Step 2: For each conflicted file: read, find markers, resolve, edit
  - Step 3: Test the resolution
  - Step 4: Mark as resolved with git_add
  - Step 5: Complete merge with git_commit
  - Include actual command examples
- Aborting a merge (10 lines):
  - git_merge({"path": "/project", "abort": true})
  - Returns to pre-pull state
- Common conflict scenarios (20 lines):
  - Same line edited
  - File moved/deleted
  - Binary files
  - Multiple files
- Preventing future conflicts (15 lines):
  - Pull frequently
  - Commit before pulling
  - Use feature branches
  - Communicate with team

**Remove from current prompt_conflicts():**
- Multiple verbose resolution strategies (lines 571-610): consolidate to one clear workflow
- Extended "tips for easier resolution" section (lines 668-695): too long, integrate key points
- Redundant "conflict resolution workflow" repetitions
- Excessive "preventing future conflicts" explanations (lines 697-726): summarize to 3-4 points

### DELETE ENTIRELY: `prompt_comprehensive()` (556 lines)

This entire scenario is pure duplication:
- Basic usage: Already in prompt_basic()
- Parameters: Already documented in each scenario
- Merge vs rebase: Already in prompt_rebase()
- Common workflows: Already in each scenario
- Conflict handling: Already in prompt_conflicts()
- Best practices: Scattered across scenarios
- Decision tree: Redundant

**Delete all 556 lines** (lines 727-1282).

### DELETE: `prompt_remotes()` (221 lines)

This scenario covers a single parameter variation (the "remote" parameter). Instead of a dedicated scenario, essential remote examples and one fork sync workflow are consolidated into `prompt_basic()`.

**Delete all 221 lines** (lines 318-538).

### Update Scenario Routing

**Before** (lines 17-23):
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("rebase") => prompt_rebase(),
    Some("remotes") => prompt_remotes(),
    Some("conflicts") => prompt_conflicts(),
    _ => prompt_comprehensive(),
}
```

**After** (simplified to 3 scenarios):
```rust
match args.scenario.as_deref() {
    Some("rebase") => prompt_rebase(),
    Some("conflicts") => prompt_conflicts(),
    _ => prompt_basic(),
}
```

Also update `prompt_arguments()` description (lines 27-33):

**Before:**
```rust
PromptArgument {
    name: "scenario".to_string(),
    title: None,
    description: Some("Scenario to show (basic, rebase, remotes, conflicts)".to_string()),
    required: Some(false),
}
```

**After:**
```rust
PromptArgument {
    name: "scenario".to_string(),
    title: None,
    description: Some("Scenario to show (basic, rebase, conflicts)".to_string()),
    required: Some(false),
}
```

---

## Implementation Checklist

- [ ] **Step 1**: Merge remote examples into prompt_basic()
  - Add fork sync workflow (one example, ~20 lines)
  - Add origin/upstream/colleague examples (~15 lines)
  - Keep all existing basic content

- [ ] **Step 2**: Trim prompt_rebase()
  - Consolidate merge vs rebase comparison (keep one side-by-side example)
  - Keep "when to use each" section (~30 lines total)
  - Keep rebase warning (~15 lines)
  - Remove multiple workflow examples, keep one
  - Target: 100-120 lines

- [ ] **Step 3**: Trim prompt_conflicts()
  - Keep conflict markers explanation (20 lines)
  - Keep ONE clear resolution workflow (50 lines)
  - Keep aborting section (10 lines)
  - Keep common scenarios (20 lines)
  - Keep prevention tips (10 lines)
  - Target: 100-120 lines

- [ ] **Step 4**: Delete prompt_remotes() function
  - Remove the entire function definition (221 lines)
  - Function spans approximately lines 318-538

- [ ] **Step 5**: Delete prompt_comprehensive() function
  - Remove the entire function definition (556 lines)
  - Function spans approximately lines 727-1282
  - This removes all redundant content

- [ ] **Step 6**: Update routing match statement
  - Change match to route "rebase" and "conflicts" to their functions
  - Default case returns prompt_basic()
  - Remove remotes and comprehensive cases

- [ ] **Step 7**: Update prompt_arguments() description
  - Change scenario description from "(basic, rebase, remotes, conflicts)" 
  - To: "(basic, rebase, conflicts)"

---

## Success Criteria

All criteria MUST be met:

- ✓ **Line count**: Total file is 310-340 lines (measured with `wc -l`)
- ✓ **Scenario count**: Exactly 3 scenario functions
  - `prompt_basic()` exists and is ~120 lines
  - `prompt_rebase()` exists and is ~110 lines
  - `prompt_conflicts()` exists and is ~100 lines
- ✓ **Deleted scenarios**: 
  - No `prompt_remotes()` function
  - No `prompt_comprehensive()` function
  - Verify with: `grep "^fn prompt_remotes\|^fn prompt_comprehensive" prompts.rs` returns 0 results
- ✓ **Remote examples**: prompt_basic() includes:
  - Pull from origin example
  - Pull from upstream example
  - Fork sync workflow (one example)
- ✓ **Routing updated**: Match statement has only 3 cases
  - Some("rebase") -> prompt_rebase()
  - Some("conflicts") -> prompt_conflicts()
  - _ -> prompt_basic()
- ✓ **Prompt argument updated**: description says "(basic, rebase, conflicts)"
- ✓ **Readability**: Can understand all three scenarios and workflows in 5 minutes of reading

---

## Validation Checklist

After trimming, execute these verification steps:

1. **Line count**:
   ```bash
   wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/pull/prompts.rs
   # Must be 310-340
   ```

2. **Scenario functions**:
   ```bash
   grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/pull/prompts.rs
   # Output must be exactly 3 lines:
   # fn prompt_basic()
   # fn prompt_rebase()
   # fn prompt_conflicts()
   ```

3. **No deleted scenarios**:
   ```bash
   grep "fn prompt_remotes\|fn prompt_comprehensive" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/pull/prompts.rs
   # Must return 0 results (no output)
   ```

4. **Routing correctness**:
   ```bash
   grep -A 5 "fn generate_prompts" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/pull/prompts.rs | grep "Some"
   # Should show only: "rebase", "conflicts", and default
   # No "remotes" or "comprehensive"
   ```

5. **Compile check** (ensure no syntax errors):
   ```bash
   cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
   cargo check
   ```

6. **Remote examples in basic**: Verify prompt_basic() contains:
   - "origin" mentioned in examples
   - "upstream" mentioned in examples
   - "colleague" or fork-related example

---

## Notes

- This task follows **Complexity 3 template** from PRECURSOR_03_git_branch_create.md
- Target size of 310-340 lines matches the "2-3 focused scenarios" pattern
- Each scenario addresses a specific parameter or use case:
  - `basic`: default pulling with remote/branch variations
  - `rebase`: the rebase parameter and merge vs rebase decision
  - `conflicts`: conflict handling (important practical use case)
- No decorative headers or section dividers needed
- Focus on practical patterns and workflows, not exhaustive documentation
