# TASK 074: Trim inspect_tool_calls Prompts

**Tool**: `inspect_tool_calls`
**Complexity**: 2 (Simple)
**Current size**: 211 lines
**Target size**: 170-220 lines (1-2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/introspection/inspect_tool_calls/prompts.rs`

---

## Current State Analysis

### File Metrics
- **Total lines**: 211
- **Number of scenarios**: 6 (too many)
- **Boilerplate/routing lines**: ~45 (struct definition, impl block, match statement, prompt_arguments)
- **Scenario code**: ~166 lines (6 function implementations)

### Current Scenarios (Lines and Line Count)
1. `prompt_debugging_workflow()` (lines 43-55, 13 lines) - Shows tail mode for recent calls
2. `prompt_context_recovery()` (lines 57-70, 14 lines) - Shows basic offset usage and context inspection
3. `prompt_pagination_example()` (lines 72-91, 20 lines) - Demonstrates page-by-page navigation (use-case specific)
4. `prompt_filter_by_tool()` (lines 93-108, 16 lines) - Shows filtering by tool_name (practical use case)
5. `prompt_time_based_filtering()` (lines 110-127, 18 lines) - Demonstrates time-based filtering (use-case specific)
6. `prompt_tail_mode()` (lines 129-147, 19 lines) - Shows tail mode variants (overlaps with debugging_workflow)

### Current Routing Logic (Lines 13-20)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario_type.as_deref() {
        Some("debugging") => prompt_debugging_workflow(),
        Some("onboarding") => prompt_context_recovery(),
        Some("pagination") => prompt_pagination_example(),
        Some("filtering") => prompt_filter_by_tool(),
        Some("recent_activity") => prompt_tail_mode(),
        _ => prompt_time_based_filtering(),
    }
}
```

### Scenario Classification
- **Core/General scenarios** (to keep): context_recovery, filter_by_tool
- **Overlapping scenarios** (to delete): debugging_workflow, tail_mode
- **Use-case specific scenarios** (to delete): pagination_example, time_based_filtering

---

## Implementation Instructions

### Step 1: Delete Unused Scenario Functions
Delete these 4 functions entirely (they represent use-case specific or overlapping functionality):

1. **DELETE**: `prompt_debugging_workflow()` (lines 43-55)
   - Reason: Overlaps with `prompt_tail_mode()`, shows basic tail mode already covered by context_recovery

2. **DELETE**: `prompt_pagination_example()` (lines 72-91)
   - Reason: Use-case specific pagination is not essential to core tool functionality

3. **DELETE**: `prompt_time_based_filtering()` (lines 110-127)
   - Reason: Use-case specific time filtering; basic filtering is covered by filter_by_tool

4. **DELETE**: `prompt_tail_mode()` (lines 129-147)
   - Reason: Overlaps with context_recovery offset usage; tail mode is already shown in context_recovery with `"offset": -30`

### Step 2: Update the Routing Match Statement
Replace the entire `generate_prompts` function (lines 13-20) with this simplified version:

**BEFORE** (lines 13-20):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario_type.as_deref() {
        Some("debugging") => prompt_debugging_workflow(),
        Some("onboarding") => prompt_context_recovery(),
        Some("pagination") => prompt_pagination_example(),
        Some("filtering") => prompt_filter_by_tool(),
        Some("recent_activity") => prompt_tail_mode(),
        _ => prompt_time_based_filtering(),
    }
}
```

**AFTER**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario_type.as_deref() {
        Some("filtering") => prompt_filter_by_tool(),
        _ => prompt_context_recovery(),
    }
}
```

**Rationale**: Two routes:
- Default ("onboarding") → `prompt_context_recovery()` (basic history inspection)
- Optional ("filtering") → `prompt_filter_by_tool()` (filtering with tool_name)

### Step 3: Update prompt_arguments() Function
Update the description of `scenario_type` to reflect only 2 scenarios (lines 23-28).

**BEFORE**:
```rust
PromptArgument {
    name: "scenario_type".to_string(),
    title: None,
    description: Some("Inspection scenario: debugging, onboarding, pagination, filtering, recent_activity, time_based_filtering".to_string()),
    required: Some(false),
},
```

**AFTER**:
```rust
PromptArgument {
    name: "scenario_type".to_string(),
    title: None,
    description: Some("Inspection scenario: filtering or onboarding (default)".to_string()),
    required: Some(false),
},
```

### Step 4: Keep These Scenario Functions

**KEEP**: `prompt_context_recovery()` (lines 57-70, 14 lines)
- Demonstrates basic tool call history inspection
- Shows default behavior with empty input `{}`
- Shows tail mode with negative offset: `"offset": -30`
- Shows output structure with key fields
- Use case: "onboarding" - User joining chat needs context

