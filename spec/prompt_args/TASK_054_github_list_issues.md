# TASK 054: Trim github_list_issues

**Tool**: `github_list_issues`
**Complexity**: 2 (Simple)
**Current size**: 933 lines
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/list_issues/prompts.rs`

---

## Current State Analysis

The prompts.rs file currently contains **5 scenario functions** and a routing system:

### Existing Scenarios (Lines 44-933)
1. **prompt_basic()** (lines 44-117, ~74 lines)
   - User question: "How do I list issues from a GitHub repository?"
   - Content: Basic listing examples, required/optional parameters, response structure, authentication, common patterns
   - Status: KEEP

2. **prompt_filtering()** (lines 119-348, ~230 lines)
   - User question: "How can I filter issues by labels, assignees, and other criteria?"
   - Content: Filter by state, labels, assignee, creator, mentioned, milestone, date; combining filters; special values; best practices
   - Status: KEEP but TRIM to ~100 lines (reduce from 230 lines by removing advanced filters)

3. **prompt_sorting()** (lines 350-530, ~180 lines)
   - User question: "How do I sort issues by creation date, update time, or comment count?"
   - Content: Sort options, direction, combining with filters, best practices
   - Status: DELETE (use-case scenario)

4. **prompt_workflows()** (lines 532-730, ~198 lines)
   - User question: "What are common workflows for managing issues with github_list_issues?"
   - Content: 10 workflows (triage, sprint planning, assigned work, bug tracking, etc.)
   - Status: DELETE (use-case scenario)

5. **prompt_comprehensive()** (lines 732-933, ~201 lines)
   - Default case answering complete guide question
   - Content: All parameters, all filtering, sorting, pagination, response structure, authentication, workflows, error handling, best practices
   - Status: DELETE (comprehensive scenario not needed)

### Routing Logic (Lines 13-21)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("filtering") => prompt_filtering(),
        Some("sorting") => prompt_sorting(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

Current description on line 27: "Scenario to show (basic, filtering, sorting, workflows)"

---

## Implementation Steps

### Step 1: Trim prompt_filtering() (Lines 119-348)

Keep ONLY state and label filtering sections. Remove assignee, creator, mentioned, milestone, date, and combining filters sections.

**Keep these sections from prompt_filtering():**
- Filter by state (shows open, closed, all)
- Filter by labels (single label, multiple labels, combined with state)
- Combined filter section (just the first example)
- Label syntax explanation
- Basic best practices

**Delete these sections from prompt_filtering():**
- Filter by assignee (entire section with none and * examples) - delete ~60 lines
- Filter by creator (entire section) - delete ~15 lines
- Filter by mentioned user (entire section) - delete ~10 lines
- Filter by milestone (entire section with all variants) - delete ~25 lines
- Filter by date (entire section) - delete ~10 lines
- "Combining filters" section (keep only bug triage example, delete sprint planning and recent activity) - delete ~35 lines
- "Special values" section - delete ~5 lines
- Advanced best practices about mentioned, milestone, creator - delete ~5 lines

**Result:** Reduce from 230 lines to approximately 85-100 lines

### Step 2: Delete prompt_sorting() Function (Lines 350-530)

Delete the entire function including:
- Function signature: `fn prompt_sorting() -> Vec<PromptMessage> {`
- Both PromptMessage items (user and assistant content)
- Closing brace: `}`

This removes approximately 180 lines.

### Step 3: Delete prompt_workflows() Function (Lines 532-730)

Delete the entire function including:
- Function signature: `fn prompt_workflows() -> Vec<PromptMessage> {`
- Both PromptMessage items containing all 10 workflows
- Closing brace: `}`

This removes approximately 198 lines.

### Step 4: Delete prompt_comprehensive() Function (Lines 732-933)

Delete the entire function including:
- Function signature: `fn prompt_comprehensive() -> Vec<PromptMessage> {`
- Both PromptMessage items containing complete guide
- Closing brace: `}`

This removes approximately 201 lines.

### Step 5: Update Routing Match Statement (Lines 19-21)

Replace the current match statement:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("filtering") => prompt_filtering(),
    Some("sorting") => prompt_sorting(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

With the new match statement:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("filtering") => prompt_filtering(),
    _ => prompt_basic(),
}
```

