# TASK 053: Trim github_list_commits

**Tool**: `github_list_commits`
**Complexity**: 2 (Simple)
**Current size**: 843 lines
**Target size**: 170-220 lines
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/list_commits/prompts.rs`

---

## Current State Analysis

### File Structure
The prompts.rs file currently contains:
- **Lines 1-10**: Module header and imports
- **Lines 12-43**: `ListCommitsPrompts` struct and trait implementation
  - **Lines 18-27**: `generate_prompts()` match statement with 5 scenario cases
  - **Lines 29-35**: `prompt_arguments()` function defining available scenarios
- **Lines 37-43**: Comment header for helper functions
- **Lines 45-155**: `prompt_basic()` function (~100 lines, KEEP)
- **Lines 157-375**: `prompt_filtering()` function (~220 lines, KEEP)
- **Lines 377-555**: `prompt_branches()` function (~180 lines, DELETE)
- **Lines 557-770**: `prompt_workflows()` function (~215 lines, DELETE)
- **Lines 772-843**: `prompt_comprehensive()` function (~72+ lines, DELETE)

### Current Scenarios (5 total)
1. **"basic"** - Basic commit listing (how to list commits, pagination, response structure)
2. **"filtering"** - Filtering by author, path, date range (KEEP)
3. **"branches"** - Branch/tag selection and SHA parameter usage (DELETE - use-case scenario)
4. **"workflows"** - Analysis workflows and patterns (DELETE - comprehensive scenario)
5. **Default (None)** - Comprehensive complete guide (DELETE - comprehensive scenario)

### Scenario Descriptions
- **basic**: Covers simple commit listing, pagination parameters, response structure, basic use cases, and authentication. ~100 lines, focused and practical.
- **filtering**: Covers filtering by author, path, date with multiple combinations. ~220 lines, detailed filter documentation.
- **branches**: Advanced branch/tag selection (DELETE)
- **workflows**: Analysis patterns and complex use cases (DELETE)
- **comprehensive**: Complete 500+ line guide covering everything (DELETE)

---

## Implementation Instructions

### Step 1: Update Match Statement (Lines 18-27)
Remove branches and workflows cases, keep only basic and filtering:

**BEFORE:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("filtering") => prompt_filtering(),
        Some("branches") => prompt_branches(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("filtering") => prompt_filtering(),
        _ => prompt_basic(),
    }
}
```

### Step 2: Update Prompt Arguments Description (Line 31)
Update the scenario description to list only available scenarios:

**BEFORE:**
```rust
description: Some("Scenario to show (basic, filtering, branches, workflows)".to_string()),
```

**AFTER:**
```rust
description: Some("Scenario to show (basic, filtering)".to_string()),
```

### Step 3: Delete prompt_branches() Function (Lines 377-555)
This entire function must be deleted. It spans from the line `/// Branch-specific commits` comment through to just before the `/// Commit analysis workflows` comment.

The function contains:
- User question: "How do I list commits from specific branches or starting from a particular commit?"
- Detailed documentation of sha parameter usage
- Branch comparison workflows
- Practical use cases for branches/tags

### Step 4: Delete prompt_workflows() Function (Lines 557-770)
This entire function must be deleted. It spans from the line `/// Commit analysis workflows` comment through to just before the `/// Comprehensive guide to listing commits` comment.

The function contains:
- User question: "What are common workflows and patterns for analyzing commit history?"
- Changelog generation steps
- Release planning examples
- Contributor analysis patterns
- Code audit workflows
- Integration with other tools
- Analysis patterns and best practices

### Step 5: Delete prompt_comprehensive() Function (Lines 772-843)
This entire function must be deleted. It spans from the line `/// Comprehensive guide to listing commits` comment through to the end of the file.

The function contains:
- User question: "Give me a complete guide to using github_list_commits for all use cases."
- Extremely detailed documentation covering:
  - Overview section
  - Basic usage section
  - Parameters section
  - Filtering section with all filters
  - Branch and tag selection section
  - Response structure section
  - Pagination section
  - Authentication section
  - Common workflows section
  - Error handling section
  - Best practices section
  - Quick reference section

---

## Step-by-Step Editing Process

### Edit 1: Simplify Match Statement
Location: Lines 18-27
Action: Replace the entire match statement block with the simplified version keeping only "basic" and "filtering" cases, with "basic" as default.

```rust
// EXACT REPLACEMENT TEXT:
    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("filtering") => prompt_filtering(),
            _ => prompt_basic(),
        }
    }
```

### Edit 2: Update Description String
Location: Line 31 (in prompt_arguments function)
Action: Change description text to only mention basic and filtering:

Old string: `"Scenario to show (basic, filtering, branches, workflows)"`
New string: `"Scenario to show (basic, filtering)"`

### Edit 3: Delete Decorative Header Comment
Location: Line 43
Action: Delete the entire header comment block:
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO LIST COMMITS
// ============================================================================
```

This is a decorative header that adds 3 lines. Deleting it helps reach target size.

### Edit 4: Delete prompt_branches() Function
Location: Lines 377-555 (approximately)
Action: Delete entire function including:
- `/// Branch-specific commits` doc comment
- `fn prompt_branches() -> Vec<PromptMessage> {` through closing `}`
- All content between these markers

Search for: `/// Branch-specific commits`
Delete everything until you find: `/// Commit analysis workflows`

