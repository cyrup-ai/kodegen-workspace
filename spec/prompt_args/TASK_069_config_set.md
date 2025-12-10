# TASK 069: Trim config_set

**Tool**: `config_set`
**Complexity**: 2 (Simple)
**Original Size**: 760 lines (5 scenarios + comprehensive)
**Current Size**: 207 lines (2 scenarios)
**Target Size**: 200 lines (1-2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/config/config_set/prompts.rs`

---

## Current State Analysis

### Original Structure (760 lines)
The prompts.rs file contained 5 separate scenario functions plus a default comprehensive guide:

1. **prompt_basic()** - Lines 43-95 (~100 lines)
   - Basic config changes with different value types
   - Shows syntax and common keys
   - Purpose: Teaching simple config_set usage

2. **prompt_security_aware()** - Lines 98-184 (~87 lines) - **DELETED**
   - Use-case scenario covering security implications
   - Warnings about dangerous keys and best practices
   - Not essential for basic understanding

3. **prompt_value_types()** - Lines 187-298 (~112 lines)
   - Value type formats and type matching
   - Examples of string, number, boolean, array, float types
   - Common type errors and how to avoid them
   - Purpose: Teaching correct value type formatting

4. **prompt_workflow()** - Lines 299-497 (~199 lines) - **DELETED**
   - Use-case scenario with inspect-modify-verify workflow
   - 3 detailed examples (timeout, directories, logging)
   - Why the workflow matters
   - Not essential for basic understanding

5. **prompt_comprehensive()** - Lines 498-760 (~263 lines) - **DELETED**
   - Default scenario covering ALL aspects of config_set
   - Duplicate information from basic and value_types
   - Decorative section headers and extensive examples
   - Redundant content when basic + value_types exist

### Routing Match Statement (Lines 16-23)
**Original**:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("security_aware") => prompt_security_aware(),
    Some("value_types") => prompt_value_types(),
    Some("workflow") => prompt_workflow(),
    _ => prompt_comprehensive(),
}
```

**Updated**:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("value_types") => prompt_value_types(),
    _ => prompt_basic(),
}
```

### Prompt Arguments (Lines 25-33)
**Original**:
- Description: "Scenario to show (basic, security_aware, value_types, workflow)"

**Updated**:
- Description: "Scenario to show (basic, value_types)"

### Final State (207 lines)
Successfully trimmed from 760 to 207 lines by:
- Keeping prompt_basic() function (~100 lines)
- Keeping prompt_value_types() function (~107 lines)
- Removing prompt_security_aware() function (87 lines)
- Removing prompt_workflow() function (199 lines)
- Removing prompt_comprehensive() function (263 lines)
- Updating routing logic (5 lines saved)
- Removing decorative headers (3 lines saved)

**Result**: 207 lines total - within 170-220 line target

---

## Implementation Steps

### Step 1: Understand Current State
- Original file has 760 lines with 5 scenarios + comprehensive default
- Scenarios are named: basic, security_aware, value_types, workflow
- Default routing goes to comprehensive() which is the largest function
- Need to reduce to 1-2 scenarios with focus on teaching core concepts

### Step 2: Identify Scenarios to Delete
**Security-Aware Scenario**: This is a use-case scenario showing security implications. While important for advanced users, it duplicates information in the basic scenario and is not essential for core functionality understanding.

**Workflow Scenario**: This is a use-case scenario with 3 detailed examples. The inspect-modify-verify pattern is important but can be learned from basic + value_types. Removes ~199 lines without losing core teaching.

**Comprehensive Scenario**: This is the default fallback that tries to cover everything. It contains extensive duplicate information from basic and value_types, plus decorative headers and complex examples. Can be completely eliminated.

### Step 3: Keep Core Scenarios
**Basic Scenario** (kept): Shows fundamental config_set usage with syntax, parameters, key names, response format, and important notes. Essential for all users.

**Value Types Scenario** (kept): Shows value type formats (string, number, boolean, array, float) with specific examples and common errors. Essential for correct API usage. This is the most complex aspect of config_set.

### Step 4: Update Routing Logic
In the PromptProvider implementation, update the match statement to:
1. Route "basic" to prompt_basic()
2. Route "value_types" to prompt_value_types()
3. Default case (_) now returns prompt_basic() instead of comprehensive
   - This ensures users always get useful content
   - Avoids confusion from 404-style errors for unknown scenarios

### Step 5: Update Prompt Arguments
The prompt_arguments() function must list available scenarios for discovery. Update description from:
- "Scenario to show (basic, security_aware, value_types, workflow)"
To:
- "Scenario to show (basic, value_types)"

### Step 6: Verify Line Count
After deletions:
- Check total line count is between 170-220 (target is 200)
- Verify file structure is intact with proper Rust syntax
- Confirm both prompt_basic() and prompt_value_types() remain complete

---

## Implementation Details

### Functions to Delete (Complete)

**Remove prompt_security_aware() block:**
- Search for: `/// Security-aware configuration modification with warnings`
- Delete from comment through closing `]}`
- Approximately 87 lines including blank lines

