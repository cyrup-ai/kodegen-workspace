# TASK 034: Trim git_status

**Tool**: `git_status`
**Complexity**: 2 (Simple)
**Target size**: 200 lines (1-2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/status/prompts.rs`

---

## Current State Analysis

### File Overview
- **Current path**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/status/prompts.rs`
- **Current line count**: 761 lines total
- **Current scenarios**: 5 scenarios (basic, interpreting, workflows, options, comprehensive)
- **Current structure**: PromptProvider trait implementation with match-based scenario routing

### Current Scenario Breakdown
The file contains these functions with exact line ranges:

| Scenario | Lines | Content |
|----------|-------|---------|
| prompt_basic() | 41-96 (56 lines) | Basic status checking and understanding output |
| prompt_interpreting() | 97-224 (128 lines) | Understanding and interpreting status output |
| prompt_workflows() | 225-378 (154 lines) | **DELETE** - Status in git workflows |
| prompt_options() | 380-530 (151 lines) | **DELETE** - Status options and advanced usage |
| prompt_comprehensive() | 532-761 (230 lines) | **DELETE** - Complete guide with decorative headers |

**Lines 1-40**: Module header, imports, StatusPrompts struct, PromptProvider impl (KEEP)

### Current Match Statement (Lines 18-24)
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("interpreting") => prompt_interpreting(),
    Some("workflows") => prompt_workflows(),
    Some("options") => prompt_options(),
    _ => prompt_comprehensive(),
}
```

### Current prompt_arguments() (Lines 26-33)
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, interpreting, workflows, options)".to_string()),
            required: Some(false),
        }
    ]
}
```

---

## Implementation Instructions

### Step 1: Update Match Statement in PromptProvider Implementation

**Location**: Lines 18-24

Replace the current match statement with a simplified version that only routes 2 scenarios:

```rust
// BEFORE (5 scenarios):
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("interpreting") => prompt_interpreting(),
    Some("workflows") => prompt_workflows(),
    Some("options") => prompt_options(),
    _ => prompt_comprehensive(),
}

// AFTER (2 scenarios):
match args.scenario.as_deref() {
    Some("interpreting") => prompt_interpreting(),
    _ => prompt_basic(),
}
```

**What this does**: Routes requests to interpreting scenario if explicitly requested, otherwise defaults to basic scenario.

### Step 2: Update prompt_arguments() Function Description

**Location**: Lines 26-33, specifically line 31

Change the description string from listing all 4 scenarios to only the 2 remaining scenarios:

```rust
// BEFORE:
description: Some("Scenario to show (basic, interpreting, workflows, options)".to_string()),

// AFTER:
description: Some("Scenario to show (basic, interpreting)".to_string()),
```

**What this does**: Updates the help text to accurately reflect only the available scenarios.

### Step 3: Delete prompt_workflows() Function

**Location**: Lines 225-378 (entire function)

Delete the entire `fn prompt_workflows()` function. This function contains workflow examples and integration patterns. Remove all 154 lines.

### Step 4: Delete prompt_options() Function

**Location**: Lines 380-530 (entire function)

Delete the entire `fn prompt_options()` function. This function contains status options and advanced usage documentation. Remove all 151 lines.

### Step 5: Delete prompt_comprehensive() Function

**Location**: Lines 532-761 (entire function)

Delete the entire `fn prompt_comprehensive()` function. This function is the "complete guide" with decorative section headers (lines of equals signs). Remove all 230 lines that comprise this function.

### Step 6: Verify Final Structure

After all edits, the file structure should be:

```
Lines 1-16:      Module header and imports
Lines 17-40:     StatusPrompts struct and PromptProvider impl
Line 41:         Comment separator
Lines 42-97:     prompt_basic() function (56 lines) - UNCHANGED
Lines 98-225:    prompt_interpreting() function (128 lines) - UNCHANGED
Total:           ~215 lines
```

---

## Before and After Code Examples

### Complete PromptProvider Implementation - BEFORE
```rust
impl PromptProvider for StatusPrompts {
    type PromptArgs = GitStatusPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("interpreting") => prompt_interpreting(),
            Some("workflows") => prompt_workflows(),
            Some("options") => prompt_options(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, interpreting, workflows, options)".to_string()),
                required: Some(false),
            }
        ]
    }
}
```

