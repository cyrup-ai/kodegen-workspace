# TASK 032: Trim git_reset Prompts

**Tool**: `git_reset`
**Complexity**: 2 (Simple)
**Current size**: 659 lines
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/reset/prompts.rs`

---

## Current State Analysis

The `prompts.rs` file contains 659 lines with the following structure:

### File Structure
- **Lines 1-6**: File header and imports
- **Lines 7-36**: `ResetPrompts` struct and `PromptProvider` impl
- **Lines 37-44**: Unused helper comment section (delete)
- **Lines 45-112**: `prompt_unstage()` function (68 lines) - **DELETE**
- **Lines 114-195**: `prompt_soft()` function (82 lines) - **KEEP**
- **Lines 197-327**: `prompt_mixed()` function (131 lines) - **KEEP**
- **Lines 329-489**: `prompt_hard()` function (161 lines) - **DELETE**
- **Lines 491-597**: `prompt_safety()` function (107 lines) - **DELETE**
- **Lines 599-659**: `prompt_comprehensive()` function (61 lines) - **DELETE**

### Current Scenarios (6 total)
1. **unstage** - Use-case scenario showing unstaging specific files - DELETE
2. **soft** - Core reset mode: keeps changes staged - KEEP
3. **mixed** - Core reset mode: keeps changes unstaged (default) - KEEP
4. **hard** - Core reset mode: discards changes (dangerous) - DELETE
5. **safety** - Use-case scenario showing safe practices - DELETE
6. **comprehensive** (default) - Dictionary/reference format - DELETE

### PromptProvider Routing (Lines 18-24)
Current match statement routes to 6 scenarios:
```rust
match args.scenario.as_deref() {
    Some("unstage") => prompt_unstage(),
    Some("soft") => prompt_soft(),
    Some("mixed") => prompt_mixed(),
    Some("hard") => prompt_hard(),
    Some("safety") => prompt_safety(),
    _ => prompt_comprehensive(),
}
```

After trimming, reduce to 2 scenarios with mixed as default.

---

## Implementation Instructions

### Step 1: Update PromptProvider Match Statement

The `generate_prompts` method (lines 18-24) must be simplified to route only soft and mixed scenarios.

**Current routing (7 lines)**:
```rust
match args.scenario.as_deref() {
    Some("unstage") => prompt_unstage(),
    Some("soft") => prompt_soft(),
    Some("mixed") => prompt_mixed(),
    Some("hard") => prompt_hard(),
    Some("safety") => prompt_safety(),
    _ => prompt_comprehensive(),
}
```

**New routing (5 lines)**:
```rust
match args.scenario.as_deref() {
    Some("soft") => prompt_soft(),
    Some("mixed") => prompt_mixed(),
    _ => prompt_mixed(),
}
```

**Why**: Defaults to mixed mode (safest reset mode) when no scenario specified. Removes all deleted scenario branches.

### Step 2: Update PromptArgument Description

The `prompt_arguments` method (line 33) specifies available scenarios.

**Current description**:
```rust
description: Some("Scenario to show (unstage, soft, mixed, hard, safety)".to_string()),
```

**New description**:
```rust
description: Some("Scenario to show (soft, mixed)".to_string()),
```

**Why**: Reflects only the two remaining scenarios.

### Step 3: Delete All Decorative Headers

**Delete lines 25-27**: The large comment block before functions:
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT RESET
// ============================================================================
```

**Replace with minimal comment (1 line)**:
```rust
// Prompt scenarios
```

### Step 4: Delete Five Scenario Functions

Delete the following functions entirely:

1. **prompt_unstage()** - Lines 45-112 (68 lines)
   - Starts: `fn prompt_unstage() -> Vec<PromptMessage> {`
   - Ends: The closing `}` after its last PromptMessage

2. **prompt_hard()** - Lines 329-489 (161 lines)
   - Starts: `fn prompt_hard() -> Vec<PromptMessage> {`
   - Ends: The closing `}` after "Remember: Hard reset is PERMANENT..."

3. **prompt_safety()** - Lines 491-597 (107 lines)
   - Starts: `fn prompt_safety() -> Vec<PromptMessage> {`
   - Ends: The closing `}` after "Remember: When in doubt, stash or branch..."

4. **prompt_comprehensive()** - Lines 599-659 (61 lines)
   - Starts: `fn prompt_comprehensive() -> Vec<PromptMessage> {`
   - Ends: Final `}` of entire file

5. **Delete blank lines** - Remove any blank lines that separated the deleted functions

### Step 5: Keep Only Two Scenario Functions

**Keep prompt_soft()**: Lines 114-195 (82 lines)
- Starts with: `fn prompt_soft() -> Vec<PromptMessage> {`
- Contains: User question "How do I use soft reset..." + Assistant response about soft reset mechanics
- Ends with: Closing `}` 

**Keep prompt_mixed()**: Lines 197-327 (131 lines)
- Starts with: `fn prompt_mixed() -> Vec<PromptMessage> {`
- Contains: User question "How do I use mixed reset..." + Assistant response about mixed reset (default mode)
- Ends with: Closing `}`

