# TASK 049: Trim github_get_pull_request_files Prompts

**Tool**: `github_get_pull_request_files`
**Complexity**: 2 (Simple)
**Current size**: 1167 lines
**Target size**: 200 lines (1-2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_pull_request_files/prompts.rs`

---

## Current State Analysis

### File Structure
- **Total lines**: 1167 lines
- **Module-level documentation**: Lines 1-12 (comment block)
- **Struct definition**: Line 14 (`pub struct GetPullRequestFilesPrompts;`)
- **PromptProvider impl block**: Lines 16-35
- **Helper function definitions**: Lines 37-1167

### Current Scenarios (Functions)

1. **prompt_basic()** (Lines 41-171, ~130 lines)
   - User question: "How do I get the list of files changed in a pull request?"
   - Content: Core retrieval patterns, parameter documentation, response structure, file status values, patch format explanation, pagination, authentication, rate limits, error handling, best practices
   - Status: KEEP - Essential fundamental scenario

2. **prompt_analysis()** (Lines 173-373, ~200 lines)
   - User question: "How do I analyze the scope and complexity of changes in a pull request?"
   - Content: Complexity metrics, impact analysis, PR size assessment, risk assessment, change patterns, code churn analysis, file dependency analysis
   - Status: TRIM to ~80-90 lines - Keep core metrics, remove automated analysis workflows

3. **prompt_workflows()** (Lines 375-825, ~450 lines)
   - User question: "What are the complete workflows for reviewing pull requests using file change data?"
   - Content: Six complete workflows (triage, systematic review, security review, dependency updates, pre-merge checklist, large PR management)
   - Status: DELETE - Use-case scenario per instructions

4. **prompt_collaboration()** (Lines 827-1100, ~273 lines)
   - User question: "How do I collaborate with team members on PR reviews using file change information?"
   - Content: Collaborative workflows, review assignment patterns, structured feedback, comment templates, team policies, approval workflows, automated helpers
   - Status: DELETE - Use-case scenario per instructions

5. **prompt_comprehensive()** (Lines 1102-1167, ~65 lines)
   - User question: "Give me a complete guide to using github_get_pull_request_files effectively."
   - Content: Fallback comprehensive guide
   - Status: DELETE - Comprehensive scenario per instructions

### Routing Logic (Current)

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("analysis") => prompt_analysis(),
        Some("workflows") => prompt_workflows(),
        Some("collaboration") => prompt_collaboration(),
        _ => prompt_comprehensive(),
    }
}

fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, analysis, workflows, collaboration)".to_string()),
            required: Some(false),
        }
    ]
}
```

---

## Implementation Instructions

### Step 1: Update PromptProvider Implementation (Lines 18-35)

Update the `generate_prompts` match statement to remove "workflows" and "collaboration" cases:

**BEFORE** (current lines 18-35):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("analysis") => prompt_analysis(),
        Some("workflows") => prompt_workflows(),
        Some("collaboration") => prompt_collaboration(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER** (replacement):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("analysis") => prompt_analysis(),
        _ => prompt_analysis(),
    }
}
```

**RATIONALE**: Removes routing to deleted scenarios; defaults to "analysis" as the fallback.

---

Update the `prompt_arguments()` function (lines 37-45):

**BEFORE** (current):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, analysis, workflows, collaboration)".to_string()),
            required: Some(false),
        }
    ]
}
```

