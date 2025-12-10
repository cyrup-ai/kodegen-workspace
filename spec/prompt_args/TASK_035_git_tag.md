# TASK 035: Trim git_tag

**Tool**: `git_tag`
**Complexity**: 2 (Simple)
**Current size**: 214 lines (5 scenarios + impl)
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/tag/prompts.rs`

---

## Reference

See **PRECURSOR_02_fs_read_file.md** for Complexity 2 template and example implementation.

---

## Current State Analysis

The prompts.rs file contains:
- **Lines 1-32**: `TagPrompts` struct and `PromptProvider` impl with match-based routing
- **Lines 33-62**: `prompt_create()` - lightweight & annotated tag creation (30 lines)
- **Lines 63-104**: `prompt_annotated()` - explains lightweight vs annotated distinction (42 lines)
- **Lines 105-160**: `prompt_semver()` - semantic versioning naming conventions (56 lines) ← **DELETE**
- **Lines 161-197**: `prompt_list()` - listing and deleting tags (37 lines)
- **Lines 198-214**: `prompt_comprehensive()` - duplicates all other scenarios (17 lines) ← **DELETE**

**Current routing** (lines 13-18):
```rust
match args.scenario.as_deref() {
    Some("create") => prompt_create(),
    Some("annotated") => prompt_annotated(),
    Some("semver") => prompt_semver(),
    Some("list") => prompt_list(),
    _ => prompt_comprehensive(),
}
```

**Current scenario argument** (lines 20-28):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario: create, annotated, semver, list".to_string()),
            required: Some(false),
        }
    ]
}
```

---

## Trimming Strategy

### Scenario Decisions

**KEEP**: `prompt_create()` + `prompt_list()`
- Will **MERGE** into single `prompt_basic()` function (~100 lines)
- Together they cover the two primary workflows: creating tags and listing/filtering tags
- These are core tool actions, not use cases

**KEEP**: `prompt_annotated()`
- Explains the important distinction between lightweight and annotated tags
- Teaches a critical feature that affects how users approach tag creation
- Advanced concept that deserves its own scenario (~42 lines)
- Matches PRECURSOR template pattern of basic + advanced scenario

**DELETE**: `prompt_semver()`
- This is a **USE CASE**, not a tool feature
- Semantic versioning is just a naming convention, not a git_tag parameter
- Teaches versioning philosophy, not tool mechanics
- No unique parameters demonstrated
- 56 lines of redundant examples

**DELETE**: `prompt_comprehensive()`
- Pure duplication of content from other scenarios
- Violates PRECURSOR template guideline: no comprehensive scenarios
- Current implementation duplicates create + list + semantic info
- 17 lines of unnecessary duplication

### Merge Instructions: Create + List → Basic

The new `prompt_basic()` function will:
1. **Start with a user question** covering basic usage (lines 1-2 of create)
2. **Assistant response** showing:
   - Basic creation examples (lightweight, annotated, specific commit)
   - List all tags example
   - Filter tags with pattern example
   - Delete tag example
   - Mention pushing tags to remote
3. **Remove** from create: Push tags section (can mention once in basic)
4. **Remove** from list: Delete remote tag (too specialized, covered by basic delete)
5. **Target**: 95-110 lines total (combining ~30 from create + ~37 from list + some dedup)

### Routing Update

**Before** (6 scenarios):
```rust
match args.scenario.as_deref() {
    Some("create") => prompt_create(),
    Some("annotated") => prompt_annotated(),
    Some("semver") => prompt_semver(),
    Some("list") => prompt_list(),
    _ => prompt_comprehensive(),
}
```

**After** (2 scenarios):
```rust
match args.scenario.as_deref() {
    Some("annotated") => prompt_annotated(),
    _ => prompt_basic(),
}
```

### Prompt Arguments Update

**Before**:
```rust
description: Some("Scenario: create, annotated, semver, list".to_string()),
```

**After**:
```rust
description: Some("Scenario: annotated (optional)".to_string()),
```

---

## Step-by-Step Implementation

