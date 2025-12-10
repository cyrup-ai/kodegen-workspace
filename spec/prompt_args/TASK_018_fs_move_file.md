# TASK 018: Trim fs_move_file Prompts

**Tool**: `fs_move_file`
**Complexity**: 2 (Simple)
**Current size**: 898 lines (5 scenarios + boilerplate)
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/move_file/prompts.rs`

---

## Current State Analysis

### File Structure
The prompts.rs file has the following composition:
- **Lines 1-44**: Imports, struct definition, and PromptProvider trait implementation
- **Lines 45-58**: `prompt_arguments()` function (returns description of available scenarios)
- **Lines 59-234**: `prompt_rename()` function (~176 lines, basic same-directory rename operations)
- **Lines 235-477**: `prompt_relocate()` function (~243 lines, cross-directory move operations)
- **Lines 478-777**: `prompt_organize()` function (~299 lines, organizing files into structures - USE CASE, DELETE)
- **Lines 778-898**: `prompt_backup()` function (~121 lines, backup patterns - USE CASE, DELETE)
- **Lines 899+**: `prompt_comprehensive()` function (very large, covers all operations - DELETE PER SPEC)

### Current Match Statement (Lines 18-26)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("rename") => prompt_rename(),
        Some("relocate") => prompt_relocate(),
        Some("organize") => prompt_organize(),
        Some("backup") => prompt_backup(),
        _ => prompt_comprehensive(),
    }
}
```

### Current Scenario Descriptions (Line 30)
```rust
description: Some("Scenario to show (rename, relocate, organize, backup)".to_string()),
```

---

## Implementation Strategy

### Scenarios to Keep (2 complementary scenarios)
1. **`prompt_rename()`** - Basic scenario for same-directory rename/extension change operations (~95-100 lines after trimming)
   - Keep: 3 basic examples (file rename, directory rename, extension change)
   - Keep: RENAME DEFINITION section
   - Keep: RENAME BEHAVIOR section (condensed)
   - Keep: BEST PRACTICES section (condensed to 5 items max)
   - **DELETE**: "RENAMING MULTIPLE FILES" section (lines ~130-135)
   - **DELETE**: "RENAMING EXAMPLES BY USE CASE" section (lines ~140-155) - too many edge case variations
   - **TRIM**: Reduce "COMMON RENAME PATTERNS" from 4 examples to 3, keep first 3

2. **`prompt_relocate()`** - Advanced scenario for cross-directory and mixed operations (~105-110 lines after trimming)
   - Keep: 3 basic examples (move to different dir, move directory, move+rename)
   - Keep: DESTINATION DIRECTORY REQUIREMENTS section (with example workflow)
   - Keep: OVERWRITE BEHAVIOR section (important safety info)
   - Keep: SAME-FILESYSTEM vs CROSS-FILESYSTEM comparison
   - Keep: 3 COMMON RELOCATION PATTERNS (archive, downloads, backup)
   - **DELETE**: "MOVING MULTIPLE FILES" section (lines ~360-366)
   - **DELETE**: "DIRECTORY MOVES" section (lines ~368-378) - detailed but redundant
   - **DELETE**: Reduce ERROR CASES from 6 to 4 most common

### Scenarios to Delete
- `prompt_organize()` - This is a use-case scenario, not a core operation
- `prompt_backup()` - This is a use-case scenario, not a core operation
- `prompt_comprehensive()` - Delete entirely per spec ("No comprehensive")

---

## Step-by-Step Implementation

### Step 1: Update the Match Statement (Lines 18-26)
**REPLACE THIS:**
```rust
match args.scenario.as_deref() {
    Some("rename") => prompt_rename(),
    Some("relocate") => prompt_relocate(),
    Some("organize") => prompt_organize(),
    Some("backup") => prompt_backup(),
    _ => prompt_comprehensive(),
}
```

**WITH THIS:**
```rust
match args.scenario.as_deref() {
    Some("rename") => prompt_rename(),
    Some("relocate") => prompt_relocate(),
    _ => prompt_rename(),  // Default to rename scenario
}
```

### Step 2: Update Prompt Arguments Description (Line 30)
**REPLACE THIS:**
```rust
description: Some("Scenario to show (rename, relocate, organize, backup)".to_string()),
```

**WITH THIS:**
```rust
description: Some("Scenario to show (rename, relocate)".to_string()),
```

### Step 3: Trim prompt_rename() Function
**Keep all content up to line 128** (through RENAME BEHAVIOR section ending with "- Preserves timestamps (modified, accessed)")

**DELETE from line 129 to line 155** - This is:
- RENAMING MULTIPLE FILES section
- RENAMING EXAMPLES BY USE CASE section (too detailed, edge cases)

**MODIFY COMMON RENAME PATTERNS section** (lines ~106-126):
- Keep only the first 3 patterns (Add prefix, Add suffix, Normalize filename)
- Delete the 4th pattern (Version increment)

