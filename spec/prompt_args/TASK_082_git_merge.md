# TASK 082: Trim git_merge

**Tool**: `git_merge`
**Complexity**: 3 (Medium)
**Current size**: 847 lines (5 scenarios)
**Target size**: 310-340 lines (3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/merge/prompts.rs`

---

## Reference

See **PRECURSOR_03_git_branch_create.md** for Complexity 3 template and overall approach.

---

## Current State Analysis

**File: prompts.rs (847 lines)**

Current scenario functions:
1. `prompt_basic()` - 141 lines (lines 42-184) ← KEEP, TRIM to 110 lines
2. `prompt_strategies()` - 217 lines (lines 187-402) ← KEEP, TRIM to 100 lines
3. `prompt_conflicts()` - 207 lines (lines 405-611) ← KEEP, TRIM to 110 lines
4. `prompt_workflows()` - 195 lines (lines 614-811) ← DELETE entirely
5. `prompt_comprehensive()` - 202 lines (lines 814-847, partially) ← DELETE entirely

The git_merge tool parameters:
- `path` (required): Repository path
- `branch` (required): Branch to merge FROM
- `message` (optional): Custom merge commit message
- `no_ff` (optional): Force merge commit even if fast-forward possible
- `squash` (optional): Squash commits into one
- `strategy` (optional): Merge strategy (ours, theirs, recursive)
- `abort` (optional): Abort ongoing merge
- `continue` (optional): Continue after resolving conflicts

**Redundancy issues**:
- prompt_comprehensive contains ~95% duplication from basic, strategies, conflicts, workflows
- prompt_workflows contains full workflow examples that don't add distinct value
- prompt_strategies has 6 strategy types with redundant explanations per type
- prompt_conflicts has 4+ workflow examples showing the same resolution pattern

---

## Trimming Instructions

### KEEP & TRIM: `prompt_basic()` (Target: 110 lines)

**Current**: 141 lines covering basic branch merging with simple checkout/merge patterns

**Keep** (50 lines):
- Merge direction explanation (8 lines):
  - You must be ON the target branch
  - Branch being merged INTO = current branch
  - Branch being merged FROM = specified branch
  - Visual example with checkout + merge

- Basic merge examples (15 lines):
  - Simple merge: `git_merge({"path": "/repo", "branch": "feature"})`
  - With message: custom message parameter
  - Remote branch: fetch then merge
  - Response format (success and conflict cases)

- Workflow example (20 lines):
  - Step 1: Check current branch with git_status
  - Step 2: Switch to target with git_checkout
  - Step 3: Perform merge
  - Step 4: Verify with git_log or git_status

- Merge types (7 lines):
  - Fast-forward: pointer movement, no merge commit, linear history
  - Three-way: diverged branches, creates merge commit, preserves both histories

**Keep** (35 lines):
- Parameters documentation (25 lines):
  - path, branch (required explanations)
  - message, no_ff, squash, strategy, abort, continue (brief one-liners only)

- Common patterns (10 lines):
  - Pattern 1: Simple merge
  - Pattern 2: Merge with message
  - Pattern 3: Ensure working dir clean before merge
  - Pattern 4: Merge remote branch (fetch first)

**Keep** (25 lines):
- Before merging checklist (8 lines):
  - Working directory clean
  - Correct target branch
  - Latest changes pulled
  - Optional backup branch

- After merging checklist (8 lines):
  - Check result (success/conflicts)
  - Resolve conflicts if needed
  - Test merged code
  - Push to remote

- When to use basic merge (9 lines):
  - Simple feature integrations
  - No need to preserve detailed commit history
  - Team doesn't require merge commits

**Remove** (~31 lines):
- Verbose introductions and repetitive sections
- Extended "MERGE TYPES" explanations beyond the essential 2 types
- Redundant parameter examples already shown above
- Multiple workflow examples (keep ONE essential workflow only)

### KEEP & TRIM: `prompt_conflicts()` (Target: 110 lines)

**Current**: 207 lines covering merge conflict detection and resolution

**Keep** (40 lines):
- 5-step conflict resolution workflow (40 lines):
  1. Conflict occurs: merge pauses, files marked with markers, merging state
  2. View conflicts: git_status shows all conflicted files
  3. Resolve manually: fs_read_file to view, fs_edit_block to fix
  4. Mark resolved: git_add the resolved files
  5. Continue or abort: either complete merge or revert

**Keep** (30 lines):
- Conflict markers explanation (20 lines):
  - Visual representation with markers: `<<<<<<< HEAD`, `=======`, `>>>>>>> branch`
  - Marker breakdown: HEAD section = yours, separator, branch section = theirs
  - How to identify conflict sections

- Resolution strategies (10 lines):
  - Keep yours: remove their section
  - Keep theirs: remove your section
  - Keep both: combine both sections
  - Merge manually: create synthesized solution

**Keep** (25 lines):
- Complete workflow (25 lines):
  - Attempt merge (may show conflicts)
  - git_status to see all conflicted files
  - For EACH file: read → resolve → stage
  - After ALL resolved: git_merge with continue: true
  - Verify final state

**Keep** (15 lines):
- Multiple conflicts handling (10 lines):
  - Resolve each file independently
  - Stage each resolved file
  - Only continue after ALL files resolved

- Preventing conflicts tips (5 lines):
  - Merge main into feature frequently
  - Keep branches short-lived
  - Pull before starting work

**Remove** (~97 lines):
- Lines with excessive "RESOLUTION STRATEGIES" section (show ONE example per type)
- Multiple identical "COMPLETE WORKFLOW" examples (keep ONE core workflow)
- Verbose "TIPS" section with 8+ items (keep 5 essential tips)
- Redundant examples of conflict resolution (show pattern once, not 3+ times)
- Extended "PREVENTING CONFLICTS" section (5 tips max)

### KEEP & TRIM: `prompt_strategies()` (Target: 100 lines)

**Current**: 217 lines covering 6 merge strategies with extensive explanations

**Keep** (45 lines):
- Strategy reference guide (45 lines), ONE paragraph per strategy:
  1. Fast-forward (8 lines): When it happens, what it does, result
  2. No-FF (8 lines): When to use, force merge commit, result
  3. Squash (8 lines): When to use, combines commits, requires manual commit
  4. Ours (8 lines): When to use, keeps current branch version
  5. Theirs (8 lines): When to use, accepts their version
  6. Recursive (5 lines): Default, automatic, three-way merge

**Keep** (30 lines):
- Decision tree (30 lines):
  - Need to preserve feature branch history? → Use no_ff: true
  - Feature has messy commit history? → Use squash: true
  - Know which version to keep on conflicts? → Use strategy: "ours" or "theirs"
  - Simple update with no expected conflicts? → Use default (fast-forward if possible)
  - Complex merge with uncertainty? → Test in separate branch first
  - Each decision point shows example code

**Keep** (20 lines):
- 2 workflow examples showing strategy usage (20 lines):
  - Example 1: Feature completion with no_ff
  - Example 2: Cleanup before merge with squash

**Keep** (5 lines):
- Best practices for strategy selection (5 lines):
  - Use no-ff for feature branches
  - Use squash for cleanup
  - Let Git choose (recursive) for most merges

**Remove** (~117 lines):
- Extensive "WHEN IT HAPPENS" explanations per strategy (already in decision tree)
- Multiple examples per strategy (show ONE pattern per strategy)
- Verbose "CHOOSING STRATEGY" section with redundant explanations
- Extended "BEST PRACTICES" with 8+ items (keep 3 essential points)
- Large comparison tables (use compact decision tree instead)

### DELETE ENTIRELY: `prompt_workflows()` (195 lines)

This scenario contains 8 complete workflow examples that are duplicative:
- Feature branch completion (30 lines) - covered in basic + strategies
- Sync fork with upstream (25 lines) - covered in basic
- Update feature branch (20 lines) - covered in basic
- Release merge (20 lines) - covered in strategies
- Hotfix workflow (30 lines) - covered in basic
- Squash merge (25 lines) - covered in strategies
- Conflict resolution workflow (25 lines) - covered in conflicts
- Safe merge with testing (20 lines) - covered in basic/conflicts

**Delete all 195 lines.**

### DELETE ENTIRELY: `prompt_comprehensive()` (202 lines)

This scenario is pure duplication containing:
- BASIC USAGE (already in prompt_basic)
- PARAMETERS (already documented in prompt_basic)
- MERGE TYPES (already in prompt_basic)
- STRATEGIES (already in prompt_strategies)
- CONFLICT RESOLUTION (already in prompt_conflicts)
- COMMON WORKFLOWS (being consolidated into other scenarios)
- BEST PRACTICES (already in each scenario)
- DECISION GUIDE (already in prompt_strategies)
- QUICK REFERENCE (redundant)

**Delete all 202 lines.**

---

## Implementation Steps

### Step 1: Update Routing Logic (lines 19-25)

Replace the current match statement:
```rust
// Before
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("strategies") => prompt_strategies(),
    Some("conflicts") => prompt_conflicts(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}

// After
match args.scenario.as_deref() {
    Some("strategies") => prompt_strategies(),
    Some("conflicts") => prompt_conflicts(),
    _ => prompt_basic(),
}
```

### Step 2: Update prompt_arguments() (lines 27-35)

Change the scenario description from:
```
"Scenario to show (basic, strategies, conflicts, workflows)"
```

To:
```
"Scenario to show (strategies, conflicts, basic)"
```

### Step 3: Rewrite `prompt_basic()` (lines 42-184 → ~110 lines)

Structure the new version:
- 15-line user question and intro
- 20-line merge direction explanation with example
- 25-line parameters documentation
- 20-line common patterns (4 patterns)
- 15-line complete workflow example (4-5 steps)
- 15-line before/after checklist

### Step 4: Rewrite `prompt_conflicts()` (lines 405-611 → ~110 lines)

Structure the new version:
- 15-line user question and intro
- 40-line 5-step resolution workflow with brief examples
- 20-line conflict markers explanation with visual diagram
- 15-line resolution strategies (4 types)
- 15-line complete multi-file workflow
- 5-line preventing conflicts tips

### Step 5: Rewrite `prompt_strategies()` (lines 187-402 → ~100 lines)

Structure the new version:
- 15-line user question and intro
- 45-line strategy reference (6 strategies, one para each)
- 30-line decision tree (5 decision points with code)
- 5-line best practices for strategy selection

### Step 6: Delete `prompt_workflows()` function entirely

Remove all 195 lines of the prompt_workflows() function definition.

### Step 7: Delete `prompt_comprehensive()` function entirely

Remove all 202 lines of the prompt_comprehensive() function definition.

### Step 8: Verify and Validate

After edits:
1. Check file line count: `wc -l prompts.rs` → Should be 310-340 lines
2. Count scenario functions: `grep "^fn prompt_" prompts.rs` → Should be exactly 3
3. Verify no workflows: `grep "fn prompt_workflows" prompts.rs` → Should return nothing
4. Verify no comprehensive: `grep "fn prompt_comprehensive" prompts.rs` → Should return nothing
5. Check routing default: Ensure `_ => prompt_basic()` in match statement
6. Verify all scenarios in routing: Both `strategies` and `conflicts` explicitly handled

---

## Success Criteria

**Line count**: 310-340 lines total

**Scenario functions**: Exactly 3
- `fn prompt_basic()` - 105-115 lines
- `fn prompt_strategies()` - 95-105 lines
- `fn prompt_conflicts()` - 105-115 lines

**Deletions**: 
- `fn prompt_workflows()` - completely removed
- `fn prompt_comprehensive()` - completely removed

**Routing**:
- Some("strategies") → prompt_strategies()
- Some("conflicts") → prompt_conflicts()
- _ (default) → prompt_basic()

**Content quality**:
- Each scenario covers distinct use case
- Conflict resolution is focused on markers, resolution steps, and workflows
- Strategies section has clear decision tree
- Basic section explains merge direction and core patterns
- No redundancy across scenarios
- All essential information present
- Readable within 5-10 minutes
- Clear code examples for each scenario

**No test/documentation artifacts**:
- No test cases added
- No README updates
- No documentation generation changes
