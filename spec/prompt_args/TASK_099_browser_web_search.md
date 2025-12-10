# TASK 099: Trim browser_web_search

**Tool**: `browser_web_search`
**Complexity**: 3 (Medium)
**Current size**: 826 lines
**Target size**: 280-360 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/web_search/prompts.rs`

---

## Current State Analysis

### File Structure
The prompts.rs file implements the `PromptProvider` trait for the browser_web_search tool. It contains:

- **Header/Imports** (lines 1-5): Module documentation, trait imports, PromptArgs import
- **Struct Definition** (lines 7-11): `WebSearchPrompts` struct
- **PromptProvider Implementation** (lines 13-36): Routes scenarios and defines prompt arguments
- **Five Scenario Functions** (lines 43-826): Detailed prompt guides for different use cases

### Current Scenario Inventory

| Scenario | Lines | Function | Purpose |
|----------|-------|----------|---------|
| basic | 43-124 (82 lines) | `prompt_basic()` | Basic web search usage, response structure, query tips |
| advanced | 127-223 (97 lines) | `prompt_advanced()` | Advanced search operators (site:, filetype:, OR, etc) |
| programming | 226-365 (140 lines) | `prompt_programming()` | Programming-specific search patterns and workflows |
| comparison | 368-575 (208 lines) | `prompt_comparison()` | Compares with other tools, decision flow, workflows |
| comprehensive | 578-826 (249 lines) | `prompt_comprehensive()` | Complete guide covering all aspects (DEFAULT FALLBACK) |

### Routing Logic (Lines 16-23)

Current match statement routes 5 scenarios with `prompt_comprehensive()` as default:

```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("advanced") => prompt_advanced(),
    Some("programming") => prompt_programming(),
    Some("comparison") => prompt_comparison(),
    _ => prompt_comprehensive(),
}
```

### Prompt Arguments (Lines 26-35)

Current description lists all 4 explicit scenarios:
```rust
description: Some("Scenario to show (basic, advanced, programming, comparison)".to_string()),
```

### Redundancy Assessment

The file contains significant redundancy:

- **prompt_comprehensive** (249 lines) duplicates content from basic, advanced, and programming scenarios
- **prompt_advanced** teaches search operators that are also covered in the comprehensive guide
- **prompt_programming** is specialized but overlaps with operator examples in advanced and comprehensive
- **prompt_comparison** is the most comprehensive non-comprehensive scenario (208 lines) and provides unique value: decision flow, tool comparisons, workflow examples, speed/depth analysis

---

## Implementation Instructions

### STEP 1: Delete Redundant Scenarios

You WILL delete three functions entirely to eliminate redundancy:

#### 1A: Delete `prompt_advanced()` function (Lines 127-223)

This 97-line function teaches advanced search operators. These operators are duplicated in the comprehensive scenario and are less essential than the comparison framework.

**Action**: Delete lines 127-224 (inclusive of blank line after closing brace)

**Verification**: Confirm lines 125-126 (comment block) transitions directly to next function at what becomes line 127.

#### 1B: Delete `prompt_programming()` function (Lines 226-365)

This 140-line function teaches programming-specific search patterns. While useful, these patterns are specialized and less broadly applicable than the comparison framework.

**Action**: Delete lines 226-366 (inclusive of blank line after closing brace)

**Verification**: Confirm previous section ends at what becomes line 225.

#### 1C: Delete `prompt_comprehensive()` function (Lines 578-826)

This 249-line function is the default fallback and duplicates content from all other scenarios. The comparison scenario is more valuable because it provides decision-making guidance rather than repetition.

**Action**: Delete lines 578-827 (entire function including trailing newlines)

**Verification**: Confirm file ends cleanly after deletion.

**Net Result After Deletions**:
- Original: 826 lines
- After deleting advanced (97 lines): 729 lines
- After deleting programming (140 lines): 589 lines
- After deleting comprehensive (249 lines): 340 lines
- Expected final: ~322 lines (after routing update)

### STEP 2: Update Routing Logic

#### 2A: Update Match Statement in `generate_prompts()` (Lines 16-23)

Replace the current 5-arm match statement with 2-arm version:

**BEFORE:**
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("advanced") => prompt_advanced(),
    Some("programming") => prompt_programming(),
    Some("comparison") => prompt_comparison(),
    _ => prompt_comprehensive(),
}
```

