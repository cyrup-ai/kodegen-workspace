# TASK 078: Trim fs_edit_block

**Tool**: `fs_edit_block`
**Complexity**: 3 (Medium)
**Current size**: 436 lines
**Target size**: 280-360 lines (3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/edit_block/prompts.rs`

---

## Current State Analysis

### File Structure
The prompts.rs file contains the EditBlockPrompts struct and 6 prompt functions:

```
Lines 1-44:       EditBlockPrompts struct with PromptProvider impl
Lines 46-115:     prompt_basic() - 70 lines - basic string replacement
Lines 117-189:    prompt_precision() - 73 lines - context-based uniqueness
Lines 191-264:    prompt_multiline() - 74 lines - multi-line code blocks
Lines 266-354:    prompt_safety() - 89 lines - safe editing practices
Lines 356-392:    prompt_workflows() - 37 lines - common workflow patterns
Lines 394-436:    prompt_comprehensive() - 43 lines - comprehensive fallback (default)
```

### Match Statement (Lines 17-24)
Currently routes 6 scenarios to their functions, with comprehensive as default fallback.

---

## Implementation Instructions

### Step 1: Enhance prompt_precision() Function
**Target**: Expand from 73 to 100-120 lines by adding multiline examples and expected_replacements focus.

**Location**: Lines 117-189 (keep this function, but expand it)

**Action**: Replace the existing prompt_precision() function with enhanced version that includes:
1. Keep all 3 existing precision examples (context, function scope, markers)
2. Add 2-3 multiline examples (from deleted prompt_multiline):
   - Replace entire function with new implementation
   - Replace struct definition with added fields
   - Replace import block with new imports
3. Add expected_replacements validation examples (emphasizing count validation)
4. Rename user question to cover both "precision" AND "expected replacements"

**New prompt_precision content structure**:
```rust
fn prompt_precision() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I make precise edits and validate replacement counts?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "PRECISE EDITING & VALIDATION:\n\n\
                 1. Include context for uniqueness:\n\
                    // Exact context matching...\n\n\
                 2. Edit specific function:\n\
                    // Function-level precision...\n\n\
                 3. Validate replacement count:\n\
                    // expected_replacements parameter...\n\n\
                 4. Replace multi-line blocks:\n\
                    // Struct replacement example...\n\n\
                 5. Import block updates:\n\
                    // Multiple imports at once...\n\n\
                 PRECISION & SAFETY:\n\
                 - Include enough context\n\
                 - Use expected_replacements to validate count\n\
                 - Preserve exact whitespace\n\
                 - Combine with fs_search for confidence",
            ),
        },
    ]
}
```

### Step 2: Delete prompt_multiline() Function
**Location**: Lines 191-264 (entire function)

**Action**: Completely remove prompt_multiline() - its content is merged into enhanced prompt_precision().

### Step 3: Delete prompt_safety() Function  
**Location**: Lines 266-354 (entire function)

**Action**: Completely remove prompt_safety() - its essential safety practices about expected_replacements are incorporated into prompt_precision().

### Step 4: Delete prompt_comprehensive() Function
**Location**: Lines 394-436 (entire function)

**Action**: Completely remove prompt_comprehensive() - the comprehensive/default fallback scenario is eliminated per requirements. Will use prompt_basic() as default instead.

### Step 5: Update PromptProvider impl Match Statement
**Location**: Lines 17-24

**Current routing**:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("precision") => prompt_precision(),
    Some("multiline") => prompt_multiline(),
    Some("safety") => prompt_safety(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

**New routing**:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("precision") => prompt_precision(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_basic(),
}
```

**Changes**:
- Remove: `Some("multiline") => prompt_multiline(),`
- Remove: `Some("safety") => prompt_safety(),`
- Change default from `prompt_comprehensive()` to `prompt_basic()`

### Step 6: Update prompt_arguments() Description
**Location**: Lines 30-35

**Current text**:
```rust
description: Some("Scenario to show (basic, precision, multiline, safety, workflows, comprehensive)".to_string()),
```

**New text**:
```rust
description: Some("Scenario to show (basic, precision, workflows)".to_string()),
```

---

## Detailed Line-by-Line Changes