These are the only two functions that should remain after all deletions.

---

## Implementation Code Patterns

### Before and After: Full File Structure

**BEFORE** (659 lines):
```rust
//! Prompt messages for git_reset tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitResetPromptArgs;

/// Prompt provider for git_reset tool
...
pub struct ResetPrompts;

impl PromptProvider for ResetPrompts {
    type PromptArgs = GitResetPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("unstage") => prompt_unstage(),    // DELETE
            Some("soft") => prompt_soft(),           // KEEP
            Some("mixed") => prompt_mixed(),         // KEEP
            Some("hard") => prompt_hard(),           // DELETE
            Some("safety") => prompt_safety(),       // DELETE
            _ => prompt_comprehensive(),             // DELETE
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (unstage, soft, mixed, hard, safety)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT RESET
// ============================================================================

/// Unstaging files with git_reset
fn prompt_unstage() -> Vec<PromptMessage> { ... }  // 68 lines - DELETE

/// Soft reset - keep changes staged
fn prompt_soft() -> Vec<PromptMessage> { ... }     // 82 lines - KEEP

/// Mixed reset - keep changes unstaged (default)
fn prompt_mixed() -> Vec<PromptMessage> { ... }    // 131 lines - KEEP

/// Hard reset - DANGEROUS: discards all changes
fn prompt_hard() -> Vec<PromptMessage> { ... }     // 161 lines - DELETE

/// Safe reset practices and recovery options
fn prompt_safety() -> Vec<PromptMessage> { ... }   // 107 lines - DELETE

/// Comprehensive git_reset guide
fn prompt_comprehensive() -> Vec<PromptMessage> { ... }  // 61 lines - DELETE
```

**AFTER** (210 lines total):
```rust
//! Prompt messages for git_reset tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitResetPromptArgs;

/// Prompt provider for git_reset tool
...
pub struct ResetPrompts;

impl PromptProvider for ResetPrompts {
    type PromptArgs = GitResetPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("soft") => prompt_soft(),
            Some("mixed") => prompt_mixed(),
            _ => prompt_mixed(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (soft, mixed)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// Prompt scenarios

/// Soft reset - keep changes staged
fn prompt_soft() -> Vec<PromptMessage> { ... }     // 82 lines

/// Mixed reset - keep changes unstaged (default)
fn prompt_mixed() -> Vec<PromptMessage> { ... }    // 131 lines
```

### Exact Changes to Make

1. **Line 22**: Remove `Some("unstage") => prompt_unstage(),`
2. **Line 24**: Remove `Some("hard") => prompt_hard(),`
3. **Line 25**: Remove `Some("safety") => prompt_safety(),`
4. **Line 26**: Change `_ => prompt_comprehensive(),` to `_ => prompt_mixed(),`
5. **Line 33**: Change description to `"Scenario to show (soft, mixed)"`
6. **Lines 37-44**: Replace large comment block with single line: `// Prompt scenarios`
7. **Lines 45-112**: Delete entire `prompt_unstage()` function and trailing blank line
8. **Lines 329-489**: Delete entire `prompt_hard()` function and trailing blank line
9. **Lines 491-597**: Delete entire `prompt_safety()` function and trailing blank line
10. **Lines 599-659**: Delete entire `prompt_comprehensive()` function

---

## Success Criteria

The implementation is complete when the file meets ALL of these criteria:

1. **Line count**: File is exactly 170-220 lines total
   - Current: 659 lines
   - Target: ~210 lines (6 + 82 + 131 lines for headers/impl/two functions)

2. **Scenario count**: Exactly 2 scenarios remain
   - `prompt_soft()` - for soft reset with staged changes
   - `prompt_mixed()` - for mixed reset with unstaged changes

3. **No comprehensive scenario**: The fallback `prompt_comprehensive()` is completely removed
   - Match statement default: `_ => prompt_mixed(),` (not comprehensive)

4. **Router match statement**: Updated to only reference soft and mixed
   - Exact pattern:
     ```rust
     match args.scenario.as_deref() {
         Some("soft") => prompt_soft(),
         Some("mixed") => prompt_mixed(),
         _ => prompt_mixed(),
     }
     ```

5. **PromptArgument updated**: Reflects only 2 scenarios
   - Description must be: `"Scenario to show (soft, mixed)"`

6. **No decorative headers**: Large separator comment blocks are removed
   - Maximum comment per section: 1 line
   - Example: `// Prompt scenarios`

7. **Functions remain intact**: The content of `prompt_soft()` and `prompt_mixed()` is unchanged
   - No modifications to the actual prompt messages or examples
   - Only the routing/structure changes, not the content

8. **File compiles cleanly**: No Rust compilation errors
   - Run: `cargo check -p kodegen-mcp-schema`
   - Expected: Success with no warnings or errors

---

## Definition of Done

The task is complete when:
- The file contains exactly 2 prompt scenario functions
- All 4 other scenario functions are deleted
- The PromptProvider routing is simplified to 2 arms + default
- The file is 170-220 lines
- No decorative headers remain
- `cargo check` passes for the package
