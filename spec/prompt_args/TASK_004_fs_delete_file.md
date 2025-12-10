# TASK 004: Trim fs_delete_file Prompts

**Tool**: `fs_delete_file`  
**Complexity**: 1 (Trivial)  
**Current Size**: 734 lines (5 scenarios)  
**Target Size**: 90-110 lines (1 scenario)  
**File Path**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/delete_file/prompts.rs`

---

## Overview

This task requires consolidating the fs_delete_file prompt file from 5 redundant scenarios down to a single, focused "basic" scenario that covers essential usage, safety warnings, and common patterns. This is a straightforward trimming operation that removes feature bloat while preserving core educational content.

---

## Current State Analysis

### File Structure
The file is located at: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/delete_file/prompts.rs`

### Current Scenarios (All Must Be Consolidated)
1. **prompt_basic()** (~80 lines) - Basic usage, examples, parameters, error cases
2. **prompt_verification()** (~180 lines) - Pre-deletion verification workflow  
3. **prompt_cleanup()** (~250 lines) - Batch cleanup patterns
4. **prompt_safety()** (~220 lines) - When NOT to delete, safer alternatives
5. **prompt_comprehensive()** (~700+ lines, default fallback) - Comprehensive guide combining all above

### Current Implementation Pattern
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("verification") => prompt_verification(),
        Some("cleanup") => prompt_cleanup(),
        Some("safety") => prompt_safety(),
        _ => prompt_comprehensive(),  // DEFAULT - Too large, causes bloat
    }
}
```

**Problem**: The comprehensive scenario is too large and acts as a catch-all default for undefined scenarios. Multiple redundant scenarios create confusion and waste space.

---

## Target Implementation

### Primary Change: Keep ONLY "basic" Scenario

The `prompt_basic()` function represents the most fundamental and reusable educational content. It covers:
- **What**: Tool purpose and what it does
- **How**: Basic usage with examples
- **Response**: Structure and fields explained
- **Errors**: Common error cases and solutions
- **When**: Common use cases and applicability

### Refined prompt_basic() Structure (~100 lines)

The refined basic scenario MUST include exactly these sections with line allocations:

```
Lines 1-10:   Tool description & critical warning
              - What fs_delete_file does
              - PERMANENT nature warning
              - Key differences from fs_delete_directory

Lines 11-20:  Basic usage example
              - Single call example with parameters
              - Expected response structure
              - Required vs optional fields

Lines 21-30:  Response structure detailed
              - Success case response
              - Error case responses  
              - Response field meanings
              - Status indicators

Lines 31-45:  When to use / Common patterns
              - Safe deletion candidates (build artifacts, logs, cache, temp files)
              - 3-4 concrete examples with expected outcomes
              - Pattern: search → review → delete (essential workflow)

Lines 46-75:  Error handling
              - File not found scenario
              - Directory instead of file
              - Permission denied
              - File in use/locked scenarios
              - Clear solutions for each

Lines 76-100: Quick reference
              - One-line usage reminder
              - Related tools (fs_get_file_info, fs_delete_directory, fs_move_file)
              - Safety checklist (3-5 items)
              - Golden rule: "Verify before deleting"
```

### Updated prompt_arguments()

**BEFORE**: Documents 4 scenarios (basic, verification, cleanup, safety)
```rust
PromptArgument {
    name: "scenario".to_string(),
    title: None,
    description: Some("Scenario to show (basic, verification, cleanup, safety)".to_string()),
    required: Some(false),
}
```

**AFTER**: Remove scenario argument since only one exists
```rust
// Empty vec - no arguments needed, only one scenario exists
vec![]
```

### Updated generate_prompts() Implementation

**BEFORE**: 7-line match statement with 5 branches
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("verification") => prompt_verification(),
        Some("cleanup") => prompt_cleanup(),
        Some("safety") => prompt_safety(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER**: Single-line implementation
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

---

## Exact Implementation Steps

### Step 1: Delete Four Scenario Functions
Remove these functions entirely (in order, each deletion reduces file significantly):
- [ ] Delete `prompt_verification()` function (~180 lines)
- [ ] Delete `prompt_cleanup()` function (~250 lines)  
- [ ] Delete `prompt_safety()` function (~220 lines)
- [ ] Delete `prompt_comprehensive()` function (~700+ lines total)

**Result**: File reduced from 734 → ~90-100 lines

### Step 2: Simplify generate_prompts()
Replace the 7-line match statement (lines 17-23 in current file) with:
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

This removes all scenario routing logic since only one scenario remains.

### Step 3: Update prompt_arguments()
Replace the scenario PromptArgument vector (lines 25-33 in current file) with:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![]  // No arguments - single fixed scenario
}
```

