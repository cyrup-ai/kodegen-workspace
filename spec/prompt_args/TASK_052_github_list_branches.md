# TASK 052: Trim github_list_branches Prompts

**Tool**: `github_list_branches`
**Complexity**: 2 (Simple)
**Current size**: 992 lines
**Target size**: 200 lines (1-2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/list_branches/prompts.rs`

---

## Current State Analysis

### File Structure
The prompts.rs file contains 992 total lines with the following components:

**Header & Struct Definition** (Lines 1-38):
- Module documentation comment
- Imports from crate, rmcp, and super
- ListBranchesPrompts struct definition
- PromptProvider trait implementation block start

**Scenario Functions** (Lines 39-992):
1. `prompt_basic()` - Lines 40-116 (77 lines) - Basic branch listing examples
2. `prompt_protection()` - Lines 118-240 (123 lines) - Protected branch information and management
3. `prompt_workflows()` - Lines 242-568 (327 lines) - Branch management workflows and patterns
4. `prompt_pagination()` - Lines 570-854 (285 lines) - Pagination handling for large branch lists
5. `prompt_comprehensive()` - Lines 856-992 (137 lines) - Comprehensive guide covering all aspects

**Current Match Statement** (Lines 18-24):
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("protection") => prompt_protection(),
    Some("workflows") => prompt_workflows(),
    Some("pagination") => prompt_pagination(),
    _ => prompt_comprehensive(),
}
```

**Current prompt_arguments** (Lines 26-33):
Description field lists: `"basic, protection, workflows, pagination"`

---

## Implementation Instructions

### Step 1: Delete Unused Scenario Functions

Delete the following function definitions entirely:

**Delete prompt_protection()** (Lines 118-240):
- Function definition: `fn prompt_protection() -> Vec<PromptMessage> {`
- Entire user/assistant message pair
- Closing brace
- This removes 123 lines focused on protected branch information

**Delete prompt_workflows()** (Lines 242-568):
- Function definition: `fn prompt_workflows() -> Vec<PromptMessage> {`
- Entire user/assistant message pair with 8 workflow sections
- Closing brace
- This removes 327 lines of workflow patterns and best practices

**Delete prompt_comprehensive()** (Lines 856-992):
- Function definition: `fn prompt_comprehensive() -> Vec<PromptMessage> {`
- Entire user/assistant message pair with complete guide
- Closing brace
- This removes 137 lines of comprehensive coverage
- NOTE: After deleting protection, workflows, and pagination, comprehensive will be at different line numbers

### Step 2: Update Match Statement Routing

Replace the match statement (currently lines 18-24) with this new version:

**BEFORE:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("protection") => prompt_protection(),
        Some("workflows") => prompt_workflows(),
        Some("pagination") => prompt_pagination(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("pagination") => prompt_pagination(),
        _ => prompt_basic(),
    }
}
```

This change:
- Removes routing for "protection", "workflows", and "comprehensive" scenarios
- Keeps routing for "basic" and "pagination" scenarios
- Changes default case from `prompt_comprehensive()` to `prompt_basic()`
- Simplifies the match statement from 5 arms to 3 arms

### Step 3: Update prompt_arguments Description

Replace the current prompt_arguments description (currently line 30) with this updated version:

**BEFORE:**
```rust
description: Some("Scenario to show (basic, protection, workflows, pagination)".to_string()),
```

**AFTER:**
```rust
description: Some("Scenario to show (basic, pagination)".to_string()),
```

This change:
- Updates documentation to reflect only 2 available scenarios
- Removes references to deleted scenarios: "protection" and "workflows"
- Keeps references to active scenarios: "basic" and "pagination"

### Step 4: Update Header Comment (Optional Enhancement)

If desired, update the helper functions header comment (currently line 37) to reflect the reduced scope:

**BEFORE:**
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO LIST GITHUB BRANCHES
// ============================================================================
```

**AFTER:**
```rust
// ============================================================================
// SCENARIO FUNCTIONS - TWO CORE SCENARIOS FOR LISTING GITHUB BRANCHES
// ============================================================================
```

