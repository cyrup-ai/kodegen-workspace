# TASK 070: Trim memory_memorize Prompts

**Tool**: `memory_memorize`
**Complexity**: 2 (Simple)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/memory/memorize/prompts.rs`

---

## Current State Analysis

### File Metrics
- **Current size**: 772 lines (very large)
- **Target size**: 170-220 lines (80% reduction)
- **Current scenarios**: 5 total
  - `prompt_basic()`: lines 47-180 (134 lines) - KEEP and trim to 100-110 lines
  - `prompt_organization()`: lines 182-450 (269 lines) - KEEP and trim to 80-90 lines
  - `prompt_patterns()`: lines 452-700 (249 lines) - DELETE entirely
  - `prompt_workflows()`: lines 702-900+ (200+ lines) - DELETE entirely
  - `prompt_comprehensive()`: lines 900+ (very large) - DELETE entirely

### Routing Logic (PromptProvider impl)
Located in lines 14-36:
- **Current match statement** (lines 18-23): Routes to 5 scenarios, defaults to `prompt_comprehensive()`
- **Current prompt_arguments** (line 31): Description lists all 4 optional scenarios
- These MUST be updated to only support `"basic"` and `"organization"`

### Decorative Headers to Remove
- Lines 37-38: Large comment block "// HELPER FUNCTIONS - TEACH AI AGENTS..."
- This adds no functional value and wastes ~2 lines

---

## Implementation Instructions

### Step 1: Update the PromptProvider Match Statement

**Location**: Lines 18-23 in `impl PromptProvider for MemorizePrompts`

**Before**:
```rust
    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("organization") => prompt_organization(),
            Some("patterns") => prompt_patterns(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }
```

**After**:
```rust
    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("organization") => prompt_organization(),
            _ => prompt_basic(),
        }
    }
```

**Action**: Replace the match statement with the "After" version. This removes 3 branches and changes the default from `prompt_comprehensive()` to `prompt_basic()`.

### Step 2: Update the prompt_arguments Description

**Location**: Line 31 in `fn prompt_arguments() -> Vec<PromptArgument>`

**Before**:
```rust
            description: Some("Scenario to show (basic, organization, patterns, workflows)".to_string()),
```

**After**:
```rust
            description: Some("Scenario to show (basic, organization)".to_string()),
```

**Action**: Replace the description string to remove references to deleted scenarios.

### Step 3: Delete Decorative Header Comment

**Location**: Lines 37-38

**Content to delete**:
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE MEMORY_MEMORIZE
// ============================================================================
```

**Action**: Remove these 2 lines entirely. Add one blank line after line 36 (closing brace of impl block) before the `prompt_basic()` function.

### Step 4: Trim prompt_basic() Function

**Location**: Lines 47-180 (currently 134 lines, trim to ~100-110 lines)

**Strategy**: Remove verbose sections while keeping core teaching content:
- KEEP: User question, core explanation, STORING MEMORIES, RESPONSE, PARAMETERS
- TRIM: Reduce from 4 usage examples to 2-3 concise examples
- TRIM: Consolidate LIBRARY NAMING and CONTENT GUIDELINES sections
- KEEP: BEST PRACTICES section (abbreviated)
- REMOVE: Duplicate explanations of "What happens" and "Response fields"

**Specific trimming targets**:
- Remove "BASIC USAGE EXAMPLES" header and condense 4 examples → 2 examples
- Shorten LIBRARY NAMING bullets from 4 bullets to 2
- Consolidate CONTENT GUIDELINES into 3-4 bullet points
- Keep BEST PRACTICES but reduce from 5 items to 3

**Expected outcome**: Function should be ~100-110 lines after trimming

### Step 5: Trim prompt_organization() Function

**Location**: Lines 182-450 (currently 269 lines, trim to ~80-90 lines)

