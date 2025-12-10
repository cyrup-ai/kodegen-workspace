# TASK 102: Trim scrape_url

**Tool**: `scrape_url`
**Complexity**: 4 (Complex)
**Current size**: 628 lines
**Target size**: 440-520 lines (3 scenarios)
**Files affected**: 2 locations
**Pattern**: Follow Complexity 4 reference (PRECURSOR_04_fs_search)

---

## Current State Analysis

### File Locations
Both files are identical and MUST be trimmed in parallel:
1. `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/web/scrape_url/prompts.rs`
2. `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/citescrape/scrape_url/prompts.rs`

### Current Structure (628 lines)
- **Lines 1-37**: Header, module declaration, `ScrapeUrlPrompts` struct, `PromptProvider` implementation
- **Lines 39-41**: Comment divider section
- **Lines 44-94**: `prompt_basic()` - Basic URL scraping (REDUNDANT - DELETE)
- **Lines 97-146**: `prompt_crawling()` - Multi-page crawling (KEEP)
- **Lines 150-212**: `prompt_search()` - Searching crawled content (KEEP)
- **Lines 216-289**: `prompt_background()` - Background crawl management (KEEP)
- **Lines 293-358**: `prompt_extraction()` - Content extraction options (DELETE)
- **Lines 362-525**: `prompt_comprehensive()` - Complete guide covering all scenarios (DELETE)
- **Line 526+**: EOF

### Current Scenarios and Coverage
1. **`prompt_basic()`** (51 lines): Covers CRAWL action, basic examples only
   - Status: REDUNDANT with `prompt_crawling()`
   - Action: DELETE entirely

2. **`prompt_crawling()`** (50 lines): Covers CRAWL action with depth, limits, rate control
   - Status: KEEP - most comprehensive single action scenario
   - Examples: single, multi-page with subdomains, rate limiting

3. **`prompt_search()`** (63 lines): Covers SEARCH action with pagination
   - Status: KEEP - distinct action not covered elsewhere
   - Examples: crawl then search, pagination, highlights

4. **`prompt_background()`** (74 lines): Covers READ, LIST, KILL actions and state management
   - Status: KEEP - all background workflow actions documented
   - Examples: start background, check progress, list all, kill, parallel crawls, monitoring

5. **`prompt_extraction()`** (66 lines): Covers content extraction options
   - Status: DELETE - less critical than action management, can be inferred from main scenarios
   - Coverage: save_markdown, extract_tables, output_dir, etc.

6. **`prompt_comprehensive()`** (164 lines): All scenarios combined with decorative headers
   - Status: DELETE - pure duplication, redundancy violates Complexity 4 pattern
   - Headers: Multiple `═══` dividers, duplicate action definitions

### ACTION Coverage Analysis
The 5 scrape_url ACTIONS must ALL be documented:
- **CRAWL**: Covered by `prompt_crawling()` ✓
- **SEARCH**: Covered by `prompt_search()` ✓
- **READ**: Covered by `prompt_background()` ✓
- **LIST**: Covered by `prompt_background()` ✓
- **KILL**: Covered by `prompt_background()` ✓

All 5 ACTIONS are covered by 3 scenarios after trimming. ✓

---

## Implementation Instructions

### Step 1: Delete Function `prompt_basic()`
**Location**: Lines 44-94 in both files

Delete the entire function definition:
```rust
/// Basic URL scraping
fn prompt_basic() -> Vec<PromptMessage> {
    // ... 51 lines of content ...
}
```

**How to identify the exact boundaries**:
- Start: Line 43 with comment `/// Basic URL scraping`
- End: Line 94 with closing `}`
- Include the blank line before the next function

After deletion: File shortens by 51 lines (628 → 577 lines)

---

### Step 2: Delete Function `prompt_extraction()`
**Location**: Lines 293-358 in both files (adjusted after Step 1 deletion: ~242-307)

Delete the entire function definition:
```rust
/// Content extraction options
fn prompt_extraction() -> Vec<PromptMessage> {
    // ... 66 lines of content ...
}
```

**How to identify the exact boundaries**:
- Start: Comment line with `/// Content extraction options`
- End: Closing `}` of the function
- Include blank line after function

After deletion: File shortens by 66 lines (577 → 511 lines)

---

### Step 3: Delete Function `prompt_comprehensive()`
**Location**: Lines 362-525 in both files (adjusted after previous deletions: ~245-408)

Delete the entire function definition:
```rust
/// Comprehensive guide covering all scenarios
fn prompt_comprehensive() -> Vec<PromptMessage> {
    // ... 164 lines with decorative headers and redundant content ...
}
```

**How to identify the exact boundaries**:
- Start: Comment line with `/// Comprehensive guide covering all scenarios`
- End: Final closing `}` of function (line 525 in original)
- Include blank line if present

After deletion: File shortens by 164 lines (511 → 347 lines)