**AFTER** (replacement):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, analysis)".to_string()),
            required: Some(false),
        }
    ]
}
```

**RATIONALE**: Updates help text to reflect only available scenarios.

---

### Step 2: Keep prompt_basic() Function - UNCHANGED

Lines 41-171: Keep this entire function as-is. This is the fundamental retrieval scenario and is already correctly sized (~130 lines, within the 80-120 target).

---

### Step 3: Trim prompt_analysis() Function

The current prompt_analysis() is approximately 200 lines. Trim it to approximately 80-90 lines by removing the detailed workflow examples and keeping only the core analysis patterns.

**Sections to KEEP** (first ~200 lines of the function):
- User question
- COMPLEXITY METRICS section (up to "Find largest changes")
- IMPACT ANALYSIS section (up to "Calculate change ratio")
- PR SIZE ASSESSMENT section
- RISK ASSESSMENT section (all indicators)
- CHANGE PATTERNS section (all four patterns: feature, bug fix, refactoring, breaking change)

**Sections to DELETE** (after line ~350):
- AUTOMATED ANALYSIS WORKFLOW (entire section ~50 lines)
- CODE CHURN ANALYSIS (entire section ~20 lines)
- FILE DEPENDENCY ANALYSIS (entire section ~25 lines)
- BEST PRACTICES section at the end

**Starting point for deletion**: Look for the line containing `AUTOMATED ANALYSIS WORKFLOW:` and delete everything from there to the closing brace of the function.

The function signature and first 200 lines should remain:
```rust
/// Analyzing PR changes for complexity and impact
fn prompt_analysis() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I analyze the scope and complexity of changes in a pull request?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use github_get_pull_request_files to perform comprehensive change analysis and impact assessment.\n\n\
                 ...
                 [Keep content through CHANGE PATTERNS section]
                 ...
                 - Version number changes\n\n\
                 BEST PRACTICES:\n\
                 - Always analyze change distribution, not just total lines\n\
                 - Look for imbalanced changes (all additions or deletions)\n\
                 - Identify if tests were added/updated\n\
                 - Check for configuration or migration changes\n\
                 - Flag PRs with >500 lines for potential splitting\n\
                 - Verify renamed files didn't also change significantly\n\
                 - Compare changed files against known critical paths\n\
                 - Use metrics to prioritize review focus areas",
            ),
        },
    ]
}
```

---

### Step 4: Delete prompt_workflows() Function

**Action**: Delete the entire `fn prompt_workflows()` function.

**Location**: This function spans approximately lines 375-825 (~450 lines).

**Identify by**: Look for the function definition:
```rust
/// Complete code review workflows
fn prompt_workflows() -> Vec<PromptMessage> {
```

And delete everything from this line through its closing brace `}` at the end of the function.

**Confirmation**: After deletion, the next function definition should be `fn prompt_collaboration()`.

---

### Step 5: Delete prompt_collaboration() Function

**Action**: Delete the entire `fn prompt_collaboration()` function.

**Location**: This function spans approximately lines 827-1100 (~273 lines) in the current file.

**Identify by**: Look for the function definition:
```rust
/// Team review and commenting patterns
fn prompt_collaboration() -> Vec<PromptMessage> {
```

And delete everything from this line through its closing brace `}` at the end of the function.

**Confirmation**: After deletion, the next function definition should be `fn prompt_comprehensive()`.

---

### Step 6: Delete prompt_comprehensive() Function

**Action**: Delete the entire `fn prompt_comprehensive()` function.

**Location**: This function spans approximately lines 1102-1167 (~65 lines) in the current file.

**Identify by**: Look for the function definition:
```rust
/// Comprehensive guide covering all aspects
fn prompt_comprehensive() -> Vec<PromptMessage> {
```

And delete everything from this line through the final closing brace `}` at the end of the file.

---

## Expected Result After Trimming

### File Structure (Post-Trim)

- **Lines 1-12**: Module documentation (unchanged)
- **Lines 14-16**: Struct definition (unchanged)
- **Lines 18-35**: PromptProvider impl block (UPDATED routing)
- **Lines 37-45**: prompt_arguments function (UPDATED description)
- **Lines 47-177**: prompt_basic() function (UNCHANGED)
- **Lines 179-265**: prompt_analysis() function (TRIMMED from 200 to ~87 lines)

### Final File Size

**Expected total: 170-220 lines** (target met)

### Verification Checklist

- File compiles without errors
- Only 2 scenarios remain: "basic" and "analysis"
- prompt_basic() preserved completely
- prompt_analysis() trimmed to core metrics and patterns only
- prompt_workflows() deleted entirely
- prompt_collaboration() deleted entirely
- prompt_comprehensive() deleted entirely (no fallback, routes to "analysis" instead)
- PromptProvider routing updated to handle only "basic" and "analysis"
- prompt_arguments description shows "basic, analysis"
- No orphaned function definitions
- No trailing commas or syntax errors in routing match statement

---

## Success Criteria

- **✓ 170-220 lines total** - File size within acceptable range
- **✓ 1-2 scenarios** - Exactly 2 scenarios remain (basic, analysis)
- **✓ No use-case workflows** - Workflows and collaboration scenarios deleted
- **✓ No comprehensive fallback** - Comprehensive scenario deleted
- **✓ Basic retrieval preserved** - prompt_basic() untouched
- **✓ Analysis trimmed** - prompt_analysis() reduced by 50%+ through removal of detailed workflow examples
- **✓ Routing updated** - match statement reflects only available scenarios
- **✓ Clean deletion** - No leftover code or comments from deleted functions
- **✓ Compiles successfully** - No syntax errors

---

## Code Patterns Reference

### Identifying Function Boundaries

Each scenario function follows this pattern:
```rust
/// Documentation comment
fn prompt_<name>() -> Vec<PromptMessage> {
    vec![
        PromptMessage { ... },
        PromptMessage { ... },
    ]
}
```

The function ends with the closing brace `}` of the vec! macro, followed by the next function definition.

### Section Markers in Content

Large sections within the scenario text are marked with separator lines:
```
=============================================================================
SECTION NAME
=============================================================================
```

These help identify where to trim within prompt_analysis().

---

## Notes

- This is a Complexity 2 (Simple) task focused on reducing prompt scope
- The goal is to have a lean, focused set of scenarios for basic retrieval and analysis
- More complex workflows should be learned through practice or external documentation
- The "analysis" scenario provides essential guidance without overwhelming detail
- After trimming, agents can learn PR analysis patterns through the remaining "analysis" scenario without needing 6 complete workflows
