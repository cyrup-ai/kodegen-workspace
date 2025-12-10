# TASK 094: Trim github_search_issues Prompts

**Tool**: `github_search_issues`
**Package**: `kodegen-mcp-schema`
**Complexity**: 3 (Medium)
**Current size**: 839 lines
**Target size**: 320-340 lines
**Scenarios**: Reduce from 5 to 3
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/search_issues/prompts.rs`

---

## Current State Analysis

### File Structure
The prompts.rs file implements the `PromptProvider` trait for the `github_search_issues` tool. It contains:

**Lines 1-36**: Module documentation, imports, and SearchIssuesPrompts struct
- `use crate::tool::PromptProvider;` - Imports the sealed trait
- `pub struct SearchIssuesPrompts;` - Unit struct implementing the trait
- `impl PromptProvider for SearchIssuesPrompts` - Contains routing logic

**Lines 26-35**: `prompt_arguments()` method
- Current description: `"Scenario to show (basic, syntax, patterns, workflows)"`
- Must be updated to: `"Scenario to show (basic, syntax, patterns)"`

**Lines 16-24**: `generate_prompts()` match statement
- Current matches: `Some("basic")`, `Some("syntax")`, `Some("patterns")`, `Some("workflows")`, and `_ => prompt_comprehensive()`
- Must be updated to only match: `Some("basic")`, `Some("syntax")`, `Some("patterns")`, and `_ => prompt_basic()`

### Current Scenarios (5 total)

**1. prompt_basic() [Lines 43-125, ~83 lines]**
- Teaches fundamental usage: keyword search, repository filtering, state filters, sorting, pagination
- Response structure with example JSON
- Common basic search patterns (4 examples)
- Parameters documentation
- Authentication and pagination info
- Best practices (5 points)
- **ACTION: KEEP - No changes**

**2. prompt_syntax() [Lines 128-242, ~115 lines]**
- Complete reference of all GitHub search qualifiers
- Organized into categories: TYPE, STATE, REPOSITORY, AUTHOR & ASSIGNEE, LABEL, DATE, METADATA, MILESTONE & PROJECT, STATUS CHECKS, REVIEW STATUS, TEXT SEARCH, SEARCH IN SPECIFIC FIELDS
- Examples combining qualifiers (7 real-world query examples)
- Tips for effective searching (6 points)
- **ACTION: KEEP - No changes**

**3. prompt_patterns() [Lines 245-407, ~163 lines]**
- Current organization: 20+ numbered search patterns across 9 sections
  - Section 1: TRIAGE PATTERNS (3 patterns: unlabeled, unassigned bugs, no comments)
  - Section 2: PRIORITY MANAGEMENT (3 patterns: high-priority, critical bugs, ready for sprint)
  - Section 3: STALE CONTENT IDENTIFICATION (2 patterns: old issues, stale PRs)
  - Section 4: CONTRIBUTOR MANAGEMENT (3 patterns: good first issues, help wanted, contributions)
  - Section 5: PERSONAL WORKFLOW (5 patterns: open issues, assigned work, open PRs, review requests, involved)
  - Section 6: QUALITY ASSURANCE (3 patterns: failed CI, needs changes, approved/unmerged)
  - Section 7: DUPLICATE DETECTION (1 pattern: similar issues)
  - Section 8: CROSS-REPO PATTERNS (2 patterns: org-wide bugs, security issues)
  - Section 9: TIPS FOR EFFECTIVE PATTERNS (6 tips)
- **ACTION: TRIM from ~163 to ~100-120 lines**
- **Trimming plan**:
  - Keep Section 1 (TRIAGE PATTERNS, patterns 1-3): ~30 lines
  - Keep only first pattern from Section 2 (high-priority): ~15 lines
  - Delete Section 3 (STALE CONTENT IDENTIFICATION): 2 patterns
  - Delete Section 4 (CONTRIBUTOR MANAGEMENT): 3 patterns
  - Keep 2 patterns from Section 5 (PERSONAL WORKFLOW): patterns 13 (assigned work) and 15 (review requests): ~20 lines
  - Delete Section 6 (QUALITY ASSURANCE): 3 patterns
  - Delete Section 7 (DUPLICATE DETECTION): 1 pattern
  - Delete Section 8 (CROSS-REPO PATTERNS): 2 patterns
  - Keep Section 9 (TIPS): ~10 lines
  - Keep user/assistant prompt structure: ~20 lines

**4. prompt_workflows() [Lines 410-593, ~184 lines]**
- 8 complete workflow scenarios with multi-step instructions
- Workflows: Daily Triage, Sprint Planning, Before Creating Issue, Release Planning, Code Review Management, Stale Content Cleanup, Contributor Onboarding, Security Response
- **ACTION: DELETE ENTIRELY** - Redundant with patterns; workflows can be inferred from syntax + patterns

**5. prompt_comprehensive() [Lines 596-838, ~243 lines]**
- Duplicates content from all other scenarios in one massive reference
- Sections: Tool Overview, Basic Usage, Parameters, Search Qualifiers (complete), Response Structure, Common Patterns, Authentication/Rate Limits, Best Practices, Error Handling, Advanced Tips
- **ACTION: DELETE ENTIRELY** - Comprehensive view created by combining specific scenarios makes this redundant

### Current Line Distribution
- Header/struct/trait: 36 lines
- Helper function section header: 4 lines
- prompt_basic(): 83 lines
- prompt_syntax(): 115 lines
- prompt_patterns(): 163 lines
- prompt_workflows(): 184 lines
- prompt_comprehensive(): 243 lines
- **Total: 839 lines**

### Target State
- Header/struct/trait: 36 lines
- Helper function section header: 4 lines
- prompt_basic(): 83 lines (unchanged)
- prompt_syntax(): 115 lines (unchanged)
- prompt_patterns(): 100-110 lines (trimmed)
- **Total target: 338-348 lines**

---

## Step 1: Update Routing Logic

**Location**: Lines 16-24 in `impl PromptProvider for SearchIssuesPrompts`

**Current code**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("syntax") => prompt_syntax(),
        Some("patterns") => prompt_patterns(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**New code**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("syntax") => prompt_syntax(),
        Some("patterns") => prompt_patterns(),
        _ => prompt_basic(),
    }
}
```

