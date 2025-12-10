# TASK 081: Trim git_log

**Tool**: `git_log`
**Complexity**: 3 (Medium)
**Current size**: 772 lines (5 scenarios)
**Target size**: 320 lines (3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/log/prompts.rs`

---

## Reference

See **PRECURSOR_03_git_branch_create.md** for Complexity 3 template structure and principles.

---

## Context

The git_log tool displays Git commit history with parameters: `path`, `max_count`, `skip`, `path_filter`. The 772-line prompt has legitimate complexity (multiple important parameters with pagination and filtering) but includes redundancy through a dedicated pagination scenario and a comprehensive scenario that duplicates focused content.

**Key parameters**:
- `path` (required): Path to Git repository
- `max_count` (optional): Limit number of commits returned
- `skip` (optional): Pagination offset
- `path_filter` (optional): Filter commits by file/directory path

---

## Current Scenario Analysis

**Current scenarios** (772 lines total):
1. `prompt_basic()` - 119 lines (lines 47-166) ← KEEP, TRIM
2. `prompt_filtering()` - 178 lines (lines 169-347) ← KEEP, TRIM
3. `prompt_pagination()` - 197 lines (lines 350-547) ← CONSOLIDATE INTO BASIC
4. `prompt_file_history()` - 238 lines (lines 550-788) ← KEEP AS 3RD SCENARIO
5. `prompt_comprehensive()` - 298 lines (lines 791-1089) ← DELETE

---

## Detailed Trimming Instructions

### KEEP & TRIM: `prompt_basic()` (Target: ~110 lines)

**Current**: 119 lines covering basic commit history viewing and simple pagination (max_count parameter)

**Keep:**
- Basic viewing patterns (30 lines):
  - View all commits: `git_log({"path": "/project"})`
  - View recent commits: `git_log({"path": "/project", "max_count": 10})`
  - View latest commit: `git_log({"path": "/project", "max_count": 1})`
- Parameter documentation (20 lines):
  - `path` (required): Repository location
  - `max_count` (optional): Limit results
  - Brief explanation of each
- Response structure (20 lines):
  - Commit object format
  - Key fields: id, author, summary, time
  - Understanding the count field
- Commit order explanation (10 lines):
  - Newest first (reverse chronological)
  - Most recent at index 0
- Common use cases (15 lines):
  - Review recent work
  - Check latest change
  - Quick commits snapshot
- Performance tips (15 lines):
  - Use max_count on large repos
  - Start with 10-20, increase if needed

**Remove:**
- Extended parameter list (lines 95-110): Too verbose, move essential to brief docs
- Multiple workflow examples (lines 119-145): Keep ONE workflow only
- "INTERPRETING RESULTS" section (lines 134-144): Redundant with response structure
- "EXAMPLE WORKFLOW" section (lines 147-155): Too detailed for basic scenario

**Integration with pagination**: Move simple `skip` examples to basic scenario:
- Pagination basics (15 lines):
  - Page 1: `skip: 0, max_count: 10`
  - Page 2: `skip: 10, max_count: 10`
  - Page 3: `skip: 20, max_count: 10`
  - Pagination formula: `skip = (P - 1) × S`
  - Detecting last page: `count < max_count`

**Total breakdown**: 30 + 20 + 20 + 10 + 15 + 15 + 15 = 125 lines (acceptable range)

---

### KEEP & TRIM: `prompt_filtering()` (Target: ~105 lines)

**Current**: 178 lines covering path_filter parameter for file and directory filtering

**Keep:**
- Filter by file examples (35 lines):
  - Specific file: `path_filter: "src/main.rs"`
  - File with limit: `path_filter: "README.md", "max_count": 5`
  - Configuration file: `path_filter: "config.toml"`
  - Test file: `path_filter: "tests/integration.rs"`
- Filter by directory examples (25 lines):
  - Module directory: `path_filter: "src/auth/"`
  - Feature directory: `path_filter: "src/features/payments/"`
  - Documentation: `path_filter: "docs/"`
  - Tests directory: `path_filter: "tests/"`
- Path filter syntax (15 lines):
  - Git pathspec format
  - Relative to repo root
  - Glob patterns supported
  - Trailing slash optional for directories
- Combine filtering with limits (15 lines):
  - File + max_count example
  - Directory + max_count example
  - Quick reference examples
- Common use cases (15 lines):
  - Debugging: find bug introduction
  - Code archaeology: understand evolution
  - Feature tracking: development history
  - Who changed what: contributor tracking

**Remove:**
- Extended use cases section (lines 180-229): Too many scenarios, consolidate to 3-4
- "EMPTY RESULTS" subsection (lines 231-236): Too obvious, remove
- "PERFORMANCE" subsection (lines 238-241): Covered in basic scenario
- "BEST PRACTICES" list (lines 243-248): Too prescriptive, keep essential only
- Duplicate workflow examples (lines 189-220): Keep ONE workflow only

**Total breakdown**: 35 + 25 + 15 + 15 + 15 = 105 lines (perfect)

---

### CONSOLIDATE: `prompt_pagination()` (197 lines) → Merge into prompt_basic()

**Action**: Extract advanced pagination patterns (~15 lines) and move to `prompt_basic()`:
- Large skip values for deep history
- Pagination workflow with last-page detection
- Different page sizes (10, 25, 50, 100)

**Delete the rest** (182 lines):
- Basic pagination (already in basic scenario after trimming)
- Pagination formula (moving to basic)
- Different page size examples (too many variants)
- Pagination with filtering section (covered in filtering scenario)
- Multiple workflow examples (keep ONE in basic)
- Detecting last page (moving to basic)
- Use cases for pagination (redundant with basic scenario)
- Performance considerations (covered in basic)
- Best practices (redundant)

**Result**: Full `prompt_pagination()` function is DELETED. Content is integrated into `prompt_basic()`.

---

### KEEP: `prompt_file_history()` (Target: ~95-105 lines)

**Current**: 238 lines covering code archaeology patterns and file history tracking

**Keep:**
- Basic file tracking (25 lines):
  - Track all changes: `path_filter: "src/main.rs"`
  - Recent changes: `path_filter: "src/auth.rs", max_count: 10`
  - Find contributors: review author field
- Use cases (50 lines):
  - Bug investigation workflow (10 lines): find commits → review messages → use git_diff
  - Feature tracking (8 lines): see how feature evolved
  - Module evolution (8 lines): track module changes over time
  - Configuration changes (8 lines): track config modifications
  - Dependency tracking (8 lines): monitor dependency updates
- Code archaeology workflow (20 lines):
  1. Identify file/module
  2. Get complete history
  3. Review commit messages
  4. Narrow suspects with max_count
  5. Use git_diff for changes
  6. Identify root cause
- Extracting insights from commits (10 lines):
  - Author information usage
  - Timing information usage
  - Commit message insights
  - Commit ID usage for git_diff

**Remove:**
- Redundant tracking examples (lines 364-385): Too many variants
- "TRACK FILE CHANGES" duplicate section
- Extended investigation scenarios (lines 412-529): Too many edge cases
- "ADVANCED PATTERNS" section (lines 531-548): Move essential into workflow
- Multiple dependency examples (lines 414-419): Keep ONE example
- Security audit section (lines 546-549): Too specialized
- Performance section (lines 560-564): Covered in basic scenario
- BEST PRACTICES list (lines 566-584): Too long, consolidate
- "COMMON INVESTIGATION SCENARIOS" (lines 586-616): Keep only ONE (bug investigation)
- Redundant "REMEMBER" section (lines 618-622)

**Rename function**: `prompt_file_history()` → `prompt_code_archaeology()` (semantically clearer)

**Update prompt argument**: Change scenario name from "file_history" to "code_archaeology"

**Total breakdown**: 25 + 50 + 20 + 10 = 105 lines (acceptable)

---

### DELETE ENTIRELY: `prompt_comprehensive()` (298 lines)

Pure duplication of the 3 focused scenarios. Contains:
- Basic usage (already in basic scenario after trimming)
- Parameters (already documented in each scenario)
- Core features (already in basic scenario)
- File filtering (already in filtering scenario)
- Pagination (being merged into basic scenario)
- Common workflows (already in each scenario)
- Use cases (already covered in each scenario)
- Best practices (already in each scenario)
- Performance tips (redundant)
- Troubleshooting (redundant)
- Commit order (already in basic scenario)
- Integration patterns (redundant)
- Quick reference (redundant)

**Delete all 298 lines.**

---

## Implementation Steps

### Step 1: Update Function Names and Routing (Lines 20-32)

**Before** (lines 20-26):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("filtering") => prompt_filtering(),
        Some("pagination") => prompt_pagination(),
        Some("file_history") => prompt_file_history(),
        _ => prompt_comprehensive(),
    }
}

fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, filtering, pagination, file_history)".to_string()),
            required: Some(false),
        }
    ]
}
```

**After** (new routing):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("filtering") => prompt_filtering(),
        Some("code_archaeology") => prompt_code_archaeology(),
        _ => prompt_basic(),
    }
}

fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (filtering, code_archaeology)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Changes**:
- Default scenario changes from `prompt_comprehensive()` to `prompt_basic()`
- Remove "pagination" and "file_history" scenario options
- Add "code_archaeology" scenario option
- Update prompt_arguments description to list only 2 scenarios

### Step 2: Trim `prompt_basic()` (Lines 47-166 → Target: ~110 lines)

**Action**: Keep core viewing examples, add simple pagination, remove verbose sections

**What to keep from original**:
- Lines 49-91: BASIC COMMIT HISTORY section (keep all, ~30 lines)
- Lines 93-101: RESPONSE STRUCTURE section (keep but trim to ~15 lines)
- Lines 103-110: READING THE OUTPUT section (keep essential ~10 lines)
- Lines 112-115: COMMIT ORDER section (keep all ~5 lines)
- Lines 117-127: PARAMETERS section (keep essential ~8 lines)
- Lines 129-138: COMMON USE CASES section (keep 3-4 examples ~12 lines)
- Lines 140-147: PERFORMANCE TIPS section (keep all ~10 lines)

**Add new content** (integrated from pagination scenario):
- Simple pagination workflow (15 lines):
  ```
  PAGINATION BASICS:
  PAGE 1: git_log({"path": "/repo", "max_count": 10, "skip": 0})
  PAGE 2: git_log({"path": "/repo", "max_count": 10, "skip": 10})
  PAGE 3: git_log({"path": "/repo", "max_count": 10, "skip": 20})
  FORMULA: skip = (page - 1) × page_size
  DETECT LAST PAGE: if count < max_count, this is final page
  ```

