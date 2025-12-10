# TASK 059: Trim web_search Prompts

**Tool**: `web_search`
**Complexity**: 2 (Simple)
**Current size**: 357 lines (5 scenarios + comprehensive guide)
**Target size**: 170-220 lines (2 core scenarios)
**Primary file**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/web/web_search/prompts.rs`
**Secondary file**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/web/web_search/prompt_args.rs`

---

## Current State Analysis

### prompts.rs Structure (357 lines total)
The file contains 5 scenario functions plus a PromptProvider implementation:

**Keep (Essential):**
- Lines 1-7: File header and imports
- Lines 9-34: WebSearchPrompts struct and PromptProvider impl
- Lines 48-80: `prompt_basic()` - Basic web searching (33 lines)
- Lines 82-134: `prompt_queries()` - Effective query patterns (53 lines)

**Delete (Use-case/Comprehensive scenarios):**
- Lines 136-202: `prompt_research()` - Research workflows (67 lines) - DELETE
- Lines 204-285: `prompt_integration()` - Integration workflows (82 lines) - DELETE
- Lines 287-357: `prompt_comprehensive()` - Full comprehensive guide (71 lines) - DELETE

**Total after trimming**: ~138 lines core code

### prompt_args.rs Structure (17 lines)
Currently documents all 5 scenarios in the doc comments (lines 10-15). Must update to remove references to deleted scenarios.

---

## Step 1: Update Routing Logic in prompts.rs

The current match statement (lines 18-23) handles all scenarios:

```rust
// CURRENT (4 match arms + default)
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("queries") => prompt_queries(),
    Some("research") => prompt_research(),
    Some("integration") => prompt_integration(),
    _ => prompt_comprehensive(),
}
```

Change to (2 match arms + default to basic):

```rust
// AFTER (2 match arms + default)
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("queries") => prompt_queries(),
    _ => prompt_basic(),  // Default to basic instead of comprehensive
}
```

**Action**: Replace lines 18-23 with the new match statement above.

---

## Step 2: Update Scenario Arguments Documentation

Current prompt_arguments() function (lines 25-32) mentions all scenarios:

```rust
// CURRENT
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, queries, research, integration, comprehensive)".to_string()),
            required: Some(false),
        }
    ]
}
```

Change to (only basic and queries):

```rust
// AFTER
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, queries)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Action**: Replace the description string in the PromptArgument on line 30.

---

## Step 3: Delete Unused Scenario Functions

Delete the following entire functions and their helper comments:

1. **Delete lines 136-202**: `prompt_research()` function
2. **Delete lines 204-285**: `prompt_integration()` function  
3. **Delete lines 287-357**: `prompt_comprehensive()` function

Delete the decorative section header on line 37-39:
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE WEB SEARCH
// ============================================================================
```

Optionally simplify to a single-line comment:
```rust
// Scenario functions for web_search prompts
```

**Action**: Execute 3 separate deletions for the unused functions.

---

## Step 4: Update prompt_args.rs Documentation

Current doc comment (lines 10-15) lists all scenarios:

```rust
/// Scenario to show examples for
/// - "basic": Basic web searches
/// - "queries": Effective query patterns
/// - "research": Research workflows
/// - "integration": Integration with other tools
/// - "comprehensive": All scenarios combined
```

Change to (only basic and queries):

```rust
/// Scenario to show examples for
/// - "basic": Basic web searches
/// - "queries": Effective query patterns
```

**Action**: Replace lines 10-15 with the trimmed documentation above.

---

## Implementation Instructions

### Phase 1: Modify prompts.rs

1. **Replace routing match statement** (lines 18-23)
   - Replace the 5-arm match with the 2-arm version
   - Default case now returns `prompt_basic()` instead of `prompt_comprehensive()`

2. **Update prompt_arguments description** (line 30)
   - Replace the full scenario list with just `"basic, queries"`

3. **Simplify or remove decorative headers** (lines 37-39)
   - Remove or simplify the "HELPER FUNCTIONS" section header

4. **Delete prompt_research function** (lines 136-202)
   - Entire function and its doc comment

5. **Delete prompt_integration function** (lines 204-285)
   - Entire function and its doc comment

6. **Delete prompt_comprehensive function** (lines 287-357)
   - Entire function and its doc comment

### Phase 2: Modify prompt_args.rs

1. **Update scenario documentation** (lines 10-15)
   - Remove references to "research", "integration", "comprehensive"
   - Keep only "basic" and "queries"

### Phase 3: Verification

After editing, verify the file structure:

**prompts.rs should contain:**
- File header and imports (7 lines)
- WebSearchPrompts struct and impl (26 lines)
- Brief comment introducing scenarios (1 line)
- prompt_basic() function (33 lines)
- prompt_queries() function (53 lines)
- **Total: ~120 lines**

Expected line count after edits: **120-140 lines** (account for whitespace and comments)

---

## Success Criteria

All of the following must be true:

- ✓ **File size**: prompts.rs is 120-150 lines (down from 357)
- ✓ **Scenario count**: Only 2 scenario functions exist (prompt_basic, prompt_queries)
- ✓ **No comprehensive function**: prompt_comprehensive() completely removed
- ✓ **Routing logic**: Match statement has 2 arms + default to basic
- ✓ **Argument docs**: Description mentions only "basic, queries"
- ✓ **prompt_args.rs updated**: Doc comment lists only 2 scenarios
- ✓ **No decorative headers**: Removed "HELPER FUNCTIONS" section header
- ✓ **Code compiles**: `cargo check` passes in kodegen-mcp-schema package
- ✓ **No orphaned functions**: All match arms have corresponding function definitions

---

## Code Patterns Reference

### Before: Match statement with 5 arms
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("queries") => prompt_queries(),
    Some("research") => prompt_research(),
    Some("integration") => prompt_integration(),
    _ => prompt_comprehensive(),
}
```

### After: Match statement with 2 arms
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("queries") => prompt_queries(),
    _ => prompt_basic(),
}
```

---

## Files to Modify

1. **Primary**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/web/web_search/prompts.rs`
   - Delete 4 functions (research, integration, comprehensive, + header)
   - Update routing match statement
   - Update prompt_arguments description
   
2. **Secondary**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/web/web_search/prompt_args.rs`
   - Update doc comment to remove deleted scenarios

---

## Testing After Completion

Run in the kodegen-mcp-schema package directory:
```bash
cargo check
cargo clippy
```

Both commands must succeed with no errors or warnings.