**Keep lines 156-170** (BEST PRACTICES section, but condense):
- Reduce from 5 items to best 5 (keep exactly as is)

**Keep lines 171+ through end of function** - closing brace

Final prompt_rename() should be approximately **95-100 lines** (content text only, not counting the function wrapper)

### Step 4: Trim prompt_relocate() Function
**Keep all content up to line 305** (through SAME-FILESYSTEM MOVES section)

**DELETE from line 306 to line 395** - This is:
- COMMON RELOCATION PATTERNS section (lines 306-356) - KEEP ONLY FIRST 3
- Actually keep all 4 patterns, delete the "4. Move temporary to permanent" is redundant with other sections

**Delete "MOVING MULTIPLE FILES" section entirely** (lines ~360-366)

**DELETE "DIRECTORY MOVES" section** (lines ~368-378)

**KEEP "ERROR CASES" but condense** (lines ~380-395):
- Keep only 4 most common errors
- Remove "Moving directory into itself"
- Remove "Insufficient disk space (cross-filesystem)"

**Keep "BEST PRACTICES" section** (7 items, as is)

Final prompt_relocate() should be approximately **105-110 lines** (content text only)

### Step 5: Delete Unused Functions
Delete the entire `prompt_organize()` function (lines 478-777)
Delete the entire `prompt_backup()` function (lines 778-898)  
Delete the entire `prompt_comprehensive()` function (lines 899+)

---

## Trimming Specifics for prompt_rename()

### What to Cut
1. **Lines ~129-135**: "RENAMING MULTIPLE FILES" section - Single shot instruction, not needed for basic scenario
2. **Lines ~140-155**: "RENAMING EXAMPLES BY USE CASE" section - Too verbose with 4 separate use cases
3. **In COMMON RENAME PATTERNS**: Remove "4. Version increment" example - covered implicitly

### Result
Content should flow: Intro → 3 basic examples → RENAME DEFINITION → COMMON RENAME PATTERNS (3 examples) → RENAME BEHAVIOR → BEST PRACTICES → close

Estimated final line count: 95-100 lines of content

---

## Trimming Specifics for prompt_relocate()

### What to Cut
1. **"MOVING MULTIPLE FILES" section** - Instruction to "call for each" is redundant, covered in examples
2. **"DIRECTORY MOVES" section** - Verbose explanation of structure preservation already covered in main examples
3. **In "ERROR CASES"**: Delete these two:
   - "Moving directory into itself → Error"
   - "Insufficient disk space (cross-filesystem) → Error"

### What to Keep
- All 3 basic moving examples
- DESTINATION DIRECTORY REQUIREMENTS (critical)
- OVERWRITE BEHAVIOR (critical safety)
- CROSS-FILESYSTEM vs SAME-FILESYSTEM comparison (essential)
- All 4 COMMON RELOCATION PATTERNS
- 5 of 7 ERROR CASES (most critical only)
- All 7 BEST PRACTICES items

### Result
Content should flow: Intro → 3 examples → REQUIREMENTS → OVERWRITE → Filesystem comparison → Patterns → Errors → Best practices → close

Estimated final line count: 105-110 lines of content

---

## Success Criteria (Measurable)

- ✓ **Line count**: Final file is 200-215 lines total (boilerplate ~44 + rename ~100 + relocate ~110)
- ✓ **Scenarios**: Exactly 2 functions exist: `prompt_rename()` and `prompt_relocate()`
- ✓ **Match statement**: Has exactly 2 Some() arms + 1 wildcard default (pointing to rename)
- ✓ **No deleted functions**: `prompt_organize()`, `prompt_backup()`, `prompt_comprehensive()` are completely removed
- ✓ **Prompt args**: Description updated to "(rename, relocate)"
- ✓ **No decorative headers**: Comment header at line 59-62 stays, no additional decorative separators added
- ✓ **File compiles**: `cargo check` passes in kodegen-mcp-schema package
- ✓ **Behavior**: Both scenarios respond to their names, default returns rename scenario

---

## Complementary Scenario Design

The two retained scenarios are **intentionally complementary**:

- **rename**: Covers "same directory, different name" operations
  - File renaming, directory renaming, extension changes
  - Basic, focused, 80-120 line range

- **relocate**: Covers "different directory" operations (with optional name change)
  - Moving to different locations, understanding filesystem implications
  - Advanced cross-directory concerns, 70-110 line range

Together they cover the **complete fs_move_file operation space** without redundancy or bloat:
- Rename (samedir) + Relocate (diffdir) = all move/rename combinations
- Each scenario is independently useful and focused
- No use-case scenarios (organize, backup) which are applications of the basic two
- No comprehensive sprawl

---

## File Location
`/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/move_file/prompts.rs`