**Remove prompt_workflow() block:**
- Search for: `/// Proper inspect-modify-verify workflow`
- Delete from comment through closing `]}`
- Approximately 199 lines including blank lines

**Remove prompt_comprehensive() block:**
- Search for: `/// Comprehensive guide covering all aspects of config_set`
- Delete from comment through closing `]}`
- Approximately 263 lines including blank lines

### Lines to Update

**Match statement (approximately line 16-23):**
- Delete lines referencing security_aware, workflow, comprehensive
- Update default case from `_ => prompt_comprehensive()` to `_ => prompt_basic()`
- Result: 9 lines becomes 5 lines

**Prompt arguments description (approximately line 30):**
- Find: `description: Some("Scenario to show (basic, security_aware, value_types, workflow)"`
- Replace with: `description: Some("Scenario to show (basic, value_types)"`

---

## Successful Completion Criteria

- Line Count: 170-220 total lines (expect ~207)
- Scenarios: Exactly 2 scenarios (basic, value_types)
- Routing: Match statement handles basic and value_types, defaults to basic
- Arguments: Description updated to list only 2 scenarios
- Syntax: File compiles without errors (valid Rust)
- Functions: prompt_basic() and prompt_value_types() complete and intact
- No decorative headers with equals signs
- No use-case scenarios (security_aware, workflow, comprehensive)

---

## Code References

### Current Match Statement Location
File: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/config/config_set/prompts.rs`
Lines: ~16-23 (in PromptProvider implementation)

### Function Locations (Before Trimming)
- Line 43: `fn prompt_basic()` starts
- Line 98: `fn prompt_security_aware()` starts (DELETE)
- Line 187: `fn prompt_value_types()` starts
- Line 299: `fn prompt_workflow()` starts (DELETE)
- Line 498: `fn prompt_comprehensive()` starts (DELETE)
- Line 760: End of file

### Expected Final Structure
```rust
//! Prompt messages for config_set tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::SetConfigValuePromptArgs;

/// Prompt provider for config_set tool
pub struct SetConfigValuePrompts;

impl PromptProvider for SetConfigValuePrompts {
    type PromptArgs = SetConfigValuePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("value_types") => prompt_value_types(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, value_types)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO MODIFY CONFIG SAFELY
// ============================================================================

/// Basic configuration changes with different value types
fn prompt_basic() -> Vec<PromptMessage> {
    // ... (~100 lines - KEEP INTACT)
}

/// Value type formats and type matching
fn prompt_value_types() -> Vec<PromptMessage> {
    // ... (~107 lines - KEEP INTACT)
}
```

---

## Why This Approach Works

### Knowledge Consolidation
- Basic scenario teaches what config_set does and common keys
- Value_types scenario teaches correct formatting (the hardest part)
- Together they cover 95% of typical config_set usage
- Security and workflow patterns can be learned through experimentation

### Reduced Cognitive Load
- Two focused scenarios vs five overlapping scenarios
- Less duplication means easier to maintain
- Users find what they need faster
- Aligns with Complexity 2 (Simple) classification

### Meets Requirements
- 207 lines (within 170-220 target)
- 2 scenarios kept (basic, value_types)
- Default behavior improved (basic instead of comprehensive 404)
- Matches "Complexity 2 template" pattern from PRECURSOR_02

---

## Testing the Result

After completing this task, verify:

1. File exists: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/config/config_set/prompts.rs`
2. Line count: 207 lines (viewable with `wc -l` or file manager)
3. Rust syntax: No compilation errors (run `cargo check` in the package)
4. Routing works: Test with `Some("basic")`, `Some("value_types")`, and `Some("unknown")`
5. Both scenarios complete: Verify prompt_basic() and prompt_value_types() have matching braces

---

## File Path for Verification
**Output File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/config/config_set/prompts.rs`
**Current Status**: TRIMMED (207 lines)
**Date Completed**: 2025-12-05