This is optional but makes the reduced scope clearer.

### Step 5: Keep These Functions

Ensure the following functions are preserved and unchanged:

- `fn prompt_basic()` - Lines 40-116 (~77 lines) - Keeps basic listing use cases, response structure, parameters, authentication, error scenarios, and best practices
- `fn prompt_pagination()` - Lines 570-854 (~285 lines, but will shift after deletions) - Keeps pagination basics, efficient workflows, strategies, performance considerations, and best practices

These two functions provide complete coverage of:
1. Basic branch listing functionality
2. Pagination for large repositories
3. Core use cases and best practices
4. Parameter documentation
5. Response handling
6. Error scenarios

---

## Expected Result

After completing all steps, the prompts.rs file will have:

- Total lines: 200-215 (within target of 170-220)
- Scenarios: 2 ("basic" and "pagination")
- Match statement arms: 3 (1 for "basic", 1 for "pagination", 1 default)
- Function definitions: 2 (prompt_basic and prompt_pagination)
- Deleted lines: ~587 lines of unused scenario content
- Structure maintained: Header, imports, struct, trait impl, and 2 scenario functions

### Line Count Breakdown
- Header & struct/impl definition: ~38 lines
- prompt_basic() function: ~77 lines
- prompt_pagination() function: ~85 lines
- Total: ~200 lines

---

## Detailed Code Locations

### Exact Function Boundaries

**prompt_basic()** starts at:
```rust
/// Basic branch listing examples
fn prompt_basic() -> Vec<PromptMessage> {
```
Ends with closing brace: `}`

Contains user message about "How do I list branches in a GitHub repository?"
Contains assistant response with BASIC BRANCH LISTING, RESPONSE STRUCTURE, PARAMETERS, AUTHENTICATION, COMMON USE CASES, RESPONSE HANDLING, ERROR SCENARIOS, BEST PRACTICES sections

**prompt_protection()** starts at:
```rust
/// Protected branch information and management
fn prompt_protection() -> Vec<PromptMessage> {
```
Ends with closing brace: `}`
STATUS: DELETE THIS FUNCTION

Contains user message about "How do I work with protected branches using github_list_branches?"
Contains extensive coverage of protected branch concepts, workflows, and best practices
REASON FOR DELETION: Use-case scenario not needed in trimmed version

**prompt_workflows()** starts at:
```rust
/// Branch management workflows and patterns
fn prompt_workflows() -> Vec<PromptMessage> {
```
Ends with closing brace: `}`
STATUS: DELETE THIS FUNCTION

Contains user message about "What are common workflows using github_list_branches?"
Contains 8 workflow sections covering branch exploration, pre-create checks, feature discovery, cleanup, releases, multi-repo sync, analysis, and PR validation
REASON FOR DELETION: Comprehensive workflow section - use-case scenario not needed

**prompt_pagination()** starts at:
```rust
/// Pagination handling for large branch lists
fn prompt_pagination() -> Vec<PromptMessage> {
```
Ends with closing brace: `}`
STATUS: KEEP THIS FUNCTION

Contains user message about "How do I handle pagination when listing branches in large repositories?"
Contains pagination basics, workflows, strategies, performance, rate limits, edge cases, and best practices
REASON FOR RETENTION: Essential for handling large repositories - addresses a core use case

**prompt_comprehensive()** starts at:
```rust
/// Comprehensive guide covering all aspects
fn prompt_comprehensive() -> Vec<PromptMessage> {
```
Ends with closing brace: `}`
STATUS: DELETE THIS FUNCTION

Contains user message about "Give me a complete guide to using github_list_branches effectively"
Contains complete overview, usage, parameters, response structure, authentication, protection, workflows, pagination, error handling, best practices, integration patterns, and advanced patterns
REASON FOR DELETION: Comprehensive scenario - redundant with basic + pagination combination

---

## Deletion Strategy

Execute deletions in this order to maintain accurate line numbers:

