# TASK 046: Trim github_get_file_contents

**Tool**: `github_get_file_contents`
**Complexity**: 2 (Simple)
**Current Size**: 903 lines
**Target Size**: 170-220 lines
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_file_contents/prompts.rs`

---

## Current State Analysis

### File Structure
- **Lines 1-11**: Imports and doc comment
- **Lines 12-31**: PromptProvider trait implementation with match statement
- **Lines 33-42**: prompt_arguments() function
- **Lines 47-174**: `prompt_files()` function (~128 lines) - KEEP but TRIM to 95 lines
- **Lines 176-425**: `prompt_directories()` function (~250 lines) - DELETE
- **Lines 427-660**: `prompt_branches()` function (~234 lines) - KEEP but TRIM to 85 lines
- **Lines 662-898**: `prompt_workflows()` function (~237 lines) - DELETE
- **Lines 900+**: `prompt_comprehensive()` function (~440+ lines) - DELETE

### Current Match Statement (Lines 19-25)
```rust
match args.scenario.as_deref() {
    Some("files") => prompt_files(),
    Some("directories") => prompt_directories(),
    Some("branches") => prompt_branches(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

### Current Scenarios
The tool has 5 scenarios that need to be reduced to 2:
1. `"files"` - Reading file contents (KEEP)
2. `"directories"` - Listing directory contents (DELETE)
3. `"branches"` - Reading from refs/branches/tags/commits (KEEP)
4. `"workflows"` - Common workflows and patterns (DELETE - this is use case)
5. Default/comprehensive - Full guide (DELETE)

---

## Implementation Instructions

### Step 1: Trim prompt_files() Function

**Current structure** (lines 47-174):
- User question: "How do I read file contents from GitHub repositories?"
- Assistant response with sections:
  - READING FILE CONTENTS (5 basic examples)
  - FILE METADATA IN RESPONSE (7 fields explained)
  - CONTENT HANDLING (4 points)
  - AUTHENTICATION (3 points)
  - COMMON USE CASES (4 detailed workflows with examples) - DELETE
  - ERROR HANDLING (5 error codes)
  - BEST PRACTICES (6 bullet points)

**Trim to ~95 lines by**:
1. Keep the user question (1 line)
2. Keep READING FILE CONTENTS section with 4 core examples - ~10 lines
3. Keep FILE METADATA IN RESPONSE brief - ~8 lines
4. Keep CONTENT HANDLING brief - ~3 lines
5. Keep AUTHENTICATION brief - ~3 lines
6. DELETE COMMON USE CASES section (workflow/use-case focused) - saves ~30 lines
7. Keep ERROR HANDLING condensed - ~3 lines
8. Keep BEST PRACTICES condensed - ~4 lines

**Target output**: Single PromptMessage with condensed assistant response focused on core reading functionality

### Step 2: Trim prompt_branches() Function

**Current structure** (lines 427-660):
- User question: "How do I read files from specific branches, tags, or commits?"
- Assistant response with sections:
  - READING FROM BRANCHES (3 examples)
  - READING FROM TAGS (2 examples)
  - READING FROM COMMITS (2 examples)
  - REF_NAME PARAMETER (detailed explanation)
  - REF RESOLUTION (explanation)
  - COMPARISON WORKFLOWS (3 detailed workflows) - DELETE (use case)
  - VERSION PINNING (explanation) - DELETE (design pattern)
  - HISTORICAL ANALYSIS (4 points) - DELETE (use case)
  - ERROR HANDLING (3 errors)
  - BEST PRACTICES (7 bullet points)

**Trim to ~85 lines by**:
1. Keep the user question (1 line)
2. Keep READING FROM BRANCHES with 2 concise examples - ~6 lines
3. Keep READING FROM TAGS with 2 examples - ~6 lines
4. Keep READING FROM COMMITS with 1 example - ~4 lines
5. Keep REF_NAME PARAMETER condensed - ~4 lines
6. Keep REF RESOLUTION brief - ~2 lines
7. DELETE COMPARISON WORKFLOWS section (saves ~25 lines)
8. DELETE VERSION PINNING section (saves ~10 lines)
9. DELETE HISTORICAL ANALYSIS section (saves ~10 lines)
10. Keep ERROR HANDLING condensed - ~2 lines
11. Keep BEST PRACTICES condensed - ~3 lines

**Target output**: Single PromptMessage with condensed assistant response, no use-case workflows

### Step 3: Update Match Statement

**Replace lines 19-25** with this trimmed version:
```rust
match args.scenario.as_deref() {
    Some("files") => prompt_files(),
    Some("branches") => prompt_branches(),
    _ => prompt_files(),  // Default to basic file scenario
}
```

This removes 2 match arms and changes default to prompt_files() instead of comprehensive.

### Step 4: Update prompt_arguments() Function

**Replace lines 33-42** with:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (files, branches)".to_string()),
            required: Some(false),
        }
    ]
}
```

Change description from "Scenario to show (files, directories, branches, workflows)" to "Scenario to show (files, branches)"

### Step 5: Delete Functions Completely

Remove these functions in their entirety:
- `prompt_directories()` - All ~250 lines (lines 176-425 in current file)
- `prompt_workflows()` - All ~237 lines (lines 662-898 in current file)
- `prompt_comprehensive()` - All ~440+ lines (lines 900+ in current file)

This removes use-case focused and comprehensive documentation content.

### Step 6: Remove Decorative Header Comments

Delete or simplify comment blocks:
- Line 44-46: The decorative header `// ============================================================================...`

