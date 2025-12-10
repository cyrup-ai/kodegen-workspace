# TASK 055: Trim github_list_pull_requests

**Tool**: `github_list_pull_requests`
**Complexity**: 2 (Simple)
**Current size**: 938 lines (4 scenarios)
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/list_pull_requests/prompts.rs`

---

## Template Reference

Follow the **PRECURSOR_02_fs_read_file.md** pattern for Complexity 2 simplification. A Complexity 2 tool should:
- Teach ONLY tool parameters and core features
- Eliminate use-case scenarios (these teach workflows, not the tool)
- Remove comprehensive/duplicate scenarios
- Keep 1-2 focused scenarios
- Target 170-220 lines total

---

## Current State Analysis

### File Structure
- **Lines 1-40**: Header, imports, PromptProvider impl, comment section
- **Lines 43-116**: `prompt_basic()` - 73 lines
- **Lines 118-366**: `prompt_filtering()` - 248 lines
- **Lines 368-708**: `prompt_review()` - 340 lines
- **Lines 710-938**: `prompt_workflows()` - 228 lines
- **Total**: 938 lines across 4 scenario functions

### Current Routing Logic (Lines 16-23)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("filtering") => prompt_filtering(),
        Some("review") => prompt_review(),           // DELETE
        Some("workflows") => prompt_workflows(),     // DELETE
        _ => prompt_comprehensive(),                 // DELETE (default fallback)
    }
}
```

### Scenario Assessment

**KEEP - prompt_basic() [Lines 43-116]**
- Teaches core usage: listing all open PRs, required parameters (owner, repo)
- Demonstrates response structure with key fields (number, title, state, author, etc.)
- Shows basic filtering by repository
- This is a TOOL FEATURE scenario (parameter usage)
- Current: 73 lines → Target: 90-100 lines (add more detail to parameters)

**KEEP - prompt_filtering() [Lines 118-366]**
- Teaches filtering parameters: state, base, head
- Demonstrates sorting options: created, updated, popularity, long-running
- Shows pagination (page, per_page)
- Shows combining filters
- This is a TOOL FEATURE scenario (parameter combinations)
- Current: 248 lines → Target: 90-100 lines (extensive trimming needed)

**DELETE - prompt_review() [Lines 368-708]**
- This is a USE CASE scenario (code review workflows)
- Teaches PR review patterns, not tool parameters
- Content: "Finding PRs needing review", "Identifying draft PRs", "Review workflow patterns"
- No new parameters taught beyond basic/filtering
- Removed: 340 lines

**DELETE - prompt_workflows() [Lines 710-938]**
- This is a USE CASE scenario (PR management patterns)
- Teaches workflows: daily standup, release prep, stale PR cleanup, hotfix management
- No new parameters taught
- Removed: 228 lines

**DELETE - prompt_comprehensive()**
- Not explicitly shown in current match (default fallback)
- Would duplicate content from basic and filtering
- Removed by updating routing logic

---

## Trimming Instructions

### STEP 1: Trim prompt_basic() - Target ~100 lines

**KEEP (essential content):**
- User question: "How do I list pull requests..." (2 lines)
- Assistant intro: Tool description (3-5 lines)
- BASIC LISTING section: 3 examples (15 lines)
  - All open PRs (default)
  - Specific repository
  - Organization repository
- RESPONSE STRUCTURE: Show complete JSON response (15 lines)
- KEY FIELDS: Explain each field once (12-15 lines)
  - number, title, state, author, head_ref, base_ref, created_at, draft
- REQUIRED PARAMETERS (2-3 lines)
  - owner, repo
- AUTHENTICATION section (8 lines)
  - Mention GITHUB_TOKEN requirement
  - Note about rate limiting
- NEXT STEPS: Link to other tools (4 lines)

**DELETE from basic:**
- Excessive examples (more than 3 basic usage patterns)
- Decorative `═══` separators
- "COMMON PATTERNS" section (save patterns for filtering scenario)
- Repeated "Key Fields" explanations
- Code comments within examples

