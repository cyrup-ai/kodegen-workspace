# TASK 019: Trim fs_read_multiple_files

**Tool**: `fs_read_multiple_files`
**Complexity**: 2 (Simple)
**Current size**: 790 lines
**Target size**: 170-220 lines (1 scenario)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/read_multiple_files/prompts.rs`

---

## Current State Analysis

### File Structure (790 lines total)

The prompts.rs file currently contains:

1. **Module setup and imports** (lines 1-6)
   - Docstring, imports for PromptProvider, rmcp model types

2. **ReadMultipleFilesPrompts struct and impl** (lines 8-37)
   - Struct definition (lines 8-14)
   - PromptProvider impl with generate_prompts() match statement (lines 16-28)
   - prompt_arguments() function (lines 30-37)

3. **Helper comment header** (lines 39-41)
   - Decorative comment block

4. **Prompt scenario functions** (lines 43-790)
   - `prompt_basic()` (lines 43-170, ~128 lines) - KEEP
   - `prompt_related_files()` (lines 172-354, ~183 lines) - DELETE (use-case scenario)
   - `prompt_config_set()` (lines 356-505, ~150 lines) - DELETE (use-case scenario)
   - `prompt_error_handling()` (lines 507-672, ~166 lines) - DELETE (use-case scenario)
   - `prompt_comprehensive()` (lines 674-790, ~117 lines) - DELETE (comprehensive scenario)

### Routing Logic (Current Match Statement, lines 18-23)

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("related_files") => prompt_related_files(),
        Some("config_set") => prompt_config_set(),
        Some("error_handling") => prompt_error_handling(),
        _ => prompt_comprehensive(),
    }
}
```

This routing currently:
- Routes to 4 named scenarios (basic, related_files, config_set, error_handling)
- Falls back to comprehensive for unknown scenarios
- Allows 5 different prompt variants

### Prompt Arguments Description (Current, line 32)

```rust
description: Some("Scenario to show (basic, related_files, config_set, error_handling)".to_string()),
```

This currently documents 4 available scenarios. After trimming, only "basic" will be available.

---

## Implementation Instructions

### Step 1: Delete Use-Case Scenario Functions

The task requires keeping only 1 core scenario. The following functions teach use-cases rather than core functionality and must be deleted:

**Delete `prompt_related_files()` function**
- Location: Lines 172-354 (~183 lines)
- Reason: This is a use-case scenario showing how to read related code files together
- Content covered: Teaching specific patterns (tests with implementation, interfaces with implementations, modules, etc.)
- This information is NICE-TO-HAVE, not core functionality
- The basic scenario is sufficient for teaching how to use the tool

**Delete `prompt_config_set()` function**
- Location: Lines 356-505 (~150 lines)
- Reason: This is a use-case scenario showing configuration file reading patterns
- Content covered: Project configuration, environment configs, build CI configs, etc.
- This is specific application context, not core functionality
- The basic scenario already covers reading multiple files with parameters

**Delete `prompt_error_handling()` function**
- Location: Lines 507-672 (~166 lines)
- Reason: Although critical, this is a specialized scenario
- Content covered: Partial failures, error types, recovery patterns
- Decision: Remove in favor of including error handling content IN the basic scenario
- Note: The basic scenario's response structure section should mention "success" field handling

### Step 2: Delete Comprehensive Scenario

**Delete `prompt_comprehensive()` function**
- Location: Lines 674-790 (~117 lines)
- Reason: Comprehensive scenario provides redundant information
- Content: Duplicates all information from basic + use-case scenarios combined
- Task requirement: "No comprehensive" in success criteria
- Strategy: Basic scenario is sufficient; removes duplication

### Step 3: Update Routing Logic (lines 18-23)

**Replace the match statement** with simplified routing that only handles the basic scenario:

