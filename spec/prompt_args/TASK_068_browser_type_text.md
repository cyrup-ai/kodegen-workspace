# TASK 068: Trim browser_type_text prompts.rs

**Tool**: `browser_type_text`
**Complexity**: 2 (Simple)
**Current size**: 874 lines (6 scenarios)
**Target size**: 170-220 lines (1 scenario)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/type_text/prompts.rs`

---

## Current State Analysis

### File Structure
- **Location**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/type_text/prompts.rs`
- **Total lines**: 874
- **Scenarios defined**: 6
- **Routing method**: Match statement in `PromptProvider::generate_prompts()`

### Current Scenarios (to be deleted except 1)
1. **prompt_basic()** (~90 lines) - Basic text input patterns with selector strategies
2. **prompt_forms()** (~140 lines) - Form filling workflows (USE-CASE - DELETE)
3. **prompt_special_keys()** (~180 lines) - Special keys and characters (USE-CASE - DELETE)
4. **prompt_clearing()** (~160 lines) - Clearing existing content (USE-CASE - DELETE)
5. **prompt_sensitive()** (~150 lines) - Handling sensitive data (USE-CASE - DELETE)
6. **prompt_comprehensive()** (~140 lines) - Complete guide covering all patterns (COMPREHENSIVE - DELETE)

### Routing Logic (lines 17-23)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("forms") => prompt_forms(),
        Some("special_keys") => prompt_special_keys(),
        Some("clearing") => prompt_clearing(),
        Some("sensitive") => prompt_sensitive(),
        _ => prompt_comprehensive(),
    }
}
```

---

## Implementation Instructions

### Step 1: Update the Match Statement
Replace the entire `generate_prompts` function match statement (lines 17-23) to:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    // All scenarios default to basic - single comprehensive scenario
    match args.scenario.as_deref() {
        Some("basic") | _ => prompt_basic(),
    }
}
```

This eliminates routing to deleted functions and makes basic the only (and default) scenario.

### Step 2: Update prompt_arguments() Description
Update line 27 description from:
```rust
description: Some("Scenario to show (basic, forms, special_keys, clearing, sensitive)".to_string()),
```

To:
```rust
description: Some("Scenario to show (basic)".to_string()),
```

This reflects that only basic scenario is available.

### Step 3: Delete Decorative Header
Delete the entire header comment block at lines 44-45:
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO TYPE TEXT IN BROWSERS
// ============================================================================
```

This removes unnecessary decoration.

### Step 4: Delete prompt_forms() Function
**Lines to delete**: From `/// Form filling workflows` through end of function (approximately lines ~145-285)

Search for: `/// Form filling workflows` and delete through the closing `])`

This removes the form-specific use-case scenario.

### Step 5: Delete prompt_special_keys() Function
**Lines to delete**: From `/// Special keys and characters` through end of function (approximately lines ~287-467)

Search for: `/// Special keys and characters` and delete through the closing `])`

This removes the special keys use-case scenario.

### Step 6: Delete prompt_clearing() Function
**Lines to delete**: From `/// Clearing existing content` through end of function (approximately lines ~469-629)

Search for: `/// Clearing existing content` and delete through the closing `])`

This removes the clearing use-case scenario.

### Step 7: Delete prompt_sensitive() Function
**Lines to delete**: From `/// Handling sensitive data` through end of function (approximately lines ~631-781)

Search for: `/// Handling sensitive data` and delete through the closing `])`

This removes the sensitive data use-case scenario.

### Step 8: Delete prompt_comprehensive() Function
**Lines to delete**: From `/// Comprehensive guide covering all patterns` through end of file (approximately lines ~783-874)

Search for: `/// Comprehensive guide covering all patterns` and delete through the closing `])`

This removes the comprehensive scenario.

---

## Detailed Code Patterns

### BEFORE: Current Match Statement
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("forms") => prompt_forms(),
        Some("special_keys") => prompt_special_keys(),
        Some("clearing") => prompt_clearing(),
        Some("sensitive") => prompt_sensitive(),
        _ => prompt_comprehensive(),
    }
}
```

### AFTER: Updated Match Statement
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    // All scenarios default to basic - single comprehensive scenario
    match args.scenario.as_deref() {
        Some("basic") | _ => prompt_basic(),
    }
}
```

### BEFORE: Argument Description
```rust
description: Some("Scenario to show (basic, forms, special_keys, clearing, sensitive)".to_string()),
```

### AFTER: Argument Description
```rust
description: Some("Scenario to show (basic)".to_string()),
```

---

## Success Criteria

- ✓ **File size**: 170-220 lines total (target: ~200 lines)
- ✓ **Scenarios**: Exactly 1 scenario remaining (prompt_basic)
- ✓ **Routing**: Match statement routes all inputs to prompt_basic()
- ✓ **No use-case scenarios**: All form, special_keys, clearing, sensitive functions deleted
- ✓ **No comprehensive scenario**: The old catch-all comprehensive guide deleted
- ✓ **No orphaned functions**: All deleted functions completely removed (no dead code)
- ✓ **Clean routing**: Simple, understandable match logic (basic | _ => prompt_basic())
- ✓ **Documentation accurate**: Argument descriptions reflect actual scenarios
- ✓ **Functional integrity**: Tool still works, just with single scenario instead of 6

---

## Verification Steps

After completing the implementation:

1. **Count lines**: `wc -l packages/kodegen-mcp-schema/src/browser/type_text/prompts.rs` should output 170-220
2. **Verify compilation**: Run from repo root: `cd packages/kodegen-mcp-schema && cargo check`
3. **Search for deleted functions**: Grep for `prompt_forms`, `prompt_special_keys`, `prompt_clearing`, `prompt_sensitive`, `prompt_comprehensive` - should return 0 results
4. **Verify routing**: Confirm match statement in `generate_prompts()` only references `prompt_basic()`
5. **Manual review**: File should contain only: imports, TypeTextPrompts struct, PromptProvider impl, and prompt_basic() function

---

## Notes

- This is a straightforward deletion task with minimal logic changes
- The prompt_basic() function is comprehensive enough to serve as the primary scenario
- No new code needs to be written, only deletion and match statement simplification
- The tool's functionality is preserved - it now always returns the basic scenario regardless of input
- This reduces maintenance burden by eliminating duplication and scattered documentation

