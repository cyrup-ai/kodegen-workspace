# TASK 101: Trim browser_research prompts

**Tool**: `browser_research`
**Complexity**: 4 (Complex)
**Current size**: 703 lines
**Target size**: 480-487 lines (5 focused scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/research/prompts.rs`

---

## Current State Analysis

### File Structure (703 lines total)

**Header and Struct (lines 1-37)**:
- Module documentation
- Trait imports and type definitions
- `ResearchPrompts` struct definition
- `PromptProvider` trait implementation skeleton

**Scenario Functions (lines 38-703)**:

1. **prompt_basic()** (lines 44-102, ~58 lines)
   - Demonstrates basic research usage patterns
   - Shows when to use browser_research vs simpler tools
   - Covers RESEARCH action with examples
   - Line count: Within acceptable range

2. **prompt_deep_research()** (lines 105-184, ~79 lines)
   - Covers multi-page crawling with max_pages and max_depth parameters
   - Explains depth vs breadth tradeoffs
   - Shows timeout management for long-running research
   - Line count: Matches "Multi-page crawling scenario (100-120 lines)" from task template

3. **prompt_technical_docs()** (lines 187-268, ~81 lines)
   - Demonstrates finding and understanding technical documentation
   - API documentation and framework guide patterns
   - Error troubleshooting with search queries
   - Line count: Targeted scenario, KEEP

4. **prompt_comparison()** (lines 271-364, ~93 lines)
   - Comparative research and technology analysis
   - Feature/performance/ecosystem comparison patterns
   - Year qualifiers and decision-making queries
   - Line count: Matches target range

5. **prompt_monitoring()** (lines 367-486, ~119 lines)
   - Managing long-running research sessions
   - Covers READ, LIST, KILL actions (not just RESEARCH)
   - Timeout strategies and fire-and-forget patterns
   - Line count: Matches "Progress monitoring scenario (100-120 lines)" - perfect fit

6. **prompt_comprehensive()** (lines 489-703, ~214 lines)
   - Complete guide covering all features and strategies
   - Covers RESEARCH, READ, LIST, KILL actions
   - Redundant with other 5 scenarios combined
   - Contains extensive decision trees and best practices
   - **ACTION REQUIRED**: DELETE entirely

### Scenario Summary by Action Coverage

| Scenario | RESEARCH | READ | LIST | KILL | Use Case |
|----------|----------|------|------|------|----------|
| basic | ✓ | | | | Starting simple research |
| deep_research | ✓ | | | | Multi-page crawling, depth control |
| technical_docs | ✓ | | | | Finding technical documentation |
| comparison | ✓ | | | | Comparative analysis |
| monitoring | ✓ | ✓ | ✓ | ✓ | Progress tracking, session management |
| **comprehensive** | ✓ | ✓ | ✓ | ✓ | **REMOVE - Redundant** |

All ACTIONS (RESEARCH, READ, LIST, KILL) are covered by the targeted scenarios.

---

## Step 1: Analyze the generate_prompts() Match Statement

**Current Implementation (lines 16-24)**:

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("deep_research") => prompt_deep_research(),
        Some("technical_docs") => prompt_technical_docs(),
        Some("comparison") => prompt_comparison(),
        Some("monitoring") => prompt_monitoring(),
        _ => prompt_comprehensive(),  // <-- THIS IS THE DEFAULT FALLBACK
    }
}
```

**Issue**: The default fallback `_ => prompt_comprehensive()` points to the function being deleted.

**Required Change**: Change default fallback to use one of the remaining scenarios. Use `prompt_basic()` as the sensible default for users who don't specify a scenario.

---

## Step 2: Delete prompt_comprehensive() Function Entirely

**Location**: Lines 488-703 (216 lines including blank lines and function signature)

**Before**:
```rust
/// Comprehensive guide covering all features and strategies
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using browser_research effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Browser_research is an autonomous research tool that searches the web, crawls multiple pages, extracts content, and generates AI-powered summaries.\n\n\
                 ... [214 lines of content] ... \
                 Remember: Browser_research synthesizes information from multiple sources into a coherent summary with citations. Perfect for comparative analysis, technical research, and exploring new topics!",
            ),
        },
    ]
}
```

**After**: Function removed entirely (0 lines)

**Action**: Delete all lines from line 488 to the end of file (line 703).

---

## Step 3: Update Default Case in generate_prompts()

**Current** (line 23):
```rust
_ => prompt_comprehensive(),
```

**Must Change To**:
```rust
_ => prompt_basic(),
```

**Rationale**: prompt_basic() is the most appropriate default scenario - it explains basic usage patterns and when to use the tool.

**Revised generate_prompts() Function** (final version):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("deep_research") => prompt_deep_research(),
        Some("technical_docs") => prompt_technical_docs(),
        Some("comparison") => prompt_comparison(),
        Some("monitoring") => prompt_monitoring(),
        _ => prompt_basic(),  // <-- UPDATED: Use basic as fallback
    }
}
```

---

## Step 4: Verify prompt_arguments() Documentation

**Location**: Lines 27-36 (no changes needed, but verify)

**Current Content**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, deep_research, technical_docs, comparison, monitoring)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Status**: CORRECT - The description already lists the 5 scenarios WITHOUT comprehensive. No changes needed.

---

## Step 5: Verify Remaining Function Signatures

After deletions, verify these function signatures remain and are accessible via the match statement:

- **Line 44**: `fn prompt_basic() -> Vec<PromptMessage> {` ✓ KEEP
- **Line 105**: `fn prompt_deep_research() -> Vec<PromptMessage> {` ✓ KEEP
- **Line 187**: `fn prompt_technical_docs() -> Vec<PromptMessage> {` ✓ KEEP
- **Line 271**: `fn prompt_comparison() -> Vec<PromptMessage> {` ✓ KEEP
- **Line 367**: `fn prompt_monitoring() -> Vec<PromptMessage> {` ✓ KEEP
- **Line 489**: `fn prompt_comprehensive() -> Vec<PromptMessage> {` ✗ DELETE

---

## Implementation Steps (In Order)

1. Read the file to verify current state
2. Locate the `fn prompt_comprehensive()` function starting at line 489
3. Delete the entire `prompt_comprehensive()` function (lines 489-703)
   - Include the blank line before the function
   - Include the closing brace `}`
4. Change line 23 from `_ => prompt_comprehensive(),` to `_ => prompt_basic(),`
5. Verify the file now ends at line ~487 (approximately)
6. Verify all remaining functions are syntactically valid (no dangling references)
7. Ensure no references to `prompt_comprehensive` remain in the file

---

## Success Criteria

The task is COMPLETE when ALL of these conditions are met:

- **Line Count**: File is exactly 480-487 lines (currently 703, must remove ~216 lines)
  - Confirm with: `wc -l prompts.rs` should return ~487

- **Scenario Functions**: Exactly 5 scenario functions remain
  - ✓ prompt_basic()
  - ✓ prompt_deep_research()
  - ✓ prompt_technical_docs()
  - ✓ prompt_comparison()
  - ✓ prompt_monitoring()
  - ✗ prompt_comprehensive() deleted

- **Default Case**: Lines 22-24 must show
  ```rust
  Some("monitoring") => prompt_monitoring(),
  _ => prompt_basic(),  // <-- Updated from prompt_comprehensive()
  ```

- **Action Coverage**: All four ACTIONS must be covered
  - RESEARCH: ✓ Covered by basic, deep_research, technical_docs, comparison
  - READ: ✓ Covered by monitoring
  - LIST: ✓ Covered by monitoring
  - KILL: ✓ Covered by monitoring

- **No Compilation Errors**: File must compile with no warnings or errors
  - Run: `cargo check` in kodegen-mcp-schema package

- **Prompt Arguments List**: Description at line ~32 must still list all 5 scenarios
  - Should read: "Scenario to show (basic, deep_research, technical_docs, comparison, monitoring)"

---

## Code Patterns: Before vs After

### Match Statement Pattern

**Before** (current):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("deep_research") => prompt_deep_research(),
        Some("technical_docs") => prompt_technical_docs(),
        Some("comparison") => prompt_comparison(),
        Some("monitoring") => prompt_monitoring(),
        _ => prompt_comprehensive(),  // DEFAULT: Too comprehensive, 214 lines
    }
}
```

**After** (target):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("deep_research") => prompt_deep_research(),
        Some("technical_docs") => prompt_technical_docs(),
        Some("comparison") => prompt_comparison(),
        Some("monitoring") => prompt_monitoring(),
        _ => prompt_basic(),  // DEFAULT: Simple, focused, ~58 lines
    }
}
```

### Function Deletions

Delete the entire block from its `///` comment line through the final closing brace:

**Lines to Delete** (exact boundaries):
- Start: Line 488 (blank line before `fn prompt_comprehensive()`)
- End: Line 703 (final `}` of the function)
- Total: 216 lines removed

---

## Decision Tree for Agent Verification

After completing the deletion, verify the file with this decision tree:

1. Does file end at line 487-490? → YES: Continue to step 2
2. Are there exactly 5 function definitions? → YES: Continue to step 3
3. Do all 5 functions match the list above? → YES: Continue to step 4
4. Does generate_prompts() default to `prompt_basic()`? → YES: Continue to step 5
5. Does prompt_arguments() list all 5 scenarios? → YES: SUCCESS

If any answer is NO, review the changes and retry.

---

## Related Files (Reference Only)

- **Tool Implementation**: `packages/kodegen-tools-citescrape/src/main.rs` (uses browser_research)
- **Schema Args**: `packages/kodegen-mcp-schema/src/browser/research/prompt_args.rs`
- **Tool Trait**: `packages/kodegen-mcp-tool/src/lib.rs`

---

## Complexity Notes

This is a Complexity 4 task because:
1. Requires understanding the prompt routing pattern
2. Must preserve 5 functional scenarios without breaking any
3. Needs careful line-by-line deletion
4. Involves updating match statement logic
5. Requires verification of cascading dependencies

This is NOT Complexity 5 because there is no refactoring needed - only deletion and one line change.
