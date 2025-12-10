# TASK 023: Trim git_branch_list Prompts

**Tool**: `git_branch_list`
**Complexity**: 2 (Simple)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/branch_list/prompts.rs`
**Current size**: 834 lines
**Target size**: 170-220 lines (target: ~200 lines)

---

## Current State Analysis

### File Structure (before changes)
The `prompts.rs` file currently contains:

| Component | Lines | Content |
|-----------|-------|---------|
| Imports + struct definition | 1-44 | BranchListPrompts struct, PromptProvider impl, match statement with 5 scenarios |
| prompt_basic() function | 45-155 | ~111 lines - Basic branch listing with 80+ lines of explanations |
| prompt_remote() function | 157-293 | ~137 lines - Remote branch operations |
| prompt_filtering() function | 295-483 | ~189 lines - Filtering by merge status, patterns, commits |
| prompt_analysis() function | 485-779 | ~295 lines - Branch health analysis and maintenance workflows |
| prompt_comprehensive() function | 781-834 | ~54 lines - Complete guide covering all features |

### Current Scenarios (5 total)
1. **basic** - Essential branch listing operations
2. **remote** - Remote branch tracking and relationships
3. **filtering** - Filter branches by merge status, patterns, commits (USE-CASE - DELETE)
4. **analysis** - Branch health analysis (USE-CASE - DELETE)
5. **comprehensive** (default) - Complete feature guide (DELETE - reduces to single scenario)

### Match Statement (Current - Lines 19-25)
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("remote") => prompt_remote(),
    Some("filtering") => prompt_filtering(),
    Some("analysis") => prompt_analysis(),
    _ => prompt_comprehensive(),
}
```

---

## Implementation Instructions

You MUST follow these steps in exact order. Do NOT skip steps.

### Step 1: Delete Unused Scenario Functions

Delete the entire `prompt_filtering()` function (lines 295-483 in current file):
- Start: Line 295 begins with `/// Filtering scenarios`
- End: Line 483 ends with the closing brace `}` of the function
- This removes ~189 lines including the user/assistant pair for filtering scenarios
- This scenario covers use-case filtering by merge status and patterns - DELETE per requirements

Delete the entire `prompt_analysis()` function (lines 485-779 in current file):
- Start: Line 485 begins with `/// Analysis workflows`
- End: Line 779 ends with the closing brace `}` of the function
- This removes ~295 lines of analysis and maintenance workflows
- This is a use-case focused scenario - DELETE per requirements

Delete the entire `prompt_comprehensive()` function (lines 781-834 in current file):
- Start: Line 781 begins with `/// Comprehensive guide covering all scenarios`
- End: Line 834 ends with the closing brace `}` of the function
- This removes ~54 lines of comprehensive guide
- This function is referenced by the current default branch - DELETE per requirements

**After Step 1**: File should have only `prompt_basic()` and `prompt_remote()` functions remaining, reducing from 834 to approximately 300 lines.

### Step 2: Trim prompt_basic() Function

The `prompt_basic()` function currently has excessive explanatory text. Trim it to keep ONLY:

**KEEP these sections** (examples and core parameter info):
- The User message (line ~46)
- The Assistant message START (line ~48)
- Sections: "LISTING BRANCHES" with numbered examples (1-4)
- Section: "INTERPRETING OUTPUT" (brief, ~6 lines)
- Section: "PARAMETERS" (reduce to essential params only)
- Section: "BRANCH NAMING CONVENTIONS" (keep but reduce to 4-5 lines)
- Section: "USE CASES" (reduce to 2-3 critical use cases)

**DELETE from prompt_basic()** (verbose/redundant content):
- Decorative header sections with many "=" symbols
- The "COMMON PATTERNS" section (lines ~103-121) - DELETE (redundant with examples)
- Any sections with "BEST PRACTICES" (not in basic but if found, delete)
- Reduce multi-line explanations of each parameter to single-line descriptions

**Target for prompt_basic()**: Reduce from ~111 lines to ~70 lines