**Changes**:
- Remove line: `Some("workflows") => prompt_workflows(),`
- Change default case from `_ => prompt_comprehensive()` to `_ => prompt_basic()`

---

## Step 2: Update Prompt Arguments

**Location**: Lines 26-35 in `fn prompt_arguments()`

**Current code**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, syntax, patterns, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

**New code**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, syntax, patterns)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Changes**:
- Line 31: Remove `workflows` and `prompt_comprehensive` from description text

---

## Step 3: Keep Existing Scenarios

### Keep prompt_basic() [Lines 43-125]
Do NOT modify this function. It correctly demonstrates:
- Basic keyword search
- Repository-scoped search
- State filtering
- Sorting and pagination with parameters
- Response structure with JSON example
- Common search patterns (4 examples)
- Authentication info
- Pagination mechanics
- Best practices (5 items)

**Validation**: Function should remain at ~83 lines

### Keep prompt_syntax() [Lines 128-242]
Do NOT modify this function. It is the complete reference covering:
- All search qualifier categories (TYPE, STATE, REPOSITORY, AUTHOR & ASSIGNEE, LABEL, DATE, RELATIVE DATES, METADATA, MILESTONE & PROJECT, STATUS CHECKS, REVIEW STATUS, TEXT SEARCH, SEARCH IN SPECIFIC FIELDS)
- How to combine qualifiers
- 7 real-world example queries
- Tips for effective syntax usage

**Validation**: Function should remain at ~115 lines

---

## Step 4: Trim prompt_patterns() Function

**Location**: Lines 245-407

**Current structure**: 20 numbered patterns across 9 sections totaling ~163 lines

**New structure**: Keep only 8 core patterns across 3 main sections + tips, targeting ~100-110 lines

### Patterns to Keep

**SECTION 1: TRIAGE PATTERNS - Keep all 3 patterns**
```
1. Unlabeled issues (need triage)
2. Unassigned bugs
3. No comments (need attention)
```

**SECTION 2: PRIORITY MANAGEMENT - Keep only first pattern**
```
4. High-priority issues
(DELETE patterns 5-6: Critical bugs, Ready for sprint)
```

