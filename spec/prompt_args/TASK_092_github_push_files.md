# TASK 092: Trim github_push_files

**Tool**: `github_push_files`
**Complexity**: 3 (Medium)
**Current size**: 102 lines (1 monolithic scenario)
**Target size**: 320 lines (2 focused scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/push_files/prompts.rs`

---

## Current State Analysis

### File Structure (102 lines)

The file contains a single monolithic User/Assistant prompt pair (lines 22-100) embedded directly in the `generate_prompts()` function:

```
Lines 1-8:    Module comment and imports
Lines 10-12:  PushFilesPrompts struct definition
Lines 14-21:  PromptProvider trait impl + generate_prompts() header
Lines 22-100: Single monolithic PromptMessage pair:
              - User question (line 22): "How do I use github_push_files to commit multiple files at once?"
              - Assistant response (lines 24-100): ~77 lines of content
Lines 102:    Closing vec! and function
Lines 103:    Closing brace for impl block
```

### Current Scenario Content

The single scenario covers:
1. **BASIC USAGE** (lines 26-37): 4 usage examples
   - Push single file
   - Push multiple files  
   - Update configuration
   - Batch code generation

2. **PARAMETERS** (lines 39-41): Documentation of 5 required parameters
   - owner, repo, branch, message, files

3. **AUTHENTICATION** (lines 43-47): GitHub token requirements

4. **RESPONSE** (lines 49-55): Expected response format with 6 fields

5. **COMMON WORKFLOWS** (lines 57-68): Three workflow patterns
   - Documentation generation
   - Code generation
   - Batch configuration updates

6. **RATE LIMITING** (lines 70-73): Authenticated vs unauthenticated limits

7. **ERROR SCENARIOS** (lines 75-81): Three error types with fixes
   - 404 Not Found
   - 422 Unprocessable
   - 409 Conflict

8. **BEST PRACTICES** (lines 83-100): Twelve best practice guidelines

### Problem Assessment

The current implementation violates Complexity 3 patterns:
- Single monolithic scenario instead of 2-3 focused scenarios
- No scenario routing logic (no match on args.scenario)
- All content crammed into one User/Assistant pair
- No separation of concerns by use case or parameter focus
- Mixing basic usage with advanced error handling and best practices

---

## Trimming Instructions

### SCENARIO 1: Create `prompt_basic()` (~150 lines)

**Purpose**: Basic multi-file push to existing branch

**Structure**:
```rust
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I push multiple files to a repository branch at once?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                // ~140 lines of focused content
            ),
        },
    ]
}
```

**Content breakdown** (target: ~140 lines):

1. **Opening paragraph** (5 lines):
   - What the tool does: atomic commit of multiple files
   - When to use: batch updates, generated code, documentation

2. **BASIC USAGE** (35 lines) - KEEP 3 EXAMPLES FROM CURRENT, DELETE 1:
   - Push documentation files: `github_push_files({owner, repo, branch, message, files: {file1, file2}})`
   - Push configuration batch: Example with 2-3 config files
   - Push generated code: Code generation output
   - DELETE: "Push single file" example (use github_create_or_update_file instead, per best practices)

3. **PARAMETERS** (20 lines) - BRIEF:
   - owner (required): Repository owner
   - repo (required): Repository name
   - branch (required): Target branch (must exist)
   - message (required): Commit message (single for all files)
   - files (required): Map of path→base64-content pairs

4. **FILE ENCODING REQUIREMENT** (10 lines):
   - All file content MUST be base64-encoded
   - No exceptions; tool validates base64 format
   - Common mistake: forgetting to encode

5. **SINGLE WORKFLOW EXAMPLE** (35 lines) - DOCUMENTATION UPDATE PATTERN:
   - Generate API documentation
   - Base64-encode all doc files
   - Push to main or feature branch
   - Complete example with 3-4 files

6. **AUTHENTICATION** (10 lines):
   - GITHUB_TOKEN environment variable required
   - Token scope: "repo" for private, "public_repo" for public
   - No embedded credentials

7. **ERROR HANDLING** (15 lines) - COMMON ERRORS ONLY:
   - 404 Not Found: Branch doesn't exist
   - 422 Unprocessable: Invalid base64 encoding
   - Fix: Validate before calling tool

8. **BEST PRACTICES** (10 lines) - ESSENTIAL ONLY:
   - Always base64-encode
   - Use for multiple files; single files use github_create_or_update_file
   - File paths relative to repo root (no leading /)
   - Atomic operation: all files in one commit

**Actions**:
- Keep examples 2, 3, 4 from current prompt (delete single file example)
- Delete RATE LIMITING section (move to separate scenario if needed)
- Delete verbose best practices (keep only essential 4-5)
- Keep ERROR SCENARIOS but reduce to 2 most common
- Consolidate RESPONSE format into parameters or omit (implementation detail)

---

### SCENARIO 2: Create `prompt_feature_branch()` (~150 lines)

**Purpose**: Feature branch workflow with file generation and pushing

**Structure**:
```rust
fn prompt_feature_branch() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use push_files to commit generated code to a feature branch?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                // ~140 lines of focused content
            ),
        },
    ]
}
```

**Content breakdown** (target: ~140 lines):

1. **Opening paragraph** (5 lines):
   - Feature branch workflow context
   - Typical use: code generation, batch updates, PR preparation
   - Benefits: isolated changes, atomic commits, easy review

2. **FEATURE BRANCH PATTERN** (30 lines):
   - First create branch (git_branch_create):
     * `git_branch_create({path, name: "feature/generated-code", checkout: false})`
   - Then push files (github_push_files):
     * `github_push_files({owner, repo, branch: "feature/generated-code", message, files})`
   - Why separate steps: branch must exist before pushing

3. **COMMON GENERATION SCENARIOS** (40 lines) - 3 EXAMPLES:
   - OpenAPI client generation: Generate from spec → push to feature/openapi-client
   - GraphQL schema generation: Generate from schema → push to feature/graphql-update
   - Code scaffold generation: Generate from config → push to feature/scaffold-[date]

4. **COMPLETE WORKFLOW EXAMPLE** (45 lines) - CODE GENERATION PIPELINE:
   - Fetch main branch (git_clone or git_fetch)
   - Create feature branch (git_branch_create)
   - Run code generator (external tool, not MCP)
   - Base64-encode all generated files
   - Push files to feature branch (github_push_files)
   - Create pull request (github_create_pull_request) - optional
   - Review, approve, merge workflow

5. **ATOMIC OPERATION BENEFITS** (12 lines):
   - All generated files committed together
   - Single commit message for entire generation
   - Easy to revert: one commit to revert
   - PR reviewer sees all changes at once

6. **BATCH UPDATES FOR MULTIPLE REPOSITORIES** (10 lines):
   - Pattern: Loop through repos, create branch, push files
   - One github_push_files call per repo
   - Atomic per-repo, not across repos

7. **ERROR HANDLING** (10 lines):
   - Branch not found: Must create first with git_branch_create
   - Invalid base64: Verify encoding before pushing
   - Conflict: Rebase feature branch if main updated

**Actions**:
- New content: Focus on feature branch workflow (not in current prompt)
- Include integration pattern with git_branch_create
- Show one complete multi-step workflow
- Explain atomic operation benefits
- Add batch update pattern for multiple repos
- Delete RATE LIMITING (not specific to this scenario)

---

### DELETE ENTIRELY

**Lines to delete from current implementation**:
- RATE LIMITING section (current lines 70-73): 4 lines
  - Generic GitHub API info, not specific to this tool
  - Can be referenced in documentation, not in prompts

**No comprehensive scenario in current file** - only one scenario exists, so nothing to delete there.

---

### Update Scenario Routing

**BEFORE** (current lines 18-21):
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
        // Single hardcoded prompt
    ]
}
```

