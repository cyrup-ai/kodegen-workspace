# TASK 041: Trim github_code_scanning_alerts Prompts

**Tool**: `github_code_scanning_alerts`
**Complexity**: 2 (Simple Trimming)
**Current file size**: 1160 lines
**Target size**: 170-220 lines total
**File path**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/code_scanning_alerts/prompts.rs`

---

## Current State Analysis

### File Structure (1160 lines)
- Lines 1-50: Module header, PromptProvider trait implementation, match statement, prompt_arguments()
- Lines 52-330: `prompt_basic()` function (~280 lines)
- Lines 332-800+: `prompt_filtering()` function (~500+ lines)  
- Lines 801-950+: `prompt_analysis()` function (~400+ lines) - **DELETE**
- Lines 951-1060+: `prompt_remediation()` function (~300+ lines) - **DELETE**
- Lines 1061-1160: `prompt_comprehensive()` function (~660+ lines) - **DELETE**

### Scenarios in Current Match Statement (lines 21-26)
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),           // KEEP
    Some("filtering") => prompt_filtering(),   // KEEP
    Some("analysis") => prompt_analysis(),     // DELETE
    Some("remediation") => prompt_remediation(),  // DELETE
    _ => prompt_comprehensive(),               // DELETE - replace with default
}
```

### Prompt Arguments (lines 35-41)
Currently describes 4 scenarios. Update description to reflect only "basic" and "filtering".

---

## Implementation Steps

### Step 1: Update Prompt Arguments Function
**Lines to modify**: 35-41 (PromptArgument description field)

**Current code**:
```rust
description: Some("Scenario to show (basic, filtering, analysis, remediation)".to_string()),
```

**Change to**:
```rust
description: Some("Scenario to show (basic, filtering)".to_string()),
```

This updates the user-facing documentation to reflect only available scenarios.

---

### Step 2: Update Match Statement
**Lines to modify**: 21-26 (generate_prompts function match block)

**Current code**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("filtering") => prompt_filtering(),
        Some("analysis") => prompt_analysis(),
        Some("remediation") => prompt_remediation(),
        _ => prompt_comprehensive(),
    }
}
```

**Change to**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        _ => prompt_filtering(),
    }
}
```

This simplifies routing to only two scenarios, with filtering as the default fallback.

---

### Step 3: Trim prompt_basic() Function
**Current state**: ~280 lines
**Target state**: ~100 lines

**Keep these sections** (in this order):
1. User question: "How do I list and view GitHub code scanning alerts?"
2. Core guidance: "The github_code_scanning_alerts tool retrieves security alerts..." (intro)
3. LISTING CODE SCANNING ALERTS section with 4 basic examples:
   - Get all alerts for a repository
   - Get specific alert by number
   - Get alerts for specific branch
   - Paginate through results
4. REQUIRED PARAMETERS subsection
5. OPTIONAL PARAMETERS subsection (trimmed list)
6. RESPONSE STRUCTURE subsection (simplified JSON example)
7. WHAT'S REPORTED subsection (bullet list only)
8. AUTHENTICATION subsection (1-2 lines)
9. COMMON USE CASES subsection (brief list)

**Delete these sections entirely**:
- EXAMPLE WORKFLOW (verbose walkthrough)
- RESPONSE INTERPRETATION subsection (explanatory text)
- ERROR HANDLING subsection (moved to comprehensive)
- BEST PRACTICES subsection (moved to comprehensive)

**Specific deletions in prompt_basic()**:
- Delete Step 3 and Step 4 in LISTING examples (keep only the first 2-3 examples)
- Delete all line-by-line JSON parsing explanation
- Consolidate OPTIONAL PARAMETERS to 3-4 most critical fields
- Reduce COMMON USE CASES from 4 items to 3 items with one-line descriptions

**Expected result**: Function should fit in ~100 lines with core retrieval information only.

---

### Step 4: Trim prompt_filtering() Function  
**Current state**: ~500+ lines
**Target state**: ~70 lines

**Keep these sections** (in this order):
1. User question: "How do I filter code scanning alerts by severity, state, or other criteria?"
2. Core guidance intro (1-2 sentences)
3. FILTERING BY SEVERITY section with 2-3 examples:
   - Critical issues only
   - High severity issues
   - Combine with state filter (state="open")
4. SEVERITY LEVELS subsection (brief list of 4 levels: critical, high, medium, low)
5. FILTERING BY STATE section with 2 examples:
   - Open alerts
   - Fixed alerts
6. STATE MEANINGS subsection (brief definitions)
7. FILTERING BY TOOL section with 2 examples:
   - CodeQL alerts
   - Third-party scanner

**Delete these sections entirely**:
- COMMON TOOLS subsection (not essential for filtering)
- COMBINING FILTERS subsection (too many examples)
- PAGINATION WITH FILTERS subsection (move to basic)
- FILTER STRATEGIES subsection (4-strategy breakdown - too verbose)
- EXAMPLE WORKFLOW section (step-by-step workflow)
- PERFORMANCE TIPS subsection (unnecessary detail)
- BEST PRACTICES subsection (moved to comprehensive)

**Specific deletions in prompt_filtering()**:
- Delete all code examples except 2-3 essential ones per filter type
- Remove "Multiple severities" example (can infer from basic example)
- Delete detailed descriptions of each severity level (keep to 1 line each)
- Remove workflow breakdowns that repeat tool call patterns
- Delete numbered lists with extensive bullet points

