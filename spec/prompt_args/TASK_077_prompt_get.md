# TASK 077: Trim prompt_get

**Tool**: `prompt_get`
**Complexity**: 2 (Simple)
**Current size**: 459 lines
**Target size**: 150-180 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/prompt/prompt_get/prompts.rs`

---

## Current State Analysis

### File Structure
The `prompts.rs` file implements the `PromptProvider` trait for the `get_prompt` tool with the following components:

- **Header & Imports** (lines 1-6): Standard module documentation and trait imports
- **PromptGetPrompts Struct** (lines 8-31): Implements PromptProvider with scenario routing and prompt arguments
- **Five Scenario Functions** (lines 34-459):
  1. `prompt_basic()` (lines 36-73, ~38 lines): Basic prompt retrieval with examples
  2. `prompt_variables()` (lines 75-133, ~59 lines): Variable substitution patterns
  3. `prompt_rendering()` (lines 135-177, ~43 lines): Template rendering examples - **DELETE THIS**
  4. `prompt_workflows()` (lines 179-228, ~50 lines): Usage workflows/use cases - **DELETE THIS**
  5. `prompt_comprehensive()` (lines 230-459, ~230 lines): Full comprehensive guide with decorative headers - **DELETE THIS**

### Routing Logic
Current scenario matching in `generate_prompts()` method (lines 13-18):
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("variables") => prompt_variables(),
    Some("rendering") => prompt_rendering(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

Current prompt arguments description (lines 22-28):
```rust
description: Some("Scenario to show examples for (basic, variables, rendering, workflows, comprehensive)".to_string()),
```

---

## Implementation Instructions

### Step 1: Update Scenario Routing
Replace the entire match statement in `generate_prompts()` method (lines 13-18) with the new minimal routing that keeps only basic and variables scenarios.

**BEFORE** (lines 13-18):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("variables") => prompt_variables(),
        Some("rendering") => prompt_rendering(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("variables") => prompt_variables(),
        _ => prompt_basic(),
    }
}
```

**Rationale**: The basic scenario is the foundational use case (retrieving prompts), so it becomes the default. The variables scenario is for more advanced users needing specific variable handling guidance. All workflow and comprehensive scenarios are removed to reduce bulk.

### Step 2: Update Prompt Arguments
Replace the prompt arguments description (line 24) to list only valid scenarios.

**BEFORE** (line 24):
```rust
description: Some("Scenario to show examples for (basic, variables, rendering, workflows, comprehensive)".to_string()),
```

**AFTER**:
```rust
description: Some("Scenario to show examples for (basic, variables)".to_string()),
```

### Step 3: Delete Unnecessary Scenario Functions

Delete the following three functions entirely from the file:

#### Delete `prompt_rendering()` function
- **Lines to delete**: 135-177 (43 lines total)
- **Function starts with**: `/// Template rendering examples`
- **Function starts with**: `fn prompt_rendering() -> Vec<PromptMessage> {`
- **Function ends with**: matching closing brace and blank line

#### Delete `prompt_workflows()` function
- **Lines to delete**: 179-228 (50 lines total, after previous deletion will shift up)
- **Function starts with**: `/// Usage workflows`
- **Function starts with**: `fn prompt_workflows() -> Vec<PromptMessage> {`
- **Function ends with**: matching closing brace and blank line

#### Delete `prompt_comprehensive()` function
- **Lines to delete**: 230-459 (230 lines total, after previous deletions will shift up)
- **Function starts with**: `/// Comprehensive guide`
- **Function starts with**: `fn prompt_comprehensive() -> Vec<PromptMessage> {`
- **File ends after this function's closing brace**

**Delete Strategy**: Perform three separate deletion operations, or perform one large deletion of lines 135-459 (324 lines total). The result after all deletions will be approximately 135-150 lines of clean, focused code.

### Step 4: Verify Structure
After deletions, the file structure must be:
- Lines 1-6: Header and imports
- Lines 8-31: PromptGetPrompts struct with updated match statement and prompt_arguments
- Lines 33-34: Empty line and comment header
- Lines 36-73: prompt_basic() function (keep as-is, ~38 lines)
- Lines 75-133: prompt_variables() function (keep as-is, ~59 lines)
- Line 134: Final closing brace

Total expected lines: approximately 135-145 lines

---

## Before and After Code Comparison

