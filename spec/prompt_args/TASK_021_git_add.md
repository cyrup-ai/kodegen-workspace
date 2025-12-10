# TASK 021: Trim git_add

**Tool**: `git_add`
**Complexity**: 2 (Simple)
**Current size**: 735 lines
**Target size**: 170-220 lines (2 scenarios max)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/add/prompts.rs`

---

## Current State Analysis

### File Structure
The `prompts.rs` file contains 735 lines with the following components:
- Lines 1-45: Module header, imports, `AddPrompts` struct, and `PromptProvider` impl
- Lines 46-180: `prompt_specific_files()` function (~130 lines)
- Lines 181-310: `prompt_patterns()` function (~130 lines)
- Lines 311-440: `prompt_all_changes()` function (~130 lines) - USE-CASE
- Lines 441-590: `prompt_partial()` function (~150 lines) - USE-CASE
- Lines 591-740: `prompt_workflows()` function (~150 lines) - USE-CASE
- Lines beyond 740: `prompt_comprehensive()` function (~190 lines) - COMPREHENSIVE

### Current Routing Logic
The `generate_prompts()` function (lines 15-27) uses a match statement with 6 scenarios:
```rust
match args.scenario.as_deref() {
    Some("specific_files") => prompt_specific_files(),
    Some("patterns") => prompt_patterns(),
    Some("all_changes") => prompt_all_changes(),
    Some("partial") => prompt_partial(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

The `prompt_arguments()` function (lines 29-37) documents 5 scenario options in its description.

### Scenario Classification

**KEEP - Core Teaching Scenarios:**
- `prompt_specific_files()`: Teaches fundamental skill of staging individual files (lines 46-180)
- `prompt_patterns()`: Teaches glob pattern-based staging (lines 181-310)

**DELETE - Use-Case and Comprehensive Scenarios:**
- `prompt_all_changes()`: Use-case scenario for `all: true` flag (lines 311-440)
- `prompt_partial()`: Use-case scenario for selective staging strategies (lines 441-590)
- `prompt_workflows()`: Use-case scenario for complete development workflows (lines 591-740)
- `prompt_comprehensive()`: Comprehensive reference guide (lines beyond 740)

---

## Implementation Instructions

### Step 1: Update Routing Logic
Replace the `generate_prompts()` function (lines 15-27) with simplified routing.

**Current code:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("specific_files") => prompt_specific_files(),
        Some("patterns") => prompt_patterns(),
        Some("all_changes") => prompt_all_changes(),
        Some("partial") => prompt_partial(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**Replace with:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("patterns") => prompt_patterns(),
        _ => prompt_specific_files(),
    }
}
```

### Step 2: Update Prompt Arguments Documentation
Replace the `prompt_arguments()` function (lines 29-37).

**Current code:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (specific_files, patterns, all_changes, partial, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Replace with:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (specific_files, patterns)".to_string()),
            required: Some(false),
        }
    ]
}
```

### Step 3: Delete Use-Case Function Blocks
Delete the following function blocks entirely (do NOT keep stubs):

**Delete: `prompt_all_changes()` function**
- Lines 311-440 (entire function including opening vec! and closing bracket)
- This is a use-case scenario showing the `all: true` flag, not a teaching scenario

**Delete: `prompt_partial()` function**
- Lines 441-590 (entire function including opening vec! and closing bracket)
- This is a use-case scenario showing selective staging strategies

**Delete: `prompt_workflows()` function**
- Lines 591-740 (entire function including opening vec! and closing bracket)
- This is a use-case scenario showing complete development workflows

**Delete: `prompt_comprehensive()` function**
- Everything from line 741 onwards (entire function including opening vec! and closing bracket)
- This is the comprehensive reference guide that is too verbose

### Step 4: Trim `prompt_specific_files()` Function
Reduce from ~130 lines to ~80 lines by removing secondary examples and redundant sections.

**Keep these sections:**
- User question (1 line): "How do I add specific files to the staging area?"
- Core examples (20 lines): Single file, multiple files, directory
- Response format (5 lines)
- When to use (4 lines)
- File path rules (4 lines)
- Best practices (7 lines)

**Delete these sections:**
- Add renamed file example (4 lines)
- Rename detection section (5 lines)
- Elaborate on rename preservation (too detailed)

**Target:** Lines 46-125 (keep approximately 80 lines)

### Step 5: Trim `prompt_patterns()` Function
Reduce from ~130 lines to ~85 lines by consolidating examples and removing verbose sections.

**Keep these sections:**
- User question (1 line): "How do I add files using patterns or wildcards?"
- Pattern syntax explanation (15 lines): *, **, ?, [] with examples
- Useful patterns overview (7 lines)
- Practical examples (20 lines): Common use cases with code
- Pattern safety notes (5 lines)

**Delete these sections:**
- "WHEN TO USE PATTERNS" extended section (too verbose)
- "WHEN TO AVOID PATTERNS" extended section (too verbose)
- Redundant best practices paragraph

**Target:** Lines 126-210 (keep approximately 85 lines)

### Step 6: Update Comment Header
Verify the decorative comment header at line 40-45 is removed:
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO STAGE FILES
// ============================================================================
```

Keep simple function comments above each `fn prompt_*()` function only.

---

## Code Changes Summary

### Before
```
Total lines: 735
Scenarios: 6 (specific_files, patterns, all_changes, partial, workflows, comprehensive)
Match arms: 6
Functions: 6
```

### After
```
Total lines: 195-220 (estimated)
Scenarios: 2 (specific_files, patterns)
Match arms: 2 (pattern match + default)
Functions: 2
```

### Function Signature Changes
**prompt_arguments() description changes from:**
```
"Scenario to show (specific_files, patterns, all_changes, partial, workflows)"
```

**To:**
```
"Scenario to show (specific_files, patterns)"
```

---

## Success Criteria

- **Line count**: 170-220 total lines (measured with `wc -l`)
- **Scenario count**: Exactly 2 scenarios (specific_files, patterns)
- **Routing**: Match statement has 2 arms (Some("patterns") and _ default)
- **Deletions**: No traces of all_changes, partial, workflows, or comprehensive scenarios
- **Trims**: Both remaining scenarios reduced by ~40-50% without losing core teaching value
- **Functionality**: File still compiles without errors
- **Documentation**: prompt_arguments() correctly lists only 2 scenarios

---

## Implementation Execution Steps

Execute these modifications in order:

1. Open `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/add/prompts.rs`
2. Update `generate_prompts()` function routing (lines 15-27)
3. Update `prompt_arguments()` documentation (lines 29-37)
4. Delete `prompt_all_changes()` function entirely
5. Delete `prompt_partial()` function entirely
6. Delete `prompt_workflows()` function entirely
7. Delete `prompt_comprehensive()` function entirely
8. Trim `prompt_specific_files()` function (remove named file example and rename section)
9. Trim `prompt_patterns()` function (consolidate use-case sections into brief notes)
10. Verify file compiles: `cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check`
11. Verify line count: `wc -l src/git/add/prompts.rs` should show 170-220 lines
