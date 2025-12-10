# TASK 064: Trim browser_click

**Tool**: `browser_click`
**Complexity**: 2 (Simple)
**Current Size**: 1000 lines (5 scenarios)
**Target Size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/click/prompts.rs`

---

## Current State Analysis

### Existing Scenarios (5 total, must reduce to 2)

1. **prompt_selectors()** (Lines 44-254, ~210 lines)
   - CSS selector patterns for element targeting
   - Covers ID, class, attribute, text content, hierarchy, nth-child, pseudo-selectors
   - STATUS: KEEP as primary scenario
   - Rationale: Most fundamental for clicking elements, teaches selector priority

2. **prompt_coordinates()** (Lines 256-410, ~155 lines)
   - Absolute vs offset coordinates
   - Canvas/map/SVG use cases
   - STATUS: DELETE
   - Rationale: Specialized use case, less common than timing/waiting

3. **prompt_waiting()** (Lines 412-645, ~234 lines)
   - Dynamic content handling
   - Timing scenarios (AJAX, animations, spinners, modals)
   - wait_timeout_ms, wait_for_clickable, wait_for_navigation
   - STATUS: KEEP as optional/fallback scenario
   - Rationale: Most agents need dynamic content handling; second most common use case

4. **prompt_troubleshooting()** (Lines 647-900+, ~254 lines)
   - Debugging click failures
   - Element not found, not clickable, timeout errors
   - STATUS: DELETE
   - Rationale: Use-case scenario; diagnostic content covered by other scenarios

5. **prompt_comprehensive()** (Lines 902-1000, ~98 lines)
   - Combined guide with all features
   - STATUS: DELETE (explicitly forbidden by task rules)
   - Rationale: Task requires "No comprehensive" - must delete completely

### Routing Logic (Lines 20-27)

Current match statement has 5 cases + default:
```rust
match args.scenario.as_deref() {
    Some("selectors") => prompt_selectors(),
    Some("coordinates") => prompt_coordinates(),
    Some("waiting") => prompt_waiting(),
    Some("troubleshooting") => prompt_troubleshooting(),
    _ => prompt_comprehensive(),
}
```

Must be simplified to 2 cases:
```rust
match args.scenario.as_deref() {
    Some("selectors") => prompt_selectors(),
    _ => prompt_waiting(),
}
```

---

## Step-by-Step Implementation Instructions

### Step 1: Delete Three Unused Scenario Functions

Delete these three functions completely (in order):

**DELETE: prompt_coordinates() function**
- Starts: Line 256 (`fn prompt_coordinates() -> Vec<PromptMessage> {`)
- Ends: Line 410 (closing brace of function)
- Total lines to delete: ~155 lines
- After deletion, prompt_waiting() will move up

**DELETE: prompt_troubleshooting() function**
- Starts: Line 647 (`fn prompt_troubleshooting() -> Vec<PromptMessage> {`)
- Ends: Line 900+ (closing brace of function)
- Total lines to delete: ~254 lines
- After deletion, prompt_comprehensive() will move up

**DELETE: prompt_comprehensive() function**
- Starts: Line 902 (`fn prompt_comprehensive() -> Vec<PromptMessage> {`)
- Ends: Line 1000 (last closing brace in file)
- Total lines to delete: ~98 lines
- After deletion, file ends at prompt_waiting() function

### Step 2: Update PromptProvider Implementation

Replace the match statement (Lines 20-27) with this:

**BEFORE:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("selectors") => prompt_selectors(),
        Some("coordinates") => prompt_coordinates(),
        Some("waiting") => prompt_waiting(),
        Some("troubleshooting") => prompt_troubleshooting(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("selectors") => prompt_selectors(),
        _ => prompt_waiting(),
    }
}
```

Change: Remove 3 scenario branches (coordinates, waiting, troubleshooting), change default from `prompt_comprehensive()` to `prompt_waiting()`.

### Step 3: Update PromptArgument Description

Update the description text in prompt_arguments() (around Line 32):

**BEFORE:**
```rust
description: Some("Scenario to show (selectors, coordinates, waiting, troubleshooting)".to_string()),
```

**AFTER:**
```rust
description: Some("Scenario to show (selectors, waiting)".to_string()),
```

Change: Remove "coordinates, waiting, troubleshooting" references, keep only "selectors, waiting".

### Step 4: Verify Final Structure

After all deletions and updates, file should contain:

1. Module header and imports (Lines 1-10)
2. ClickPrompts struct and PromptProvider impl (Lines 11-40)
3. Helper comment section (Lines 41-43)
4. **KEEP:** prompt_selectors() function (~210 lines)
5. **KEEP:** prompt_waiting() function (~234 lines)
6. EOF (no more functions)

Total expected: 40 + 210 + 234 = ~484 lines

Wait, this is larger than 220 target. This means prompt_selectors() needs condensing. Let me verify the actual target...

Actually, re-reading task: target is "1-2 scenarios (~200 lines total)" means the scenarios combined should be ~200 lines, not the whole file. This means we need to trim individual scenario content.

### Step 4 (REVISED): Condense Scenario Content

The 200-line target applies to the scenario content only, not boilerplate. We need:
- prompt_selectors(): ~100 lines (condense CSS patterns section)
- prompt_waiting(): ~100 lines (condense timing scenarios section)

**For prompt_selectors():**
- Keep User prompt (1-2 lines)
- Keep CSS SELECTOR PATTERNS header and priority rules (~40 lines)
- CONDENSE common examples section from ~30 lines to ~10 lines
- Remove extended decorative sections

**For prompt_waiting():**
- Keep User prompt (1-2 lines)
- Keep HANDLING DYNAMIC CONTENT section (~30 lines)
- Keep COMMON TIMING SCENARIOS section (~35 lines)
- Remove TIMEOUT GUIDELINES and WORKFLOW PATTERNS

This achieves the 170-220 line total target for the file.

---

## Code Examples: Before & After

### Before: Full File Structure
```
Lines 1-10:    Module header + imports
Lines 11-40:   ClickPrompts + PromptProvider impl (with 5 routing cases)
Lines 44-254:  prompt_selectors() - 210 lines
Lines 256-410: prompt_coordinates() - 155 lines [DELETE]
Lines 412-645: prompt_waiting() - 234 lines
Lines 647-900: prompt_troubleshooting() - 254 lines [DELETE]
Lines 902-1000: prompt_comprehensive() - 98 lines [DELETE]

Total: 1000 lines
```

### After: Trimmed File Structure
```
Lines 1-10:    Module header + imports
Lines 11-35:   ClickPrompts + PromptProvider impl (with 2 routing cases)
Lines 37-140:  prompt_selectors() - 100 lines [CONDENSED]
Lines 142-200: prompt_waiting() - 100 lines [CONDENSED]

Total: ~200 lines
```

---

## Specific Modifications Required

### Modification 1: Routing Match Statement
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/click/prompts.rs`
**Lines**: 20-27
**Action**: Replace entire match block
**Old Lines**: 8 lines (including braces)
**New Lines**: 4 lines (including braces)
**Description**: Remove 3 scenario branches, simplify routing

### Modification 2: Delete prompt_coordinates()
**File**: Same
**Lines**: 256-410 (approximate - verify during implementation)
**Action**: Delete entire function including closing brace
**Impact**: Removes ~155 lines

### Modification 3: Delete prompt_troubleshooting()
**File**: Same
**Lines**: 647-900+ (approximate - line numbers shift after deletion #2)
**Action**: Delete entire function including closing brace
**Impact**: Removes ~254 lines

### Modification 4: Delete prompt_comprehensive()
**File**: Same
**Lines**: 902-1000 (approximate - line numbers shift after deletions #2-3)
**Action**: Delete entire function including closing brace
**Impact**: Removes ~98 lines

### Modification 5: Condense prompt_selectors()
**File**: Same
**Lines**: 44-254 (will change to 44-140 after other deletions)
**Action**: Remove unnecessary example sections, keep core patterns
**Target**: Reduce to ~100 lines
**Keep**: User prompt, Assistant prompt with CSS patterns, selector priority, best practices
**Remove**: Excessive example repetitions in "COMMON EXAMPLES" section

### Modification 6: Condense prompt_waiting()
**File**: Same
**Lines**: 412-645 (will shift after earlier deletions)
**Action**: Remove TIMEOUT GUIDELINES and WORKFLOW PATTERNS sections
**Target**: Reduce to ~100 lines
**Keep**: User prompt, HANDLING DYNAMIC CONTENT, COMMON TIMING SCENARIOS
**Remove**: TIMEOUT GUIDELINES (10 lines), WORKFLOW PATTERNS (20 lines), extensive examples

### Modification 7: Update prompt_arguments description
**File**: Same
**Lines**: ~32
**Action**: Update description string
**Change**: Remove "coordinates, waiting, troubleshooting" from description

---

## Definition of Done - Measurable Criteria

The task is complete when ALL of these conditions are met:

1. **Line Count**: File contains 170-220 total lines
   - Verify with: `wc -l packages/kodegen-mcp-schema/src/browser/click/prompts.rs`
   - Expected: 170-220

2. **Scenario Count**: File contains exactly 2 functions
   - prompt_selectors() function exists
   - prompt_waiting() function exists
   - Verify: `grep -c "^fn prompt_" prompts.rs`
   - Expected: 2

3. **No Forbidden Scenarios**: These functions must NOT exist
   - prompt_coordinates() - must be deleted
   - prompt_troubleshooting() - must be deleted
   - prompt_comprehensive() - must be deleted
   - Verify: `grep "fn prompt_" prompts.rs | wc -l`
   - Expected: 2 (only selectors and waiting)

4. **Routing Simplified**: Match statement has exactly 2 cases
   - Case 1: Some("selectors") -> prompt_selectors()
   - Case 2: _ -> prompt_waiting() (new default)
   - Verify: Count lines in match block
   - Expected: 4 lines (including braces)

5. **No Syntax Errors**: File compiles successfully
   - Run: `cd packages/kodegen-mcp-schema && cargo check`
   - Expected: No errors

6. **No Clippy Warnings**: Code passes linting
   - Run: `cd packages/kodegen-mcp-schema && cargo clippy -- -D warnings`
   - Expected: No warnings

---

## Implementation Order

1. Read the entire prompts.rs file carefully
2. Delete prompt_coordinates() function and associated lines
3. Delete prompt_troubleshooting() function and associated lines
4. Delete prompt_comprehensive() function and associated lines
5. Update PromptProvider::generate_prompts() match statement
6. Update prompt_arguments() description string
7. Condense prompt_selectors() by removing excessive examples
8. Condense prompt_waiting() by removing TIMEOUT GUIDELINES and WORKFLOW PATTERNS
9. Run `cargo check` and `cargo clippy` to verify no errors
10. Verify line count is 170-220 with `wc -l`
11. Verify exactly 2 scenario functions with `grep "fn prompt_"`

---

## Success Criteria Summary

MUST HAVE:
- ✓ 170-220 lines total
- ✓ Exactly 2 scenario functions (selectors, waiting)
- ✓ No prompt_coordinates, prompt_troubleshooting, prompt_comprehensive
- ✓ Routing match statement simplified to 2 cases
- ✓ No compilation errors
- ✓ No clippy warnings
- ✓ prompt_arguments description updated

MUST NOT HAVE:
- ✗ Comprehensive scenario
- ✗ Coordinates scenario
- ✗ Troubleshooting scenario
- ✗ More than 2 routing cases
- ✗ Decorative headers for deleted sections
- ✗ Syntax errors after trimming