**Strategy**: Remove redundant content while preserving organization teaching:
- KEEP: User question, intro paragraph
- KEEP: 2 main organization strategies (reduce from 4 to 2):
  - Strategy A: BY PROJECT
  - Strategy B: BY TYPE (or DOMAIN - choose one)
- TRIM: Remove "4. BY TASK OR INVESTIGATION" strategy entirely
- TRIM: Consolidate NAMING CONVENTIONS bullets
- KEEP: LISTING LIBRARIES and RECALL FROM LIBRARY with code examples
- TRIM: CHOOSING A STRATEGY from 4 items to 2 items
- REMOVE: EXAMPLE MULTI-LIBRARY WORKFLOW (redundant with main examples)
- TRIM: BEST PRACTICES from 5 items to 3 items

**Specific trimming targets**:
- Remove "2. BY CONTENT TYPE" entirely (keep either this OR BY DOMAIN, not both)
- Reduce Strategy section from 4 items to 2 items
- Remove redundant explanation paragraphs
- Shorten CHOOSING A STRATEGY to 2-3 key points

**Expected outcome**: Function should be ~80-90 lines after trimming

### Step 6: Delete All Remaining Functions

These functions must be deleted entirely:

**1. Delete prompt_patterns() function**
- Location: Lines 452-700 (entire function, 249 lines)
- Includes: User/Assistant message structure and all pattern examples
- Action: Remove the entire function definition

**2. Delete prompt_workflows() function**
- Location: Lines 702-900+ (entire function, 200+ lines)
- Includes: User/Assistant message structure and all workflow examples
- Action: Remove the entire function definition

**3. Delete prompt_comprehensive() function**
- Location: Lines 900+ to EOF (entire function, very large)
- Includes: User/Assistant message structure and comprehensive guide
- Action: Remove the entire function definition

**Verification**: After deletion, the file should have ONLY these functions:
- `prompt_basic()`
- `prompt_organization()`
- NO other helper functions

---

## Code Patterns: Before and After

### Pattern 1: Match Statement
**Before** (lines 18-23):
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("organization") => prompt_organization(),
    Some("patterns") => prompt_patterns(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

**After**:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("organization") => prompt_organization(),
    _ => prompt_basic(),
}
```

### Pattern 2: prompt_basic Consolidation
**Before**: 134 lines including 4 separate usage examples
```rust
1. Store API documentation:
   memory_memorize({...})
2. Store code insight:
   memory_memorize({...})
3. Store decision:
   memory_memorize({...})
4. Store finding:
   memory_memorize({...})
```

**After**: ~100 lines with 2 consolidated examples
```rust
// Example 1: Store configuration knowledge
memory_memorize({
    "library": "project-notes",
    "content": "The API uses JWT tokens with 1-hour expiry for authentication"
})

