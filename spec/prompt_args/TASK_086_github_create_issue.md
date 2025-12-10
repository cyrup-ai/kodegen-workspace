# TASK 086: Trim github_create_issue prompts.rs

**Tool**: `github_create_issue`
**Complexity**: 3 (Medium)
**Current size**: 824 lines
**Target size**: 320-360 lines (2-3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_issue/prompts.rs`

---

## Current State Analysis

The prompts.rs file contains 5 scenario functions:
1. `prompt_bug_report()` (lines 49-165): 117 lines - Bug report patterns and best practices
2. `prompt_feature_request()` (lines 167-310): 144 lines - Feature request patterns and proposal structure
3. `prompt_metadata()` (lines 312-485): 174 lines - Metadata usage patterns (labels, assignees, milestones)
4. `prompt_templates()` (lines 487-685): 199 lines - Issue template patterns and formats
5. `prompt_comprehensive()` (lines 687-824): 138 lines - Comprehensive guide covering all aspects

The routing logic in `PromptProvider::generate_prompts()` (lines 17-23) matches on scenario names:
- "bug_report" → `prompt_bug_report()`
- "feature_request" → `prompt_feature_request()`
- "metadata" → `prompt_metadata()`
- "templates" → `prompt_templates()`
- Default (None or unknown) → `prompt_comprehensive()`

The prompt_arguments() function (lines 25-33) describes available scenarios as "bug_report, feature_request, metadata, templates".

---

## Core Objective

Reduce the file from 824 lines to 280-360 lines by removing redundant scenarios while maintaining comprehensive coverage through 3 focused scenarios:
1. **bug_report** scenario (~120 lines) - Basic issue creation covering bug patterns
2. **metadata** scenario (~120 lines) - Labels, assignees, milestones usage
3. **templates** scenario (~100 lines) - Issue template patterns and formats

This removes feature_request and comprehensive scenarios which overlap with other content.

---

## Step-by-Step Implementation

### Step 1: Delete prompt_feature_request() Function
**Lines to delete: 167-310 (144 lines)**

This function starts at line 167 with the function definition and ends at line 310 with the closing brace.

**Before (line 167):**
```rust
/// Feature request patterns and proposal structure
fn prompt_feature_request() -> Vec<PromptMessage> {
```

**After Step 1:**
Line 167 will be deleted entirely. All subsequent lines shift up by 144 lines.

**Verify**: Use grep to confirm deletion:
```bash
grep -n "fn prompt_feature_request" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_issue/prompts.rs
```
Should return no results after this step.

---

### Step 2: Delete prompt_comprehensive() Function
**Lines to delete: 543-680 (after Step 1, previously 687-824)**

After Step 1, the comprehensive function will be at a different line number. The comprehensive function is the largest and must be completely removed.

**To find exact line numbers after Step 1:**
```bash
grep -n "fn prompt_comprehensive" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_issue/prompts.rs
```

Delete everything from "/// Comprehensive guide..." comment through the final closing brace `}` of the function.

**Verify**: Grep should return no results:
```bash
grep -n "fn prompt_comprehensive" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_issue/prompts.rs
```

---

### Step 3: Update PromptProvider::generate_prompts() Match Statement
**Location: Lines 17-23 (in generate_prompts function)**

**Current state:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("bug_report") => prompt_bug_report(),
        Some("feature_request") => prompt_feature_request(),
        Some("metadata") => prompt_metadata(),
        Some("templates") => prompt_templates(),
        _ => prompt_comprehensive(),
    }
}
```

**Change to:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("bug_report") => prompt_bug_report(),
        Some("metadata") => prompt_metadata(),
        Some("templates") => prompt_templates(),
        _ => prompt_metadata(),  // Default to metadata as fallback
    }
}
```

**Changes made:**
- Remove line: `Some("feature_request") => prompt_feature_request(),`
- Change default case from `prompt_comprehensive()` to `prompt_metadata()`

---

### Step 4: Update prompt_arguments() Description
**Location: Lines 25-33 (in prompt_arguments function)**

**Current state:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (bug_report, feature_request, metadata, templates)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Change to:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (bug_report, metadata, templates)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Changes made:**
- Remove "feature_request" from the description string
- Keep only: bug_report, metadata, templates

---

## Verification Checklist

After completing all steps, verify:

1. **File compiles:**
   ```bash
   cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
   cargo check
   ```
   Should complete without errors.

2. **Line count is in range:**
   ```bash
   wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_issue/prompts.rs
   ```
   Should show 300-370 lines (allowing 10-20 line margin for the deletion of headers and whitespace).

3. **All 3 scenarios exist and are callable:**
   ```bash
   grep -n "fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_issue/prompts.rs
   ```
   Should show exactly 3 function definitions:
   - `prompt_bug_report()`
   - `prompt_metadata()`
   - `prompt_templates()`

4. **No references to deleted functions:**
   ```bash
   grep "feature_request\|prompt_comprehensive" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_issue/prompts.rs
   ```
   Should return no results.

5. **Match statement is correct:**
   ```bash
   grep -A 4 "fn generate_prompts" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_issue/prompts.rs
   ```
   Should show only 3 Some() branches and one default case pointing to `prompt_metadata()`.

---

## Success Criteria

✓ **File size**: 280-360 lines total
✓ **Scenario count**: Exactly 3 functions (bug_report, metadata, templates)
✓ **No feature_request**: All references removed
✓ **No comprehensive**: Function and all references deleted
✓ **Routing updated**: Match statement handles 3 scenarios correctly
✓ **Compilation**: `cargo check` passes with no warnings
✓ **Arguments updated**: prompt_arguments() description matches available scenarios

---

## Implementation Notes

- **Order of deletion matters**: Delete feature_request first (lines 167-310), then comprehensive. This keeps line numbers easier to track.
- **Use exact string matching**: The match arms are case-sensitive ("bug_report" not "BugReport").
- **Default behavior**: After changes, if an unknown scenario is requested, the tool defaults to metadata (a good middle-ground).
- **No structural changes**: Only remove function definitions and update routing logic. Do not modify existing scenario content.
- **Preserve comments**: Keep the comment header "// ============================================================================" and other inline documentation.

---

## Reference Materials

See **PRECURSOR_03_git_branch_create.md** for Complexity 3 template patterns (similar scope of changes).

Similar tasks completed:
- TASK_085: Trim git_branch_create (similar pattern of removing comprehensive scenario)
- TASK_084: Trim git_clone (similar metadata reduction)
