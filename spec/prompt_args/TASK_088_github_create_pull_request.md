# TASK 088: Trim github_create_pull_request

**Tool**: `github_create_pull_request`
**Complexity**: 3 (Medium)
**Current size**: 847 lines (5 scenarios)
**Target size**: 320 lines (2-3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_pull_request/prompts.rs`

---

## Context

This tool creates pull requests on GitHub with parameters: `owner`, `repo`, `title`, `head`, `base`, `body`, `draft`, `maintainer_can_modify`. The 847-line prompt includes useful depth but contains significant redundancy across 5 scenarios, with a comprehensive guide that duplicates the focused scenarios.

---

## Current State Analysis

**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_pull_request/prompts.rs`

**Current scenarios** (847 lines total):
1. `prompt_basic()` - 103 lines (lines 44-147) ← KEEP, TRIM TO ~100
2. `prompt_description()` - 165 lines (lines 150-315) ← DELETE (redundant with options)
3. `prompt_options()` - 195 lines (lines 318-513) ← KEEP, TRIM TO ~110
4. `prompt_workflows()` - 195 lines (lines 516-711) ← KEEP, TRIM TO ~90
5. `prompt_comprehensive()` - 136 lines (lines 714-847) ← DELETE

**Routing** (lines 18-27):
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("description") => prompt_description(),     // DELETE
    Some("options") => prompt_options(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),                     // DELETE
}
```

**PromptArgument documentation** (lines 29-36):
```rust
description: Some("Scenario to show (basic, description, options, workflows)".to_string()),
```
Must be updated to: `(basic, options, workflows)`

---

## Trimming Instructions

### KEEP & TRIM: `prompt_basic()` (Target: ~100 lines)

**Current**: 103 lines covering basic PR creation with required parameters

**Keep everything** - it's already concise:
- Simple PR example (15 lines): owner, repo, title, head, base
- PR with description example (15 lines): adds body parameter
- Multiple commits example (12 lines): shows more complex description
- Response format (8 lines): success response structure
- Required parameters (8 lines): owner, repo, title, head, base
- Understanding branches (10 lines): head vs base, changes flow
- Common base branches (8 lines): main, develop, staging, release
- Authentication (6 lines): GITHUB_TOKEN requirement
- Error handling (12 lines): 404, 422, 403 errors
- Best practices (8 lines): clear titles, conventional prefixes

**No deletions needed** - this scenario serves as the essential foundation

---

### DELETE ENTIRELY: `prompt_description()` (165 lines)

**Current**: 165 lines dedicated to PR description formatting

**Rationale for deletion**:
- This is largely redundant with `prompt_options()` which covers draft PRs
- Body parameter is already covered in `prompt_basic()` with an example
- PR templates and markdown formatting are too specialized for medium complexity
- Linking issues is standard GitHub feature, not tool-specific
- Content better suited for GitHub documentation, not tool teaching

**Delete all 165 lines** - all content is either:
- Covered in basic scenario (PR body examples)
- Covered in options scenario (when to use features)
- GitHub documentation territory (not tool-specific)

---

### KEEP & TRIM: `prompt_options()` (Target: ~110 lines)

**Current**: 195 lines covering draft, fork, maintainer modifications, and branch targeting

**Keep sections** (in order of importance):

1. **Draft PRs** (40 lines):
   - Keep: `draft: true` example (5 lines)
   - Keep: Benefits of draft PRs (8 lines): show WIP, get feedback, prevent merge, run CI
   - Keep: When to use draft (8 lines): not ready, seeking feedback, running CI, showing progress
   - Keep: Converting to ready (5 lines): change draft to false
   - Keep: 1 workflow example (9 lines): draft → ready transition

2. **Cross-Repository PRs (Forks)** (30 lines):
   - Keep: Head branch format with fork (5 lines): `username:branch`
   - Keep: Why this matters (5 lines): contributing from fork
   - Keep: Simple fork workflow (15 lines): fork → clone → feature → push → PR
   - Keep: Note about owner/repo being upstream (5 lines)

3. **Maintainer Modifications** (30 lines):
   - Keep: Parameter description (5 lines): maintainer_can_modify true/false
   - Keep: When to disable (8 lines): security, bots, policy
   - Keep: When to enable (8 lines): open source, help wanted, collaborative
   - Keep: Example showing parameter (4 lines)

4. **Brief Best Practices** (10 lines):
   - Use draft for WIP
   - Enable maintainer_can_modify for open source
   - Add detailed description
   - Link related issues
   - Request reviews after creation

**Remove sections** (80 lines total):
- Lines 328-360: Excessive fork examples (keep only 1)
- Lines 366-385: Branch targeting patterns (too detailed, not core to PR creation)
- Lines 387-443: Excessive maintainer modification explanation
- Lines 445-533: Redundant parameter reference and post-creation tools
- Lines 535-545: Redundant best practices

---

### KEEP & TRIM: `prompt_workflows()` (Target: ~90 lines)

**Current**: 195 lines covering 6 complete workflows plus explanations

**Keep sections** (in order of essentiality):

1. **Feature Branch Workflow** (30 lines):
   - Keep: git_push example (8 lines)
   - Keep: github_create_pull_request example (15 lines) with title, head, base, body
   - Keep: Post-creation actions (7 lines): request reviews, add labels
   - Remove: Duplicate workflow variations

2. **Fork Contribution Workflow** (25 lines):
   - Keep: numbered steps (8 lines): fork, clone, feature, push, PR
   - Keep: github_create_pull_request example (12 lines) with head="username:branch"
   - Keep: maintainer_can_modify: true (5 lines)

3. **Pre/Post-Creation Checklist** (20 lines):
   - Keep: Pre-creation checklist (10 lines): branch pushed, tests pass, no conflicts, clean commits
   - Keep: Post-creation steps (10 lines): request reviewers, add labels, monitor CI, address feedback

4. **Brief Workflow Patterns** (15 lines):
   - Keep: GitFlow pattern (7 lines): main, develop, feature/* → develop
   - Keep: GitHub Flow pattern (5 lines): main always deployable
   - Keep: Trunk-based note (3 lines)

**Remove sections** (105 lines total):
- Lines 583-676: ISSUE-TO-PR WORKFLOW (not essential, similar to feature workflow)
- Lines 679-733: DRAFT-TO-READY WORKFLOW (covered in options scenario)
- Lines 736-782: HOTFIX WORKFLOW (too specialized for medium complexity)
- Lines 785-827: RELEASE WORKFLOW (too specialized for medium complexity)
- Lines 830-900: Verbose GitFlow/GitHub Flow/Trunk-based detailed explanations
- Lines 903-935: Redundant best practices and workflow pattern lists

---

### DELETE ENTIRELY: `prompt_comprehensive()` (136 lines)

**Current**: 136 lines of comprehensive reference (lines 714-847)

**Rationale for deletion**:
- Pure duplication of basic, options, and workflows scenarios
- Contains: basic usage (basic scenario), parameters (basic), options (options scenario), workflows (workflows scenario)
- Has redundant quick reference section
- Medium complexity tools should NOT have comprehensive fallback

**Delete all 136 lines** - content is exhaustively covered in the 3 focused scenarios.

---

## Implementation Steps

### Step 1: Trim `prompt_options()`

**Before** (lines 318-513, ~195 lines):
```rust
fn prompt_options() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What options and features are available when creating pull requests?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Pull requests support various options for different workflows and requirements.\n\n\
                 PR OPTIONS:\n\n\
                 1. Draft PR (work in progress):\n\
                 ... [excessive examples and explanations]
                 ... [branch targeting details]
                 ... [redundant maintainer modification content]
                 ... [parameter reference]
