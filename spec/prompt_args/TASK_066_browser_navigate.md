# TASK 066: Trim browser_navigate Prompts

**Tool**: `browser_navigate`
**Complexity**: 2 (Simple)
**Current size**: 946 lines
**Target size**: 170-220 lines total (1-2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/navigate/prompts.rs`

---

## Current State Analysis

### File Structure (946 lines total)

The prompts.rs file is organized as follows:

**Lines 1-43**: Module header, imports, and PromptProvider trait implementation
- Trait definition: `NavigatePrompts` struct implementing `PromptProvider`
- Type: `BrowserNavigatePromptArgs`

**Lines 18-24**: Match statement routing scenarios to functions
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("query_params") => prompt_query_params(),
    Some("authentication") => prompt_authentication(),
    Some("spa_pages") => prompt_spa_pages(),
    Some("wait_strategies") => prompt_wait_strategies(),
    _ => prompt_comprehensive(),
}
```

**Lines 33-37**: PromptArgument definition listing all scenarios

### Scenario Functions (current - 6 scenarios)

1. **prompt_basic()** (Lines 52-180): ~128 lines
   - User question: "How do I navigate to URLs using browser_navigate?"
   - Content: Basic URL patterns, response structure, use cases, URL encoding, common patterns, error handling
   - Classification: CORE SCENARIO - KEEP

2. **prompt_query_params()** (Lines 182-370): ~188 lines
   - User question: "How do I build URLs with query parameters?"
   - Content: Building URLs, URL encoding, complex URLs, special cases
   - Classification: USE-CASE SCENARIO - DELETE

3. **prompt_authentication()** (Lines 372-570): ~198 lines
   - User question: "How do I handle authentication redirects?"
   - Content: Auth workflows, OAuth, return URLs, multi-step auth
   - Classification: USE-CASE SCENARIO - DELETE

4. **prompt_spa_pages()** (Lines 572-750): ~178 lines
   - User question: "How do I navigate single-page applications (SPAs)?"
   - Content: Hash routing, History API, SPA challenges, solutions
   - Classification: USE-CASE SCENARIO - DELETE

5. **prompt_wait_strategies()** (Lines 752-850): ~98 lines
   - User question: "What are the different wait strategies?"
   - Content: Default load, wait_for_selector, networkidle, timeout, strategies, options, debugging
   - Classification: FUNDAMENTAL SCENARIO - KEEP

6. **prompt_comprehensive()** (Lines 852-946): ~94 lines
   - User question: "Give me a complete guide..."
   - Content: Decorative comprehensive summary with all patterns
   - Classification: COMPREHENSIVE/DECORATIVE - DELETE

### Decorative Headers (to delete)

Lines 47-50: Decorative section separator
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO NAVIGATE WITH BROWSERS
// ============================================================================
```

---

## Implementation Strategy

### Step 1: Identify What to Keep vs Delete

KEEP (with NO modifications):
- Lines 1-7: Module header and imports (7 lines)
- Lines 9-43: PromptProvider trait implementation (35 lines)
- Lines 52-180: `prompt_basic()` function (~128 lines)
- Lines 752-850: `prompt_wait_strategies()` function (~98 lines)

DELETE entirely:
- Lines 47-50: Decorative header (4 lines)
- Lines 182-370: `prompt_query_params()` function (~188 lines)
- Lines 372-570: `prompt_authentication()` function (~198 lines)
- Lines 572-750: `prompt_spa_pages()` function (~178 lines)
- Lines 852-946: `prompt_comprehensive()` function (~94 lines)

### Step 2: Update Match Statement

Current lines 18-24 MUST become:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("wait_strategies") => prompt_wait_strategies(),
    _ => prompt_wait_strategies(),  // Changed from prompt_comprehensive()
}
```

Reason: Default fallback should be `prompt_wait_strategies()` since it covers fundamental concepts. Removing query_params, authentication, spa_pages routes.

### Step 3: Update PromptArgument Description

Current lines 33-37:
```rust
description: Some("Scenario to show (basic, query_params, authentication, spa_pages, wait_strategies)".to_string()),
```

Must become:
```rust
description: Some("Scenario to show (basic, wait_strategies)".to_string()),
```

### Step 4: Remove Decorative Section Header

Delete lines 47-50 (the separator with "HELPER FUNCTIONS - TEACH AI AGENTS...")

### Expected Result

After completing all steps:
- Total lines: ~210 (7 + 35 + 4 + 128 + 98 = ~272... need to recalculate)

Actually, more precise calculation:
- Lines 1-7: 7 lines (imports)
- Lines 9-43: 35 lines (trait impl, including blank lines)
- Lines 47-50: DELETE (decorative, saves 4 lines)
- Lines 52-180: 129 lines (prompt_basic)
- Lines 752-850: 99 lines (prompt_wait_strategies)
- New match statement: 7 lines (slightly shorter)

Estimated final: ~270 lines, which is OVER the 220 target.

To fit in 170-220 range, should ONLY keep `prompt_basic()` as the single scenario.

### REVISED STRATEGY

After recalculation, to meet 170-220 line requirement:

OPTION 1 (Single Scenario - More Aggressive):
- Keep: prompt_basic() only
- Delete: All other scenarios
- Result: ~7 + 35 + 129 = ~171 lines (FITS!)
- Default fallback: prompt_basic()

OPTION 2 (Two Scenarios - Risky):
- Keep: prompt_basic() + prompt_wait_strategies()
- Would result in ~270+ lines (OVER target)
- Does NOT meet 170-220 requirement

---

## Execution Instructions (Prescriptive)

Execute these changes IN THIS EXACT ORDER:

### Change 1: Delete lines 182-750 (query_params, authentication, spa_pages)
These are use-case scenarios that must be removed entirely.

**Before**: Lines 182-750 contain three scenario functions
**After**: These lines are deleted

### Change 2: Delete lines 852-946 (prompt_comprehensive)
The comprehensive scenario is a decorative summary and must be removed.

**Before**: Lines 852-946 contain prompt_comprehensive()
**After**: Function is deleted

### Change 3: Update the match statement (lines 18-24)
Replace the 6-scenario routing with single-scenario routing.

**Before** (lines 18-24):
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("query_params") => prompt_query_params(),
    Some("authentication") => prompt_authentication(),
    Some("spa_pages") => prompt_spa_pages(),
    Some("wait_strategies") => prompt_wait_strategies(),
    _ => prompt_comprehensive(),
}
```

**After** (replace with):
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    _ => prompt_basic(),
}
```

