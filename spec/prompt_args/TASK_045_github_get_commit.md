# TASK 045: Expand github_get_commit to Complexity 2 Standard

**Tool**: `github_get_commit`
**Complexity**: 2 (Simple)
**Current size**: 104 lines (1 inline scenario)
**Target size**: 170-220 lines (1 expanded scenario)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_commit/prompts.rs`

---

## Current State Analysis

### File Structure (104 lines total)

**Current organization:**
```
Lines 1-12:    Module documentation and imports
Lines 14-16:   GetCommitPrompts struct definition
Lines 18-102:  impl PromptProvider with 2 methods
                 - generate_prompts(): Returns 1 Q&A pair (lines 19-102)
                 - prompt_arguments(): Empty (line 104)
Line 104:      Closing brace
```

### Current Scenario Breakdown

**Single inline scenario in generate_prompts()** (lines 28-103, ~76 lines):
- **Basic usage examples** (lines 28-50, ~23 lines):
  - Get commit by full SHA (40-character hex)
  - Get commit by short SHA (7+ characters)
  - Get HEAD commit
  - Get commit by branch name
  - Note: Missing tag reference and merge commit examples

- **Parameters documentation** (lines 51-60, ~10 lines):
  - owner (required): Repository owner
  - repo (required): Repository name
  - sha (required): Commit SHA (full/short), branch name, or tag

- **Authentication** (lines 61-70, ~10 lines):
  - GITHUB_TOKEN required
  - Scopes: repo (private), public_repo (public only)

- **Response structure** (lines 71-87, ~17 lines):
  - success flag
  - owner, repo identifiers
  - commit object with sha, message, author, committer, parents
  - stats (additions, deletions, total)
  - files array with patches
  - html_url

- **Content sections** (lines 88-103, ~16 lines):
  - COMMON WORKFLOWS (3): code review, change tracking, verification
  - RATE LIMITING: authenticated (5k/hr) vs unauthenticated (60/hr)
  - ERROR SCENARIOS (3): 404, 422, 403 with fixes
  - BEST PRACTICES: brief guidelines

**Assessment**: Single comprehensive scenario but UNDER-detailed for Complexity 2. Lacks:
- Separate scenario function (currently inline)
- Helper function pattern for routing
- Sufficient examples (only 4, should have 6-8)
- Detailed workflows (only 3, should have 5-6)
- Error scenarios (only 3, should have 4+)
- Advanced patterns section (missing entirely)
- Integration examples with related tools (missing)

Current 104 lines is 66 lines SHORT of 170-line minimum target.

---

## Reference Pattern

Compare to `list_branches` tool (packages/kodegen-mcp-schema/src/github/list_branches/prompts.rs):

**Pattern structure** (lines 13-36):
```rust
impl PromptProvider for ListBranchesPrompts {
    type PromptArgs = ListBranchesPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("protection") => prompt_protection(),
            Some("workflows") => prompt_workflows(),
            Some("pagination") => prompt_pagination(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                description: Some("Scenario to show (basic, protection, workflows, pagination)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// Then separate functions starting at line 42:
fn prompt_basic() -> Vec<PromptMessage> { ... }        // Lines 43-132 (90 lines)
fn prompt_protection() -> Vec<PromptMessage> { ... }   // Lines 135-262 (128 lines)
fn prompt_workflows() -> Vec<PromptMessage> { ... }    // Lines 265-488 (224 lines)
fn prompt_pagination() -> Vec<PromptMessage> { ... }   // Lines 491-698 (208 lines)
fn prompt_comprehensive() -> Vec<PromptMessage> { ... } // Lines 701-992 (292 lines)
```

**Key differences from github_get_commit:**
1. Uses match statement for scenario routing
2. Each scenario in separate function returning Vec<PromptMessage>
3. prompt_arguments() defines available scenarios
4. Each scenario typically 90-100+ lines (or 60-90 for basic)

**For github_get_commit**: Follow same pattern but simpler - only 1 scenario needed at 170-190 lines total.

---

## Implementation Plan - REQUIRED

This IS how to expand the file. Do not deviate.

### Step 1: Create Helper Function Pattern (REQUIRED)

Convert from inline scenario in generate_prompts() to separate function:

**FROM (current, lines 18-104):**
```rust
impl PromptProvider for GetCommitPrompts {
    type PromptArgs = GetCommitPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_get_commit to retrieve commit details?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_commit tool retrieves detailed...",  // 76-line response
            ),
        },
    ]
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}
```

**TO (new structure):**
```rust
impl PromptProvider for GetCommitPrompts {
    type PromptArgs = GetCommitPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()  // Delegate to helper function
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]  // No scenario routing needed for single scenario
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO GET GITHUB COMMITS
// ============================================================================