**AFTER**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("feature_branch") => prompt_feature_branch(),
        _ => prompt_basic(),  // Default to basic scenario
    }
}
```

**Changes**:
- Change `_args` to `args` (currently unused)
- Add match statement on `args.scenario`
- Route to `prompt_feature_branch()` when scenario="feature_branch"
- Default to `prompt_basic()` for no scenario or other values
- Both functions return `Vec<PromptMessage>` directly

---

## Implementation Sequence

### Step 1: Refactor current content into `prompt_basic()`

1. Extract lines 22-100 (current PromptMessage pair)
2. Create new function `fn prompt_basic() -> Vec<PromptMessage>`
3. Trim the Assistant response to ~140 lines:
   - Keep: BASIC USAGE (3 examples), PARAMETERS, ENCODING, WORKFLOW, AUTHENTICATION, ERROR HANDLING, BEST PRACTICES
   - Delete: RATE LIMITING, reduce redundant sections
   - Consolidate: Response format into parameters documentation

### Step 2: Create `prompt_feature_branch()` function

1. Create new function `fn prompt_feature_branch() -> Vec<PromptMessage>`
2. User question: "How do I use push_files to commit generated code to a feature branch?"
3. Write ~140-line Assistant response with the structure defined above:
   - Feature branch pattern (30 lines)
   - 3 generation scenarios (40 lines)
   - Complete workflow example (45 lines)
   - Atomic operation benefits (12 lines)
   - Batch updates pattern (10 lines)
   - Error handling (10 lines)

### Step 3: Update routing logic

1. Change `generate_prompts(_args: ...)` to `generate_prompts(args: ...)`
2. Replace function body with match statement routing on `args.scenario`
3. Verify both `prompt_basic()` and `prompt_feature_branch()` are called

### Step 4: Update `prompt_arguments()`

Keep unchanged (returns empty vec, no customization args).

---

## Code Pattern Examples

### Pattern 1: Basic Scenario Structure
```rust
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I push multiple files to a repository branch at once?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_push_files tool commits multiple files atomically to a branch.\n\n\
                BASIC USAGE:\n\
                [examples...]\n\n\
                PARAMETERS:\n\
                [param docs...]\n\n\
                [other sections...]\
                "
            ),
        },
    ]
}
```

### Pattern 2: Routing Logic
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("feature_branch") => prompt_feature_branch(),
        _ => prompt_basic(),
    }
}
```