### Step 1: Analyze Current Scenarios
- Read lines 33-62 (prompt_create): Extract all creation examples
- Read lines 161-197 (prompt_list): Extract list and delete examples
- Identify overlaps and redundancy between the two

### Step 2: Create prompt_basic() Function
Create new function that:
1. Combines User/Assistant structure from both create and list
2. Single User question: "How do I create and manage tags?"
3. Assistant response containing:
   - Create lightweight tag (5 lines)
   - Create annotated tag (5 lines)
   - Tag specific commit (5 lines)
   - List all tags (4 lines)
   - List tags with pattern (4 lines)
   - Delete tag (4 lines)
   - Push tags reminder (3 lines)
   - Total: ~90 lines with proper formatting

### Step 3: Keep prompt_annotated() Unchanged
This function is already appropriately sized (42 lines) and content is unique.
Do not modify this function.

### Step 4: Delete prompt_semver() Function
Completely remove lines 105-160. This is a use-case scenario that doesn't teach tool features.

### Step 5: Delete prompt_comprehensive() Function
Completely remove lines 198-214. This duplicates content from create, annotated, and list scenarios.

### Step 6: Update PromptProvider Impl
Modify the `generate_prompts` method:
- Keep match expression but reduce to 2 cases
- Remove Some("create"), Some("semver"), Some("list") arms
- Keep Some("annotated") arm
- Change default case from `prompt_comprehensive()` to `prompt_basic()`

### Step 7: Update prompt_arguments()
Change scenario description string from:
```
"Scenario: create, annotated, semver, list"
```
To:
```
"Scenario: annotated (optional)"
```

---

## Expected Output

After trimming:

**Lines 1-32**: PromptProvider impl (MODIFIED)
- Simpler match with 2 cases
- Updated argument description

**Lines 33-120**: prompt_basic() (NEW - merged from create + list)
- User question about creating and listing
- Assistant response with 7 example JSON blocks
- ~90 lines total

**Lines 121-160**: prompt_annotated() (KEPT UNCHANGED)
- Explains lightweight vs annotated distinction
- ~42 lines

**Total**: 170-190 lines

---

## Success Criteria

- ✓ File is 170-220 lines total (target: ~185)
- ✓ Exactly 2 scenario functions: `prompt_basic()` and `prompt_annotated()`
- ✓ No `prompt_semver()` function exists
- ✓ No `prompt_comprehensive()` function exists
- ✓ No `prompt_create()` or `prompt_list()` functions (merged into basic)
- ✓ Match statement has exactly 2 cases (Some("annotated") and _ default)
- ✓ Scenario argument description mentions only "annotated"
- ✓ Creating, listing, and deleting all demonstrated in basic scenario
- ✓ Lightweight vs annotated distinction clear in annotated scenario
- ✓ No repetition of "always push tags" (mentioned once)
- ✓ No use-case scenarios (semantic versioning removed)
- ✓ No decorative headers or empty padding

---

## Validation Checklist

After making changes, verify:

1. **Line count**: `wc -l prompts.rs` → must show 170-220 lines
2. **Function count**: `grep "^fn prompt_" prompts.rs` → exactly 2 functions shown
3. **No deleted content remains**: 
   - `grep "semver" prompts.rs` → 0 results
   - `grep "comprehensive" prompts.rs` → 0 results
4. **Routing simplified**: Match statement contains only `Some("annotated")` and `_` case
5. **Scenario description**: Updated to remove deleted scenario names
6. **All actions present**:
   - `grep -c "create" prompts.rs` → at least 1 (in basic)
   - `grep -c "list" prompts.rs` → at least 1 (in basic)
   - `grep -c "delete" prompts.rs` → at least 1 (in basic)
7. **Read-through test**: Can understand all tool capabilities in 2 minutes

---

## Pattern Reference

This follows the Complexity 2 template (PRECURSOR_02):
- 1 basic scenario covering primary workflows
- 1 advanced scenario for special concepts
- Total lines: 170-220
- No use-case scenarios
- No comprehensive scenario
- Clear, prescriptive examples only