```

**After** (target: ~110 lines):
Keep only:
- Draft PR explanation with 1 example, benefits, when to use (25 lines)
- Convert to ready (5 lines)
- Cross-repository format explanation with 1 example (15 lines)
- Maintainer modifications with 2 examples (20 lines)
- Brief best practices (10 lines)
- Parameter reference for these 3 features only (20 lines)
- Remove: all other examples and branch targeting discussions

**Action items**:
1. Delete all fork examples except one (lines ~345-365)
2. Delete branch targeting section entirely (lines ~387-435)
3. Delete verbose maintainer modification explanations (lines ~445-520)
4. Consolidate parameter reference to just draft, maintainer_can_modify (lines ~525-545)
5. Delete redundant best practices

---

### Step 2: Trim `prompt_workflows()`

**Before** (lines 516-711, ~195 lines):
Contains 6 complete workflows + verbose pattern explanations

**After** (target: ~90 lines):
Keep only:
- Feature branch workflow with 1 example (30 lines)
- Fork contribution workflow with 1 example (25 lines)
- Pre/post-creation checklists (20 lines)
- Brief pattern notes (15 lines)

**Action items**:
1. Delete ISSUE-TO-PR WORKFLOW entirely (lines ~583-625)
2. Delete DRAFT-TO-READY WORKFLOW entirely (lines ~628-675)
3. Delete HOTFIX WORKFLOW entirely (lines ~678-715)
4. Delete RELEASE WORKFLOW entirely (lines ~718-760)
5. Delete verbose GITFLOW/GITHUB FLOW/TRUNK-BASED sections (lines ~763-900)
6. Delete redundant best practices list (lines ~903-935)
7. Keep PRE-CREATION and POST-CREATION checklists (brief versions)
8. Keep simple 1-2 sentence notes on GitFlow and GitHub Flow patterns

---

### Step 3: Delete `prompt_description()`

**Before** (lines 150-315, 165 lines):
Entire scenario on PR descriptions

**After**: Function deleted entirely

**Action**: Remove the entire `fn prompt_description() -> Vec<PromptMessage> { ... }` function

---

### Step 4: Delete `prompt_comprehensive()`

**Before** (lines 714-847, 136 lines):
Comprehensive reference covering all aspects

**After**: Function deleted entirely

**Action**: Remove the entire `fn prompt_comprehensive() -> Vec<PromptMessage> { ... }` function

---

### Step 5: Update Routing Logic

**Before** (lines 18-27):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("description") => prompt_description(),
        Some("options") => prompt_options(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}

fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, description, options, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

**After** (new routing with 3 scenarios, default to basic):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("options") => prompt_options(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_basic(),
    }
}

fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, options, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

---

## Line Count Targets and Validation

### Expected Line Counts After Trimming

- **prompt_basic()**: Keep all ~103 lines (already concise) = **103 lines**
- **prompt_options()**: Trim from 195 → **110 lines**
- **prompt_workflows()**: Trim from 195 → **90 lines**
- **Routing + helpers**: ~30 lines (unchanged)
- **File header + comments**: ~20 lines (unchanged)

**Total target**: 103 + 110 + 90 + 30 + 20 = **353 lines**
**Acceptable range**: 280-360 lines ✓

### Validation Checklist

After implementation:
- [ ] `wc -l prompts.rs` → 280-360 lines (target: ~353)
- [ ] `grep "^fn prompt_" prompts.rs` → exactly 3 functions: basic, options, workflows
- [ ] `grep "fn prompt_description" prompts.rs` → 0 results
- [ ] `grep "fn prompt_comprehensive" prompts.rs` → 0 results
- [ ] `grep "description, options, workflows" prompts.rs` → 1 match (in prompt_arguments)
- [ ] `grep "description.*scenarios" prompts.rs` → 0 results (old description removed)
- [ ] Each scenario fits on 2-3 screen pages (readable)
- [ ] All required parameters documented (owner, repo, title, head, base)
- [ ] Draft PRs explained (15+ lines in options)
- [ ] Forks explained (15+ lines in options or workflows)
- [ ] At least one workflow example (feature branch in workflows)
- [ ] File compiles: `cd packages/kodegen-mcp-schema && cargo check`

---

## Expected Output Examples

### prompt_basic() keeps these examples:
```
Simple PR:
github_create_pull_request({
    "owner": "user",
    "repo": "project",
    "title": "Add feature",
    "head": "feature/x",
    "base": "main"
})