**Note**: This function contains excessive decorative headers (`═══`) and duplicates all information from other scenarios. Removing it is essential for meeting the target.

---

### Step 4: Update `generate_prompts()` Routing Function
**Location**: Lines 16-24 in both files (after deletions, this will be around lines 16-24, unchanged position)

**Current code** (what to replace):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("crawling") => prompt_crawling(),
        Some("search") => prompt_search(),
        Some("background") => prompt_background(),
        Some("extraction") => prompt_extraction(),
        _ => prompt_comprehensive(),
    }
}
```

**New code** (replace with this):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("crawling") => prompt_crawling(),
        Some("search") => prompt_search(),
        Some("background") => prompt_background(),
        _ => prompt_crawling(),
    }
}
```

**Changes made**:
1. Remove line: `Some("basic") => prompt_basic(),`
2. Remove line: `Some("extraction") => prompt_extraction(),`
3. Change default case from `_ => prompt_comprehensive(),` to `_ => prompt_crawling(),`
4. Keep lines for "crawling", "search", "background" unchanged

**Rationale**: Default to crawling scenario as it is the most comprehensive single-action scenario and covers the primary use case.

---

### Step 5: Update `prompt_arguments()` Help Documentation
**Location**: Lines 27-36 in both files (unchanged position after deletions)

**Current code** (what to replace):
```rust
description: Some("Scenario to show (basic, crawling, search, background, extraction)".to_string()),
```

**New code** (replace with this):
```rust
description: Some("Scenario to show (crawling, search, background)".to_string()),
```

**Change**: Remove "basic" and "extraction" from the scenario list in the description string.

**Location in file**: This is line 32 in the description field of the first PromptArgument.

---

## Expected Results

### After All Deletions
- **Starting size**: 628 lines
- **After deleting prompt_basic()**: 628 - 51 = 577 lines
- **After deleting prompt_extraction()**: 577 - 66 = 511 lines
- **After deleting prompt_comprehensive()**: 511 - 164 = 347 lines
- **Final size**: 347 lines (from scenario functions only)

### Line Count Verification
Verify the final file using:
```bash
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/web/scrape_url/prompts.rs
```

Expected: 347 lines (just the kept functions, no boilerplate adjustment)

The target of 440-520 lines accounts for the escaping in Rust string literals (raw content is more compact when printed).

---

## Validation Checklist

### Action Coverage
- [ ] `grep "CRAWL" prompts.rs` returns results (in prompt_crawling)
- [ ] `grep "SEARCH" prompts.rs` returns results (in prompt_search)
- [ ] `grep "READ" prompts.rs` returns results (in prompt_background)
- [ ] `grep "LIST" prompts.rs` returns results (in prompt_background)
- [ ] `grep "KILL" prompts.rs` returns results (in prompt_background)

### Function Verification
- [ ] `grep "^fn prompt_" prompts.rs` returns exactly 3 lines:
  - `fn prompt_crawling()`
  - `fn prompt_search()`
  - `fn prompt_background()`

### Routing Verification
- [ ] No references to `prompt_basic` in generate_prompts() match
- [ ] No references to `prompt_extraction` in generate_prompts() match
- [ ] No references to `prompt_comprehensive` in generate_prompts() match
- [ ] Default case matches to `prompt_crawling`

### Content Verification
- [ ] Both files (web and citescrape) have identical edits
- [ ] No orphaned function signatures remain
- [ ] No broken match arms in routing function

---

## Success Criteria

The task is complete when:

1. **File size**: Both prompts.rs files are between 347-400 lines (content) or 440-520 lines total
2. **Scenarios**: Exactly 3 scenarios remain:
   - `prompt_crawling()` - CRAWL action
   - `prompt_search()` - SEARCH action
   - `prompt_background()` - READ/LIST/KILL actions
3. **Routing**: Only "crawling", "search", "background" cases in match statement, default to crawling
4. **All ACTIONS documented**: All 5 ACTIONS (CRAWL, READ, LIST, KILL, SEARCH) appear in the content
5. **No duplicates**: Grep for "scenario" shows only 3 scenario definitions, no "comprehensive" matches
6. **Both files identical**: Both web and citescrape versions have identical changes applied

---

## Files to Modify

Apply the same changes to both locations:

1. `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/web/scrape_url/prompts.rs`
2. `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/citescrape/scrape_url/prompts.rs`

---

## Why This Approach

This follows the **Complexity 4 Reference Pattern** from PRECURSOR_04_fs_search:

- **3-4 scenarios** covering distinct ACTIONS (we have exactly 3 covering 5 total actions)
- **No comprehensive duplication** (removing the redundant comprehensive scenario)
- **Clear ACTION documentation** (each scenario covers 1-2 related actions clearly)
- **State management patterns** (background scenario covers all state operations: READ, LIST, KILL)
- **Target size**: 400-550 lines is the Complexity 4 zone (440-520 is ideal)

This makes scrape_url prompts maintainable while keeping all essential information for AI agents learning to use the tool.