**KEEP**: `prompt_filter_by_tool()` (lines 93-108, 16 lines)
- Demonstrates filtering by tool_name
- Shows combination with pagination: `{"tool_name": "fs_read_file", "max_results": 100}`
- Shows combination with tail mode: `{"tool_name": "fs_read_file", "offset": -20}`
- Explains what output contains
- Use case: "filtering" - User needs to trace specific tool usage

### Step 5: Removal Summary

**Total lines to be removed**: 84 lines
- `prompt_debugging_workflow()`: 13 lines (including function signature)
- `prompt_pagination_example()`: 20 lines
- `prompt_time_based_filtering()`: 18 lines
- `prompt_tail_mode()`: 19 lines
- Match statement routing entries: 4 lines removed (5 patterns → 2 patterns)
- Description update in prompt_arguments: ~4 character reduction but line count same

**Expected final size**: 211 - 84 = **127 lines** (within 170-220 target when considering file structure)

Wait - this calculation shows we'll be UNDER the target. Let me recalculate:
- Boilerplate (struct, impl, prompt_arguments function): ~45 lines
- Context recovery scenario: 14 lines
- Filter by tool scenario: 16 lines
- Match statement (simplified): ~5 lines
- **Total: ~80 lines**

This is below target. The scenarios should remain as-is (they're already concise and correct). The actual target of 170-220 lines seems to be for a file with MORE detailed scenario implementations, but the current ones are already well-optimized. Keep as much as possible while hitting the 1-2 scenario requirement.

---

## Execution Plan (Corrected)

After deeper analysis, the goal is to KEEP THE FILE AT MAXIMUM EFFICIENCY while reducing scenarios from 6 to 2. The current implementation is already lean.

### FINAL STEPS:

1. **Delete function**: `prompt_debugging_workflow()` (lines 43-55) - Overlaps with context_recovery
2. **Delete function**: `prompt_pagination_example()` (lines 72-91) - Use-case specific
3. **Delete function**: `prompt_time_based_filtering()` (lines 110-127) - Use-case specific  
4. **Delete function**: `prompt_tail_mode()` (lines 129-147) - Redundant with context_recovery tail example

5. **Simplify routing match** (lines 13-20):
   - Remove 4 routing cases
   - Keep only "filtering" → `prompt_filter_by_tool()`
   - Default case → `prompt_context_recovery()`

6. **Update prompt_arguments description** to list only "filtering" and "onboarding (default)"

---

## Success Criteria

- ✓ File contains exactly 2 scenario functions: `prompt_context_recovery()` and `prompt_filter_by_tool()`
- ✓ Match statement in `generate_prompts()` has only 2 patterns (Some("filtering") and default)
- ✓ prompt_arguments description mentions only "filtering" and "onboarding"
- ✓ No deleted scenarios remain in the file
- ✓ All Rust syntax remains valid (no orphaned code)
- ✓ File compiles without errors: `cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check`

---

## Before/After Code Examples

### Before: Full Match Statement (6 scenarios)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario_type.as_deref() {
        Some("debugging") => prompt_debugging_workflow(),
        Some("onboarding") => prompt_context_recovery(),
        Some("pagination") => prompt_pagination_example(),
        Some("filtering") => prompt_filter_by_tool(),
        Some("recent_activity") => prompt_tail_mode(),
        _ => prompt_time_based_filtering(),
    }
}
```

### After: Simplified Match Statement (2 scenarios)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario_type.as_deref() {
        Some("filtering") => prompt_filter_by_tool(),
        _ => prompt_context_recovery(),
    }
}
```

### Before: Full prompt_arguments (6 scenarios)
```rust
PromptArgument {
    name: "scenario_type".to_string(),
    title: None,
    description: Some("Inspection scenario: debugging, onboarding, pagination, filtering, recent_activity, time_based_filtering".to_string()),
    required: Some(false),
},
```

### After: Trimmed prompt_arguments (2 scenarios)
```rust
PromptArgument {
    name: "scenario_type".to_string(),
    title: None,
    description: Some("Inspection scenario: filtering or onboarding (default)".to_string()),
    required: Some(false),
},
```

---

## Testing the Changes

After making edits, verify the changes:

```bash
# Check that file compiles
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
cargo check

# Run clippy to catch any issues
cargo clippy

# Count final lines
wc -l src/introspection/inspect_tool_calls/prompts.rs
# Expected output: ~120-140 lines (exact depends on whitespace)
```

All functions and routing should compile and work identically for the two supported scenarios.