This removes 2 match arms and changes default from comprehensive to basic.

### Step 6: Update prompt_arguments() Description (Line 27)

Change line 27 from:
```rust
description: Some("Scenario to show (basic, filtering, sorting, workflows)".to_string()),
```

To:
```rust
description: Some("Scenario to show (basic, filtering)".to_string()),
```

### Step 7: Remove Comment Header (Line 33)

The comment on line 33 reads:
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO LIST GITHUB ISSUES
// ============================================================================
```

Delete this decorative header (3 lines).

---

## Code Patterns: Before and After

### Before: Routing (Lines 14-21)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("filtering") => prompt_filtering(),
        Some("sorting") => prompt_sorting(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

### After: Routing
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("filtering") => prompt_filtering(),
        _ => prompt_basic(),
    }
}
```

### Before: Argument Description (Line 27)
```rust
description: Some("Scenario to show (basic, filtering, sorting, workflows)".to_string()),
```

### After: Argument Description
```rust
description: Some("Scenario to show (basic, filtering)".to_string()),
```

---

## Specific Sections to Delete from prompt_filtering()

The prompt_filtering() function assistant content contains these deletable sections (within the single large string):

1. **"FILTER BY ASSIGNEE" section** (~60 lines)
   - Start: Line with "FILTER BY ASSIGNEE:"
   - End: Line with example github_list_issues({... "assignee": "*"})
   - Delete everything from "FILTER BY ASSIGNEE" through "Issues with any assignee" examples

2. **"FILTER BY CREATOR" section** (~15 lines)
   - Start: "FILTER BY CREATOR:"
   - End: Combined with labels example

3. **"FILTER BY MENTIONED USER" section** (~10 lines)
   - Single section with "mentioned" examples

4. **"FILTER BY MILESTONE" section** (~25 lines)
   - Start: "FILTER BY MILESTONE:"
   - End: None, wildcard, none examples

5. **"FILTER BY DATE" section** (~10 lines)
   - Start: "FILTER BY DATE:"
   - End: since parameter example

6. **"COMBINING FILTERS" section - PARTIALLY DELETE** (~35 lines to delete)
   - Keep: Bug triage example only
   - Delete: Sprint planning example, Recent activity example

7. **"SPECIAL VALUES" section** (~5 lines)
   - Start: "SPECIAL VALUES:"
   - End: "- milestone=\"*\": Any milestone"

8. **Advanced "BEST PRACTICES" entries** (~5 lines)
   - Delete: "- creator filter for tracking user submissions"
   - Delete any references to mentioned, milestone, assignee="none" beyond basic usage

---

## Success Criteria

**Line Count**: 170-220 lines total (currently 933)
- prompt_basic(): ~90 lines
- prompt_filtering() (trimmed): ~85-100 lines
- File overhead (header comments, struct, impl block, routing, args): ~25-30 lines
- Total: 200-220 lines

**Scenario Count**: Exactly 2 scenarios
- prompt_basic()
- prompt_filtering() (trimmed to state/label filtering only)

**Routing**: Only matches "basic" and "filtering", defaults to "basic"

**No Comprehensive**: Delete prompt_comprehensive() entirely

**No Decorative Headers**: Remove comment line separator before helper functions

**No Use-Case Scenarios**: Delete prompt_sorting() and prompt_workflows() entirely

**Description Updated**: Line 27 must read "Scenario to show (basic, filtering)"

---

## Verification Steps

After completing all edits:

1. Count total lines: Should be 170-220 lines
2. Count function definitions: Should be exactly 2 (prompt_basic, prompt_filtering)
3. Check routing: Exactly 3 match arms (basic, filtering, default)
4. Search for "sorting": Should find 0 results
5. Search for "workflows": Should find 0 results
6. Search for "comprehensive": Should find 0 results (except in comments if any remain)
7. Verify prompt_filtering() still teaches state filtering and label filtering clearly
8. Ensure no orphaned closing braces or syntax errors