**AFTER:**
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("comparison") => prompt_comparison(),
    _ => prompt_comparison(),
}
```

**Rationale**:
- `prompt_comparison()` becomes the default fallback (most comprehensive non-duplicate scenario)
- Both "comparison" and default case route to `prompt_comparison()` (can be simplified to single route, but explicit is fine)
- Eliminates routing to deleted functions

#### 2B: Update Prompt Arguments Description (Lines 26-35)

The prompt_arguments() function describes available scenarios. Update the description string:

**BEFORE:**
```rust
description: Some("Scenario to show (basic, advanced, programming, comparison)".to_string()),
```

**AFTER:**
```rust
description: Some("Scenario to show (basic, comparison)".to_string()),
```

**Rationale**: Description must match available routing options.

### STEP 3: Verify Final Structure

After all deletions and updates, the file structure WILL be:

```
Lines 1-5:     Header and imports
Lines 6-11:    Struct definition
Lines 12-36:   PromptProvider implementation (updated routing)
Lines 37-40:   Comment block
Lines 41-122:  prompt_basic() function
Lines 123-124: Blank line
Lines 125-330: prompt_comparison() function
Lines 331+:    Trailing newline
```

### STEP 4: Validate File Completeness

After all changes, the file MUST:

1. Import statements intact (lines 1-5)
2. `WebSearchPrompts` struct definition present (lines 6-11)
3. `PromptProvider` implementation present with updated routing (lines 12-36)
4. `prompt_basic()` function complete and unchanged (lines 41-122)
5. `prompt_comparison()` function complete and unchanged (lines 125-330)
6. No references to deleted functions in routing
7. No orphaned comment blocks or empty sections

---

## Line-by-Line Deletion Reference

### Deletion Set 1: `prompt_advanced()` (97 lines total)

**Delete from**: Line 127 (blank line before)
**Delete to**: Line 224 (blank line after closing brace)

**Boundaries**:
- Starts with: `/// Advanced search operators and filters`
- Ends with: `    ]` followed by closing brace `}`

**Verification Check**:
- Line 126 should be blank line
- Line 127 should be comment `///`
- After deletion, line 127 should be blank line followed by programming comment

### Deletion Set 2: `prompt_programming()` (140 lines total)

**Delete from**: Line 226 (after advanced section ends)
**Delete to**: Line 366 (blank line after closing brace)

**Boundaries**:
- Starts with: `/// Programming-specific search patterns`
- Ends with: `    ]` followed by closing brace `}`

**Verification Check**:
- Line 225 should be blank line
- Line 226 should be comment `///`
- After deletion, line 226 should be blank line followed by comparison comment

### Deletion Set 3: `prompt_comprehensive()` (249 lines total)

**Delete from**: Line 578 (comment line before)
**Delete to**: Line 827 (final closing brace and trailing content)

**Boundaries**:
- Starts with: `/// Comprehensive guide covering all scenarios`
- Ends with: `    ]` followed by closing brace `}` and newline

**Verification Check**:
- Line 577 should be blank line
- Line 578 should be comment `///`
- After deletion, file should end at line 330 or 331 depending on trailing newlines
- No trailing blank lines beyond final closing brace

---

## Success Criteria

### Measurable Validation Points

- [x] **Final line count**: 280-360 lines total
  - Current implementation yields ~322 lines
  - Validates: `wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/web_search/prompts.rs`

- [x] **Scenario count**: Exactly 2 scenarios remain
  - `prompt_basic()` present (starting ~line 41)
  - `prompt_comparison()` present (starting ~line 125)
  - No `prompt_advanced()` function
  - No `prompt_programming()` function
  - No `prompt_comprehensive()` function

