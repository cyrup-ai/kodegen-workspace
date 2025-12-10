# TASK 002: Trim db_list_schemas Prompts

**Tool**: `db_list_schemas`  
**Complexity**: 1 (Trivial)  
**Current size**: 645 lines (4 scenarios)  
**Target size**: 90-110 lines (1 scenario)  
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/list_schemas/prompts.rs`

---

## Objective

Reduce the db_list_schemas prompt file to a single, focused scenario providing essential guidance without comprehensive documentation or multiple use cases. This maintains prompt quality while eliminating redundancy and complexity.

---

## Current State Analysis

### File Structure (645 lines)
- **Lines 1-43**: Module imports and PromptProvider trait implementation
- **Lines 44-143**: `prompt_basic()` function (~100 lines)
  - User question: "How do I list all database schemas?"
  - Response covers: basic usage, response format, when to list, schema types, database-specific behavior, safety, example output, filtering, next steps
  
- **Lines 145-235**: `prompt_exploration()` function (~90 lines) - DELETE
  - Covers database exploration workflow workflow with detailed steps and examples
  
- **Lines 237-456**: `prompt_multi_schema()` function (~220 lines) - DELETE
  - Covers working with multiple schemas, cross-schema queries, permissions, patterns
  
- **Lines 458-645**: `prompt_comprehensive()` function (~188 lines) - DELETE
  - Complete guide with decorative headers (WHAT ARE DATABASE SCHEMAS?, HOW TO LIST SCHEMAS, etc.)
  - Best practices, common use cases, troubleshooting sections

### Routing Logic (lines 17-22)
Current matching logic accepts scenario argument with three choices: "basic", "exploration", "multi_schema", defaulting to "comprehensive". The goal is to eliminate all routes except basic.

---

## Implementation Path

### Step 1: Simplify PromptProvider Trait Implementation

**Location**: Lines 14-32 in prompts.rs

**Current code**:
```rust
impl PromptProvider for ListSchemasPrompts {
    type PromptArgs = ListSchemasPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("exploration") => prompt_exploration(),
            Some("multi_schema") => prompt_multi_schema(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, exploration, multi_schema)".to_string()),
                required: Some(false),
            }
        ]
    }
}
```

**New code**:
```rust
impl PromptProvider for ListSchemasPrompts {
    type PromptArgs = ListSchemasPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}
```

**Why**: Single scenario means no routing needed. The function always returns `prompt_basic()`, and there are no prompt arguments since there's only one scenario option.

---

### Step 2: Keep prompt_basic() Function Unchanged

**Location**: Lines 44-143  
**Action**: KEEP AS-IS  
**Current content**: 
- User question about listing schemas
- Assistant response with structured sections covering basic usage, response format, when to use, schema types, database-specific behavior, safety guarantees, example output, filtering guidance, and next steps.

**Why it works**: The current `prompt_basic()` already follows the required structure with approximately 100 lines and covers all essential information without excessive detail.

---

### Step 3: Delete prompt_exploration() Function

**Location**: Lines 145-235  
**Action**: DELETE ENTIRELY  
**Reason**: Task requires only ONE scenario. The exploration workflow is redundant with basic scenario and adds 90 lines of unnecessary content.

---

### Step 4: Delete prompt_multi_schema() Function

**Location**: Lines 237-456  
**Action**: DELETE ENTIRELY  
**Reason**: Task requires only ONE scenario. Multi-schema patterns are advanced use cases not needed for basic schema discovery.

---

### Step 5: Delete prompt_comprehensive() Function

**Location**: Lines 458-645  
**Action**: DELETE ENTIRELY  
**Reason**: Task explicitly states "No comprehensive" in success criteria. This is the largest function (188 lines) and contains decorative headers (=== WHAT ARE DATABASE SCHEMAS ===), best practices sections, and extensive organization patterns that contradict the goal of trimming to basics.

---

### Step 6: Update Comment Block Above prompt_basic()

**Location**: Lines 40-43  
**Current**:
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO LIST DATABASE SCHEMAS
// ============================================================================
```

**New**:
```rust
// ============================================================================
// PROMPT FUNCTIONS - BASIC SCHEMA LISTING GUIDANCE
// ============================================================================
```

**Why**: Single scenario no longer needs "HELPER FUNCTIONS" label; more accurate to call it "PROMPT FUNCTIONS" since PromptProvider directly calls it.

---

## Expected Result

After completing all steps:
- **Total line count**: 90-110 lines (currently targeting ~105 lines)
- **File structure**: 
  - Lines 1-11: Imports and module doc comment
  - Lines 13-26: PromptProvider struct and impl (simplified)
  - Lines 28-30: Comment block
  - Lines 32-105: `prompt_basic()` function only
  
- **Routing behavior**: All requests to db_list_schemas return the same basic prompt, with no scenario selection available
- **Module interface**: `prompt_arguments()` returns empty vec since no options exist
- **Success**: Single focused prompt teaching schema listing without multi-scenario complexity

---

## Code Diff Summary

```diff
- Four functions → One function
- 645 lines → ~105 lines  
- Complex match routing → Simple direct call
- prompt_arguments with 3 scenarios → empty prompt_arguments
- Decorative section headers (===) → Removed entirely
- Best practices section → Removed entirely
- Multi-schema patterns → Removed entirely
- Exploration workflow → Removed entirely
+ Keep essential schema listing guidance intact
+ Maintain prompt quality with focused content
+ Simplify maintenance and clarity
```

---

## Success Criteria

✓ Total lines between 90-110 (target: ~105 lines)  
✓ Exactly ONE scenario (basic)  
✓ No comprehensive function  
✓ No decorative headers (remove all === separators)  
✓ No best practices sections  
✓ No multi-schema or exploration functions  
✓ `generate_prompts()` directly returns `prompt_basic()`  
✓ `prompt_arguments()` returns empty vec  
✓ File compiles without errors  
✓ PromptProvider trait fully implemented

---

## Files Modified

| File | Change | Lines Before | Lines After |
|------|--------|--------------|-------------|
| `packages/kodegen-mcp-schema/src/database/list_schemas/prompts.rs` | Delete 3 functions, simplify routing | 645 | ~105 |

---

## Architecture Notes

The `kodegen-mcp-schema` package defines the PromptProvider trait (sealed trait, only implementable within the crate). This ensures all prompts follow consistent patterns. The `ListSchemasPrompts` struct implements this trait to provide AI-guidance prompts for the db_list_schemas tool.

By eliminating multi-scenario routing, we:
1. Reduce cognitive load for developers maintaining prompts
2. Simplify tool behavior (one response style)
3. Decrease file complexity for a Complexity-1 task
4. Maintain all essential information without decorative content

The simplified version focuses exclusively on teaching agents how to list schemas effectively, avoiding tangential topics like permissions, schema organization patterns, or comprehensive database guides that belong in external documentation.

---

## Related Patterns

This task follows the Complexity 1 (Trivial) template:
- Single focused responsibility
- No configuration options
- Direct, simple implementation
- Minimal architectural complexity
- Efficient code reuse
