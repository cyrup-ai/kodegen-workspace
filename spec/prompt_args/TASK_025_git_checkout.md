# TASK 025: Trim git_checkout Prompts

**Tool**: `git_checkout`
**Complexity**: 2 (Simple)
**Current size**: 968 lines
**Target size**: 200 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/checkout/prompts.rs`

---

## Current State Analysis

### File Overview
- **Total lines**: 968 lines of Rust source code
- **Structure**: 6 scenario functions + 1 routing match statement + 1 argument descriptor
- **Current scenarios**: switch_branch, create_branch, restore_files, detached, workflows, comprehensive

### Current Scenarios (BEFORE)

| Scenario | Lines | Type | Status |
|----------|-------|------|--------|
| `prompt_switch_branch()` | Lines 54-120 (~67 lines) | Basic branch switching | KEEP |
| `prompt_create_branch()` | Lines 122-296 (~175 lines) | Use-case scenario | DELETE |
| `prompt_restore_files()` | Lines 298-358 (~61 lines) | Use-case scenario | DELETE |
| `prompt_detached()` | Lines 360-487 (~128 lines) | Core concept (detached HEAD) | KEEP (trim to 80-100 lines) |
| `prompt_workflows()` | Lines 489-728 (~240 lines) | Comprehensive workflows | DELETE |
| `prompt_comprehensive()` | Lines 730-968 (~239 lines) | Comprehensive guide | DELETE |

### Routing Implementation (BEFORE)

**Location**: Lines 18-25 in `impl PromptProvider for GitCheckoutPrompts`

Current match statement with 5 explicit cases + 1 default:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("switch_branch") => prompt_switch_branch(),
        Some("create_branch") => prompt_create_branch(),
        Some("restore_files") => prompt_restore_files(),
        Some("detached") => prompt_detached(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

### Argument Descriptor (BEFORE)

**Location**: Lines 27-34

Current implementation lists all 5 scenarios:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (switch_branch, create_branch, restore_files, detached, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

---

## Implementation Instructions

### Step 1: Delete Non-Core Scenario Functions

Delete the following 4 functions entirely (they are use-case and comprehensive scenarios):

1. **Delete `prompt_create_branch()`**
   - Lines 122-296
   - This is a use-case scenario (branch creation)
   - Covered by switch_branch basic usage

2. **Delete `prompt_restore_files()`**
   - Lines 298-358
   - This is a use-case scenario (file restoration)
   - Not core checkout functionality

3. **Delete `prompt_workflows()`**
   - Lines 489-728
   - This is a comprehensive workflow collection
   - Too much detail for Complexity 2 task

4. **Delete `prompt_comprehensive()`**
   - Lines 730-968
   - This is the main comprehensive guide
   - Exceeds target scope

After deletion, keep only:
- `prompt_switch_branch()` (basic usage)
- `prompt_detached()` (core concept with trim)

### Step 2: Trim prompt_detached() Function

**Current state**: Lines 360-487 (~128 lines)
**Target state**: 80-100 lines total

The function should retain core detached HEAD concepts but trim verbose examples:

**KEEP these sections**:
- What IS detached HEAD definition (WHY section)
- VISUAL REPRESENTATION (helps understanding)
- WHEN DETACHED HEAD HAPPENS (causes)
- SAFE USES vs DANGEROUS USES (critical distinction)
- Saving work from detached HEAD (recovery)
- Exiting detached HEAD (critical task)
- CHECKING IF DETACHED (status check)

**TRIM/REMOVE these sections**:
- Most specific code examples (reduce from 20+ examples to 5-6 essential ones)
- Verbose "COMMON SCENARIOS" section (Testing Historical Bug, Building Release Tag, Creating Hotfix) - keep only 1-2
- Detailed "BEST PRACTICES" section (reduce to 3-4 bullet points)
- Repeating example patterns that say same thing in different ways

**Trimming target**: Remove approximately 30-40 lines to bring it from 128 to 90-100 lines

### Step 3: Update Match Statement

**Location**: Lines 18-25

**Replace with** (new 3-case match):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("switch_branch") => prompt_switch_branch(),
        Some("detached") => prompt_detached(),
        _ => prompt_switch_branch(),
    }
}
```

**Changes**:
- Remove lines for: create_branch, restore_files, workflows
- Change default from `prompt_comprehensive()` to `prompt_switch_branch()`
- Result: 6 lines instead of 8 lines (handles 2 explicit scenarios + default)

### Step 4: Update prompt_arguments() Descriptor

**Location**: Lines 27-34

**Replace with**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (switch_branch, detached)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Changes**:
- Update description string from 5 scenarios to just: "switch_branch, detached"
- Remove references to: create_branch, restore_files, workflows
- Single line change with significant scope reduction

### Step 5: Verify Structure

After all changes, the file structure should be:

```rust
//! Prompt messages for git_checkout tool
// (imports and doc comments)

pub struct GitCheckoutPrompts;

impl PromptProvider for GitCheckoutPrompts {
    type PromptArgs = GitCheckoutPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        // 2-case match statement
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        // Updated to reflect 2 scenarios only
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT CHECKOUT
// ============================================================================

/// Switching between existing branches
fn prompt_switch_branch() -> Vec<PromptMessage> {
    // ~67-100 lines: User question + Assistant answer
}

/// Understanding detached HEAD state
fn prompt_detached() -> Vec<PromptMessage> {
    // ~80-100 lines (trimmed): User question + Assistant answer
}
```