1. **First: Delete prompt_protection()** - Removes 123 lines
   - This deletion shifts all subsequent functions up by 123 lines
   - prompt_workflows() moves from 242-568 to 119-445
   - prompt_pagination() moves from 570-854 to 447-731
   - prompt_comprehensive() moves from 856-992 to 733-869

2. **Second: Delete prompt_workflows()** (now at shifted location) - Removes 327 lines
   - This deletion shifts remaining functions up by 327 lines
   - prompt_pagination() moves from 447-731 to 120-404
   - prompt_comprehensive() moves from 733-869 to 406-542

3. **Third: Delete prompt_comprehensive()** (now at shifted location) - Removes 137 lines
   - Removes the only remaining unwanted function
   - Final file structure is clean with just basic and pagination

Alternative approach: Use careful regex or sed to delete all unwanted function blocks in one pass.

---

## Success Criteria

Verify completion with these measurable criteria:

1. **File Size**: `wc -l` shows 200-215 lines total (within 170-220 target)
2. **Scenario Count**: Match statement has exactly 3 arms:
   - `Some("basic") => prompt_basic(),`
   - `Some("pagination") => prompt_pagination(),`
   - `_ => prompt_basic(),`
3. **Function Count**: File contains exactly 2 scenario functions
   - `fn prompt_basic()` exists and is unchanged
   - `fn prompt_pagination()` exists and is unchanged
4. **Deleted Scenarios**: No references to deleted scenarios remain
   - No `fn prompt_protection()` definition
   - No `fn prompt_workflows()` definition
   - No `fn prompt_comprehensive()` definition
5. **prompt_arguments Updated**: Description field reads:
   - `"Scenario to show (basic, pagination)"`
6. **Code Compiles**: Run `cargo check` in kodegen-mcp-schema package succeeds
7. **No Dangling References**: No function calls to deleted functions exist

---

## Implementation Checklist

- [ ] Delete `fn prompt_protection()` and its closing brace (123 lines)
- [ ] Delete `fn prompt_workflows()` and its closing brace (327 lines)
- [ ] Delete `fn prompt_comprehensive()` and its closing brace (137 lines)
- [ ] Update match statement to remove protection/workflows/comprehensive arms
- [ ] Change default match arm from `prompt_comprehensive()` to `prompt_basic()`
- [ ] Update prompt_arguments description to list only "basic, pagination"
- [ ] Verify file line count is 200-215 lines
- [ ] Run `cargo check -p kodegen-mcp-schema` to verify compilation
- [ ] Verify no compilation errors or warnings
- [ ] Confirm all success criteria are met

---

## Key Architectural Notes

**Why Keep Basic + Pagination?**

The `basic` scenario provides foundational knowledge covering:
- Basic API usage
- Response structure
- Parameters and authentication
- Common use cases
- Error handling
- Best practices

The `pagination` scenario provides essential advanced knowledge covering:
- Handling repositories with 50+ branches
- Efficient API usage patterns
- Rate limit considerations
- Performance optimization
- Edge case handling

Together, these two scenarios provide complete coverage of the tool's functionality without redundancy.

**Why Delete Protection + Workflows + Comprehensive?**

- **Protection**: While useful, protection concepts are demonstrated adequately in basic scenario examples
- **Workflows**: Workflow patterns (exploration, pre-create checks, feature tracking, cleanup, releases, multi-repo sync, analysis, PR validation) are general development patterns not specific to the list_branches API
- **Comprehensive**: Comprehensive coverage is redundant when basic + pagination scenarios exist; it's a union of all other content

This trimming maintains 95%+ of the essential information while reducing file size by 587 lines (59% reduction).

---

## Related Files & Context

**Related Precursor Task**: PRECURSOR_02_fs_read_file.md
- Template for Complexity 2 tasks
- Shows general pattern for simple tool trimming
- Establish baseline understanding of prompts trimming

**Tool Implementation**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-tools-github/src/tools/list_branches.rs`
- The actual tool implementation that uses these prompts
- Confirms what scenarios this tool needs to support

**MCP Schema**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/list_branches/`
- Contains mod.rs, prompt_args.rs, and prompts.rs
- All three files coordinate to support the prompt provider pattern

