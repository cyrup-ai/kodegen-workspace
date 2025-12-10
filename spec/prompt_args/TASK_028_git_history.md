# TASK 028: Trim git_history Prompts

**Tool**: `git_history`
**Complexity**: 2 (Simple)
**Current size**: 169 lines
**Target size**: 80-90 lines after trimming to 2 core scenarios
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/history/prompts.rs`

---

## Current State Analysis

The prompts.rs file contains 169 lines with the following structure:

### Module Structure (Lines 1-32)
- Module documentation comment
- Imports from crate::tool, rmcp::model, and prompt_args
- HistoryPrompts struct definition
- PromptProvider trait implementation with generate_prompts and prompt_arguments methods

### Scenario Functions (Lines 33-169)
1. **prompt_basic()** (Lines 33-47, ~15 lines)
   - Description: Basic scenario showing how to view commit history
   - Use case: Simple commit viewing with max_count parameter
   - Content: User question "How do I view commit history?" with assistant response showing basic JSON examples

2. **prompt_filtering()** (Lines 49-69, ~21 lines)
   - Description: Filtering scenario showing multiple filter options
   - Use cases: Filter by author, date range, specific file, specific branch
   - Content: Multiple JSON examples for different filtering scenarios

3. **prompt_searching()** (Lines 71-87, ~17 lines)
   - Description: Searching scenario for commit message searches (USE-CASE SCENARIO - TO DELETE)
   - Use cases: Search in commit messages, find commits that modified content, combine filters
   - Content: Examples with grep and search parameters
   - Status: MARKED FOR DELETION - this is a specialized use-case scenario

4. **prompt_comprehensive()** (Lines 89-169, ~81 lines)
   - Description: Comprehensive reference covering all parameters and use cases (COMPREHENSIVE SCENARIO - TO DELETE)
   - Content: Complete parameter reference, all examples, and use-case descriptions
   - Status: MARKED FOR DELETION - explicitly identified as comprehensive scenario

### Routing Logic (Lines 14-19)
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("filtering") => prompt_filtering(),
    Some("searching") => prompt_searching(),
    _ => prompt_comprehensive(),
}
```
**Note**: Currently defaults to prompt_comprehensive() which will be deleted.

---

## Implementation Instructions

### Step 1: Remove prompt_searching Function
**Action**: Delete lines 70-87 (inclusive of blank line before and entire function)

**Before**:
```rust
fn prompt_filtering() -> Vec<PromptMessage> {
    // ... content ...
}

fn prompt_searching() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I search commit messages?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Search in commit messages:\\n\\n\
                 ...
            ),
        },
    ]
}
```

**Action**: Delete this entire function and the blank line before it.

---

### Step 2: Remove prompt_comprehensive Function
**Action**: Delete lines 88-169 (entire function definition)

**Before**:
```rust
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I use git_history?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "View detailed commit history:\\n\\n\
                 BASIC USAGE:\\n\
                 ...
                 USE CASES:\\n\
                 ...
            ),
        },
    ]
}
```

**Action**: Delete this entire function (79 lines including closing brace and blank line before function).

---

### Step 3: Update Routing Match Statement
**Location**: Lines 14-19 (within generate_prompts function)

**Before**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("filtering") => prompt_filtering(),
        Some("searching") => prompt_searching(),
        _ => prompt_comprehensive(),
    }
}
```

**After**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("filtering") => prompt_filtering(),
        _ => prompt_basic(),
    }
}
```

**Changes**:
- Remove line: `Some("searching") => prompt_searching(),`
- Change default case from `_ => prompt_comprehensive(),` to `_ => prompt_basic(),`

---

### Step 4: Update prompt_arguments Description
**Location**: Lines 23-29 (within prompt_arguments function)

**Before**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario: basic, filtering, searching".to_string()),
            required: Some(false),
        }
    ]
}
```

**After**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario: basic, filtering".to_string()),
            required: Some(false),
        }
    ]
}
```

**Changes**:
- Update description string from `"Scenario: basic, filtering, searching"` to `"Scenario: basic, filtering"`

---

## Specific Deletions Required

1. **Delete prompt_searching function**: 
   - Start at line 70 (blank line before function)
   - End at line 87 (closing brace of function)
   - Total: 18 lines deleted

2. **Delete prompt_comprehensive function**:
   - Start at line 88 (blank line before function)
   - End at line 169 (closing brace of function and file)
   - Total: 82 lines deleted

3. **Total reduction**: 100 lines removed (169 - 100 = 69 lines final size)

---

## Definition of Done

The task is complete when ALL the following criteria are met:

- [ ] **Scenario Count**: File contains exactly 2 scenario functions (prompt_basic and prompt_filtering)
- [ ] **No prompt_searching**: The prompt_searching function is completely removed
- [ ] **No prompt_comprehensive**: The prompt_comprehensive function is completely removed  
- [ ] **Routing Updated**: Match statement in generate_prompts has only 3 arms (Some("basic"), Some("filtering"), and default)
- [ ] **Default Handler**: Default case returns prompt_basic() instead of prompt_comprehensive()
- [ ] **Prompt Arguments Updated**: The description string lists only "basic, filtering" (not "basic, filtering, searching")
- [ ] **File Compiles**: Run `cargo check` in the kodegen-mcp-schema package to verify no compilation errors
- [ ] **Final Size**: File should be approximately 69-75 lines (reduced from 169 lines)

---

## Code Location Reference

**File path**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/history/prompts.rs`

**Related files** (for understanding context):
- `packages/kodegen-mcp-schema/src/git/history/prompt_args.rs` - Contains GitHistoryPromptArgs struct
- `packages/kodegen-mcp-schema/src/git/history/mod.rs` - Module definition

**Verification command** (after making changes):
```bash
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
cargo check
```

---

## Implementation Notes

- This is a straightforward trimming task with no refactoring required
- The two retained scenarios (basic and filtering) cover the most common use cases
- The default fallback to prompt_basic() ensures graceful handling of unknown scenarios
- No changes to function signatures, trait implementations, or module structure
- All changes are deletions and simple line replacements in routing logic