**Before structure (Lines 43-116, 73 lines):**
- User/Assistant role exchange
- Basic listing section with examples
- Response structure (JSON)
- Key fields
- Common patterns (3 patterns shown)
- Required/optional parameters
- Authentication
- Next steps

**After structure (Target ~100 lines):**
- Keep all sections above
- Consolidate to 1 intro + 3 examples + response + 8 field descriptions + auth + next steps

### STEP 2: Trim prompt_filtering() - Target ~100 lines

**Current content (248 lines):**
- FILTERING BY STATE (18 lines) - 3 examples
- FILTERING BY BASE BRANCH (15 lines) - 3 examples
- FILTERING BY HEAD BRANCH (10 lines) - 2 examples
- SORTING OPTIONS (32 lines) - 4 sort types with examples
- SORT DIRECTIONS (4 lines)
- PAGINATION (16 lines) - 3 pagination examples
- COMBINING FILTERS (12 lines) - 3 combined examples
- FILTER PARAMETERS SUMMARY (8 lines) - parameter list
- BEST PRACTICES (8 lines)

**KEEP (essential content for 100 lines):**
- User question: "How do I filter PRs by state, branch..." (2 lines)
- Assistant intro (2-3 lines)

- FILTERING BY STATE: Keep 2 examples (12 lines)
  - Open only
  - Closed only
  - All (mention as option)

- FILTERING BY BASE/HEAD: Keep combined section (12 lines)
  - Base branch filtering (1 example)
  - Head branch filtering (1 example)
  - Format specification for head

- SORTING OPTIONS: Keep 4 sorts with brief examples (18 lines)
  - created, updated, popularity, long-running
  - Show "desc" default, mention "asc" option

- PAGINATION: Keep 2 concise examples (8 lines)
  - per_page usage
  - page parameter

- COMBINING FILTERS: Keep 2 examples (10 lines)
  - Open PRs to main, recently updated
  - Closed PRs to develop

- FILTER PARAMETERS SUMMARY: Keep parameter reference (8 lines)

- BEST PRACTICES: Keep 4-5 concise points (6 lines)

**DELETE from filtering:**
- Decorative section headers
- Verbose explanations already in basic
- "Response structure" (covered in basic)
- Extended "WHEN TO USE" explanations
- Duplicate "COMBINED FILTERS" examples (keep only 2)
- Redundant parameter descriptions
- Over 10 lines on any single feature
- "PAGINATION" more than 2 examples

### STEP 3: Delete prompt_review() and prompt_workflows()

**Action**: Remove entire function definitions

**prompt_review() [Lines 368-708]**
- Delete the entire function
- Rationale: USE CASE scenario (code review workflows) not tool feature
- This teaches how to use PRs in review context, not how to use the listing tool

**prompt_workflows() [Lines 710-938]**
- Delete the entire function
- Rationale: USE CASE scenario (PR management) not tool feature
- This teaches 10 workflows (standup, release, cleanup, etc.) not tool parameters

### STEP 4: Update Routing Logic

**Current routing (Lines 16-23):**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("filtering") => prompt_filtering(),
        Some("review") => prompt_review(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**New routing:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("filtering") => prompt_filtering(),
        _ => prompt_basic(),
    }
}
```

**Explanation:**
- Default to basic (most common use case)
- Only provide filtering when explicitly requested
- No comprehensive needed
- No review/workflows scenarios

### STEP 5: Update prompt_arguments() Description

**Current (Line 25-31):**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, filtering, review, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

**New:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, filtering)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Changes:**
- Remove "review, workflows" from description
- Keep only "basic, filtering"

### STEP 6: Verify Final Structure

After completing steps 1-5, verify:
- Line count: `wc -l prompts.rs` should show 170-220 lines
- Scenario count: `grep "^fn prompt_" prompts.rs` should return exactly 2 functions
- Check routing: Only "basic" and "filtering" handled

---

## Code Pattern Examples

### Example 1: Basic Trimming - Required Parameters

**Before (verbose):**
```rust
REQUIRED PARAMETERS:
- owner: Repository owner (username or organization)
  The owner can be either a GitHub user handle like "torvalds" or an organization name like "rust-lang".
  Examples:
  1. Personal repo: owner is your username
  2. Org repo: owner is the organization name
  3. Fork: owner is your username