### Step 3: Trim prompt_remote() Function

The `prompt_remote()` function has extensive workflow examples. Trim to keep:

**KEEP these sections** (examples and core remote concepts):
- The User message
- The Assistant message START
- Section: "REMOTE BRANCHES" with examples 1-4 showing:
  - List remote branches command with response
  - All branches (local + remote) with response
  - Tracking relationships concept
  - Response examples with "upstream", "ahead", "behind" fields
- Section: "WORKFLOW EXAMPLES" - REDUCE from 5 examples to 2 key examples:
  - Keep: "Check remote branches before pulling"
  - Keep: "Find branches to checkout"
  - DELETE: "Compare local vs remote", "After pushing new branch"

**DELETE from prompt_remote()** (verbose/redundant content):
- "REMOTE BRANCH PATTERNS" section (can infer from examples)
- "MULTIPLE REMOTES" section (lines ~299-315) - DELETE entirely
- "WORKFLOW EXAMPLES" items 3-5 (keep only first 2)
- Reduce "BEST PRACTICES" from 6 items to 3 items
- Remove decorative "====" header lines
- Reduce "INTERPRETING REMOTE OUTPUT" section to 3-4 lines

**Target for prompt_remote()**: Reduce from ~137 lines to ~85 lines

### Step 4: Update PromptProvider Match Statement

Replace the current match statement (lines 19-25) with:

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("remote") => prompt_remote(),
        _ => prompt_basic(),
    }
}
```

Changes made:
- Remove arms for "filtering", "analysis"
- Change default branch from `prompt_comprehensive()` to `prompt_basic()`
- This ensures any invalid scenario falls back to basic (safest option)

### Step 5: Update prompt_arguments() Function

Locate and update the `prompt_arguments()` function (currently lines 27-35). Change the description:

FROM:
```rust
description: Some("Scenario to show (basic, remote, filtering, analysis)".to_string()),
```

TO:
```rust
description: Some("Scenario to show (basic, remote)".to_string()),
```

This documents that only 2 scenarios are available.

### Step 6: Update File Header Comment

Verify the file header comment (lines 1-2) remains accurate. Current should be:
```rust
//! Prompt messages for git_branch_list tool
```

No changes needed - this is generic and remains accurate.

---

## Code Patterns: Before and After

### Match Statement Pattern
**BEFORE** (5 arms + default):
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("remote") => prompt_remote(),
    Some("filtering") => prompt_filtering(),
    Some("analysis") => prompt_analysis(),
    _ => prompt_comprehensive(),
}
```

