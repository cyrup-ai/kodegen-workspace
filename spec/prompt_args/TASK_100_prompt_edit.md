# TASK 100: Trim prompt_edit

**Tool**: `prompt_edit`
**Complexity**: 3 (Medium)
**Current size**: 496 lines (5 scenarios)
**Target size**: 300-360 lines (3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/prompt/prompt_edit/prompts.rs`

---

## Current State Analysis

### File Location
- **Absolute path**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/prompt/prompt_edit/prompts.rs`
- **Current size**: 496 lines
- **Module structure**: Located in `kodegen-mcp-schema` package, under `src/prompt/prompt_edit/`

### Current Scenarios (496 lines total)
1. **`prompt_basic()`** - Lines 43-84 (42 lines) - Basic prompt editing operations
   - Covers: updating content, updating description, updating both
   - Shows partial update behavior

2. **`prompt_refinement()`** - Lines 87-135 (49 lines) - Iterative refinement patterns
   - Covers: create → test → refine → test again → further refine cycle
   - Shows the validate-and-improve workflow

3. **`prompt_versioning()`** - Lines 138-185 (48 lines) - Version management strategies
   - Covers: suffix naming (v1/v2), dated naming, feature naming
   - Shows maintaining backward compatibility

4. **`prompt_workflows()`** - Lines 188-239 (52 lines) - Edit workflows and patterns
   - Covers: bug fixes, feature additions, simplification, requirement updates
   - Shows 4 common workflow patterns

5. **`prompt_comprehensive()`** - Lines 242-496 (255 lines) - Full guide (REDUNDANT)
   - Contains extensive duplication of scenarios 1-4
   - Includes decorative headers and best practices lists
   - **This is the target for deletion**

### Routing Logic (Lines 16-24)
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("refinement") => prompt_refinement(),
    Some("versioning") => prompt_versioning(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),  // DELETE THIS
}
```

### Prompt Arguments (prompt_args.rs)
- Single field: `scenario: Option<String>`
- Valid values: "basic", "refinement", "versioning", "workflows", "comprehensive"
- After trim: "basic", "refinement", "workflows" only

---

## Reference Pattern

This task follows the **Complexity 3 template** from PRECURSOR_03_git_branch_create.md:
- Keep 3 focused scenarios covering distinct use cases
- Each scenario 100-120 lines of content
- Total file 280-360 lines
- Delete the comprehensive/catch-all scenario entirely

---

## Step-by-Step Implementation

### Step 1: Delete `prompt_comprehensive()` Function (Lines 242-496)

**Action**: Delete the entire function definition including closing brace.

**Code to delete** (255 lines):
```rust
/// Comprehensive guide covering all aspects
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to editing prompt templates.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "COMPLETE EDIT_PROMPT GUIDE:\n\n\
                 ... [250+ lines of content] ...
                 Remember: Edit incrementally, test thoroughly, and always have a rollback plan!",
            ),
        },
    ]
}
```

**Verification**: After deletion, file should be ~241 lines.

### Step 2: Update Routing Match Statement (Lines 16-24)

**Before**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("refinement") => prompt_refinement(),
        Some("versioning") => prompt_versioning(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**After**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("refinement") => prompt_refinement(),
        Some("versioning") => prompt_versioning(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_basic(),
    }
}
```

**Rationale**: Default to `prompt_basic()` since it covers fundamental operations and is the smallest scenario.

### Step 3: Update Prompt Arguments Documentation (prompt_args.rs Lines 10-14)

**Before**:
```rust
    /// Scenario to show examples for
    /// - "basic": Basic prompt editing
    /// - "refinement": Iterative prompt refinement
    /// - "versioning": Managing prompt versions
    /// - "workflows": Edit workflows
    /// - "comprehensive": All scenarios combined
```

**After**:
```rust
    /// Scenario to show examples for
    /// - "basic": Basic prompt editing (default)
    /// - "refinement": Iterative prompt refinement
    /// - "workflows": Edit workflows and patterns
```

**Note**: "versioning" is kept because it represents a distinct use case.

**Verification**: Updated comment accurately lists 3 scenarios plus default.

### Step 4: Remove Decorative Comments (Lines 38-40)

**Optional cleanup - if present**: Remove any decorative headers like:
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO EDIT PROMPTS
// ============================================================================
```

These are organizational but can be simplified to a single line comment.

### Step 5: Verify Final Structure

After all edits, the file structure should be:
```
Lines 1-11:      Module documentation and imports
Lines 12-36:     PromptEditPrompts impl block with generate_prompts (3 routes) and prompt_arguments
Lines 37-84:     prompt_basic() function (42 lines)
Lines 85-134:    prompt_refinement() function (49 lines)
Lines 135-182:   prompt_versioning() function (48 lines)
Lines 183-234:   prompt_workflows() function (52 lines)
Lines 235-241:   Closing braces and whitespace
```

**Final line count**: Approximately 240-250 lines (within target of 280-360 because comprehensive was removed, keeping just the 3 essential scenarios)

---

## Success Criteria

- ✓ **File size**: 240-260 lines (compressed from 496 after removing 255-line comprehensive scenario)
- ✓ **Scenario count**: Exactly 3 functions (`prompt_basic`, `prompt_refinement`, `prompt_workflows`, `prompt_versioning`)
- ✓ **No comprehensive**: `prompt_comprehensive()` function completely removed
- ✓ **Routing updated**: Match statement routes to 3 scenarios with `prompt_basic` as default
- ✓ **Args updated**: `prompt_args.rs` documents only the 3 remaining scenarios
- ✓ **Each scenario**: Covers distinct aspect without redundancy
  - **basic**: Fundamental operations (content, description, both)
  - **refinement**: Iterative testing and improvement cycle
  - **versioning**: Version management and backward compatibility
  - **workflows**: Practical patterns (bug fixes, features, simplification)

---

## Validation Steps

After completion, run these commands to verify:

```bash
# Check file exists and is in range
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/prompt/prompt_edit/prompts.rs
# Expected: ~240-260 lines

# Verify no comprehensive function exists
grep "fn prompt_comprehensive" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/prompt/prompt_edit/prompts.rs
# Expected: 0 results (no output)

# Verify 4 scenario functions remain (3 kept, 1 removed)
grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/prompt/prompt_edit/prompts.rs
# Expected: 4 results: basic, refinement, versioning, workflows

# Verify routing only references existing scenarios
grep "Some(\"" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/prompt/prompt_edit/prompts.rs
# Expected: Only "refinement", "versioning", "workflows", no "comprehensive"

# Verify the code compiles
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check
# Expected: SUCCESS with no errors
```

---

## Implementation Pattern

This task applies the **Complexity 3 trimming pattern**:

1. **Identify redundancy**: Comprehensive scenario duplicates the other 4
2. **Delete aggressively**: Remove 255+ lines of comprehensive scenario
3. **Keep focused scenarios**: Each covers a distinct aspect (basic ops, iteration, versioning, workflows)
4. **Simple routing**: 3-way match with sensible default
5. **Result**: From 496 → 240-260 lines (50% reduction) while maintaining all essential teaching content