### Change 1: Update prompt_arguments() description
Find line 32 (inside PromptArgument description field):
- OLD: `"Scenario to show (basic, precision, multiline, safety, workflows, comprehensive)"`
- NEW: `"Scenario to show (basic, precision, workflows)"`

### Change 2: Update match statement in generate_prompts()
Find lines 17-24 in the match statement:
- Delete line: `Some("multiline") => prompt_multiline(),`
- Delete line: `Some("safety") => prompt_safety(),`
- Change line: `_ => prompt_comprehensive(),` to `_ => prompt_basic(),`

### Change 3: Keep prompt_basic() unchanged
Lines 46-115 - No changes needed. Keep exactly as-is.

### Change 4: Replace prompt_precision() 
Lines 117-189 - Replace entire function with enhanced version.

Target content should include:
- Original 3 precision examples (context, function scope, markers)
- 2-3 multiline examples (functions, structs, imports)
- 2-3 expected_replacements validation examples
- Updated PRECISION TIPS section

Result: ~100-120 lines for this function

### Change 5: Delete prompt_multiline()
Lines 191-264 - Remove entirely. No replacement.

### Change 6: Delete prompt_safety()
Lines 266-354 - Remove entirely. No replacement.

### Change 7: Keep prompt_workflows() unchanged
Lines 356-392 - No changes needed (will shift up after deletions).

### Change 8: Delete prompt_comprehensive()
Lines 394-436 - Remove entirely. No replacement.

---

## Success Criteria (Exact Measurements)

After completing all changes:

✓ **Total file size**: 280-360 lines (target: ~300-320 lines)
  - Expected: ~44 (header) + 70 (basic) + 110 (enhanced precision) + 37 (workflows) = ~261-280 lines

✓ **Number of scenarios**: Exactly 3
  - basic
  - precision (expanded to cover expected_replacements focus)
  - workflows

✓ **No comprehensive scenario**: Verify prompt_comprehensive() function is completely removed

✓ **No multiline scenario**: Verify prompt_multiline() function is completely removed

✓ **No safety scenario**: Verify prompt_safety() function is completely removed

✓ **Match statement routes exactly 3 scenarios**: basic, precision, workflows

✓ **Default fallback**: Changed to prompt_basic() instead of prompt_comprehensive()

✓ **prompt_arguments updated**: Description lists only 3 scenarios

### Verification Steps
1. Verify file compiles: `cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check`
2. Count scenarios: `grep -c "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/edit_block/prompts.rs`
   - Should output: 3
3. Count match arms: Check match statement has exactly 3 Some(...) arms + 1 default
4. Verify no orphaned function references in match statement

---

## Reference: Scenario Descriptions

### Kept Scenario 1: prompt_basic()
**Purpose**: Teach basic exact string replacement
**Length**: ~70 lines
**Content**:
- Simple replacement example
- Variable name replacement
- Multiple occurrences with expected_replacements
- KEY PARAMETERS section explaining path, old_string, new_string, expected_replacements

### Kept Scenario 2: prompt_precision() [ENHANCED]
**Purpose**: Teach precision editing, uniqueness, and expected_replacements validation
**Length**: ~100-120 lines  
**Content**:
- Context inclusion for uniqueness (original)
- Edit specific function with unique context (original)
- Unique markers using comments (original)
- Replace entire function example (from multiline)
- Replace struct definition example (from multiline)
- Replace import block example (from multiline)
- Validate replacement count example (from safety)
- Search before edit integration (from safety)
- PRECISION & SAFETY section combining both aspects
- Tips on context, expected_replacements, whitespace

### Kept Scenario 3: prompt_workflows()
**Purpose**: Teach common usage patterns
**Length**: ~37 lines
**Content**:
- Bug fix workflow (read -> edit -> verify)
- Refactoring workflow (search -> edit)
- Configuration change
- Add import
- Batch rename pattern

---

## Complexity Notes

This is a Complexity 3 task (Medium):
- Requires understanding of 6 existing scenarios
- Involves strategic trimming and selective consolidation
- Must update routing logic and argument descriptions
- Final validation requires both line count and structural verification
- Change affects prompt delivery system (routing must be precise)