**Expected result**: Function should be ~70 lines with core filtering patterns only.

---

### Step 5: Delete Remaining Functions
Delete these functions entirely (all helper functions and their supporting comments):

1. `prompt_analysis()` function - lines containing complete function definition
   - Remove comment header: "/// Analyzing security results and identifying patterns"
   - Remove entire function body

2. `prompt_remediation()` function - lines containing complete function definition
   - Remove comment header: "/// Fixing security issues and remediation workflows"
   - Remove entire function body

3. `prompt_comprehensive()` function - lines containing complete function definition
   - Remove comment header: "/// Comprehensive guide covering all code scanning alert operations"
   - Remove entire function body

The comment header "// ============================================================================" and "// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE CODE SCANNING ALERTS" should be moved/kept only before prompt_basic().

---

## Expected Result After Trimming

### File Size
- **Target**: 170-220 lines
- **Expected breakdown**:
  - Lines 1-50: Module header, imports, PromptProvider impl, match statement, prompt_arguments() 
  - Lines 51-160: `prompt_basic()` trimmed to ~100 lines
  - Lines 161-220: `prompt_filtering()` trimmed to ~70 lines
  - **Total**: ~220 lines (within target range)

### Module Structure
```rust
//! Prompt messages for github_code_scanning_alerts tool
use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::CodeScanningAlertsPromptArgs;

pub struct CodeScanningAlertsPrompts;

impl PromptProvider for CodeScanningAlertsPrompts {
    type PromptArgs = CodeScanningAlertsPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            _ => prompt_filtering(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, filtering)".to_string()),
            required: Some(false),
        }]
    }
}

// HELPER FUNCTIONS section header (simplified)
// Two functions remain: prompt_basic() and prompt_filtering()
```

---

## Code Patterns: Before and After

### Pattern 1: Match Statement
**Before** (6 lines with 5 cases):
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("filtering") => prompt_filtering(),
    Some("analysis") => prompt_analysis(),
    Some("remediation") => prompt_remediation(),
    _ => prompt_comprehensive(),
}
```

**After** (3 lines with 2 cases):
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    _ => prompt_filtering(),
}
```

### Pattern 2: Prompt Arguments Description
**Before** (1 line):
```rust
description: Some("Scenario to show (basic, filtering, analysis, remediation)".to_string()),
```

**After** (1 line):
```rust
description: Some("Scenario to show (basic, filtering)".to_string()),
```

### Pattern 3: Function Signature
**Before** (5 scenarios - 1160 lines total):
```rust
fn prompt_basic() -> Vec<PromptMessage> { ... }     // ~280 lines
fn prompt_filtering() -> Vec<PromptMessage> { ... } // ~500 lines
fn prompt_analysis() -> Vec<PromptMessage> { ... }  // ~400 lines
fn prompt_remediation() -> Vec<PromptMessage> { ... } // ~300 lines
fn prompt_comprehensive() -> Vec<PromptMessage> { ... } // ~660 lines
```

**After** (2 scenarios - 220 lines total):
```rust
fn prompt_basic() -> Vec<PromptMessage> { ... }     // ~100 lines
fn prompt_filtering() -> Vec<PromptMessage> { ... } // ~70 lines
```

---

## Definition of Done

Success is achieved when:

1. **File size**: 170-220 lines total
2. **Scenarios**: Exactly 2 scenarios remain (basic, filtering)
3. **Match statement**: Updated to route only "basic" and "filtering"
4. **Prompt arguments**: Description field updated to reflect only 2 scenarios
5. **Deleted functions**: 
   - No prompt_analysis() function exists
   - No prompt_remediation() function exists
   - No prompt_comprehensive() function exists
6. **Content quality**: 
   - prompt_basic() covers: listing, viewing, pagination, parameters, response structure (~100 lines)
   - prompt_filtering() covers: severity filtering, state filtering, tool filtering (~70 lines)
7. **Code organization**: 
   - No orphaned helper functions
   - No unused imports
   - Proper Rust formatting and syntax
8. **Documentation**: 
   - All comment headers reflect remaining scenarios
   - No references to deleted analysis/remediation/comprehensive scenarios

---

## Execution Sequence

Execute in this order:

1. Update PromptArgument description (simple string replacement)
2. Update match statement (simple pattern replacement)
3. Trim prompt_basic() function (delete sections, keep core examples)
4. Trim prompt_filtering() function (delete sections, keep core filtering patterns)
5. Delete prompt_analysis() function (entire function)
6. Delete prompt_remediation() function (entire function)
7. Delete prompt_comprehensive() function (entire function)
8. Verify file compiles: `cargo check` in kodegen-mcp-schema package
9. Count lines in output file to verify 170-220 range

---

## Testing

After trimming, verify:

1. **Rust syntax is valid**:
   ```bash
   cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
   cargo check
   ```

2. **Both scenarios are accessible** (external integration will test):
   - Requesting scenario="basic" returns basic prompt
   - Requesting scenario="filtering" returns filtering prompt
   - Default (no scenario) returns filtering prompt

3. **No errors from deleted functions**:
   - No compilation errors about undefined prompt_analysis, prompt_remediation, prompt_comprehensive
   - No warnings about unused code