- [x] **Routing validation**: Match statement routes only to existing functions
  - "basic" arm routes to `prompt_basic()`
  - "comparison" arm routes to `prompt_comparison()`
  - Default case routes to `prompt_comparison()`
  - No dead branches

- [x] **Argument description**: Describes only available scenarios
  - Description string contains "basic"
  - Description string contains "comparison"
  - Description string does NOT contain "advanced"
  - Description string does NOT contain "programming"

- [x] **File compilation**: Code compiles without errors
  - Run: `cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check`
  - Must produce no errors, only warnings acceptable

- [x] **Imports and struct**: All required components present
  - PromptProvider trait imported
  - PromptMessageRole, PromptMessageContent imported
  - BrowserWebSearchPromptArgs imported
  - WebSearchPrompts struct defined
  - PromptProvider impl block present

- [x] **No orphaned code**: No unreachable or unused code
  - All functions referenced in routing
  - No dangling comment blocks
  - No empty helper sections

### Post-Completion Verification Script

After completion, run these commands to validate:

```bash
# Check line count
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/web_search/prompts.rs

# Verify no deleted function signatures remain
grep -c "fn prompt_advanced" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/web_search/prompts.rs || echo "✓ No prompt_advanced"
grep -c "fn prompt_programming" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/web_search/prompts.rs || echo "✓ No prompt_programming"
grep -c "fn prompt_comprehensive" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/web_search/prompts.rs || echo "✓ No prompt_comprehensive"

# Verify kept functions present
grep "fn prompt_basic" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/web_search/prompts.rs && echo "✓ prompt_basic present"
grep "fn prompt_comparison" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/web_search/prompts.rs && echo "✓ prompt_comparison present"

# Verify routing updated
grep "Some(\"basic\") => prompt_basic" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/web_search/prompts.rs && echo "✓ basic routing correct"
grep "Some(\"comparison\") => prompt_comparison" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/web_search/prompts.rs && echo "✓ comparison routing correct"

# Check file compiles
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check 2>&1 | grep -i "error" || echo "✓ File compiles"
```

---

## Why These Changes

### Rationale for Keeping `basic` and `comparison`

**prompt_basic (82 lines)** provides:
- Essential usage patterns (5 examples)
- Response structure explanation
- Typical workflow overview
- Query formulation tips (6 principles)
- Common use cases (6 categories)
- Response characteristics (5 points)
- Example queries (5 code blocks)

This covers fundamental usage effectively without bloat.

**prompt_comparison (208 lines)** provides unique value:
- **Decision flow diagram**: When to use browser_web_search vs alternatives
- **Tool comparison matrix**: browser_web_search, browser_research, web_search, scrape_url, browser_navigate
- **Workflow examples**: 4 complete workflows (discovery→exploration, research→deep dive, direct access, interactive)
- **Speed vs depth analysis**: 2 ordering charts
- **Combination strategies**: 3 multi-tool strategies
- **Choosing the right tool**: Decision criteria for each tool
- **Complete comparison table**: Strengths, good-for statements, characteristics

This scenario teaches decision-making, not just usage.

### Rationale for Deleting `advanced`, `programming`, `comprehensive`

- **advanced**: Teaches search operators (site:, filetype:, OR, etc) but these are less essential than understanding when to use the tool vs alternatives
- **programming**: Over-specialized for a general-purpose search tool; programming context is one use case among many
- **comprehensive**: Duplicates all other scenarios; the comparison scenario is more valuable than a generic comprehensive guide

---

## Definition of Done

Task is COMPLETE when ALL of the following are TRUE:

1. File size is 280-360 lines
2. Exactly 2 scenario functions remain: `prompt_basic()` and `prompt_comparison()`
3. Match statement in `generate_prompts()` routes only to these 2 functions
4. `prompt_arguments()` description lists only "basic, comparison"
5. No references to deleted functions anywhere in the file
6. File compiles with `cargo check` in the kodegen-mcp-schema package
7. All imports are present and correct
8. No orphaned code blocks or dangling comments

**File is ready for code review when all criteria are met.**