/// Basic commit retrieval and analysis
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage { ... },  // User question: 3 lines
        PromptMessage { ... },  // Assistant response: 140-150 lines
    ]
}
```

**Line counts after refactoring:**
- impl block: 7 lines
- prompt_arguments: 3 lines
- Section header: 4 lines
- prompt_basic function signature: 2 lines
- Vec with messages: 145 lines
- Closing brace: 1 line
- **Total: ~162 lines** (need to add 8-58 more lines to reach 170-220)

### Step 2: Expand Assistant Response in prompt_basic (REQUIRED)

Expand from current 76-line response to 140-150 lines by adding detailed content in these sections.

#### 2A: BASIC USAGE Section - ADD 3 MORE EXAMPLES

**Current examples** (keep all 4):
1. Get commit by full SHA
2. Get commit by short SHA
3. Get HEAD commit
4. Get commit by branch name

**ADD 3 NEW examples** (~15 lines total):
```
5. Get commit by tag:
   github_get_commit({"owner": "kubernetes", "repo": "kubernetes", "sha": "v1.28.0"})
   // Retrieves commit tagged with v1.28.0
   // Useful for analyzing release commits

6. Get commit by merge reference:
   github_get_commit({"owner": "golang", "repo": "go", "sha": "8a1e5f3abc"})
   // For merge commits, includes both parent SHAs
   // Use to analyze integration of branches

7. Get specific commit across forks:
   github_get_commit({"owner": "torvalds", "repo": "linux", "sha": "abc123def456"})
   // Retrieve from any fork/clone
   // SHA must exist in that repository's history
```

**Rationale**: Shows SHA flexibility (tags, merges, across forks) beyond branch references.

#### 2B: COMMON WORKFLOWS Section - EXPAND FROM 3 TO 5 WORKFLOWS

**Keep existing 3:**
1. Code review analysis
2. Change tracking
3. Verification

**ADD 2 NEW** (~20 lines):
```
4. Release validation:
   1. Get release tag commit:
      github_get_commit({"owner": "nodejs", "repo": "node", "sha": "v20.0.0"})

   2. Verify in response:
      - author matches expected release maintainer
      - timestamp aligns with planned release
      - files array shows no unexpected changes

   3. Generate release notes:
      - Extract message field for changelog
      - Use stats for impact assessment (lines changed)
      - Check parents array for branching pattern

   4. Validate before publication:
      - If additions/deletions imbalanced, review unexpected
      - Verify no build/CI configuration changed unexpectedly

5. Performance impact analysis:
   1. Get commit details:
      github_get_commit({"owner": "facebook", "repo": "react", "sha": "abc123"})

   2. Analyze stats object:
      - additions + deletions = total lines changed
      - High impact: > 500 total lines
      - Medium impact: 100-500 lines
      - Low impact: < 100 lines

   3. Review files array:
      - Check if performance-critical files modified
      - Example: render.js, scheduler.js in React
      - Estimate testing scope needed

   4. Generate impact report:
      - Document changes by category (feature, refactor, fix)
      - Flag large modifications to core files
      - Recommend additional code review