**SECTION 3: PERSONAL WORKFLOW - Keep 2 patterns**
```
13. Your assigned work (from PERSONAL WORKFLOW section)
15. PRs awaiting your review (from PERSONAL WORKFLOW section)
(DELETE patterns 12, 14: Your open issues, Your open PRs, Your involved issues)
```

**SECTION 4: TIPS FOR EFFECTIVE PATTERNS - Keep all content**
```
- Combine sort with patterns for prioritization
- Use date filters to focus on recent activity
- Layer multiple labels for precision
- Exclude labels with - to filter out categories
- Use reactions/comments for popularity signals
- Save common patterns as search bookmarks
```

### Patterns to Delete (Delete entire sections)

**DELETE SECTION: STALE CONTENT IDENTIFICATION (2 patterns)**
- Pattern 7: Old open issues
- Pattern 8: Stale PRs

**DELETE SECTION: CONTRIBUTOR MANAGEMENT (3 patterns)**
- Pattern 9: Good first issues
- Pattern 10: Help wanted
- Pattern 11: User's contributions

**DELETE SECTION: QUALITY ASSURANCE (3 patterns)**
- Pattern 17: Failed CI
- Pattern 18: Needs changes
- Pattern 19: Approved but not merged

**DELETE SECTION: DUPLICATE DETECTION (1 pattern)**
- Pattern 20: Similar issues

**DELETE SECTION: CROSS-REPO PATTERNS (2 patterns)**
- Pattern 21: Organization-wide bugs
- Pattern 22: Security issues across repos

### Implementation Details

**What to preserve in trimmed patterns function**:
1. Function signature: `fn prompt_basic() -> Vec<PromptMessage>`
2. PromptMessage with User role (question)
3. PromptMessage with Assistant role (answer) containing:
   - Header: "Here are proven search patterns for common GitHub issue management tasks:\n\n"
   - TRIAGE PATTERNS section (patterns 1-3, ~30 lines)
   - PRIORITY MANAGEMENT section with only pattern 4 (~15 lines)
   - PERSONAL WORKFLOW section with only patterns 13 and 15 (~20 lines)
   - TIPS FOR EFFECTIVE PATTERNS section (~10 lines)

**Exact patterns to keep** (extract from existing file):

Pattern 1 (Unlabeled issues):
```
1. Unlabeled issues (need triage):
   github_search_issues({
     "query": "is:issue is:open no:label repo:user/project"
   })
   // Find issues that need categorization
```

Pattern 2 (Unassigned bugs):
```
2. Unassigned bugs:
   github_search_issues({
     "query": "is:issue is:open label:bug no:assignee repo:user/project"
   })
   // Find bugs ready for assignment
```

Pattern 3 (No comments):
```
3. No comments (need attention):
   github_search_issues({
     "query": "is:issue is:open comments:0 created:>1week repo:user/project"
   })
   // Find issues with no responses
```

Pattern 4 (High-priority issues):
```
4. High-priority issues:
   github_search_issues({
     "query": "is:issue is:open label:priority-high repo:user/project",
     "sort": "created",
     "order": "asc"
   })
   // Oldest high-priority issues first
```

Pattern 13 (Your assigned work):
```
13. Your assigned work:
    github_search_issues({
      "query": "is:issue is:open assignee:your-username",
      "sort": "updated",
      "order": "desc"
    })
    // Your assigned issues
```

Pattern 15 (PRs awaiting your review):
```
15. PRs awaiting your review:
    github_search_issues({
      "query": "is:pr is:open review-requested:your-username",
      "sort": "created",
      "order": "asc"
    })
    // Oldest review requests first
```

---

## Step 5: Delete prompt_workflows() Function Entirely

**Location**: Lines 410-593 (~184 lines)

**Action**: Remove the entire function definition:
```rust
/// Search workflows and integration examples
fn prompt_workflows() -> Vec<PromptMessage> {
    // ... 184 lines of content ...
}
```

This function contains 8 workflow scenarios (Daily Triage, Sprint Planning, Before Creating Issue, Release Planning, Code Review Management, Stale Content Cleanup, Contributor Onboarding, Security Response) that are redundant with the combination of basic + syntax + patterns scenarios.

---

## Step 6: Delete prompt_comprehensive() Function Entirely

**Location**: Lines 596-838 (~243 lines)

