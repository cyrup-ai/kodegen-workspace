# TASK 087: Refactor github_create_or_update_file into Scenario-Based Structure

**Tool**: `github_create_or_update_file`
**Complexity**: 3 (Medium)
**Current size**: 104 lines (1 monolithic scenario)
**Target size**: 310-360 lines (3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_or_update_file/prompts.rs`

---

## Reference

See **PRECURSOR_03_git_branch_create.md** for Complexity 3 template and patterns.

---

## Context

The `github_create_or_update_file` tool modifies files in GitHub repositories with parameters: `owner`, `repo`, `path`, `content`, `message`, `branch`, `sha`. The current 104-line prompt file is **monolithic** (all content in one PromptMessage pair) rather than scenario-based, leading to difficult maintenance and overwhelming context.

**Current file structure**: 
- Lines 1-13: Imports and struct declaration
- Lines 14-104: Single `generate_prompts()` function returning one PromptMessage pair
- Lines 25-103: One 79-line Assistant response covering all topics at once

This needs refactoring into 3 focused scenarios based on distinct use cases.

---

## Current Scenario Analysis

**Current state** (104 lines total):
- `generate_prompts()` - 90 lines (lines 14-104) ← REFACTOR INTO 3 FUNCTIONS

The single Assistant response currently covers (sequentially):
1. Basic usage (4 examples: create, update, feature branch, with author)
2. Parameters section (owner, repo, path, content, message, branch, sha, author fields)
3. Authentication requirements
4. Response format (success, paths, SHAs, URLs, operation type)
5. Common workflows (3 categories: docs, config, batch files)
6. Rate limiting (authenticated vs unauthenticated limits)
7. Error scenarios (404, 409 Conflict with SHA mismatch, 422)
8. Best practices (8 bullet points including base64, SHA, messages, branches, batch tool, bot info, rules, encoding)

**Content distribution across parameters**:
- `owner`, `repo`, `path`, `content`, `message` → Basic scenario (required params)
- `sha` → SHA handling scenario (update conflict prevention)
- `branch` → Branch targeting scenario (feature workflows)

---

## Trimming/Refactoring Instructions

### Step 1: Read and Analyze

Read `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_or_update_file/prompts.rs` and understand:
- Current monolithic structure (no scenario functions)
- Content coverage: all 8 topic categories in one response
- Which examples are pure creation vs pure update
- How SHA conflicts differ from basic operations
- Where branch-specific workflows appear

---

### Step 2: Create 3 Scenario Functions

Refactor the monolithic response into 3 distinct scenario functions. Each returns `Vec<PromptMessage>` with its own User question and Assistant response.

#### SCENARIO 1: `prompt_basic()` (Target: ~110 lines)

**User question**: "How do I create or update files in a GitHub repository?"

**Keep from current response:**
- Example 1: Create new file (1-2 lines): Simple call with owner, repo, path, content, message, branch
- Example 2: Update existing file (1-2 lines): Same call structure but shows it handles both operations
- Parameter documentation (~20 lines): Brief descriptions of owner, repo, path, content, message, branch - focus on what each parameter means, not how to use them
- Authentication requirements (~8 lines): GITHUB_TOKEN with repo/public_repo scopes
- Response format (~12 lines): success, owner, repo, path, sha, commit_sha, html_url, operation field
- When to use basic approach (~10 lines): Single file operations, direct commits to branches
- Common single-file workflows (~15 lines): 
  - Simple file creation
  - Updating README or docs
  - Committing configuration changes

**Remove from current response:**
- SHA conflict handling (move to scenario 2)
- Author/committer fields and examples (keep basic operation focus)
- Rate limiting details (out of scope for scenario)
- Error scenarios involving SHA mismatch (scenario 2)
- All branch-specific workflows (scenario 3)
- Batch file creation workflow (scenario 3)

**Structure**: User question → 1 short creation example → 1 short update example → Parameter docs → Auth → Response format → One workflow example

#### SCENARIO 2: `prompt_sha_handling()` (Target: ~110 lines)

**User question**: "How do I safely update files and prevent SHA conflicts?"

**Extract/create from current response:**
- SHA parameter explanation (~15 lines): What SHA is, why it matters, what it prevents
- Conflict prevention concept (~15 lines): File may change between read and update, SHA ensures atomicity
- Complete workflow example (~30 lines):
  - Step 1: Call github_get_file_contents to read current file + get SHA
  - Step 2: Modify content locally
  - Step 3: Call github_create_or_update_file with the SHA from step 1
  - Shows the exact sequence with example repo
- Error handling for 409 Conflict (~15 lines):
  - Why conflicts happen (SHA mismatch)
  - How to fix: get latest SHA, retry
  - When this happens in real workflows
- Best practices for updates (~20 lines):
  - Always fetch current SHA before updating
  - Retry logic if conflict occurs
  - Use meaningful commit messages describing change
  - Consider using git instead for complex merges
- When to use SHA handling (~15 lines): Multi-step operations, concurrent access scenarios, CI/CD workflows

**Remove from current response:**
- Basic creation info (already in scenario 1)
- Branch parameter details (scenario 3)
- General authentication (already in scenario 1)
- Non-SHA-related best practices

**Structure**: User question → SHA explanation → Why it matters → Full workflow example (get → modify → update) → Error handling → Best practices

#### SCENARIO 3: `prompt_branch_targeting()` (Target: ~90 lines) - OPTIONAL

**User question**: "How do I create/update files on feature branches and integrate with PRs?"

**Extract/focus on:**
- Branch parameter usage (~12 lines): What branch does, defaults to default branch, can target any branch
- Creating on feature branches (~15 lines):
  - Example: commit to feature/new-docs instead of main
  - Isolates work before PR review
  - Prevents direct main commits
- Example workflow (~30 lines):
  - Create feature branch via git_branch_create (separate tool)
  - Use github_create_or_update_file with branch: "feature/update-config"
  - Make multiple file changes on same branch
  - Create PR from feature to main
- Branch naming patterns (~10 lines): feature/, fix/, docs/ prefixes
- Integration with GitHub workflows (~15 lines):
  - Create files on PR branch
  - Commit triggers CI/CD
  - Review before merge

**Remove from current response:**
- Basic auth (scenario 1)
- SHA details (scenario 2)
- Non-branch-related content

**Structure**: User question → Branch parameter → Why use feature branches → One complete workflow (create branch, add files, create PR) → Branch naming → CI/CD integration

---

### Step 3: Update Routing Logic

Replace the current monolithic `generate_prompts()` function with scenario-based routing:

```rust
// BEFORE: Monolithic single call
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
        PromptMessage { ... },  // Single user question
        PromptMessage { ... },  // One huge assistant response
    ]
}

