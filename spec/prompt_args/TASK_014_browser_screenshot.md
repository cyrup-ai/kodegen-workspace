# TASK 014: Trim browser_screenshot

**Tool**: `browser_screenshot`
**Complexity**: 1 (Trivial)
**Current size**: 723 lines (5 scenarios)
**Target size**: 100 lines (1 scenario)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/screenshot/prompts.rs`

---

## Current State Analysis

The `prompts.rs` file currently contains **723 lines** with **5 scenario functions**:

1. **prompt_debugging()** (lines 43-138): ~96 lines - Teaches debugging workflow with page state inspection
2. **prompt_element_capture()** (lines 139-237): ~99 lines - Teaches element-specific screenshot syntax
3. **prompt_full_page()** (lines 238-353): ~116 lines - Teaches viewport vs full page trade-offs
4. **prompt_verification()** (lines 354-503): ~150 lines - Teaches verification patterns for actions
5. **prompt_comprehensive()** (lines 504-723): ~220 lines - Complete guide covering all patterns (MUST DELETE)

**Routing Logic** (lines 18-24):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("debugging") => prompt_debugging(),
        Some("element_capture") => prompt_element_capture(),
        Some("full_page") => prompt_full_page(),
        Some("verification") => prompt_verification(),
        _ => prompt_comprehensive(),  // DEFAULT (WRONG - MUST CHANGE)
    }
}
```

**Prompt Arguments** (lines 26-35): Documents all 4 scenarios in description string

---

## Step-by-Step Implementation

### Step 1: Choose and Keep Core Scenario

**Decision**: Keep **prompt_debugging()** as the ONLY scenario function.

**Reasoning**: The debugging scenario is the most fundamental and practical use case. It teaches agents WHEN and WHY to take screenshots (the debugging workflow), which is essential for effective browser automation. All other patterns (element capture, full page vs viewport, verification) are extensions of this core mental model.

### Step 2: Trim prompt_debugging() to 90-110 Lines

Current content spans lines 43-138 (~96 lines) but contains redundancy. Trim by removing less critical examples while preserving essential patterns.

**Current structure breakdown:**
- Lines 44-47: User question (4 lines)
- Lines 48-52: Response intro (5 lines)
- Lines 53-58: WHEN TO TAKE DEBUG SCREENSHOTS (6 lines)
- Lines 59-65: DEBUGGING WORKFLOW steps (7 lines)
- Lines 66-129: Five examples with headers (64 lines) - **TRIM TO 3 KEY EXAMPLES ONLY**
- Lines 130-138: Final sections (9 lines)

**Trimming strategy**:
- KEEP EXAMPLE 1 (Navigation Failure) - Essential pattern when navigation doesn't work
- KEEP EXAMPLE 3 (Action Had No Effect) - Essential pattern when actions don't complete
- KEEP EXAMPLE 5 (Verify Current Page State) - Essential pattern for state verification
- DELETE EXAMPLE 2 (Element Not Visible) - Similar to Example 1, covered by other patterns
- DELETE EXAMPLE 4 (Unexpected Text Content) - Less critical discovery type
- KEEP "COMMON DISCOVERIES FROM DEBUG SCREENSHOTS" section (8 lines) - Educational value
- KEEP "SCREENSHOT TIMING FOR DEBUGGING" section (5 lines) - Essential practical guidance
- DELETE "INTEGRATION WITH OTHER BROWSER TOOLS" section (5 lines) - Out of scope for core scenario
- KEEP "BEST PRACTICES" section (6 lines) - Practical do's and don'ts

Target after trim: 95-105 lines for prompt_debugging() function only.

### Step 3: Delete All Other Scenario Functions

**Delete these functions entirely** (in reverse line order to preserve accuracy):

1. **prompt_comprehensive()** (lines 504-723) - Complete 220-line guide: DELETE ALL
2. **prompt_verification()** (lines 354-503) - Verification patterns: DELETE ALL
3. **prompt_full_page()** (lines 238-353) - Viewport vs full page: DELETE ALL
4. **prompt_element_capture()** (lines 139-237) - Element capture syntax: DELETE ALL

### Step 4: Update Routing Default

**File location**: Lines 18-24 in `PromptProvider::generate_prompts()`

**Before**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("debugging") => prompt_debugging(),
        Some("element_capture") => prompt_element_capture(),
        Some("full_page") => prompt_full_page(),
        Some("verification") => prompt_verification(),
        _ => prompt_comprehensive(),
    }
}
```

**After**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("debugging") => prompt_debugging(),
        _ => prompt_debugging(),
    }
}
```