**Action**: Remove the entire function definition:
```rust
/// Comprehensive guide covering all features
fn prompt_comprehensive() -> Vec<PromptMessage> {
    // ... 243 lines of content ...
}
```

This function duplicates all content from other scenarios in a single comprehensive reference. It is not needed when the specific scenarios are available.

---

## Validation Checklist

After making all changes, verify:

1. **Routing Logic**:
   - [ ] `match args.scenario.as_deref()` contains only 3 Some arms: "basic", "syntax", "patterns"
   - [ ] Default case calls `prompt_basic()` not `prompt_comprehensive()`

2. **Prompt Arguments**:
   - [ ] Description string is `"Scenario to show (basic, syntax, patterns)"`
   - [ ] No mention of "workflows" or "comprehensive"

3. **File Structure**:
   - [ ] Lines 1-40: Header, imports, SearchIssuesPrompts struct (unchanged)
   - [ ] Lines 42-125: prompt_basic() function (~83 lines)
   - [ ] Lines 128-242: prompt_syntax() function (~115 lines)
   - [ ] Lines 245-360: prompt_patterns() function (~110-115 lines, trimmed)
   - [ ] No prompt_workflows() function
   - [ ] No prompt_comprehensive() function

4. **Function Counts**:
   - [ ] Exactly 3 scenario functions: prompt_basic, prompt_syntax, prompt_patterns
   - [ ] All other functions removed

5. **Total Line Count**:
   - [ ] File is 330-350 lines total (was 839 lines)
   - [ ] Reduction of approximately 500+ lines achieved

6. **Content Validation**:
   - [ ] prompt_patterns() still has all 6 tips at the end
   - [ ] prompt_basic() unchanged with 4 common search examples
   - [ ] prompt_syntax() unchanged with all qualifier categories and 7 example queries

---

## Definition of Done

Success when:
1. File compiles with `cargo check` in kodegen-mcp-schema package
2. File passes `cargo clippy` with no warnings
3. Total line count is 330-350 lines
4. Exactly 3 scenarios available (basic, syntax, patterns)
5. All removed functions are deleted (no commented-out code)
6. Routing logic defaults to `prompt_basic()` not `prompt_comprehensive()`
7. prompt_arguments description updated to reflect only 3 scenarios

---

## Code Patterns

### Before: Full routing with 5 scenarios
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("syntax") => prompt_syntax(),
        Some("patterns") => prompt_patterns(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

### After: Trimmed routing with 3 scenarios
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("syntax") => prompt_syntax(),
        Some("patterns") => prompt_patterns(),
        _ => prompt_basic(),
    }
}
```

### Size reduction example
**Before prompt_patterns()**: ~163 lines (20 patterns in 9 sections)
```
- TRIAGE PATTERNS: 3 patterns (~30 lines)
- PRIORITY MANAGEMENT: 3 patterns (~25 lines) -> DELETE 2, keep 1
- STALE CONTENT: 2 patterns (~20 lines) -> DELETE ALL
- CONTRIBUTOR: 3 patterns (~25 lines) -> DELETE ALL
- PERSONAL WORKFLOW: 5 patterns (~35 lines) -> DELETE 3, keep 2
- QUALITY ASSURANCE: 3 patterns (~25 lines) -> DELETE ALL
- DUPLICATE DETECTION: 1 pattern (~10 lines) -> DELETE
- CROSS-REPO: 2 patterns (~20 lines) -> DELETE ALL
- TIPS: 6 tips (~10 lines) -> KEEP
```

**After prompt_patterns()**: ~110 lines (8 patterns in 3 sections + tips)
```
- TRIAGE PATTERNS: 3 patterns (~30 lines)
- PRIORITY MANAGEMENT: 1 pattern (~15 lines)
- PERSONAL WORKFLOW: 2 patterns (~20 lines)
- TIPS: 6 tips (~10 lines)
- Structure overhead (~35 lines for function signature, user prompt, etc)
```

---

## Reference Materials

**Package**: kodegen-mcp-schema
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/search_issues/prompts.rs`
**Related files**:
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/search_issues/prompt_args.rs` (defines SearchIssuesPromptArgs)
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/tool.rs` (PromptProvider trait definition)

**Similar completed tasks**: PRECURSOR_03_git_branch_create.md (Complexity 3 reference)

**Build verification**:
```bash
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
cargo check
cargo clippy
```