// AFTER: Scenario-based routing (follow git_branch_create pattern)
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario {
        Some("sha_handling") => prompt_sha_handling(),
        Some("branch_targeting") => prompt_branch_targeting(),
        _ => prompt_basic(),  // Default to basic scenario
    }
}

// Each scenario function returns Vec<PromptMessage>
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage { role: User, content: "How do I create or update files..." },
        PromptMessage { role: Assistant, content: "The github_create_or_update_file tool...\n\n..." },
    ]
}

fn prompt_sha_handling() -> Vec<PromptMessage> {
    vec![
        PromptMessage { role: User, content: "How do I safely update files..." },
        PromptMessage { role: Assistant, content: "SHA (Secure Hash Algorithm)...\n\n..." },
    ]
}

fn prompt_branch_targeting() -> Vec<PromptMessage> {
    vec![
        PromptMessage { role: User, content: "How do I create/update files on feature branches..." },
        PromptMessage { role: Assistant, content: "The branch parameter enables...\n\n..." },
    ]
}
```

**Key changes**:
- Replace `fn generate_prompts()` implementation (lines 15-104) with routing match statement
- Create 3 new scenario functions above `generate_prompts()`
- Each scenario function has its own User question (distinct, scenario-specific)
- Each scenario has focused Assistant response (no duplication)
- Default to `prompt_basic()` for backward compatibility

---

## Success Criteria

After refactoring, verify:
- ✓ File is 310-360 lines total (expanded from 104 to accommodate 3 functions)
- ✓ THREE scenario functions: `prompt_basic()`, `prompt_sha_handling()`, `prompt_branch_targeting()`
- ✓ Each function is 100-120 lines (includes User + Assistant messages)
- ✓ Each scenario has DISTINCT user question (not rephrased versions)
- ✓ Each scenario has ONE concrete workflow example (not multiple)
- ✓ No monolithic response remaining
- ✓ Routing logic uses match statement with default
- ✓ All 3 parameters (owner, repo, path, content, message, branch, sha) documented across scenarios
- ✓ SHA conflict handling isolated in scenario 2
- ✓ Branch workflows isolated in scenario 3
- ✓ No decorative headers or repeated boilerplate
- ✓ Lines 1-13 unchanged (imports, struct, trait declaration)

---

## Validation

After refactoring:
1. **Line count**: `wc -l prompts.rs` → 310-360 lines
2. **Scenario count**: `grep "^fn prompt_" prompts.rs` → exactly 3 results (basic, sha_handling, branch_targeting)
3. **No monolithic**: Verify generate_prompts() is now ~10 lines (just match statement)
4. **Routing works**: Verify match statement has 3 branches + default
5. **No duplication**: Search for repeated phrases like "github_create_or_update_file" - should appear once per scenario (3-4 times total, not more)
6. **Content distribution**: 
   - scenario 1 (basic): Create + update examples
   - scenario 2 (sha): "github_get_file_contents" mentioned (read-modify-update pattern)
   - scenario 3 (branch): Feature branch and PR workflow
7. **Readability test**: Read each scenario start-to-finish in sequence - each should be complete and understandable without other scenarios
8. **Imports remain**: Lines 1-13 should be untouched (all use statements, struct definition)

---

## Implementation Notes

- **Each function creates its own PromptMessage vector** - this allows per-scenario customization
- **User question is the key differentiator** - each scenario starts with a distinct question that guides the response focus
- **One workflow per scenario** - not multiple examples within a scenario; one clear path instead
- **Parameters distributed by concern** - all params documented, but SHA emphasis in scenario 2, branch emphasis in scenario 3
- **Default to basic** - backward compatible; existing callers without scenario specified get the baseline behavior
- **Total line budget expansion** - current 104 lines → target 310-360 because we now have structure (3 functions with headers + routing) instead of monolithic approach

---

## This Establishes Medium-Complexity Standard

Once refactored, this file becomes a **reference implementation** for Complexity 3 scenario-based prompting:
- Monolithic → Scenario-based refactoring pattern
- Parameters mapped to distinct use cases
- Each scenario self-contained and learnable
- Routing enables flexible configuration
- Total 250-400 lines with 2-3 scenarios
