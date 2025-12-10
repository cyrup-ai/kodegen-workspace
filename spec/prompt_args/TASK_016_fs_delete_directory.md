# TASK 016: Trim fs_delete_directory Prompts

**Tool**: `fs_delete_directory`
**Complexity**: 2 (Simple)
**Current File Size**: 737 lines
**Target Size**: 200 lines (1-2 scenarios)
**File to Modify**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/delete_directory/prompts.rs`

---

## Current State Analysis

The prompts.rs file contains 737 total lines with the following structure:

### Existing Scenarios (5 total)

1. **prompt_basic()** (lines 43-106)
   - 64 lines of code
   - Content: Basic deletion syntax, recursive=true requirement, what gets deleted, warnings, use cases
   - Status: KEEP - Core foundational knowledge

2. **prompt_safety()** (lines 108-218)
   - 111 lines of code
   - Content: Why recursive=true prevents disasters, safety implications, catastrophe examples, best practices
   - Status: KEEP - Critical for understanding design decision

3. **prompt_verification()** (lines 220-393)
   - 174 lines of code
   - Content: Verification workflow, step-by-step guidance, common mistakes, verification checklist
   - Status: DELETE - Use-case scenario, excessive detail

4. **prompt_alternatives()** (lines 395-585)
   - 191 lines of code
   - Content: When NOT to delete, alternatives to deletion, decision trees, archiving strategies
   - Status: DELETE - Use-case scenario, outside core scope

5. **prompt_comprehensive()** (lines 587-737)
   - 151 lines of code
   - Content: Complete guide covering all aspects of directory deletion
   - Status: DELETE - Comprehensive scenario (explicitly excluded by task)

### PromptProvider Implementation (lines 12-40)

- Match statement routes scenarios: "basic", "safety", "verification", "alternatives", and default to comprehensive
- Prompt arguments describe available scenarios: "(basic, safety, verification, alternatives)"

---

## Core Objective

Reduce prompts.rs from 737 lines to approximately 200 lines by:
1. Keeping only the basic and safety scenarios
2. Removing verification, alternatives, and comprehensive scenarios
3. Updating the match routing statement to reflect 2 scenarios
4. Updating scenario descriptions in prompt_arguments()
5. Ensuring all safety-critical information remains

This preserves the essential information about recursive=true requirement and safety implications while removing redundant scenario guidance.

---

## Step-by-Step Implementation

### Step 1: Update PromptProvider Routing (lines 18-24)

**Current Code:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("safety") => prompt_safety(),
        Some("verification") => prompt_verification(),
        Some("alternatives") => prompt_alternatives(),
        _ => prompt_comprehensive(),
    }
}
```

**Change To:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("safety") => prompt_safety(),
        _ => prompt_basic(),
    }
}
```

**Rationale:** Default to basic scenario instead of comprehensive. Remove all references to verification and alternatives.

### Step 2: Update Prompt Arguments Description (lines 26-32)

**Current Code:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, safety, verification, alternatives)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Change To:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, safety)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Rationale:** Update documentation to reflect only 2 available scenarios.

### Step 3: Delete prompt_verification() Function (lines 220-393)

**Delete all 174 lines** of the prompt_verification() function and its entire body.

The function starts with:
```rust
/// Verification workflow before deletion
fn prompt_verification() -> Vec<PromptMessage> {
```

And ends with the closing brace of its vec! macro.

### Step 4: Delete prompt_alternatives() Function (lines 395-585)

**Delete all 191 lines** of the prompt_alternatives() function and its entire body.

The function starts with:
```rust
/// Alternatives to deletion - when NOT to delete
fn prompt_alternatives() -> Vec<PromptMessage> {
```

And ends with the closing brace of its vec! macro.

### Step 5: Delete prompt_comprehensive() Function (lines 587-737)

**Delete all 151 lines** of the prompt_comprehensive() function and its entire body.

The function starts with:
```rust
/// Comprehensive guide covering all aspects of directory deletion
fn prompt_comprehensive() -> Vec<PromptMessage> {
```

And ends with the closing brace of its vec! macro. This is the final function in the file.

### Step 6: Verify Result

After all deletions and updates, the file structure should be:

1. Module-level documentation comment (lines 1-3)
2. Use statements (lines 5-7)
3. DeleteDirectoryPrompts struct doc comment (lines 9-12)
4. DeleteDirectoryPrompts struct definition (line 13)
5. PromptProvider impl block with:
   - Updated generate_prompts() with 3 match arms (basic, safety, default)
   - Updated prompt_arguments() with 2 scenarios
6. Section comment for helper functions (lines 34-36)
7. prompt_basic() function (complete, unchanged from original)
8. prompt_safety() function (complete, unchanged from original)

**No other functions should exist below prompt_safety().**

---

## Success Criteria

The trimmed file MUST meet ALL of these criteria:

- **Line Count**: 170-220 lines total (including all comments, whitespace, and code)
- **Scenarios**: Exactly 2 prompt functions (prompt_basic and prompt_safety)
- **Routing**: Match statement has 3 arms (basic, safety, default->basic)
- **No Comprehensive**: prompt_comprehensive() function completely removed
- **No Verification**: prompt_verification() function completely removed
- **No Alternatives**: prompt_alternatives() function completely removed
- **Updated Documentation**: prompt_arguments description updated to "(basic, safety)"
- **File Compiles**: No Rust compilation errors or warnings
- **Preserves Core**: Both basic and safety scenarios remain completely intact
- **Default Behavior**: Unknown scenario defaults to prompt_basic()

---

## Verification Steps

After implementation, verify:

1. Open `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/delete_directory/prompts.rs`
2. Count total lines (should be 170-220)
3. Verify only prompt_basic() and prompt_safety() function definitions exist
4. Verify match statement has exactly 3 arms
5. Verify prompt_arguments() description shows "basic, safety" only
6. Search file for "verification" - should find 0 matches
7. Search file for "alternatives" - should find 0 matches
8. Search file for "comprehensive" - should find 0 matches
9. Verify file ends after closing brace of prompt_safety()

---

## Technical Details

**File Encoding**: UTF-8
**Line Ending**: LF (Unix-style)
**Rust Edition**: 2024
**No External Dependencies**: This file only imports from crate and rmcp

---

## Implementation Notes

- The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema
- Both preserved scenarios (basic and safety) provide complementary information:
  - Basic: HOW to use fs_delete_directory
  - Safety: WHY recursive=true is required
- Default routing to basic scenario ensures backwards compatibility
- All deletions are exact function boundaries - no partial function deletions
- The helper functions comment block (lines 34-36) remains as it still applies to the 2 retained functions