---

## Code Changes Summary

### Match Statement Change
**BEFORE** (9 lines):
```rust
match args.scenario.as_deref() {
    Some("files") => prompt_files(),
    Some("directories") => prompt_directories(),
    Some("branches") => prompt_branches(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

**AFTER** (4 lines):
```rust
match args.scenario.as_deref() {
    Some("files") => prompt_files(),
    Some("branches") => prompt_branches(),
    _ => prompt_files(),
}
```

### prompt_arguments Description Change
**BEFORE**:
```
"Scenario to show (files, directories, branches, workflows)"
```

**AFTER**:
```
"Scenario to show (files, branches)"
```

### Functions to Delete
1. Delete `fn prompt_directories() -> Vec<PromptMessage> { ... }` entirely
2. Delete `fn prompt_workflows() -> Vec<PromptMessage> { ... }` entirely
3. Delete `fn prompt_comprehensive() -> Vec<PromptMessage> { ... }` entirely

### Functions to Trim

**prompt_files() should**:
- Keep user question unchanged
- Condense assistant response from ~450 words to ~250 words
- Remove entire COMMON USE CASES section
- Keep: Examples, metadata, content handling, auth, errors, best practices (condensed)

**prompt_branches() should**:
- Keep user question unchanged
- Condense assistant response from ~550 words to ~300 words
- Remove COMPARISON WORKFLOWS, VERSION PINNING, HISTORICAL ANALYSIS sections
- Keep: Examples for branches/tags/commits, ref_name parameter, ref resolution, errors, best practices (condensed)

---

## Verification Checklist

After implementation, verify:

1. **Line Count**: Run `wc -l prompts.rs` - should output 170-220 lines
2. **Function Count**: File should have exactly 2 prompt functions (prompt_files, prompt_branches)
3. **Match Cases**: Match statement should have exactly 3 cases (Some("files"), Some("branches"), _)
4. **Deleted References**: Grep for "directories\|workflows\|comprehensive" - should return 0 results
5. **No Decorative Headers**: Grep for "===" should find 0 standalone header lines
6. **Code Compiles**: Run `cargo check -p kodegen-mcp-schema` - should pass without errors
7. **Scenario Parameter**: prompt_arguments() should list "files, branches" in description

---

## Expected Result

**Before**:
- 903 total lines
- 5 scenario functions
- Includes use-case workflows
- Includes comprehensive guide

**After**:
- 170-220 total lines
- 2 scenario functions (files, branches)
- No use-case workflows
- No comprehensive guide
- Core functionality preserved for:
  - Reading file contents from default branch
  - Reading files from specific branches/tags/commits
  - Understanding response metadata
  - Ref name parameter usage
  - Basic error handling

---

## Success Criteria - Definition of Done

- ✓ Total file size is 170-220 lines (measured by line count)
- ✓ Exactly 2 prompt scenarios available: "files" and "branches"
- ✓ No use-case workflow scenarios (directories, workflows deleted)
- ✓ No comprehensive scenario (full guide deleted)
- ✓ prompt_files() function ~95 lines with core content
- ✓ prompt_branches() function ~85 lines with core content
- ✓ Core functionality documented:
  - Basic file reading with 4 concrete examples
  - File metadata fields explained
  - Reading from branches with examples
  - Reading from tags with examples
  - Reading from commits with example
  - Ref name parameter documented
  - Error codes documented
- ✓ Match statement updated to route only files and branches
- ✓ prompt_arguments() updated to list only "files, branches"
- ✓ No references to deleted scenarios remain in code
- ✓ Code compiles without errors: `cargo check -p kodegen-mcp-schema`
- ✓ No decorative header comment blocks remain