### Complete Before State (Key Sections)
```rust
//! Prompt messages for get_prompt tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetPromptPromptArgs;

/// Prompt provider for get_prompt tool
pub struct PromptGetPrompts;

impl PromptProvider for PromptGetPrompts {
    type PromptArgs = GetPromptPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("variables") => prompt_variables(),
            Some("rendering") => prompt_rendering(),      // DELETE
            Some("workflows") => prompt_workflows(),      // DELETE
            _ => prompt_comprehensive(),                   // DELETE
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show examples for (basic, variables, rendering, workflows, comprehensive)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GET_PROMPT
// ============================================================================

/// Basic prompt retrieval
fn prompt_basic() -> Vec<PromptMessage> {
    // ... 38 lines of User/Assistant dialogue about basic retrieval
}

/// Variable substitution patterns
fn prompt_variables() -> Vec<PromptMessage> {
    // ... 59 lines of User/Assistant dialogue about variable substitution
}

/// Template rendering examples        // DELETE THIS FUNCTION
fn prompt_rendering() -> Vec<PromptMessage> {
    // ... 43 lines
}

/// Usage workflows                    // DELETE THIS FUNCTION
fn prompt_workflows() -> Vec<PromptMessage> {
    // ... 50 lines
}

/// Comprehensive guide                // DELETE THIS FUNCTION
fn prompt_comprehensive() -> Vec<PromptMessage> {
    // ... 230 lines with decorative headers
}
```

### Complete After State (Structure)
```rust
//! Prompt messages for get_prompt tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetPromptPromptArgs;

/// Prompt provider for get_prompt tool
pub struct PromptGetPrompts;

impl PromptProvider for PromptGetPrompts {
    type PromptArgs = GetPromptPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("variables") => prompt_variables(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show examples for (basic, variables)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GET_PROMPT
// ============================================================================

/// Basic prompt retrieval
fn prompt_basic() -> Vec<PromptMessage> {
    // ... 38 lines (unchanged)
}

/// Variable substitution patterns
fn prompt_variables() -> Vec<PromptMessage> {
    // ... 59 lines (unchanged)
}
```

---

## Definition of Done

### Measurable Success Criteria

✓ **File Size**: Final file is between 135-150 lines (down from 459 lines)
  - Verify with: `wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/prompt/prompt_get/prompts.rs`

✓ **Scenario Count**: Exactly 2 scenarios remaining (basic, variables)
  - Count function definitions: Should find only `fn prompt_basic()` and `fn prompt_variables()`
  - No references to `prompt_rendering`, `prompt_workflows`, or `prompt_comprehensive`

✓ **Routing Logic**: Match statement has exactly 2 arms
  - One explicit arm: `Some("variables") => prompt_variables(),`
  - One default arm: `_ => prompt_basic(),`

✓ **Prompt Arguments**: Description string updated correctly
  - Description must read: "Scenario to show examples for (basic, variables)"
  - No mention of rendering, workflows, or comprehensive

✓ **No Comprehensive Scenario**: Decorative headers and extensive guide removed
  - Search for decorative header pattern `═══════════════` should return zero matches
  - File no longer contains the 230-line comprehensive prompt function

✓ **Code Integrity**: File compiles without errors
  - All deleted functions are removed cleanly
  - No orphaned function calls remain
  - Struct and trait implementations are complete and valid

✓ **Clean Deletions**: No remnants of deleted code
  - Grep for `prompt_rendering` should return no matches
  - Grep for `prompt_workflows` should return no matches  
  - Grep for `prompt_comprehensive` should return no matches
  - No dangling comments referencing deleted scenarios

### Validation Checklist

- [ ] Open file and verify header, imports, and struct are intact
- [ ] Confirm match statement has exactly 2 arms matching the new pattern
- [ ] Confirm prompt_arguments description is updated to list only "basic, variables"
- [ ] Delete prompt_rendering() function completely (search to confirm deletion)
- [ ] Delete prompt_workflows() function completely (search to confirm deletion)
- [ ] Delete prompt_comprehensive() function completely (search to confirm deletion)
- [ ] Verify file ends cleanly after prompt_variables() function closing brace
- [ ] Run `wc -l` to confirm final size is 135-150 lines
- [ ] Verify no compilation errors: `cargo check` in the package directory

---

## Key Points for Implementation

1. **Preserve Core Content**: The `prompt_basic()` and `prompt_variables()` functions contain high-quality teaching material for AI agents using get_prompt. Keep them exactly as-is, only delete function definitions elsewhere.

2. **Order Matters**: The match statement now has variables as explicit case and basic as default. This means:
   - No scenario specified → prompt_basic (fundamental use case)
   - Scenario="variables" → prompt_variables (advanced variable patterns)

3. **No File Reorganization**: Simply delete the three unwanted functions and update the match/description. Do not reorganize or refactor the remaining code.

4. **Line Number Shifts**: After each deletion, remaining line numbers shift up. Delete from bottom to top (comprehensive first, then workflows, then rendering) to avoid recalculating line numbers.

5. **Clean Deletion**: Remove entire function blocks including:
   - The doc comment (e.g., `/// Template rendering examples`)
   - The function declaration
   - All interior PromptMessage creations
   - The closing brace
   - Any trailing blank lines before the next function

---

## Reference Materials

The prompts.rs file is located at:
`/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/prompt/prompt_get/prompts.rs`

Related files in the same directory:
- `mod.rs`: Module exports (no changes needed)
- `prompt_args.rs`: Argument type definitions (no changes needed)
- `schema.rs`: Input/output schema (no changes needed)

The PromptProvider trait is defined in:
`/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/tool.rs`
