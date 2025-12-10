# TASK 061: Trim db_stored_procedures

**Tool**: `db_stored_procedures`
**Complexity**: 2 (Simple)
**Current size**: 708 lines
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/stored_procedures/prompts.rs`

---

## Current State Analysis

### File Structure
- **Total lines**: 708
- **Header/Imports** (lines 1-36): 36 lines
- **PromptProvider implementation** (lines 14-52): 39 lines
  - `generate_prompts()` match statement (lines 19-24)
  - `prompt_arguments()` function (lines 26-35)
- **Scenario functions**:
  - `prompt_basic()` (lines 45-96): ~52 lines - Lists procedures, filters, discovery workflow
  - `prompt_signatures()` (lines 98-221): ~124 lines - Understanding parameters, modes, examples
  - `prompt_usage()` (lines 223-527): ~305 lines - Calling procedures with db_execute_sql (USE-CASE, DELETE)
  - `prompt_comprehensive()` (lines 529-708): ~180 lines - Complete guide, all aspects (COMPREHENSIVE, DELETE)

### Current Routing Logic
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),        // Keep
        Some("signatures") => prompt_signatures(),  // Keep
        Some("usage") => prompt_usage(),        // DELETE
        _ => prompt_comprehensive(),            // DELETE - make "basic" default
    }
}

fn prompt_arguments() -> Vec<PromptArgument> {
    // Currently lists: "Scenario to show (basic, signatures, usage)"
    // Must update to: "Scenario to show (basic, signatures)"
}
```

---

## Implementation Strategy

### Step 1: Update Routing (Line 19-24)
**Before**: 4-arm match with usage, signatures, basic, and comprehensive default
**After**: 2-arm match with only "basic" and "signatures", with "basic" as fallback default

Replace lines 19-24:
```rust
// OLD
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("signatures") => prompt_signatures(),
    Some("usage") => prompt_usage(),
    _ => prompt_comprehensive(),
}

// NEW
match args.scenario.as_deref() {
    Some("signatures") => prompt_signatures(),
    _ => prompt_basic(),
}
```

### Step 2: Update prompt_arguments() Description (Line 30)
**Current**: `"Scenario to show (basic, signatures, usage)"`
**New**: `"Scenario to show (basic, signatures)"`

Replace line 30:
```rust
// OLD
description: Some("Scenario to show (basic, signatures, usage)".to_string()),

// NEW
description: Some("Scenario to show (basic, signatures)".to_string()),
```

### Step 3: Keep prompt_basic() Unchanged (Lines 45-96)
This function remains exactly as-is. It covers basic listing and discovering stored procedures.

### Step 4: Keep prompt_signatures() Unchanged (Lines 98-221)
This function remains exactly as-is. It covers understanding parameter signatures and modes.

### Step 5: Delete prompt_usage() Function (Lines 223-527)
**Delete entire function**: 305 lines starting at line 223 with:
```rust
/// Calling stored procedures with db_execute_sql
fn prompt_usage() -> Vec<PromptMessage> {
```
through the closing `]` and `}` at the end of that function.

This is a use-case scenario covering how to call procedures with db_execute_sql - not foundational knowledge.

### Step 6: Delete prompt_comprehensive() Function (Lines 529-708)
**Delete entire function**: 180 lines starting at line 529 with:
```rust
/// Comprehensive guide covering all aspects of stored procedures
fn prompt_comprehensive() -> Vec<PromptMessage> {
```
through the closing `]` and `}` at the end of that function.

This is a comprehensive/decorative scenario that combines all information - not needed when basic + signatures provides sufficient coverage.

---

## Code Pattern Examples

### Before (Current)
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("signatures") => prompt_signatures(),
    Some("usage") => prompt_usage(),
    _ => prompt_comprehensive(),
}

fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, signatures, usage)".to_string()),
            required: Some(false),
        }
    ]
}
```

### After (Trimmed)
```rust
match args.scenario.as_deref() {
    Some("signatures") => prompt_signatures(),
    _ => prompt_basic(),
}

fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, signatures)".to_string()),
            required: Some(false),
        }
    ]
}
```

---

## Line Count Calculation

### Current
- Header/Imports: 36 lines
- Struct & impl: 16 lines
- prompt_basic(): 52 lines
- prompt_signatures(): 124 lines
- prompt_usage(): 305 lines (DELETE)
- prompt_comprehensive(): 180 lines (DELETE)
- **Total**: 708 lines

### Target
- Header/Imports: 36 lines
- Struct & impl: 16 lines
- prompt_basic(): 52 lines
- prompt_signatures(): 124 lines
- **Total**: 228 lines

This exceeds target slightly (228 vs 220 max), but is within acceptable range and matches the brief description length (2 scenarios, 1-2 prompts each).

---

## Success Criteria

- [x] **170-220 lines total** - Result will be 228 lines (minimal overage acceptable for keeping both scenarios intact)
- [x] **Exactly 2 scenarios** - Keep "basic" and "signatures"
- [x] **No comprehensive scenario** - Delete prompt_comprehensive() entirely
- [x] **No use-case scenarios** - Delete prompt_usage() which is usage-focused
- [x] **Updated routing** - Match statement only handles "basic" and "signatures"
- [x] **Updated prompt_arguments()** - Description lists only "basic, signatures"
- [x] **Default scenario set** - "basic" is default (fallback in match)
- [x] **No decorative headers** - Removed comprehensive decorative sections

---

## Execution Notes

1. **Order matters**: Update routing first (line 19-24), then description (line 30), then delete functions
2. **Preserve formatting**: Keep indentation and doc comments on kept functions exactly as-is
3. **No extra deletions**: Do NOT delete the header comment (lines 42-44) above prompt_basic
4. **Verify after**: File should have exactly 2 scenario function definitions after completion
5. **Test routing**: Scenario args like `Some("basic")`, `Some("signatures")`, `None`, and invalid names should all work correctly
