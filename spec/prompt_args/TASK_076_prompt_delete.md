# TASK 076: Trim prompt_delete

**Tool**: `prompt_delete`
**Complexity**: 2 (Simple)
**Current size**: 314 lines total
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/prompt/prompt_delete/prompts.rs`

---

## Current State Analysis

### File Structure
The file is located at: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/prompt/prompt_delete/prompts.rs`

### Current Scenarios (314 lines total)
1. **prompt_basic()** - Lines 41-77 (37 lines)
   - Teaches basic deletion operations with examples
   - Shows confirm=true requirement
   - Demonstrates response format
   - Status: KEEP - Fundamental usage

2. **prompt_cleanup()** - Lines 79-133 (55 lines)
   - Shows batch removal workflows
   - Covers version cleanup, namespace cleanup, deprecated removal
   - Focuses on use-case patterns and organizational workflows
   - Status: DELETE - Use-case scenario per requirements

3. **prompt_safety()** - Lines 135-186 (52 lines)
   - Covers verification before deletion
   - Explains backup strategies
   - Addresses dependency checking
   - Status: KEEP - Direct deletion safety practices

4. **prompt_comprehensive()** - Lines 188-313 (126 lines)
   - Contains decorative headers with Unicode box-drawing (═══════)
   - Has 10+ section headers (OVERVIEW, BASIC USAGE, RESPONSE FORMAT, USE CASES, etc.)
   - Combines all patterns from other scenarios
   - Status: DELETE - Comprehensive scenario per task requirements

### Routing Logic (Lines 14-20)
The current match statement handles all 4 scenarios:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("cleanup") => prompt_cleanup(),
        Some("safety") => prompt_safety(),
        _ => prompt_comprehensive(),
    }
}
```

---

## Implementation Steps

### Step 1: Delete Unused Scenarios (Lines 79-313)

**DELETE SECTION 1: prompt_cleanup function**
- Delete lines 79-133 (55 lines total)
- This removes the batch cleanup workflows scenario

**DELETE SECTION 2: prompt_safety function (PARTIAL - see Step 2)**
- Temporarily marked for replacement

**DELETE SECTION 3: prompt_comprehensive function**
- Delete lines 188-313 (126 lines total)
- This removes all decorative headers and comprehensive sections

### Step 2: Keep and Enhance prompt_basic and prompt_safety

**KEEP prompt_basic()** - Lines 41-77 (37 lines)
- No changes to basic scenario
- Covers essential deletion mechanics

**RESTRUCTURE prompt_safety()** - Lines 79-186 (108 lines after deletion adjustments)
- Current content teaches best practices
- Expand with additional verification patterns
- Add error recovery examples
- NO decorative headers - keep it clean and direct

### Step 3: Update Routing Logic (Lines 14-20)

Replace the current match statement:

**BEFORE:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("cleanup") => prompt_cleanup(),
        Some("safety") => prompt_safety(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("safety") => prompt_safety(),
        _ => prompt_safety(),
    }
}
```

### Step 4: Update prompt_arguments Documentation (Lines 25-31)

Replace the description to reflect only available scenarios:

**BEFORE:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, cleanup, safety)".to_string()),
            required: Some(false),
        }
    ]
}
```

**AFTER:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, safety)".to_string()),
            required: Some(false),
        }
    ]
}
```

---

## Detailed Deletion Instructions

### Delete prompt_cleanup function and comments (Lines 79-133)

The entire function block starting with:
```rust
/// Cleanup workflows
fn prompt_cleanup() -> Vec<PromptMessage> {
```

Through the closing brace after "Organize by removing clutter"

### Delete prompt_comprehensive function and comments (Lines 188-313)

The entire function block starting with:
```rust
/// Comprehensive guide covering all deletion patterns
fn prompt_comprehensive() -> Vec<PromptMessage> {
```

Including all decorated section headers and closing brace.

---

## Success Criteria

After executing all steps:

- Total lines: 170-220 (target met with ~190-200 lines)
- Scenarios remaining: exactly 2 (basic + safety)
- No comprehensive scenario
- No decorative Unicode headers (═══════, etc.)
- Routing handles only "basic" and "safety" scenarios
- Default scenario changed to "safety" (safer default than comprehensive)
- prompt_arguments() description updated to "(basic, safety)"
- File is valid Rust with no syntax errors
- All imports remain valid (no unused imports)

---

## Code Patterns - Before/After

### Pattern 1: Scenario Selection
Before (4 scenarios):
```rust
Some("basic") => prompt_basic(),
Some("cleanup") => prompt_cleanup(),
Some("safety") => prompt_safety(),
_ => prompt_comprehensive(),
```

After (2 scenarios):
```rust
Some("basic") => prompt_basic(),
Some("safety") => prompt_safety(),
_ => prompt_safety(),
```

### Pattern 2: Scenario Documentation
Before: "Scenario to show (basic, cleanup, safety)"
After: "Scenario to show (basic, safety)"

---

## Verification Checklist

Execute these verification steps after completion:

1. File reads without errors: `cat /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/prompt/prompt_delete/prompts.rs`
2. Line count is 170-220: `wc -l <file>` should output 170-220
3. Compile check: `cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check`
4. No syntax errors reported
5. Only 2 functions remain: prompt_basic() and prompt_safety()
6. Match statement handles exactly 2 scenarios
7. No decorative headers remain in file