// Example 2: Store implementation details
memory_memorize({
    "library": "codebase",
    "content": "Config loader in src/config/loader.rs uses TOML format with env var overrides"
})
```

### Pattern 3: prompt_organization Consolidation
**Before**: 269 lines with 4 strategies
- 1. BY PROJECT
- 2. BY CONTENT TYPE
- 3. BY DOMAIN
- 4. BY TASK OR INVESTIGATION

**After**: ~80-90 lines with 2 strategies
- Strategy A - By Project
- Strategy B - By Type

---

## Detailed Line-by-Line Changes

### File Header (Keep as-is)
Lines 1-36: Module definition, imports, PromptProvider impl
- No changes to this section

### Change 1: Match Statement (Lines 18-23)
Replace with 3-arm version (remove patterns, workflows, comprehensive)

### Change 2: prompt_arguments (Line 31)
Update description string from `"(basic, organization, patterns, workflows)"` to `"(basic, organization)"`

### Change 3: Remove Header (Lines 37-38)
Delete decorative comment header entirely

### Change 4: Edit prompt_basic (Lines 47-180)
Reduce from 134 to ~100-110 lines by:
- Consolidating examples (4 → 2)
- Shortening explanations
- Removing duplicate information

**Key content to PRESERVE**:
- User question: "How do I store content in a memory library?"
- STORING MEMORIES section with code example
- RESPONSE structure showing library and stored fields
- WHAT HAPPENS explanation
- PARAMETERS list
- BEST PRACTICES (abbreviated to 3-4 items)

### Change 5: Edit prompt_organization (Lines 182-450)
Reduce from 269 to ~80-90 lines by:
- Keep only 2 main strategies (BY PROJECT and BY TYPE)
- Remove strategy combinations
- Remove EXAMPLE MULTI-LIBRARY WORKFLOW
- Keep LISTING LIBRARIES and RECALL FROM LIBRARY
- Shorten BEST PRACTICES

**Key content to PRESERVE**:
- User question: "How should I organize my memory libraries?"
- LIBRARY ORGANIZATION STRATEGIES (2 only)
- NAMING CONVENTIONS
- LISTING LIBRARIES with code example
- RECALL FROM SPECIFIC LIBRARY with code example
- BEST PRACTICES (abbreviated to 3 items)

### Change 6: Delete prompt_patterns (Lines 452-700)
Remove entire function and closing brace

### Change 7: Delete prompt_workflows (Lines 702-900+)
Remove entire function and closing brace

### Change 8: Delete prompt_comprehensive (Remaining lines)
Remove entire function and all content to end of file

---

## Success Criteria

File must meet ALL of these criteria to be considered complete:

- **Line Count**: 170-220 total lines (target: 215-219 lines)
- **Function Count**: Exactly 2 scenario functions (`prompt_basic` and `prompt_organization`)
- **Match Statement**: Exactly 2 arms + default (no references to patterns/workflows/comprehensive)
- **Prompt Arguments**: Description updated to only mention `basic` and `organization`
- **No Decorative Headers**: All large comment blocks removed
- **No Dead Code**: File contains no function definitions for deleted scenarios
- **Compilation**: Code must compile without errors or warnings
- **Routing**: Default case must route to `prompt_basic()` not deleted function
- **Consistency**: Both scenario functions follow same message structure (User/Assistant)

### Verification Checklist
- [ ] File is between 170-220 lines (verify with `wc -l`)
- [ ] Only 2 scenario functions exist in file (search for `fn prompt_`)
- [ ] Match statement has exactly 3 cases (2 Some() + 1 default)
- [ ] No references to "patterns" in code (search the file)
- [ ] No references to "workflows" in code (search the file)
- [ ] No references to "comprehensive" in code (search the file)
- [ ] All deleted functions completely removed (no partial deletions)
- [ ] Code compiles: `cd packages/kodegen-mcp-schema && cargo check`

---

## Technical Context

**File Purpose**: Provides MCP prompt messages for the memory_memorize tool. The PromptProvider trait routes based on a scenario argument to return different educational prompts.

**Why This Trimming**: The current file is bloated with redundant scenarios. The basic and organization scenarios provide sufficient guidance for AI agents. Patterns, workflows, and comprehensive scenarios repeat the same concepts with minor variations, creating unnecessary size overhead.

**Impact**: 
- Reduces file size from 772 to ~215 lines (72% reduction)
- Maintains core functionality (still routes to 2 scenarios)
- Improves maintainability (fewer scenarios to update)
- Faster prompt loading (smaller message payloads)

---

## Implementation Order

Execute these steps IN THIS EXACT ORDER:

1. Update match statement (removes function calls)
2. Update prompt_arguments description
3. Delete decorative header comment
4. Trim prompt_basic() function
5. Trim prompt_organization() function
6. Delete prompt_patterns() function
7. Delete prompt_workflows() function
8. Delete prompt_comprehensive() function
9. Verify file compiles without errors
10. Verify final line count is 170-220 lines

Do not skip steps or do them out of order - each step depends on previous changes being clean.