```

**Rationale**: Real-world use cases for release management and code quality.

#### 2C: BEST PRACTICES Section - EXPAND FROM 3 TO 8-10 ITEMS

**Current items** (keep all):
- Use full SHA for accuracy
- Short SHAs work but may be ambiguous
- Branch names resolve to latest commit
- Tags resolve to tagged commit
- Check stats for impact analysis
- Parse files array for detailed diff
- Use commit data for changelog
- Verify author/committer for audits
- Check parent commits for merge analysis
- Combine with github_list_commits

**ADD** (~15 lines):
```
- Monitor stats growth: Commits > 1000 additions signal large features
- File analysis: Check if sensitive files modified (auth, security, config)
- Use parents array length: > 1 means merge commit, = 1 means regular commit
- Cache commit data for reporting: API calls are rate-limited
- Combine with github_get_branch: Get latest commit SHA then retrieve details
- Validate before integration: Always check commit belongs to target branch
- Extract message format: Many teams follow Conventional Commits pattern
- Analyze diff patches: Use for automated code pattern detection
```

#### 2D: ADD NEW ADVANCED PATTERNS SECTION (~20 lines)

Insert before closing (after best practices):
```
ADVANCED PATTERNS:

Pattern 1: Build complete commit history
github_list_commits({"owner": "rust-lang", "repo": "rust", "sha": "main", "per_page": 100})
// Returns list of recent commits
// For detailed analysis:
for each commit in results:
    github_get_commit({"owner": "rust-lang", "repo": "rust", "sha": commit.sha})
    // Get full details, stats, files
// Result: Complete history with line-by-line changes

Pattern 2: Track file evolution
github_get_commit({"owner": "torvalds", "repo": "linux", "sha": "abc123"})
// Examine files array for specific file
// See patch showing exact changes
// Track file through multiple commits to understand evolution

Pattern 3: Merge analysis
github_get_commit({"owner": "your-org", "repo": "app", "sha": "merge-sha"})
// Check parents array: if length > 1, it's a merge
// Retrieve each parent commit separately
github_get_commit(..., "sha": parents[0])
github_get_commit(..., "sha": parents[1])
// Compare changes from each parent branch

Pattern 4: Security audit
github_get_commit({"owner": "company", "repo": "backend", "sha": "main"})
// For each commit on critical branch:
// - Check files array for sensitive paths (secrets, keys, creds)
// - Verify author is authorized
// - Ensure commit message references ticket/issue
// - Validate commit signing status (if available)
```

**Rationale**: Shows how to combine with other tools and patterns for complex workflows.

### Step 3: Calculate Exact Line Counts (VALIDATE)

After all additions, file should be ~186 lines:

```
Lines 1-12:     Module/imports: 12 lines
Lines 14-16:    Struct: 3 lines
Lines 18-26:    impl block header + generate_prompts: 9 lines
Lines 28-30:    prompt_arguments: 3 lines
Lines 32-35:    Section header: 4 lines
Lines 37-39:    Function signature: 3 lines
Lines 41-43:    vec! open bracket + User question: 3 lines
Lines 44-186:   Assistant response: 143 lines
                  - Tool description: 5 lines
                  - BASIC USAGE (7 examples): 30 lines
                  - PARAMETERS: 8 lines
                  - AUTHENTICATION: 10 lines
                  - RESPONSE STRUCTURE: 18 lines
                  - COMMON WORKFLOWS (5): 40 lines
                  - RATE LIMITING: 5 lines
                  - ERROR SCENARIOS: 10 lines
                  - BEST PRACTICES: 15 lines
                  - ADVANCED PATTERNS: 20 lines
