# TASK 065: Trim browser_extract_text

**Tool**: `browser_extract_text`
**Complexity**: 2 (Simple)
**Current size**: 872 lines
**Target size**: 170-220 lines
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/extract_text/prompts.rs`

---

## Current State Analysis

### File Structure
The prompts.rs file contains:
- **Lines 1-10**: License and module documentation
- **Lines 12-29**: PromptProvider trait implementation with routing match statement
- **Lines 31-38**: prompt_arguments() function returning scenario descriptions
- **Lines 40-42**: Comment section header (decorative - DELETE)
- **Lines 44-180**: prompt_page_content() function (~138 lines) - KEEP WITH TRIMMING
- **Lines 182-378**: prompt_specific_elements() function (~197 lines) - KEEP WITH TRIMMING
- **Lines 380-656**: prompt_structured_data() function (~277 lines) - DELETE ENTIRELY
- **Lines 658-851**: prompt_verification() function (~194 lines) - DELETE ENTIRELY
- **Lines 853-872+**: prompt_comprehensive() function (large block) - DELETE ENTIRELY

### Scenario Functions
Current routing in generate_prompts() (lines 19-24):
```rust
match args.scenario.as_deref() {
    Some("page_content") => prompt_page_content(),
    Some("specific_elements") => prompt_specific_elements(),
    Some("structured_data") => prompt_structured_data(),
    Some("verification") => prompt_verification(),
    _ => prompt_comprehensive(),
}
```

Current prompt_arguments description (line 33):
```
"Extraction scenario: page_content, specific_elements, structured_data, verification"
```

---

## Implementation Instructions

### Step 1: Update the Routing Match Statement
Replace lines 19-24 with this simpler version:

```rust
match args.scenario.as_deref() {
    Some("page_content") => prompt_page_content(),
    Some("specific_elements") => prompt_specific_elements(),
    _ => prompt_page_content(),
}
```

This routes both missing/None and unknown scenarios to the basic page_content scenario (the most fundamental use case).

### Step 2: Update prompt_arguments Description
Replace line 33 (the description field) with:
```
"Extraction scenario: page_content (basic text extraction), specific_elements (targeted by CSS selector)"
```

This updates the user-facing description to match the 2 kept scenarios.

### Step 3: Delete Comment Header Section
Remove lines 40-42 entirely (the comment section with decorative equals signs):
```
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO EXTRACT TEXT FROM WEB PAGES
// ============================================================================
```

After deletion, the next function prompt_page_content() will follow immediately after the imports and PromptProvider implementation.

### Step 4: Trim prompt_page_content() Function

**Current content (lines 44-180, ~138 lines)**

Keep the function signature and core teaching structure, but remove redundant examples. Remove the following subsections to reduce from 138 to ~100 lines:

**Remove from "EXAMPLES BY PAGE TYPE:" section:**
- Delete the entire "Article/Blog:" example block (4 lines)
- Delete the entire "Documentation:" example block (4 lines)
- Delete the entire "E-commerce product:" example block (3 lines)
- Delete the entire "Search results:" example block (3 lines)

This removes 14 lines of redundant examples. The "EXCLUDING CONTENT:" section and "Example workflow:" that follow should remain as they teach important concepts.

**What to Keep in prompt_page_content():**
- READING PAGE CONTENT section (full page, main content, multiple sections, specific containers)
- WHEN TO READ FULL PAGE vs WHEN TO USE SELECTORS (decision logic)
- PROGRESSIVE REFINEMENT STRATEGY (core teaching pattern)
- COMMON CONTENT SELECTORS (reference list)
- HANDLING DYNAMIC CONTENT (important edge case)
- READING STRATEGY (workflow)
- SELECTOR PRECEDENCE (reference)
- Final note about text-only extraction

**Result: ~100 lines** (down from 138)

### Step 5: Trim prompt_specific_elements() Function

**Current content (lines 182-378, ~197 lines)**

Remove sections that provide advanced/optional detail, keeping only the core patterns. Reduce from 197 to ~75 lines by:

**Remove these subsections entirely:**
- Delete the "PSEUDO-SELECTORS:" section (6 lines) - pseudo-selectors are advanced patterns
- Delete the "ATTRIBUTE SELECTOR SYNTAX:" section (13 lines) - detailed syntax table is optional for basic use
- Delete half of the "COMMON EXTRACTIONS BY TYPE:" examples:
  - Keep: Page title, Error messages, Success messages, Form field values, Buttons, Links
  - Delete: Metadata, Prices, Status/badges (these are use-case specific)

**What to Keep in prompt_specific_elements():**
- EXTRACTING SPECIFIC ELEMENTS section (ID, class, attribute, nested, direct children)
- Selected COMMON EXTRACTIONS BY TYPE (the most frequent patterns)
- SELECTOR STRATEGIES (priority: ID > class > element > attribute)
- HANDLING MULTIPLE MATCHES (important for understanding output)
- DEBUGGING SELECTORS (error handling)
- BEST PRACTICES (condensed summary)

**Result: ~75 lines** (down from 197)

### Step 6: Delete Entire Functions
Delete these functions completely (all their content):
- **prompt_structured_data()** (lines 380-656, ~277 lines) - This is a use-case scenario, not fundamental
- **prompt_verification()** (lines 658-851, ~194 lines) - This is a use-case scenario, not fundamental
- **prompt_comprehensive()** (lines 853+, ~500+ lines) - This is the comprehensive fallback that must be removed

**Deletion sequence:**
1. Find the start of "fn prompt_structured_data()" (line 380, after prompt_specific_elements() ends)
2. Delete all lines from fn prompt_structured_data() through fn prompt_comprehensive()
3. This removes 3 entire function definitions and their complete bodies

---

## Code Pattern Examples

### Before: Routing Match (lines 19-24)
```rust
match args.scenario.as_deref() {
    Some("page_content") => prompt_page_content(),
    Some("specific_elements") => prompt_specific_elements(),
    Some("structured_data") => prompt_structured_data(),
    Some("verification") => prompt_verification(),
    _ => prompt_comprehensive(),
}
```

### After: Routing Match
```rust
match args.scenario.as_deref() {
    Some("page_content") => prompt_page_content(),
    Some("specific_elements") => prompt_specific_elements(),
    _ => prompt_page_content(),
}
```

### Before: prompt_arguments Description (line 33)
```rust
description: Some("Extraction scenario: page_content, specific_elements, structured_data, verification".to_string()),
```

### After: prompt_arguments Description
```rust
description: Some("Extraction scenario: page_content (basic text extraction), specific_elements (targeted by CSS selector)".to_string()),
```

---

## Specific Line Deletions and Edits

### Edit 1: Update generate_prompts() method routing
- **Location**: Lines 19-24
- **Action**: Replace the entire match statement with 4-line version
- **Impact**: All 4 match arms reduced to 2 arms + default fallback

### Edit 2: Update prompt_arguments() description
- **Location**: Line 33 (the description field value)
- **Action**: Replace the string with updated scenario list
- **Impact**: User documentation matches available scenarios

### Edit 3: Delete decorative comment header
- **Location**: Lines 40-42
- **Action**: Delete all 3 lines of the separator comment block
- **Impact**: Removes 3 lines, makes prompt_page_content() function follow immediately

### Edit 4: Remove EXAMPLES BY PAGE TYPE from prompt_page_content()
- **Location**: Approximately lines 140-153 (within prompt_page_content)
- **Action**: Delete the 4 example subsections (Article/Blog, Documentation, E-commerce, Search results)
- **Impact**: Removes ~14 lines of redundant examples

### Edit 5: Delete prompt_structured_data() function entirely
- **Location**: Approximately lines 380-656
- **Action**: Delete the entire fn prompt_structured_data() and all its content
- **Impact**: Removes ~277 lines

### Edit 6: Delete prompt_verification() function entirely
- **Location**: Approximately lines 658-851
- **Action**: Delete the entire fn prompt_verification() and all its content
- **Impact**: Removes ~194 lines

### Edit 7: Delete prompt_comprehensive() function entirely
- **Location**: Approximately lines 853-872+
- **Action**: Delete the entire fn prompt_comprehensive() and all its content
- **Impact**: Removes ~500+ lines

### Edit 8: Trim prompt_specific_elements()
- **Location**: Within the prompt_specific_elements() function
- **Action**: 
  1. Remove "PSEUDO-SELECTORS:" section entirely (~6 lines)
  2. Remove "ATTRIBUTE SELECTOR SYNTAX:" section entirely (~13 lines)
  3. Remove use-case examples from "COMMON EXTRACTIONS BY TYPE:" (Metadata, Prices, Status/badges)
- **Impact**: Reduces this function from ~197 to ~75 lines

---

## Verification Steps

After making all changes, verify the following:

### Check 1: File compiles
```bash
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
cargo check --lib browser::extract_text
```

### Check 2: Line count is within target
```bash
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/extract_text/prompts.rs
# Expected: 170-220 lines
```

### Check 3: Routing match has exactly 2 specific scenarios + default
The generate_prompts() method should have:
- Line matching `Some("page_content")`
- Line matching `Some("specific_elements")`
- Default arm `_ => prompt_page_content()`
- Total: 3 arms in the match statement

### Check 4: Only 2 functions remain
The file should contain exactly 2 scenario functions:
- `fn prompt_page_content()`
- `fn prompt_specific_elements()`
- NO `fn prompt_structured_data()`
- NO `fn prompt_verification()`
- NO `fn prompt_comprehensive()`

### Check 5: prompt_arguments description is updated
The description string should list only:
- "page_content" (with brief description)
- "specific_elements" (with brief description)
- NO references to "structured_data", "verification"

---

## Success Criteria (Definition of Done)

The task is complete when ALL of the following are met:

1. **Line count**: `wc -l prompts.rs` returns 170-220 lines (target: ~200)

2. **Scenario count**: Exactly 2 scenario functions remain
   - prompt_page_content()
   - prompt_specific_elements()

3. **Routing updated**: generate_prompts() match statement has exactly 3 arms:
   - Some("page_content") => prompt_page_content()
   - Some("specific_elements") => prompt_specific_elements()
   - _ => prompt_page_content()

4. **Functions deleted**: These functions do NOT exist in the file:
   - prompt_structured_data()
   - prompt_verification()
   - prompt_comprehensive()

5. **prompt_arguments updated**: Description lists only page_content and specific_elements scenarios

6. **Compilation passes**: `cargo check` succeeds for the browser module

7. **No decorative headers**: The "HELPER FUNCTIONS" comment block is removed

8. **Content is proportional**: 
   - prompt_page_content() is approximately 100 lines
   - prompt_specific_elements() is approximately 75 lines
   - Combined scenario content is approximately 175 lines with boilerplate

---

## Reference

This task follows the Complexity 2 (Simple) template. The work is straightforward surgical trimming:
- Remove 3 entire functions (use-case scenarios + comprehensive fallback)
- Trim 2 functions to core patterns only
- Update routing logic to match remaining scenarios
- Remove decorative formatting comments
