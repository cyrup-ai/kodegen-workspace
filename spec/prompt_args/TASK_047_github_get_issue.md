# TASK 047: Trim github_get_issue

**Tool**: `github_get_issue`
**Complexity**: 2 (Simple)
**Current size**: 882 lines
**Target size**: 170-220 lines (1-2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_issue/prompts.rs`

---

## Current State Analysis

### File Structure
- **Location**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_issue/prompts.rs`
- **Current total lines**: 882
- **Current scenario count**: 5

### Current Scenarios
1. **prompt_basic()** (lines 46-141, ~95 lines)
   - Covers basic issue retrieval by number
   - Explains response structure with core fields
   - Lists key issue fields
   - Describes how to read issue content
   - Includes parameters, authentication, common use cases, error handling, best practices
   - **Status**: KEEP (minor trimming may be needed)

2. **prompt_metadata()** (lines 143-345, ~202 lines)
   - Covers optional metadata fields: labels, assignees, milestone, comments, pull_request
   - Explains each metadata type with examples
   - Shows metadata workflows and patterns
   - **Status**: KEEP (requires significant trimming from ~202 to ~80-100 lines)

3. **prompt_comments()** (lines 347-520, ~173 lines)
   - Covers getting comments on issues
   - Workflow for comment retrieval
   - Decision logic for fetching comments
   - Use cases and best practices
   - **Status**: DELETE (use-case scenario)

4. **prompt_workflows()** (lines 522-814, ~292 lines)
   - Covers 6 different workflows: UNDERSTAND, START WORK, TRIAGE, DUPLICATE CHECK, STATUS TRACKING, ISSUE REPORTING
   - Real-world issue investigation patterns
   - **Status**: DELETE (explicit use-case scenarios)

5. **prompt_comprehensive()** (lines 816-882, ~66 lines for definition)
   - Comprehensive guide covering all aspects
   - Includes basic usage, response structure, field guide, common patterns, error handling, workflows, related tools, best practices
   - **Status**: DELETE (explicitly mentioned in task)

### Routing Implementation (lines 15-31)
Current match statement handles:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("metadata") => prompt_metadata(),
    Some("comments") => prompt_comments(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

Prompt arguments description lists: "(basic, metadata, comments, workflows)"

---

## Implementation Instructions

### Step 1: Update PromptProvider Routing (Lines 15-23)
**Current code**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("metadata") => prompt_metadata(),
        Some("comments") => prompt_comments(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**Change to**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("metadata") => prompt_metadata(),
        _ => prompt_basic(),
    }
}
```

**Why**: Simplify routing to only support two scenarios (basic as default).

### Step 2: Update prompt_arguments() (Lines 25-31)
**Current code**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, metadata, comments, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Change to**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, metadata)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Why**: Update description to reflect only two remaining scenarios.

### Step 3: Remove Decorative Header Comment (Lines 36-40)
**Delete this entire block**:
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GITHUB_GET_ISSUE
// ============================================================================
```

**Why**: Task requires removing decorative headers to reduce line count.

### Step 4: Keep prompt_basic() Function (Lines 46-141)
- **Action**: KEEP without modification
- **Line count**: ~95 lines
- **Content preserved**:
  - Getting issue details (3 examples: by number, from org, from own repo)
  - Response structure with all key fields
  - Key issue fields enumerated
  - Reading issue content guide
  - Parameters documentation
  - Authentication requirements
  - Common use cases
  - Error handling codes
  - Best practices

### Step 5: Trim prompt_metadata() Function (Lines 143-345)
**Current size**: ~202 lines
**Target size**: ~80-100 lines
**Required reductions**:
1. Remove redundant examples (keep core concepts)
2. Consolidate metadata workflow descriptions
3. Keep essential field explanations
4. Reduce example code snippets from 5+ to 2-3

**Keep**:
- ISSUE METADATA FIELDS section with response example
- LABELS explanation (condensed)
- ASSIGNEES explanation (condensed)
- MILESTONE explanation (condensed)
- COMMENTS COUNT explanation
- PULL REQUEST LINK explanation

**Delete or condense**:
- Remove "USING METADATA" section (multiple redundant examples)
- Condense "METADATA WORKFLOWS" to brief summary
- Consolidate "BEST PRACTICES"

**Specific changes**:
- Lines with "1. Filter by label:", "2. Check assignment:", "3. Milestone tracking:", etc. - keep only 1-2 most important
- Remove full "METADATA WORKFLOWS" section (lines ~236-280) - reduce to 2-3 bullet points
- Condense BEST PRACTICES list

### Step 6: Delete prompt_comments() Function (Lines 347-520)
**Action**: DELETE entire function
**Line count removed**: ~173 lines
**Reason**: This is a use-case scenario specifically requested for deletion in task

### Step 7: Delete prompt_workflows() Function (Lines 522-814)
**Action**: DELETE entire function  
**Line count removed**: ~292 lines
**Reason**: These are explicit use-case scenarios (6 different workflows), task requires deletion of use-case scenarios

### Step 8: Delete prompt_comprehensive() Function (Lines 816-882)
**Action**: DELETE entire function
**Line count removed**: ~66+ lines (entire remaining portion)
**Reason**: Task explicitly states "No comprehensive" in success criteria

---

## Expected Result After Changes

### File Structure After Trimming
```
Lines 1-7:     Header and imports (unchanged)
Lines 8-32:    GetIssuePrompts struct and impl (updated routing)
Line 33:       Empty line
Lines 34-128:  prompt_basic() function (~95 lines)
Lines 129-210: prompt_metadata() function (~80 lines, trimmed)
```

### Line Count Breakdown
- Header/imports: 7 lines
- GetIssuePrompts struct + impl: 25 lines (updated routing)
- Empty/spacing: 1 line
- prompt_basic(): ~95 lines
- prompt_metadata(): ~85 lines (trimmed from 202)
- **Total**: ~213 lines (within 170-220 target range)

### Scenarios Remaining
- ✓ basic: Retrieving issue details
- ✓ metadata: Issue metadata (labels, assignees, milestone)

### Deleted Content
- ✗ comments: Getting issue comments workflow
- ✗ workflows: 6 different issue investigation workflows
- ✗ comprehensive: All-in-one comprehensive guide

---

## Success Criteria

The task is complete when:

1. **Line count**: Total file is 170-220 lines
   - Measure with: `wc -l prompts.rs`
   - Expected: ~210-215 lines

2. **Scenario count**: Exactly 2 scenarios remain
   - Measure with: `grep -c "^fn prompt_" prompts.rs`
   - Expected: 2

3. **No comprehensive scenario**: prompt_comprehensive() function is deleted
   - Measure with: `grep -c "fn prompt_comprehensive" prompts.rs`
   - Expected: 0 (zero matches)

4. **Routing is simplified**: Match statement handles only basic and metadata
   - Check lines 15-23 have only 2 Some() arms plus default

5. **Decorative headers removed**: No lines with "=============" comment patterns
   - Measure with: `grep -c "=====" prompts.rs`
   - Expected: 0 (zero matches)

6. **Code compiles**: Run cargo check
   ```bash
   cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
   cargo check
   ```
   - Expected: No compilation errors

---

## Definition of Done

This task is **COMPLETE** when:
- The file contains exactly 170-220 lines total
- Only 2 scenario functions exist (basic + metadata)
- The prompt_comprehensive() function is completely removed
- The routing match statement only handles "basic" and "metadata"
- The decorative header comment block is removed
- Cargo check passes without errors
- No test or documentation updates required