Line 187:       Closing bracket
TOTAL: 187 lines
```

Target range: 170-220 lines ✓

### Step 4: Code Compilation Validation (REQUIRED)

After editing, MUST verify:

```bash
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
cargo check        # Verify no compilation errors
cargo clippy       # Verify no warnings (warnings = errors in this project)
wc -l src/github/get_commit/prompts.rs  # Verify line count 170-220
```

Expected output:
- `cargo check`: No errors
- `cargo clippy`: No warnings
- `wc -l`: 170-220

---

## Success Criteria - DEFINITION OF DONE

MUST achieve ALL criteria:

✓ **Line count**: 170-220 lines total (wc -l output)
✓ **Scenario functions**: Exactly 1 function named `prompt_basic()`
✓ **Routing**: `generate_prompts()` calls `prompt_basic()` without match statement
✓ **Arguments**: `prompt_arguments()` returns `vec![]` (no scenario options)
✓ **Structure**: Separate function declarations following impl block
✓ **Examples**: 7-8 usage examples in BASIC USAGE section (currently 4)
✓ **Workflows**: 5+ common workflows detailed (currently 3)
✓ **Error scenarios**: 3-4 error types with solutions (keep current 3, add 1)
✓ **Best practices**: 10-12 items (currently 10, add 2-4 more)
✓ **Advanced patterns**: NEW section with 4 patterns added
✓ **No decorations**: No "═══" borders or ASCII art
✓ **Compilation**: `cargo check` passes with no errors
✓ **Linting**: `cargo clippy` passes with no warnings
✓ **Formatting**: `cargo fmt` makes no changes
✓ **One message pair**: Exactly 1 User PromptMessage + 1 Assistant PromptMessage in prompt_basic()

---

## Detailed Code Changes Required

### File Header (UNCHANGED)
```rust
//! Prompt messages for github_get_commit tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetCommitPromptArgs;