Since there is only one scenario, users cannot select alternatives. No arguments are needed.

### Step 4: Review and Verify prompt_basic()
Ensure `prompt_basic()` function:
- [ ] Remains under 100 lines total
- [ ] Covers all required sections (description, usage, response, errors, reference)
- [ ] Includes 3-5 concrete examples showing actual fs_delete_file usage
- [ ] Contains critical warning about permanent deletion upfront
- [ ] Lists safe deletion candidates (build artifacts, logs, cache, temp)
- [ ] Explains error handling with solutions
- [ ] Provides quick reference section at end
- [ ] Uses the exact Rust formatting and PromptMessage structure from original

### Step 5: Remove Decorative Elements  
Delete anywhere in prompts.rs:
- [ ] Header comment "// ============================================================================" separator lines
- [ ] Redundant section dividers
- [ ] "COMPREHENSIVE SCENARIO", "DETAILED GUIDE" type labels

Keep only minimal structure for readability.

---

## Code Architecture Notes

### PromptProvider Trait Pattern
The sealed `PromptProvider` trait from `kodegen-mcp-schema` ensures all prompts are centralized here. This is correct and unchanged. The `generate_prompts()` function MUST:
- Accept `&Self::PromptArgs`
- Return `Vec<PromptMessage>`  
- Populate PromptMessage with User/Assistant roles
- Use PromptMessageContent::text() for content

See trait definition: [`/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/tool.rs`](../kodegen-mcp-schema/src/tool.rs)

### Module Re-exports
The `mod.rs` file already re-exports all public items:
```rust
pub use prompts::*;  // DeleteFilePrompts struct and functions
```

No changes needed there.

### Related Files (DO NOT MODIFY)
- `prompt_args.rs` - Type definitions, no changes needed
- `schema.rs` - Tool schema, no changes needed  
- `mod.rs` - Module structure, no changes needed

---

## Expected Result Validation

### File Size Check
```bash
wc -l packages/kodegen-mcp-schema/src/filesystem/delete_file/prompts.rs
# Expected: 90-110 lines (was 734)
```

### Function Count Check
The file should have ONLY these functions:
- `impl PromptProvider for DeleteFilePrompts` (lines 1-10)
- `fn generate_prompts()` (lines 11-13)
- `fn prompt_arguments()` (lines 14-16)
- `fn prompt_basic()` (lines 17-100 remaining)

Total: 4 Rust items, 1 implementation block, 2 trait methods, 1 helper function.

### Compilation Check
```bash
cd packages/kodegen-mcp-schema
cargo check
# Should compile without errors or warnings
```

### No Functional Changes
- Tool behavior unchanged
- Prompt content simplified but still educationally complete
- Single-scenario design is simpler to maintain
- Users always receive best-practice "basic" usage guide

---

## Key Design Decisions (PRESCRIPTIVE - NOT OPTIONS)

1. **Keep "basic" scenario** - Most fundamental, covers 80% of usage needs
2. **Remove "verification"** - Safety-critical info merged into basic
3. **Remove "cleanup"** - Pattern (search→review→delete) stays in basic  
4. **Remove "safety"** - Critical warnings moved to start of basic
5. **Remove "comprehensive"** - Redundant with consolidated basic
6. **Simplify routing** - Single scenario eliminates match logic
7. **Remove scenario argument** - No user selection needed
8. **Keep decorators minimal** - Focus on content, not formatting

---

## Success Criteria

- ✓ File size: 90-110 lines (confirmed with wc -l)
- ✓ Single scenario only: prompt_basic()  
- ✓ No decorative headers or section dividers
- ✓ generate_prompts() returns prompt_basic() directly
- ✓ prompt_arguments() returns empty vec
- ✓ Content includes: description, usage, response, errors, reference, examples
- ✓ No mention of other scenarios (verification/cleanup/safety/comprehensive)
- ✓ Cargo check passes with zero warnings
- ✓ Tool functionality identical from user perspective

---

## Related Reference Material

**Complexity 1 Template Example**:  
See `/Volumes/samsung_t9/kodegen-workspace/task/PRECURSOR_01_memory_list_libraries.md` for standard Complexity 1 patterns (this task follows same structure).

**Prompt Architecture**:  
Review [`packages/kodegen-mcp-schema/src/tool.rs`](../kodegen-mcp-schema/src/tool.rs) for sealed `PromptProvider` trait definition and requirements.

**Sister Tools Reference**:
- `fs_delete_directory` - Similar but for directories (multi-scenario, kept as-is)
- `fs_get_file_info` - Complementary verification tool  
- `fs_move_file` - Safer alternative to deletion