- repo: Repository name
  The repository name (not including the owner). For example, "tokio" not "tokio-rs/tokio".
```

**After (concise):**
```rust
REQUIRED PARAMETERS:
- owner: Repository owner (username or organization)
- repo: Repository name
```

### Example 2: Filtering Consolidation

**Before (multiple sections):**
```rust
FILTERING BY BASE BRANCH:
(section with 15 lines)

FILTERING BY HEAD BRANCH:
(section with 10 lines)
```

**After (combined):**
```rust
FILTERING BY BRANCH:
Base branch (target): github_list_pull_requests({"base": "main"})
Head branch (source): github_list_pull_requests({"head": "user:feature"})
Format head as "username:branch"
```

### Example 3: Pagination Simplification

**Before (16 lines):**
```rust
PAGINATION:
1. First page (default):
   github_list_pull_requests({"owner": "user", "repo": "project", "per_page": 50})
   Get up to 50 results

2. Second page:
   github_list_pull_requests({"owner": "user", "repo": "project", "page": 2, "per_page": 50})

3. Maximum results per page:
   github_list_pull_requests({"owner": "user", "repo": "project", "per_page": 100})
   GitHub max is 100 per page
```

**After (8 lines):**
```rust
PAGINATION:
Per page (default 30, max 100):
github_list_pull_requests({"per_page": 100})

Specific page:
github_list_pull_requests({"page": 2, "per_page": 100})
```

---

## Success Criteria - Definition of Done

Your work is complete when ALL of these are true:

1. **Line count**: `wc -l prompts.rs` = **170-220 lines** (not 938)

2. **Scenario functions**: Exactly 2 functions exist
   - `fn prompt_basic()`
   - `fn prompt_filtering()`
   - Verify with: `grep "^fn prompt_" prompts.rs` → 2 results only

3. **No delete remnants**: Zero functions named
   - `prompt_review()`
   - `prompt_workflows()`
   - `prompt_comprehensive()`

4. **Routing updated**: Match statement has exactly 2 arms
   ```rust
   match args.scenario.as_deref() {
       Some("filtering") => prompt_filtering(),
       _ => prompt_basic(),
   }
   ```

5. **Scenario description updated**: Line describing scenarios shows only "basic, filtering"
   - Search: `description: Some` → should mention only basic and filtering

6. **Content quality**:
   - No decorative `═══` or `-----` separators
   - No duplicate explanations (auth, response structure shown once)
   - Each parameter demonstrated with at least 1 example
   - Response structure shown once (in basic)

7. **Readability**: Any developer can understand all parameters in 2 minutes reading sequentially

8. **Tool parameters taught**:
   - Basic: owner, repo (required), state (optional, default open)
   - Filtering: state, base, head, sort, direction, page, per_page
   - All parameters have at least one example

---

## Validation Checklist

After completing the trimming, run these checks:

```bash
# Line count check
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/list_pull_requests/prompts.rs
# Expected: 170-220

# Scenario function check
grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/list_pull_requests/prompts.rs
# Expected: 2 results (basic, filtering)

# Verify no deleted functions remain
grep "prompt_review\|prompt_workflows\|prompt_comprehensive" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/list_pull_requests/prompts.rs
# Expected: 0 results

# Check routing logic
grep -A 5 "fn generate_prompts" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/list_pull_requests/prompts.rs
# Expected: match with only "filtering" and default case

# Decorative header check
grep "═════" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/list_pull_requests/prompts.rs
# Expected: 0 results
```

---

## Notes

- This tool lists pull requests - it's about DISCOVERY, not modification
- Keep focus on the parameters: owner, repo, state, base, head, sort, direction, page, per_page
- Review and Workflows scenarios teach USE CASES (how to organize team workflows) not TOOL USAGE
- Basic scenario: "How do I list?" → Focus on core usage
- Filtering scenario: "How do I filter/sort results?" → Focus on parameter combinations
- After this task, file should match PRECURSOR_02 template standard for Complexity 2 tools