**AFTER** (2 arms + default):
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("remote") => prompt_remote(),
    _ => prompt_basic(),
}
```

### prompt_arguments() Pattern
**BEFORE**:
```rust
description: Some("Scenario to show (basic, remote, filtering, analysis)".to_string()),
```

**AFTER**:
```rust
description: Some("Scenario to show (basic, remote)".to_string()),
```

### Function Deletion Pattern
For each function to delete (filtering, analysis, comprehensive), delete from the function signature line through the final closing brace `}`. Example:

```rust
// DELETE ALL OF THIS:
/// Filtering scenarios
fn prompt_filtering() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I filter branch lists...",
            ),
        },
        // ... many more lines ...
    ]
}
```

Delete the complete function including the doc comment above it.

---

## Specific Trimming Details

### prompt_basic() Content to Keep
After trimming, `prompt_basic()` must have:
- User message: "How do I list branches in a Git repository?"
- Core examples:
  - Example 1: List local branches with response showing branches array
  - Example 2: Show current branch indicator
  - Example 3: List with verbose flag
- Compact parameter descriptions:
  - path (required)
  - verbose (optional, shows commit info)
  - all (optional, include remote)
  - remote (optional, remote only)
- Branch naming conventions (brief: main, develop, feature/*, fix/*, hotfix/*, release/*, test/*)
- Essential use cases (2-3 maximum):
  - See what branches exist
  - Find branch names for checkout
  - Identify current working branch

**Total for prompt_basic()**: ~70 lines maximum

### prompt_remote() Content to Keep
After trimming, `prompt_remote()` must have:
- User message: "How do I list remote branches and track branch relationships?"
- Core examples:
  - Example 1: List remote branches only with response
  - Example 2: All branches (local + remote) with response showing upstream field
  - Example 3: Tracking relationship concept with ahead/behind explanation
  - Example 4: After fetch operation
- Reduced workflow examples (2 key ones):
  - Check remote branches before pulling
  - Find branches to checkout
- Essential best practices (3-4 maximum):
  - Use remote: true to see remote-only branches
  - Use all: true to see complete picture
  - Check upstream field for tracking relationships
  - Fetch before listing remotes for latest state

**Total for prompt_remote()**: ~85 lines maximum

---

## Line Count Verification (Measurable Success)

### Calculation Methodology
After completing all 6 steps, count the final line count:
- Use `wc -l prompts.rs` command in terminal, or
- Open file and check last line number in your editor

### Expected Final Structure
```
Lines 1-25:   Imports, struct definition, PromptProvider impl, match statement
Lines 26-35:  prompt_arguments() function
Lines 36-105: prompt_basic() function (70 lines)
Lines 106-190: prompt_remote() function (85 lines)
Total: ~190 lines (within target range 170-220)
```

### Success Criteria (MUST MEET ALL)
- ✓ Final line count is 170-220 lines (measure with `wc -l`)
- ✓ Exactly 2 scenario functions: `prompt_basic()` and `prompt_remote()`
- ✓ No `prompt_filtering()` function exists
- ✓ No `prompt_analysis()` function exists
- ✓ No `prompt_comprehensive()` function exists
- ✓ Match statement has exactly 2 arms plus default
- ✓ Default branch is `prompt_basic()` not `prompt_comprehensive()`
- ✓ prompt_arguments() lists only "basic, remote" in description
- ✓ No decorative header lines with many "=" symbols remain
- ✓ No "BEST PRACTICES" sections in scenarios (unless very brief)
- ✓ No "COMPREHENSIVE" or "FILTERING" or "ANALYSIS" in any function comments
- ✓ File compiles without errors (functions referenced in match statement exist)

---

## Verification Steps

After completing all edits, verify:

1. **Open file in editor**: Confirm you can view the complete file without scrolling indicating ~190 lines
2. **Search for deleted functions**: Search for "prompt_filtering" - should have 0 results
3. **Search for deleted functions**: Search for "prompt_analysis" - should have 0 results
4. **Search for deleted functions**: Search for "prompt_comprehensive" - should have 0 results
5. **Verify match statement**: Lines ~19-25 should show only 2 "Some()" arms
6. **Verify default**: Default branch should call `prompt_basic()`
7. **Check prompt_arguments()**: Description should mention only "basic, remote"
8. **Compile test**: Run `cargo check` in `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/` to ensure no compilation errors

---

## Key Files Referenced

- **Main file to edit**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/branch_list/prompts.rs`
- **Related file** (do NOT edit): `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/branch_list/prompt_args.rs` - defines GitBranchListPromptArgs struct used in PromptProvider trait
- **Test file** (if needed): Check `packages/kodegen-mcp-schema/tests/` for any branch_list tests

---

## Implementation Approach

This is a straightforward **deletion and trimming** task requiring:
1. Delete 3 complete functions (filtering, analysis, comprehensive)
2. Trim 2 functions (remove verbose sections, keep core examples)
3. Update 1 match statement (2 arms instead of 5)
4. Update 1 string literal (scenario list)

No new code is being added. Only removal and reduction of existing content. This reduces cognitive load and keeps functionality focused on essential scenarios.

---

## Success Definition - Done When

The task is complete when:
1. All 6 implementation steps are finished
2. All success criteria are met
3. File line count is verified as 170-220 lines
4. `cargo check` passes without compilation errors
5. Match statement only handles "basic" and "remote" scenarios
6. No references to deleted functions (filtering, analysis, comprehensive) remain