Alternative simpler form:
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_debugging()
}
```

### Step 5: Update prompt_arguments() Description

**File location**: Lines 26-35 in `prompt_arguments()` function

**Before**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (debugging, element_capture, full_page, verification)".to_string()),
            required: Some(false),
        }
    ]
}
```

**After** (option 1 - simplified):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Currently only 'debugging' scenario available (default)".to_string()),
            required: Some(false),
        }
    ]
}
```

**After** (option 2 - minimal):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![]
}
```

### Step 6: Update File Section Header Comment

**File location**: Lines 41-43 (comment before prompt_debugging function)

**Before**:
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE BROWSER SCREENSHOTS
// ============================================================================
```

**After**:
```rust
// ============================================================================
// DEBUGGING SCENARIO - PRIMARY USE CASE FOR BROWSER SCREENSHOTS
// ============================================================================
```

---

## Execution Order (CRITICAL FOR ACCURACY)

Execute changes in **reverse line order** to preserve line accuracy throughout:

1. **Delete prompt_comprehensive()** (lines 504-723, ~220 lines)
   - Remove entire function body and definition

2. **Delete prompt_verification()** (lines 354-503, ~150 lines)
   - Remove entire function body and definition

3. **Delete prompt_full_page()** (lines 238-353, ~116 lines)
   - Remove entire function body and definition

4. **Delete prompt_element_capture()** (lines 139-237, ~99 lines)
   - Remove entire function body and definition

5. **Trim prompt_debugging()** (lines 43-138, reduce to ~100 lines)
   - Keep examples 1, 3, 5; delete examples 2, 4
   - Condense repetitive sections
   - Keep all essential headings and patterns

6. **Update PromptProvider::generate_prompts()** (lines 18-24)
   - Remove match arms for element_capture, full_page, verification
   - Change default fallback from `prompt_comprehensive()` to `prompt_debugging()`

7. **Update prompt_arguments()** (lines 26-35)
   - Update description string to reflect only debugging scenario
   - Or simplify to empty vector if no arguments needed

8. **Update file header comment** (lines 41-43)
   - Change from "HELPER FUNCTIONS" to "DEBUGGING SCENARIO"

---

## Success Criteria (ALL MUST PASS)

### Measurable Line Count
- **Final file**: 90-110 lines total
- Verify with: `wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/screenshot/prompts.rs`
- Expected: 90-110 lines

### Single Scenario Function
- **Exactly 1 scenario function** exists in file
- Verify with: `grep -c "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/screenshot/prompts.rs`
- Expected: 1

### No References to Deleted Scenarios
- **Zero references to prompt_comprehensive()** anywhere
- **Zero references to prompt_element_capture()** anywhere
- **Zero references to prompt_full_page()** anywhere
- **Zero references to prompt_verification()** anywhere
- Verify with: `grep -E "prompt_(comprehensive|element_capture|full_page|verification)" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/screenshot/prompts.rs || echo "Success"`
- Expected: No matches (exit code 1, "Success" message)

### Routing Updated Correctly
- **Default fallback is prompt_debugging()**, not prompt_comprehensive()
- Verify with: `grep "_ =>" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/screenshot/prompts.rs`
- Expected: `_ => prompt_debugging(),` or function simplified entirely

### Code Compiles Successfully
- **No Rust compilation errors** in this file
- Verify with: `cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check`
- Expected: Success, no errors or warnings

### No Decorative Headers
- **Removed excessive `=====` separator lines**
- Keep only section headers like "DEBUGGING WORKFLOW" with single line separators
- Verify by inspection - max 1-2 separator lines per section

---

## Verification Commands

Run these commands after completing the task:

```bash
# Check final line count (should be 90-110)
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/screenshot/prompts.rs

# Count scenario functions (should be 1)
grep -c "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/screenshot/prompts.rs

# Verify no deleted scenarios referenced (should find nothing)
grep -E "prompt_(comprehensive|element_capture|full_page|verification)" \
  /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/screenshot/prompts.rs \
  || echo "✓ Success: No references to deleted scenarios"

# Verify routing default
grep "_ =>" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/screenshot/prompts.rs

# Compile check
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check
```

Expected results after success:
- Line count: 90-110
- Function count: 1
- Grep search: No matches (Success message)
- Routing: Default is `prompt_debugging()`
- Cargo check: Passes with no errors