/// Prompt provider for get_commit tool
///
/// This is the ONLY way to provide prompts for get_commit - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GetCommitPrompts;
```

### Implementation Block (REFACTORED - LINES 13-30)

```rust
impl PromptProvider for GetCommitPrompts {
    type PromptArgs = GetCommitPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO GET GITHUB COMMITS
// ============================================================================

fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_get_commit to retrieve commit details?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_commit tool retrieves detailed information about a specific commit including changes, statistics, and metadata.\n\n\
                 BASIC USAGE:\n\
                 1. Get commit by SHA:\n\
                    github_get_commit({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"sha\": \"abc123def456789012345678901234567890abcd\"})\n\n\
                 2. Get commit with short SHA:\n\
                    github_get_commit({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"sha\": \"abc123d\"})\n\n\
                 3. Get HEAD commit:\n\
                    github_get_commit({\"owner\": \"actix\", \"repo\": \"actix-web\", \"sha\": \"HEAD\"})\n\n\
                 4. Get commit by branch name:\n\
                    github_get_commit({\"owner\": \"serde-rs\", \"repo\": \"serde\", \"sha\": \"main\"})\n\n\
                 5. Get commit by tag:\n\
                    github_get_commit({\"owner\": \"kubernetes\", \"repo\": \"kubernetes\", \"sha\": \"v1.28.0\"})\n\n\
                 6. Get merge commit details:\n\
                    github_get_commit({\"owner\": \"golang\", \"repo\": \"go\", \"sha\": \"8a1e5f3abc\"})\n\n\
                 7. Retrieve release commit:\n\
                    github_get_commit({\"owner\": \"torvalds\", \"repo\": \"linux\", \"sha\": \"abc123def456\"})\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - sha (required): Commit SHA (full/short), branch name, or tag\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with scopes:\n\
                 - repo (for private repositories)\n\
                 - public_repo (for public repositories only)\n\n\
                 RESPONSE:\n\
                 Returns JSON with:\n\
                 - success: true/false\n\
                 - owner, repo: Repository identifiers\n\
                 - commit: Object containing:\n\
                   - sha: Full commit SHA\n\
                   - message: Commit message\n\
                   - author: Name, email, date\n\
                   - committer: Name, email, date\n\
                   - parents: Array of parent commit SHAs\n\
                   - stats: additions, deletions, total changes\n\
                   - files: Array of changed files with patches\n\
                   - html_url: Link to commit on GitHub\n\n\
                 COMMON WORKFLOWS:\n\
                 1. Code review analysis:\n\
                    - Get commit details\n\
                    - Analyze changed files\n\
                    - Review diff patches\n\
                    - Provide feedback on changes\n\
                 2. Change tracking:\n\
                    - Retrieve commit for specific feature\n\
                    - Extract file changes\n\
                    - Generate changelog entry\n\
                    - Document breaking changes\n\
                 3. Verification:\n\
                    - Get commit by SHA\n\
                    - Verify author and timestamp\n\
                    - Check GPG signature status\n\
                    - Validate commit in CI pipeline\n\
                 4. Release validation:\n\
                    - Get release tag commit\n\
                    - Verify author is release maintainer\n\
                    - Check timestamp aligns with schedule\n\
                    - Review files for unexpected changes\n\
                    - Analyze stats for impact\n\
                 5. Performance impact analysis:\n\
                    - Get commit stats (additions, deletions)\n\
                    - Categorize impact (high > 500, medium 100-500, low < 100)\n\
                    - Review files array for critical changes\n\
                    - Generate impact report for review\n\n\
                 RATE LIMITING:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Unauthenticated: 60 requests/hour\n\
                 - Check X-RateLimit-Remaining header\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found: Commit SHA doesn't exist in repository\n\
                    Fix: Verify SHA is correct and exists in repo history\n\
                 2. 422 Unprocessable: Invalid SHA format\n\
                    Fix: Use valid 40-character hex SHA or short SHA (7+ chars)\n\
                 3. 403 Forbidden: No access to private repository\n\
                    Fix: Verify GITHUB_TOKEN has repo access\n\
                 4. 401 Unauthorized: Invalid or expired token\n\
                    Fix: Regenerate GITHUB_TOKEN with correct scopes\n\n\
                 BEST PRACTICES:\n\
                 - Use full 40-character SHA for accuracy\n\
                 - Short SHAs (7+ chars) work but may be ambiguous\n\
                 - Branch names resolve to latest commit on that branch\n\
                 - Tags resolve to tagged commit\n\
                 - Check stats for impact analysis (lines changed)\n\
                 - Parse files array for detailed diff information\n\
                 - Use commit data for automated changelog generation\n\
                 - Verify author/committer for security audits\n\
                 - Check parent commits for merge analysis\n\
                 - Monitor stats growth for feature detection\n\
                 - Analyze files for sensitive file modifications\n\
                 - Use parents array to detect merge commits (length > 1)\n\n\
                 ADVANCED PATTERNS:\n\
                 Pattern 1: Build complete commit history\n\
                 - Use github_list_commits to get recent commits\n\
                 - For each commit, call github_get_commit for full details\n\
                 - Collect stats and file changes across commits\n\n\
                 Pattern 2: Track file evolution\n\
                 - Get commit and examine files array\n\
                 - See patches showing exact line-by-line changes\n\
                 - Repeat for multiple commits to track file history\n\n\
                 Pattern 3: Merge analysis\n\
                 - Check parents array: length > 1 means merge commit\n\
                 - Retrieve each parent commit separately\n\
                 - Compare changes from each branch\n\n\
                 Pattern 4: Security audit\n\
                 - For each commit on critical branch\n\
                 - Check files array for sensitive paths\n\
                 - Verify author is authorized contributor\n\
                 - Combine with github_get_branch for context",
            ),
        },
    ]
}
```

---

## Validation After Completion

Run these commands to verify success:

```bash
# 1. Check line count
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_commit/prompts.rs
# Expected: 170-220

# 2. Verify compilation
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
cargo check
# Expected: Finished with no errors

# 3. Verify linting
cargo clippy
# Expected: Finished with no warnings

# 4. Count scenario functions
grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_commit/prompts.rs
# Expected: 1 line (prompt_basic)

# 5. Verify routing
grep "prompt_basic()" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_commit/prompts.rs
# Expected: 1 match in generate_prompts()

# 6. Count examples
grep -o "github_get_commit(" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_commit/prompts.rs | wc -l
# Expected: 7-8 matches

# 7. Verify no decorations
grep "═══" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_commit/prompts.rs
# Expected: 0 matches
```

---

## Files to Check for Context

- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_commit/prompt_args.rs` - Check if GetCommitPromptArgs has scenario field
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/list_branches/prompts.rs` - Reference for pattern (lines 1-36 for structure)
- `/Volumes/samsung_t9/kodegen-workspace/task/PRECURSOR_02_fs_read_file.md` - Complexity 2 reference template