With description:
github_create_pull_request({
    ...
    "body": "Fixes #123\n\nDescription..."
})
```

### prompt_options() keeps ONE draft example:
```
github_create_pull_request({
    "owner": "user",
    "repo": "project",
    "title": "WIP: Feature",
    "head": "feature/x",
    "base": "main",
    "draft": true
})
```

### prompt_workflows() keeps ONE feature workflow:
```
1. Push changes
2. Create PR with github_create_pull_request(...)
3. Request reviewers
4. Address feedback
5. Merge when approved
```

---

## Success Criteria

The task IS DONE when ALL of these are true:

- ✓ File is 280-360 lines total (target 353)
- ✓ Exactly THREE scenario functions: prompt_basic(), prompt_options(), prompt_workflows()
- ✓ NO prompt_description() function
- ✓ NO prompt_comprehensive() function
- ✓ Routing match statement has 3 cases (options, workflows) + default to basic
- ✓ Argument description updated to "(basic, options, workflows)"
- ✓ prompt_basic() is ~103 lines (unchanged, already good)
- ✓ prompt_options() is ~110 lines (trimmed from 195)
- ✓ prompt_workflows() is ~90 lines (trimmed from 195)
- ✓ Draft PRs explained in prompt_options() (25+ lines)
- ✓ Fork PRs explained in prompt_options() (15+ lines)
- ✓ At least one feature workflow in prompt_workflows()
- ✓ File compiles without errors: `cargo check`
- ✓ No decorative headers or section dividers deleted (keep simple structure)

---

## Reference Implementation Pattern

This follows the Complexity 3 template from **PRECURSOR_03_git_branch_create.md**:
- **2-3 focused scenarios** (3 in this case)
- **~100-120 lines per scenario**
- **One workflow example per scenario** (not exhaustive)
- **Brief parameter documentation** (not verbose reference)
- **Total 250-400 lines** (this will be 353)
- **Essential patterns only** (no comprehensive fallback)