**Remove entirely**:
- Lines 151-157: Extended parameter documentation (too verbose)
- Lines 159-166: Example workflow (covered elsewhere)

### Step 3: Trim `prompt_filtering()` (Lines 169-347 → Target: ~105 lines)

**Action**: Keep examples and use cases, consolidate workflows, remove redundancy

**Keep all sections**:
- Lines 171-201: FILTER BY FILE examples (all 30 lines)
- Lines 203-226: FILTER BY DIRECTORY examples (all 23 lines)
- Lines 228-246: PATH_FILTER SYNTAX section (keep ~15 lines)
- Lines 248-270: USE CASES section - consolidate:
  - DEBUGGING (keep)
  - CODE ARCHAEOLOGY (keep)
  - FEATURE TRACKING (keep)
  - WHO CHANGED WHAT (keep)
  - Delete: DOCUMENTATION AUDIT (redundant)

**Remove**:
- Lines 272-279: EMPTY RESULTS subsection (obvious)
- Lines 281-287: PERFORMANCE subsection (covered in basic)
- Lines 289-297: BEST PRACTICES list (consolidate to essential)

---

## Success Criteria

- ✓ File is 280-360 lines total (target: ~320)
- ✓ Exactly THREE scenario functions: basic, filtering, code_archaeology
- ✓ No pagination function (merged into basic)
- ✓ No file_history function (renamed to code_archaeology)
- ✓ No comprehensive function (deleted entirely)
- ✓ Each scenario is 95-115 lines
- ✓ Routing match statement has 3 arms (filtering, code_archaeology, default to basic)
- ✓ prompt_arguments description lists only 2 scenarios
- ✓ One workflow example per scenario (not multiple)
- ✓ Parameters documented briefly in basic scenario

---

## Validation Checklist

After trimming:
1. **Line count**: `wc -l prompts.rs` should show 280-360 lines
2. **Scenario count**: `grep "^fn prompt_" prompts.rs` should show exactly 3 functions:
   - `prompt_basic()`
   - `prompt_filtering()`
   - `prompt_code_archaeology()`
3. **No removed functions**: `grep "fn prompt_pagination\|fn prompt_file_history\|fn prompt_comprehensive"` should return 0 results
4. **Routing validation**: Match statement in `generate_prompts()` has exactly 3 arms
5. **Default scenario**: `_ => prompt_basic()` is the default
6. **Pagination removed from args**: prompt_arguments description does not mention "pagination" or "file_history"
7. **Content quality**: Each scenario covers ONE distinct aspect:
   - basic: core features + simple pagination
   - filtering: path_filter parameter + use cases
   - code_archaeology: history investigation patterns
8. **Readability**: All scenarios understandable in 5 minutes each

---

## Complexity Rationale

**Complexity 3 Justification**:
- Multiple important parameters (4): path, max_count, skip, path_filter
- Parameter interactions: skip + max_count for pagination, path_filter + max_count for filtered pagination
- Important workflows: basic viewing, filtering, code archaeology
- Each scenario covers a distinct use case
- ~320 lines total matches Complexity 3 standard (~100 lines per scenario × 3)

This follows the **Complexity 3 template** established by PRECURSOR_03_git_branch_create: 2-3 focused scenarios covering distinct features/parameters, each with brief workflow examples and essential documentation.