---

## Code Patterns: Before/After

### Routing Pattern

**BEFORE** (5 explicit + 1 default):
```rust
match args.scenario.as_deref() {
    Some("switch_branch") => prompt_switch_branch(),
    Some("create_branch") => prompt_create_branch(),
    Some("restore_files") => prompt_restore_files(),
    Some("detached") => prompt_detached(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

**AFTER** (2 explicit + 1 default):
```rust
match args.scenario.as_deref() {
    Some("switch_branch") => prompt_switch_branch(),
    Some("detached") => prompt_detached(),
    _ => prompt_switch_branch(),
}
```

### Arguments Pattern

**BEFORE**:
```rust
description: Some("Scenario to show (switch_branch, create_branch, restore_files, detached, workflows)".to_string()),
```

**AFTER**:
```rust
description: Some("Scenario to show (switch_branch, detached)".to_string()),
```

### Detached Function Trimming Pattern

The detached HEAD function has this structure:

```rust
fn prompt_detached() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What is detached HEAD and how do I work with it?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                // ASSISTANT RESPONSE - THIS IS WHAT TO TRIM
                // Currently ~1000+ characters across multiple sections
                // Target: ~600-700 characters to achieve 90-100 line total
            ),
        },
    ]
}
```

To trim, condense the assistant's response by:
- Removing verbose example code blocks (keep 1-2 key examples)
- Consolidating repeated concepts
- Shortening the "COMMON SCENARIOS" from 3 to 1
- Reducing "BEST PRACTICES" from 6 items to 3
- Keeping all section headers but making content more concise

---

## Deletion Checklist

Execute these deletions in order:

- [ ] Delete lines 122-296: `fn prompt_create_branch()` function
- [ ] Delete lines 298-358: `fn prompt_restore_files()` function (line numbers will shift after first deletion)
- [ ] Delete lines 489-728: `fn prompt_workflows()` function (line numbers will shift)
- [ ] Delete lines 730-968: `fn prompt_comprehensive()` function (line numbers will shift)

**Alternative**: Delete all four functions at once by identifying their exact content and removing with a single edit operation.

---

## Testing & Verification

### Line Count Validation

After completion, verify:
```bash
wc -l packages/kodegen-mcp-schema/src/git/checkout/prompts.rs
# Should output: 170-220 (target range)
```

### Scenario Function Validation

Verify only 2 functions exist:
```bash
grep -n "^fn prompt_" packages/kodegen-mcp-schema/src/git/checkout/prompts.rs
# Should output exactly 2 functions:
# prompt_switch_branch()
# prompt_detached()
```

### Match Statement Validation

Verify routing is simplified:
```bash
grep -A 4 "fn generate_prompts" packages/kodegen-mcp-schema/src/git/checkout/prompts.rs | grep "Some("
# Should output exactly 2 lines with Some("
```

### Argument Description Validation

Verify descriptor lists only 2 scenarios:
```bash
grep -o "switch_branch[^\"]*" packages/kodegen-mcp-schema/src/git/checkout/prompts.rs | head -1
# Should show: switch_branch, detached (no other scenarios)
```

### Syntax Validation

Verify Rust code compiles:
```bash
cd packages/kodegen-mcp-schema && cargo check
# Should succeed with no errors
```

---

## Success Criteria (Definition of Done)

All of the following MUST be satisfied:

1. **Total lines**: 170-220 lines (down from 968)
   - Measured with `wc -l`
   - Include all imports, comments, and code

2. **Scenario count**: Exactly 2 scenarios
   - `prompt_switch_branch()` function exists
   - `prompt_detached()` function exists
   - NO create_branch, restore_files, workflows, or comprehensive functions

3. **Deleted content**: All of these MUST be completely removed
   - `fn prompt_create_branch()` - entirely deleted
   - `fn prompt_restore_files()` - entirely deleted
   - `fn prompt_workflows()` - entirely deleted
   - `fn prompt_comprehensive()` - entirely deleted

4. **Routing match statement**: Updated to handle 2 scenarios
   - Lines matching: `Some("switch_branch") => prompt_switch_branch(),`
   - Lines matching: `Some("detached") => prompt_detached(),`
   - Default case returns to `prompt_switch_branch()`
   - No mention of create_branch, restore_files, workflows, comprehensive

5. **Argument descriptor**: Updated in `prompt_arguments()`
   - Description string contains exactly: "switch_branch, detached"
   - No mention of: create_branch, restore_files, workflows
   - Matches count of actual scenarios (2)

6. **Detached function trimmed**: Reduced from 128 to 80-100 lines
   - Core concepts preserved (definition, visual, when/why)
   - Example code reduced but present
   - All section headers retained
   - Verbose examples and patterns consolidated

7. **Code compiles**: No Rust compilation errors
   - Run `cd packages/kodegen-mcp-schema && cargo check`
   - Zero warnings and errors
   - File is syntactically valid

8. **Helper comment preserved**: Keep the section header
   - Retain: `// ============================================================================`
   - Retain: `// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT CHECKOUT`
   - This explains the file's purpose
