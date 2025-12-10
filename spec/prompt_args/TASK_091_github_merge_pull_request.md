# TASK 091: Trim github_merge_pull_request

**Tool**: `github_merge_pull_request`
**Complexity**: 3 (Medium)
**Current size**: 852 lines (5 scenarios)
**Target size**: 320 lines (3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/merge_pull_request/prompts.rs`

---

## Reference

See **PRECURSOR_03_git_branch_create.md** for Complexity 3 template.

---

## Current State Analysis

### File Structure

**File**: `prompts.rs` (852 lines total)

**Public interface** (lines 1-36):
- `MergePullRequestPrompts` struct implementing `PromptProvider`
- `generate_prompts()` method with match statement routing (lines 16-24)
- `prompt_arguments()` defining available scenarios (lines 26-35)

### Current Scenario Functions (5 total)

1. **`prompt_basic()`** (lines 43-139, 97 lines)
   - Core: Simple merge, verify before merge, merge with verification
   - Content: Usage patterns, required parameters, response fields, common patterns, before-merge checklist
   - Issue: Includes redundant error handling and best practices sections at lines 118-124

2. **`prompt_strategies()`** (lines 142-271, 130 lines)
   - Core: Merge commit, squash merge, rebase merge strategies
   - Content: When to use each, strategy comparison, repository settings
   - Status: Focused and well-structured, needs minor trimming

3. **`prompt_messages()`** (lines 274-411, 138 lines)
   - Core: Commit message customization with custom title and message parameters
   - Content: Conventional commits, multi-line messages, templates, use cases
   - Action: DELETE ENTIRELY - this is a secondary concern for medium complexity

4. **`prompt_workflows()`** (lines 414-622, 209 lines)
   - Core: Complete merge workflows with error handling
   - Content: Basic safe merge, production merge, release merge, hotfix merge, decision tree
   - Issue: 4 complete workflows is excessive; keep basic + production, delete release + hotfix

5. **`prompt_comprehensive()`** (lines 625-851, 227 lines)
   - Pure duplication covering: basic usage, parameters, merge strategies, messages, workflows, error handling, best practices
   - Status: DELETE ENTIRELY - duplicates focused scenarios

### Routing Logic (lines 13-24)

```rust
impl PromptProvider for MergePullRequestPrompts {
    type PromptArgs = MergePullRequestPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("strategies") => prompt_strategies(),
            Some("messages") => prompt_messages(),         // DELETE
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),                   // DELETE
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, strategies, messages, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}
```

**Must be updated** to:
```rust
    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("strategies") => prompt_strategies(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_basic(),                           // NEW DEFAULT
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, strategies, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
```

---

## Trimming Instructions

### KEEP & TRIM: `prompt_basic()` (Target: 100-120 lines)

**Current**: 97 lines covering simple merge patterns with verification

**KEEP** (lines 54-117):
- Three core merge patterns (30 lines):
  ```
  1. Simple merge:
     github_merge_pr({"owner": "user", "repo": "project", "pull_number": 456})

  2. Check before merge:
     github_get_pr({...}) // Verify
     github_merge_pr({...})

  3. Merge with verification:
     github_get_pr({...}) // Check mergeable, CI status
     github_merge_pr({...})
  ```
- Required parameters documentation (10 lines): owner, repo, pull_number
- Response fields (8 lines): sha, merged, message
- Common patterns (15 lines): Quick merge, verify then merge, merge multiple PRs
- Before merging checklist (15 lines): state, conflicts, CI, reviews, branch status

**DELETE** (lines 118-139):
- Error handling section (lines 118-124): Too detailed for basic scenario; belongs in workflows
- Authentication section (lines 125-128): Too advanced for basic; belongs in comprehensive reference
- Best practices (lines 129-139): Redundant with other scenarios
- Decorative comment header (line 39)

**RESULT**: Retain the core usage examples and verification patterns. Trim extended explanations that belong in specialized scenarios.

### KEEP: `prompt_strategies()` (Target: 100-120 lines, MINOR TRIM)

**Current**: 130 lines covering merge/squash/rebase decisions

**Status**: This scenario is already well-focused. KEEP ALMOST ENTIRELY with minor cuts.

**KEEP** (most of lines 152-267):
- Merge commit strategy with example (20 lines)
- Squash merge strategy with example (20 lines)
- Rebase merge strategy with example (20 lines)
- Choosing the right strategy by use case (30 lines)
- Strategy comparison section (20 lines)
- Repository settings: enable merge methods (10 lines)

**DELETE** (minimal):
- Decorative header "MERGE STRATEGIES:" at line 154 (redundant given function context)
- Duplicate "STRATEGY COMPARISON:" header - integrate examples directly
- Extended team conventions section (lines 251-256): too prescriptive for medium complexity
- Delete "DEFAULT BEHAVIOR" section (lines 263-267): belongs in comprehensive reference

**RESULT**: Keep ~110 lines of focused strategy guidance. Trim decorative sections and overly detailed edge cases.

### KEEP & TRIM: `prompt_workflows()` (Target: 100-120 lines)

**Current**: 209 lines with 4 complete workflows

**KEEP**:
- Workflow 1: BASIC SAFE MERGE (lines 427-452, ~26 lines)
  - Step 1: Check PR status
  - Step 2: Merge PR
  - Step 3: Delete branch (optional)

- Workflow 2: PRODUCTION MERGE WITH VERIFICATION (lines 453-493, ~41 lines)
  - Step 1: Get PR details
  - Step 2: Check reviews
  - Step 3: Check CI status
  - Step 4: Merge with custom message
  - Step 5: Clean up branch

- Workflow decision tree (lines 552-571, ~20 lines):
  - Feature PR workflow
  - Release PR workflow
  - Hotfix PR workflow

- Error handling in workflows (lines 585-602, ~18 lines):
  - Not mergeable handling
  - CI checks failing handling
  - Merge fails handling

- Best practices (lines 603-618, ~16 lines):
  - Status checking, CI verification, merge strategy selection, custom messages, branch cleanup, team communication

**DELETE**:
- Workflow 3: RELEASE MERGE WORKFLOW (lines 494-527, 34 lines) - Too specialized for medium tool
- Workflow 4: HOTFIX MERGE WORKFLOW (lines 528-551, 24 lines) - Too specialized for medium tool
- BRANCH CLEANUP section (lines 572-584, 13 lines) - Integrated into workflows already
- Decorative section headers (lines 425-426, 552): Remove "WORKFLOW DECISION TREE:" and similar

**RESULT**: Keep essential basic and production workflows with decision tree and error handling. Remove redundant specialized workflows that duplicate core patterns.

### DELETE ENTIRELY: `prompt_messages()` (lines 274-411, 138 lines)

**Full deletion justified**:
- Commit message customization is secondary to core merge functionality
- Already briefly mentioned in prompt_basic() and workflows
- 138 lines of message formatting/templates is excessive for medium complexity
- No separate scenario needed; merging integrates messages into workflows

**Action**: Delete the entire function including:
- All code from `fn prompt_messages()` declaration through closing brace
- No integration of this content; pure removal

### DELETE ENTIRELY: `prompt_comprehensive()` (lines 625-851, 227 lines)

**Full deletion justified**:
- Complete duplication of focused scenarios
- Contains sections: basic usage, parameters, strategies, messages, workflows, error handling, authentication, best practices
- Belongs only in tool documentation, not in scenario prompts
- Redundant with targeted scenario design

**Action**: Delete the entire function including:
- All code from `fn prompt_comprehensive()` declaration through closing brace
- No content needs preservation

---

## Updated Routing Structure

### Step 1: Update `prompt_arguments()` (line 31)

**Before**:
```rust
description: Some("Scenario to show (basic, strategies, messages, workflows)".to_string()),
```

**After**:
```rust
description: Some("Scenario to show (basic, strategies, workflows)".to_string()),
```

### Step 2: Update `generate_prompts()` match arm (lines 17-23)

**Before**:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("strategies") => prompt_strategies(),
    Some("messages") => prompt_messages(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

**After**:
```rust
match args.scenario.as_deref() {
    Some("strategies") => prompt_strategies(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_basic(),
}
```

**Rationale**: Basic scenario becomes the default. Users most often want simple merge guidance; advanced users can request "strategies" or "workflows".

---

## Line-by-Line Edits

### Edit 1: Trim prompt_basic() error handling

**Location**: Lines 118-139 (22 lines total)

**Delete**:
```rust
ERROR HANDLING:\n\
If merge fails, check:\n\
- PR has conflicts (resolve first)\n\
- Branch protection rules (may require reviews)\n\
- CI checks failing (must pass first)\n\
- Insufficient permissions (need write access)\n\
- PR already merged or closed\n\n\
AUTHENTICATION:\n\
Requires GitHub token with:\n\
- repo scope (for private repos)\n\
- public_repo scope (for public repos)\n\n\
BEST PRACTICES:\n\
- Check PR status before merging\n\
- Verify CI checks passed\n\
- Ensure required reviews approved\n\
- Delete branch after merge\n\
- Use appropriate merge strategy\n\
- Review changes before merging
```

**Replace with** (single line):
```rust
Remember: Always verify status, check CI, confirm approvals, and clean up branches!
```

**Result**: Trim from 22 lines to 1 line. Rationale: These details belong in workflow scenario, not basic scenario.

### Edit 2: Trim prompt_strategies() team conventions

**Location**: Lines 251-267 (17 lines)

**Delete**:
```rust
TEAM CONVENTIONS:\n\
Establish team-wide merge strategy:\n\
- Feature PRs → SQUASH\n\
- Release PRs → MERGE\n\
- Hotfix PRs → REBASE\n\
- Documentation → SQUASH\n\n\
BRANCH PROTECTION:\n\
Some strategies may be restricted by:\n\
- Branch protection rules\n\
- Organization policies\n\
- Repository settings\n\
Always verify allowed strategies first.\n\n\
DEFAULT BEHAVIOR:\n\
If merge_method not specified:\n\
- Defaults to merge commit\n\
- Repository settings determine behavior\n\
- Check repo default merge method
```

**Result**: Remove 17 lines of prescriptive guidance. Keep decision tree only (lines 182-203).

### Edit 3: Delete entire prompt_messages() function

**Location**: Lines 274-411 (entire function, 138 lines)

**Delete completely**: From `fn prompt_messages() -> Vec<PromptMessage> {` through final closing brace `}` on line 411.

**Rationale**: Commit message customization is secondary to core merge function. Trimming from 5 to 3 scenarios requires removing lower-priority content.

### Edit 4: Trim prompt_workflows() - remove release workflow

**Location**: Lines 494-527 (34 lines)

**Delete**:
```rust
3. RELEASE MERGE WORKFLOW:\n\
// Step 1: Verify release PR\n\
github_get_pr({\n\
    "owner": "org",\n\
    "repo": "app",\n\
    "pull_number": 1000\n\
})\n\
// Check: Merging to main, no conflicts\n\
\n\
// Step 2: Ensure all checks pass\n\
github_get_pull_request_status({\n\
    "owner": "org",\n\
    "repo": "app",\n\
    "pull_number": 1000\n\
})\n\
\n\
// Step 3: Merge with version info\n\
github_merge_pr({\n\
    "owner": "org",\n\
    "repo": "app",\n\
    "pull_number": 1000,\n\
    "merge_method": "merge",\n\
    "commit_title": "Release v2.0.0 (#1000)",\n\
    "commit_message": "Release version 2.0.0\\n\\nChangelog:\\n- New payment system\\n- Dark mode\\n- Performance improvements\\n\\nBreaking changes documented in CHANGELOG.md"\n\
})\n\
\n\
// Step 4: Create release tag\n\
github_create_release({\n\
    "owner": "org",\n\
    "repo": "app",\n\
    "tag_name": "v2.0.0",\n\
    "name": "Version 2.0.0",\n\
    "body": "Major release with new features..."\n\
})\n\n\
```

**Rationale**: Release workflows are too specialized for medium complexity. Production merge workflow covers the essential merge patterns.

### Edit 5: Trim prompt_workflows() - remove hotfix workflow

**Location**: Lines 528-551 (24 lines)

**Delete**:
```rust
4. HOTFIX MERGE WORKFLOW:\n\
// Step 1: Quick verification\n\
github_get_pr({\n\
    "owner": "team",\n\
    "repo": "service",\n\
    "pull_number": 555\n\
})\n\
\n\
// Step 2: Emergency merge\n\
github_merge_pr({\n\
    "owner": "team",\n\
    "repo": "service",\n\
    "pull_number": 555,\n\
    "merge_method": "rebase",\n\
    "commit_title": "hotfix: Critical security patch (#555)",\n\
    "commit_message": "Fixes authentication bypass vulnerability.\\n\\nSecurity: CVE-2024-XXXX\\nSeverity: Critical\\n\\nImmediate deployment required."\n\
})\n\
\n\
// Step 3: Delete branch\n\
github_delete_branch({\n\
    "owner": "team",\n\
    "repo": "service",\n\
    "branch": "hotfix/auth-bypass"\n\
})\n\n\
```

**Rationale**: Hotfix is specialized use case. Basic and production workflows provide sufficient pattern coverage.

### Edit 6: Delete entire prompt_comprehensive() function

**Location**: Lines 625-851 (entire function, 227 lines)

**Delete completely**: From `fn prompt_comprehensive() -> Vec<PromptMessage> {` through final closing brace.

**Rationale**: Pure duplication. Three focused scenarios replace comprehensive approach.

### Edit 7: Update routing match arms (lines 16-24)

**Location**: PromptProvider implementation

**Before**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("strategies") => prompt_strategies(),
        Some("messages") => prompt_messages(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**After**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("strategies") => prompt_strategies(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_basic(),
    }
}
```

### Edit 8: Update prompt_arguments description (line 31)

**Before**:
```rust
description: Some("Scenario to show (basic, strategies, messages, workflows)".to_string()),
```

**After**:
```rust
description: Some("Scenario to show (basic, strategies, workflows)".to_string()),
```

---

## Success Criteria

✓ **Line count**: 280-360 lines total
  - Result should be: ~40 lines (header + PromptProvider impl) + 100 lines (prompt_basic) + 110 lines (prompt_strategies) + 110 lines (prompt_workflows) = 360 lines ± 20

✓ **Scenario count**: Exactly 3 functions
  - `prompt_basic()` exists
  - `prompt_strategies()` exists
  - `prompt_workflows()` exists
  - No `prompt_messages()` function (deleted)
  - No `prompt_comprehensive()` function (deleted)

✓ **Routing logic**: Only 3 arms plus default
  - match statement has: `Some("strategies")`, `Some("workflows")`, `_ => prompt_basic()`
  - No "messages" arm
  - No comprehensive default

✓ **Content focused**: Each scenario under 120 lines
  - `prompt_basic()`: 90-110 lines
  - `prompt_strategies()`: 100-120 lines
  - `prompt_workflows()`: 100-120 lines

✓ **Workflow coverage**: Core patterns present
  - Basic scenario: Simple merge, verification patterns
  - Strategies scenario: Merge/squash/rebase decisions
  - Workflows scenario: Basic + production merges, error handling, decision tree

---

## Validation Commands

After completing all edits, run these in `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/merge_pull_request/`:

1. **Line count**:
   ```
   wc -l prompts.rs
   ```
   Expected: 280-360 lines

2. **Scenario function count**:
   ```
   grep -c "^fn prompt_" prompts.rs
   ```
   Expected: 3

3. **Verify no messages scenario**:
   ```
   grep "fn prompt_messages" prompts.rs
   ```
   Expected: 0 results

4. **Verify no comprehensive scenario**:
   ```
   grep "fn prompt_comprehensive" prompts.rs
   ```
   Expected: 0 results

5. **Verify routing correctness**:
   ```
   grep -A5 "fn generate_prompts" prompts.rs
   ```
   Expected: 3 Some() arms + default

6. **Type check**:
   ```
   cargo check
   ```
   Expected: No errors

---

## Implementation Pattern Notes

This task follows the Complexity 3 template established by PRECURSOR_03_git_branch_create:

- **2-3 focused scenarios** (not exhaustive): Each covers distinct use case or important decision
- **~100-120 lines per scenario**: Sufficient for detailed guidance without duplication
- **One workflow example per scenario**: Not multiple variations
- **Total ~320 lines**: Compact while complete
- **No comprehensive scenario**: Focused approach replaces comprehensive duplication

The three scenarios work together:
1. **basic**: "I want to merge a PR, what are the simple patterns?"
2. **strategies**: "Which merge strategy should I use and when?"
3. **workflows**: "What's the complete merge workflow with error handling?"

This structure enables efficient AI agent learning without overwhelming detail.