### Edit 5: Delete prompt_workflows() Function
Location: Lines 557-770 (approximately, after deletions will shift)
Action: Delete entire function including:
- `/// Commit analysis workflows` doc comment
- `fn prompt_workflows() -> Vec<PromptMessage> {` through closing `}`
- All content between these markers

Search for: `/// Commit analysis workflows`
Delete everything until you find: `/// Comprehensive guide to listing commits`

### Edit 6: Delete prompt_comprehensive() Function
Location: Lines 772-843 (approximately, after deletions will shift)
Action: Delete entire function including:
- `/// Comprehensive guide to listing commits` doc comment
- `fn prompt_comprehensive() -> Vec<PromptMessage> {` through final closing `}`
- All content until end of file

Search for: `/// Comprehensive guide to listing commits`
Delete everything until end of file (line 843).

---

## Expected Result After Trimming

The file will contain:
- 43 lines: Struct definition and trait impl with simplified match statement
- 100 lines: prompt_basic() function (unchanged)
- 220 lines: prompt_filtering() function (unchanged)
- **Total: ~363 lines** (estimated)

Wait - this is still larger than 220 target. This means we need to ALSO trim the basic and filtering content themselves. Let me reconsider:

### Content Trimming for basic and filtering

The task description states:
- Basic scenario: 80-120 lines
- Filtering scenario: 70-100 lines
- Total target: 170-220 lines

Current content is oversized. Each prompt_xxx() function returns a Vec with 2 PromptMessage objects (User + Assistant).

**For prompt_basic()**: Currently ~100 lines. Trim to 80-90 lines by:
- Remove the "COMMON USE CASES" section (3 examples with descriptions, ~8 lines)
- Consolidate "AUTHENTICATION" subsection (currently 4 lines, keep as is)
- Consolidate "BEST PRACTICES" to 3 items instead of 5 (~3 lines saved)

**For prompt_filtering()**: Currently ~220 lines. Trim to 90-100 lines by:
- Remove "COMBINING FILTERS" subsection (shows author+path, author+date, path+date, all filters examples, ~40 lines)
- Condense "FILTER OPTIONS" to essential info only (~3 lines per filter instead of 5)
- Remove "PRACTICAL EXAMPLES" section entirely (4 examples, ~20 lines)
- Keep: Direct filtering examples and brief explanations

After content trim + function deletions:
- prompt_basic(): ~85 lines
- prompt_filtering(): ~95 lines  
- Struct + impl: ~43 lines
- **Total: ~223 lines** (within 170-220 range)

---

## Specific Content Removals

### From prompt_basic() - Remove Section:
Lines in the Assistant content string that contain:
```
COMMON USE CASES:
1. Recent activity check:...
2. Full commit history:...
3. Changelog generation:...
```
This is approximately 8 lines that can be removed.

### From prompt_basic() - Condense:
"BEST PRACTICES" currently has 5 bullet points. Reduce to 3 most essential:
- Use per_page: 100 to reduce API calls
- Check count field to know how many commits returned
- Use sha for unique identification

Remove:
- Parse date field for temporal analysis
- Combine with github_get_commit for detailed info

### From prompt_filtering() - Remove Section:
Entire "COMBINING FILTERS:" subsection (lines showing author+path, author+date, etc.)
This removes approximately 40 lines.

### From prompt_filtering() - Remove Section:
Entire "PRACTICAL EXAMPLES:" subsection (4 examples like "Find who touched a critical file", etc.)
This removes approximately 20 lines.

### From prompt_filtering() - Condense:
"FILTER OPTIONS" section - currently provides 15+ lines of explanation per filter. Reduce to 3-4 lines per filter.

---

## Success Criteria

All of the following must be true:

- ✓ **File size**: 170-220 total lines (use `wc -l` to verify)
- ✓ **Scenario count**: Exactly 2 scenarios (basic, filtering)
- ✓ **Match statement**: Only has Some("basic") and Some("filtering") arms, with basic as default
- ✓ **Prompt arguments**: Description string shows only "basic, filtering"
- ✓ **No branches function**: prompt_branches() completely deleted
- ✓ **No workflows function**: prompt_workflows() completely deleted
- ✓ **No comprehensive function**: prompt_comprehensive() completely deleted
- ✓ **No decorative headers**: Comments like "HELPER FUNCTIONS - TEACH..." removed
- ✓ **Content trimmed**: Both basic and filtering functions have unnecessary sections removed
- ✓ **Rust syntax valid**: File compiles with `cargo check`
- ✓ **No function calls remain**: No references to deleted functions anywhere in the file
- ✓ **Both scenarios functional**: Each returns Vec<PromptMessage> with User and Assistant content

---

## Definition of Done

The task is complete when:

1. **File compiles**: Running `cargo check` in the packages/kodegen-mcp-schema directory passes with no errors
2. **Line count is correct**: `wc -l prompts.rs` returns between 170-220 lines
3. **Routing works**: The match statement in generate_prompts() correctly routes to basic and filtering, with basic as fallback
4. **Only 2 scenarios**: PromptProvider implementation only defines 2 scenarios total
5. **Content removed**: No references to "branches", "workflows", or "comprehensive" in function names or routing
6. **Quality maintained**: Both remaining scenarios are complete and provide value without redundancy