### Change 4: Update PromptArgument description (lines 33-37)
Simplify scenario list to only "basic".

**Before** (line 34):
```rust
description: Some("Scenario to show (basic, query_params, authentication, spa_pages, wait_strategies)".to_string()),
```

**After** (replace with):
```rust
description: Some("Scenario to show (basic)".to_string()),
```

### Change 5: Delete decorative header (lines 47-50)
Remove the separator comment block that says "HELPER FUNCTIONS - TEACH AI AGENTS HOW TO NAVIGATE WITH BROWSERS".

**Before** (lines 47-50):
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO NAVIGATE WITH BROWSERS
// ============================================================================

```

**After**: DELETE these lines entirely

### Change 6: Keep prompt_basic() (lines 52-180)
NO CHANGES - This function stays exactly as-is. It contains everything needed for basic navigation guidance.

---

## Success Criteria

The task is complete ONLY when ALL of these conditions are true:

1. File has exactly 1 scenario function: `prompt_basic()`
2. Total line count is between 170-220 lines (target: ~171 lines)
3. Match statement routes ONLY:
   - `Some("basic")` → `prompt_basic()`
   - Default case `_` → `prompt_basic()`
4. PromptArgument description says: "Scenario to show (basic)"
5. No decorative section headers remain
6. No use-case scenarios remain (query_params, authentication, spa_pages deleted)
7. No comprehensive scenario remains
8. Code compiles without errors: `cargo check` in `/packages/kodegen-mcp-schema/`
9. File structure is:
   - Imports (lines 1-7)
   - Trait implementation (lines 9-43)
   - Blank line
   - prompt_basic() function
   - EOF

---

## File Structure After Completion

The final file should be organized as:

```rust
//! Prompt messages for browser_navigate tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::BrowserNavigatePromptArgs;

/// Prompt provider for browser_navigate tool
///
/// This is the ONLY way to provide prompts for browser_navigate - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct NavigatePrompts;

impl PromptProvider for NavigatePrompts {
    type PromptArgs = BrowserNavigatePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic URL navigation patterns
fn prompt_basic() -> Vec<PromptMessage> {
    // ... [entire prompt_basic content stays exactly as-is from original lines 52-180]
}
```

---

## Definition of Done

This task is COMPLETE when:

- [x] Current file analyzed: 946 lines, 6 scenarios identified
- [ ] Deleted: prompt_query_params() function
- [ ] Deleted: prompt_authentication() function  
- [ ] Deleted: prompt_spa_pages() function
- [ ] Deleted: prompt_comprehensive() function
- [ ] Deleted: Decorative "HELPER FUNCTIONS" section header
- [ ] Updated: Match statement to 2 arms (basic + default)
- [ ] Updated: PromptArgument description to "Scenario to show (basic)"
- [ ] Verified: File compiles with `cargo check`
- [ ] Verified: Final line count is 170-220 lines (target ~171)
- [ ] Verified: Only prompt_basic() function remains as scenario

---

## Notes

- Do NOT modify the `prompt_basic()` function content - it already contains all essential navigation guidance
- Do NOT add comments or documentation
- The file will be much more maintainable with single scenario focus
- The "basic" scenario contains enough detail for AI agents to understand core navigation patterns