### Complete PromptProvider Implementation - AFTER
```rust
impl PromptProvider for StatusPrompts {
    type PromptArgs = GitStatusPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("interpreting") => prompt_interpreting(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, interpreting)".to_string()),
                required: Some(false),
            }
        ]
    }
}
```

---

## What to Keep and Delete

### KEEP - Do Not Modify
- Module-level documentation comment (`//! Prompt messages for git_status tool`)
- All imports (use statements)
- StatusPrompts struct definition
- PromptProvider trait implementation structure
- prompt_basic() function (lines 41-96): Basic status checking scenario - 56 lines
- prompt_interpreting() function (lines 97-224): Understanding status output scenario - 128 lines
- The comment section between impl and first function

### DELETE - Remove Entirely
- prompt_workflows() function (lines 225-378): Workflow integration examples - 154 lines
- prompt_options() function (lines 380-530): Advanced options and usage - 151 lines
- prompt_comprehensive() function (lines 532-761): Complete guide with decorative headers and =====separator lines - 230 lines

---

## Definition of Done

The task is complete when ALL of the following measurable criteria are met:

1. **Line Count**: File contains 170-220 total lines (targeting ~215 lines)
   - Count from `head -1` showing line count or manual line verification

2. **Scenario Count**: Exactly 2 scenarios remain
   - Only `prompt_basic()` and `prompt_interpreting()` functions exist
   - Functions at end of file (no other functions below)

3. **Match Statement**: Simplified routing with 2 arms
   - Line pattern: `Some("interpreting") => prompt_interpreting(),`
   - Line pattern: `_ => prompt_basic(),` (default case)
   - No other Some arms for workflows, options, or comprehensive

4. **Help Text Updated**: prompt_arguments description reflects 2 scenarios
   - Description string contains: `"Scenario to show (basic, interpreting)"`
   - Does NOT contain: "workflows", "options"

5. **No Deleted Content Remains**:
   - No `fn prompt_workflows()` function exists
   - No `fn prompt_options()` function exists
   - No `fn prompt_comprehensive()` function exists
   - No decorative header lines (===== separator lines)

6. **Code Compiles**:
   - File is valid Rust syntax
   - PromptProvider implementation is complete
   - Both prompt functions return Vec<PromptMessage>

7. **Documentation Accurate**:
   - Module-level comments still describe git_status tool
   - PromptProvider struct comment unchanged
   - Function comments for prompt_basic() and prompt_interpreting() unchanged

---

## Execution Order

Execute the edits in this exact sequence:

1. First: Update the match statement (Step 1) - smallest, lowest risk change
2. Second: Update prompt_arguments description (Step 2) - no structural impact
3. Third: Delete prompt_workflows() (Step 3) - 154 lines removed
4. Fourth: Delete prompt_options() (Step 4) - 151 lines removed
5. Fifth: Delete prompt_comprehensive() (Step 5) - 230 lines removed
6. Finally: Verify final structure (Step 6) - confirm file is correct size and compiles

---

## Technical Context

### PromptProvider Trait
The PromptProvider trait (from `crate::tool::PromptProvider`) requires:
- `type PromptArgs`: The argument type for this prompt (GitStatusPromptArgs)
- `fn generate_prompts()`: Routes arguments to appropriate scenario function
- `fn prompt_arguments()`: Defines what arguments are available

### GitStatusPromptArgs
Located at `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/status/prompt_args.rs`

Structure:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitStatusPromptArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
```

The `scenario` field accepts: "basic", "interpreting", or None (defaults to basic)

### PromptMessage Structure
Each function returns `Vec<PromptMessage>` with alternating User/Assistant messages:
- User: Question about how to use git_status
- Assistant: Detailed answer with examples

Both prompt_basic() and prompt_interpreting() follow this exact pattern - DO NOT MODIFY CONTENT.

---

## Expected Result

After completion, the prompts.rs file will be a lean, focused guide with:

1. **Basic Scenario** (56 lines): How to check repository status
   - Covers: Basic checking, response structure, when to use, reading output

2. **Interpreting Scenario** (128 lines): How to understand status output
   - Covers: File states, status codes, ahead/behind, detailed field meanings, common scenarios, decision tree

3. **Streamlined Routing** (2-arm match): Default to basic, explicit routing to interpreting

This provides AI agents with essential git_status knowledge in 1-2 scenarios (~200 lines) instead of 5 scenarios (761 lines), meeting the ~3x reduction target while preserving core functionality.

---

## Reference

See **PRECURSOR_02_fs_read_file.md** for Complexity 2 template pattern.