### Pattern 3: Feature Branch Workflow Example
```
1. Create feature branch:
   git_branch_create({
     "path": "/home/user/tokio",
     "name": "feature/generated-openapi",
     "checkout": false
   })

2. Generate code:
   [External tool generates files]

3. Encode and push:
   github_push_files({
     "owner": "tokio-rs",
     "repo": "tokio",
     "branch": "feature/generated-openapi",
     "message": "chore: Generate OpenAPI client from spec",
     "files": {
       "src/generated/client.rs": "[base64 content]",
       "src/generated/models.rs": "[base64 content]"
     }
   })
```

---

## Success Criteria

### Line Count
- ✓ Total file: 280-360 lines (current 102 → target 320)
- ✓ Imports and struct: ~8 lines (unchanged)
- ✓ prompt_basic(): 140-160 lines
- ✓ prompt_feature_branch(): 140-160 lines
- ✓ Routing/closing: ~15-20 lines

### Scenario Functions
- ✓ Exactly 2 scenario functions: prompt_basic, prompt_feature_branch
- ✓ Each returns Vec<PromptMessage>
- ✓ Each contains 1 User question + 1 Assistant response

### No Redundancy
- ✓ No comprehensive scenario
- ✓ No duplication between scenarios
- ✓ Each scenario covers distinct use case
- ✓ Feature branch scenario doesn't repeat basic examples

### Content Quality
- ✓ Basic scenario: straightforward multi-file push patterns
- ✓ Feature branch scenario: shows workflow with git_branch_create integration
- ✓ Workflows are actionable and specific
- ✓ Error handling addresses real use cases
- ✓ All parameters documented once (in basic scenario)

### Measurable Validation
After implementation, verify:
1. `wc -l packages/kodegen-mcp-schema/src/github/push_files/prompts.rs` → 280-360 lines
2. `grep "^fn prompt_" packages/kodegen-mcp-schema/src/github/push_files/prompts.rs` → 2 results (prompt_basic, prompt_feature_branch)
3. `grep "fn prompt_comprehensive" packages/kodegen-mcp-schema/src/github/push_files/prompts.rs` → 0 results
4. `grep "match args.scenario" packages/kodegen-mcp-schema/src/github/push_files/prompts.rs` → 1 result (routing logic)
5. Cargo check: No compilation errors in kodegen-mcp-schema package

---

## Implementation Checklist

- [ ] Read current prompts.rs (102 lines)
- [ ] Extract and trim current content into prompt_basic()
- [ ] Create prompt_feature_branch() with new scenario content
- [ ] Update generate_prompts() routing logic
- [ ] Verify total line count: 280-360 lines
- [ ] Verify 2 scenario functions exist
- [ ] Verify routing logic matches on args.scenario
- [ ] Run cargo check in kodegen-mcp-schema package
- [ ] Verify file compiles without warnings
- [ ] Count scenarios: grep "^fn prompt_" returns exactly 2