**BEFORE (Current, lines 18-23):**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("related_files") => prompt_related_files(),
        Some("config_set") => prompt_config_set(),
        Some("error_handling") => prompt_error_handling(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER (Target, simplified routing):**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") | None => prompt_basic(),
        _ => prompt_basic(), // Always show basic, ignore unknown scenarios
    }
}
```

Or even simpler (not using match if all roads lead to basic):
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

**Choose the second approach** (simpler, ignores scenario arg entirely since there's only one scenario):
- Removes the match statement entirely
- Changes to `_args` (underscore prefix since arg is now unused)
- Single function call

### Step 4: Update Prompt Arguments Description (line 32)

**BEFORE (Current):**
```rust
description: Some("Scenario to show (basic, related_files, config_set, error_handling)".to_string()),
```

**AFTER (Updated for single scenario):**
```rust
description: Some("Scenario to show (currently only 'basic' is available)".to_string()),
```

Or even simpler, mark it as not required and describe what basic covers:
```rust
description: Some("Basic scenario covering reading multiple files with offset/length parameters".to_string()),
```

Alternatively, since there's only one scenario, consider making this optional and less descriptive:
```rust
description: Some("Basic usage guide (only scenario available)".to_string()),
```

---

## Step-by-Step Execution Plan

Execute these changes in this exact order:

### 1. Remove prompt_related_files() function
   - Delete lines 172-354 (entire function)
   - This frees up ~183 lines

### 2. Remove prompt_config_set() function
   - Delete new lines ~172-321 (adjusted after step 1)
   - This frees up ~150 lines

### 3. Remove prompt_error_handling() function
   - Delete new lines ~172-337 (adjusted after steps 1-2)
   - This frees up ~166 lines

### 4. Remove prompt_comprehensive() function
   - Delete new lines ~172-288 (adjusted after steps 1-3)
   - This frees up ~117 lines

### 5. Update routing in generate_prompts()
   - Replace the match statement (lines 18-23) with simple implementation
   - Change from 6 lines to 1-2 lines

### 6. Update prompt_arguments() description
   - Modify line 32 description string
   - Make it accurate for single scenario

### 7. Remove decorative comment header (optional)
   - Lines 39-41 are decorative and can be removed
   - Comment: "// ============================================================================"
   - This saves 3 lines but is cosmetic

---

## Code Patterns: Before and After

### Pattern 1: Match Statement Evolution

**BEFORE (Current - 6 lines):**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("related_files") => prompt_related_files(),
        Some("config_set") => prompt_config_set(),
        Some("error_handling") => prompt_error_handling(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER (Target - 2 lines):**
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

### Pattern 2: Prompt Arguments Structure

**BEFORE (Current):**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, related_files, config_set, error_handling)".to_string()),
            required: Some(false),
        }
    ]
}
```

**AFTER (Target - still same structure, different description):**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Basic usage guide (only scenario available)".to_string()),
            required: Some(false),
        }
    ]
}
```

---

## Specific Deletions Required

### Deletion 1: prompt_related_files() Function
- **Lines**: 172-354 (approximately)
- **Exact boundary markers**:
  - Start: `/// Reading related code files together`
  - End: `        },\n    ]\n}`
- **Size**: 183 lines
- **Content**: Explains when to read related code files (tests+implementation, interfaces+implementations, modules, etc.)

### Deletion 2: prompt_config_set() Function
- **Lines**: ~356-505 (after deletion 1)
- **Exact boundary markers**:
  - Start: `/// Reading configuration files`
  - End: `        },\n    ]\n}`
- **Size**: 150 lines
- **Content**: Explains reading configuration file sets (environment configs, build configs, etc.)

### Deletion 3: prompt_error_handling() Function
- **Lines**: ~507-672 (after deletions 1-2)
- **Exact boundary markers**:
  - Start: `/// Handling partial failures`
  - End: `        },\n    ]\n}`
- **Size**: 166 lines
- **Content**: Explains how to handle errors, partial failures, and per-file result inspection

### Deletion 4: prompt_comprehensive() Function
- **Lines**: ~674-790 (after deletions 1-3)
- **Exact boundary markers**:
  - Start: `/// Comprehensive guide covering all scenarios`
  - End: (end of file)
- **Size**: 117 lines
- **Content**: Duplicates all information organized with section headers

### Optional: Remove Decorative Comment Header
- **Lines**: 39-41
- **Content**: `// ============================================================================`
- **Reason**: Decorative, not informative
- **Impact**: Saves 3 lines, purely cosmetic

---

## Success Criteria (Definition of Done)

Verify the following after completing all steps:

### Size Verification
- ✓ Total file size: **170-220 lines** (target is ~175-180 lines)
- ✓ Reduction from 790 to target achieved
- Count: Use `wc -l` or read to verify

### Scenario Count
- ✓ **Exactly 1 scenario** remains: `prompt_basic()`
- ✓ All 4 use-case scenarios deleted:
  - ✗ No `prompt_related_files()`
  - ✗ No `prompt_config_set()`
  - ✗ No `prompt_error_handling()`
  - ✗ No `prompt_comprehensive()`

### Routing Verification
- ✓ `generate_prompts()` function simplified to 2 lines
- ✓ No match statement with multiple arms
- ✓ Always calls `prompt_basic()` regardless of args
- ✓ Parameter changed to `_args` (unused)

### Prompt Arguments
- ✓ Description string updated to reflect single scenario
- ✓ No references to "related_files", "config_set", "error_handling"
- ✓ Text accurately describes basic scenario

### Structure Verification
- ✓ All imports still present
- ✓ ReadMultipleFilesPrompts struct unchanged
- ✓ PromptProvider trait impl unchanged (except generate_prompts body)
- ✓ prompt_basic() function completely intact (all 128 lines)

### Optional Cleanup
- ✓ Decorative comment headers removed (if going minimal)
- ✓ File has clean structure with no dead code

### Code Quality
- ✓ No syntax errors
- ✓ Consistent formatting maintained
- ✓ No dangling references to deleted functions
- ✓ No incomplete deletions

---

## Verification Checklist

After implementation, verify each item:

```
[ ] File can be read without syntax errors
[ ] Total line count is 170-220 lines
[ ] generate_prompts() function is 2 lines or less
[ ] prompt_basic() function exists and is unchanged
[ ] prompt_related_files() function completely removed
[ ] prompt_config_set() function completely removed  
[ ] prompt_error_handling() function completely removed
[ ] prompt_comprehensive() function completely removed
[ ] prompt_arguments() description mentions only basic scenario
[ ] No references to deleted functions remain in code
[ ] Imports section is unchanged
[ ] ReadMultipleFilesPrompts struct is unchanged
[ ] No trailing whitespace or formatting issues
[ ] File ends properly with closing brace
```

---

## Implementation Notes

### Important Details

1. **The basic scenario is comprehensive enough**
   - It teaches the core API: paths, offset, length parameters
   - It shows response structure for success and failure cases
   - It explains when to use vs when to use fs_read_file
   - It includes 4 detailed examples covering different use patterns
   - Additional use-case scenarios are redundant

2. **Routing simplification rationale**
   - Since there's only 1 scenario, the match statement is overkill
   - Simplifying to direct function call saves ~4 lines
   - Makes the intent crystal clear: always show basic prompt
   - Ignoring the scenario parameter is appropriate

3. **No content changes to prompt_basic()**
   - Keep the entire prompt_basic() function as-is
   - This function is 128 lines and is the core teaching material
   - All parameter documentation is here
   - All response structure explanation is here
   - All examples are here

4. **Decorative headers decision**
   - The `// ============================================================================` on line 39 is decorative
   - Consider removing for minimal file
   - Or keep it before prompt_basic() for clarity
   - Task says "delete decorative headers" - this qualifies

5. **Line count targeting**
   - Current file: 790 lines
   - After all deletions: ~170-180 lines expected
   - Acceptable range: 170-220 lines (task spec)
   - This represents ~78% reduction in size

---

## Why This Trimming Works

**The basic scenario covers everything needed:**
- Basic usage with all parameters explained
- Response structure documented
- Success vs failure cases shown
- 4 concrete examples
- Decision tree for when to use
- Best practices section
- Performance benefits explained

**Removed scenarios are nice-to-have:**
- related_files: Just shows specific code patterns as examples
- config_set: Shows specific configuration patterns as examples
- error_handling: Error handling is already in basic's response structure
- comprehensive: Duplicates everything from basic + others combined

**Result:**
- 1 powerful, complete scenario instead of 5 redundant ones
- 78% size reduction
- Faster prompt generation
- Clearer intent (only 1 option, not 4 competing options)
- Easier to maintain (no scenario duplicates)
