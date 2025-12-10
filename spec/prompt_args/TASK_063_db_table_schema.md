# TASK 063: Trim db_table_schema

**Tool**: `db_table_schema`
**Complexity**: 2 (Simple)
**Current size**: 752 lines
**Target size**: 170-220 lines (1 scenario)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/table_schema/prompts.rs`

---

## Current State Analysis

The prompts.rs file currently contains 5 prompt scenarios:

1. **prompt_basic()** (lines 49-164, ~116 lines)
   - Teaches basic table structure inspection and column properties
   - Example: Retrieving schema for a users table with columns, types, and constraints
   - Educational and foundational - KEEP

2. **prompt_relationships()** (lines 166-323, ~158 lines)
   - Use-case: Foreign key relationships and table joins
   - Teaches how to understand relationships between tables
   - DELETE - categorized as use-case scenario

3. **prompt_query_building()** (lines 325-535, ~211 lines)
   - Use-case: Constructing SQL queries based on schema
   - Teaches patterns for SELECT, INSERT, UPDATE with schema validation
   - DELETE - categorized as use-case scenario

4. **prompt_data_types()** (lines 537-688, ~152 lines)
   - Use-case: Understanding different column data types
   - Teaches INT, VARCHAR, DECIMAL, TIMESTAMP, etc.
   - DELETE - categorized as use-case scenario

5. **prompt_comprehensive()** (lines 690-752, ~63 lines of function, contains large string)
   - Complete guide combining all topics
   - DELETE - explicitly listed as "comprehensive" to remove

**Routing Logic** (lines 19-27 in generate_prompts):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("relationships") => prompt_relationships(),
        Some("query_building") => prompt_query_building(),
        Some("data_types") => prompt_data_types(),
        _ => prompt_comprehensive(),
    }
}
```

**Prompt Arguments** (lines 29-37):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, relationships, query_building, data_types)".to_string()),
            required: Some(false),
        }
    ]
}
```

---

## Instructions

### Step 1: Delete Scenario Functions

Delete the following function definitions in their entirety:

1. **prompt_relationships()** function (lines 166-323)
   - Remove entire function including closing brace
   - This is ~158 lines total

2. **prompt_query_building()** function (lines 325-535)
   - Remove entire function including closing brace
   - This is ~211 lines total

3. **prompt_data_types()** function (lines 537-688)
   - Remove entire function including closing brace
   - This is ~152 lines total

4. **prompt_comprehensive()** function (lines 690-752)
   - Remove entire function including closing brace
   - This is ~63 lines of function definition (content much larger due to multi-line strings)

### Step 2: Update generate_prompts() Match Statement

Replace the match statement (lines 21-27) with simplified routing that only handles "basic":

**OLD CODE (lines 21-27):**
```rust
    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("relationships") => prompt_relationships(),
            Some("query_building") => prompt_query_building(),
            Some("data_types") => prompt_data_types(),
            _ => prompt_comprehensive(),
        }
    }
```

**NEW CODE:**
```rust
    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") | None => prompt_basic(),
            _ => prompt_basic(), // Default to basic for any unknown scenario
        }
    }
```

This change:
- Removes all branches except "basic"
- Defaults any unrecognized scenario to "basic"
- Simplifies routing logic to 2 lines instead of 5 match arms

### Step 3: Update prompt_arguments() Description

Replace the scenario description (line 31) to only mention "basic":

**OLD CODE (line 31):**
```rust
            description: Some("Scenario to show (basic, relationships, query_building, data_types)".to_string()),
```

**NEW CODE:**
```rust
            description: Some("Scenario to show (basic)".to_string()),
```

This change:
- Removes all scenario options that no longer exist
- Keeps only "basic" as valid option
- Updates user guidance to match available scenarios

### Step 4: Keep Core Structure

Keep these sections unchanged:
- Lines 1-17: Module header, use statements, and struct definition
- Lines 19-27: TableSchemaPrompts impl declaration
- Lines 29-37: prompt_arguments() function (after updating line 31)
- Lines 39-43: Closing impl brace and comments
- Lines 45-47: Helper functions section comment
- Lines 49-164: prompt_basic() function - complete and unchanged

---

## Before/After Structure

**BEFORE (752 lines):**
```
1-17:     Header and imports (17 lines)
19-43:    TableSchemaPrompts impl with routing (25 lines)
45-47:    Comment section (3 lines)
49-164:   prompt_basic() (116 lines)
166-323:  prompt_relationships() - DELETE (158 lines)
325-535:  prompt_query_building() - DELETE (211 lines)
537-688:  prompt_data_types() - DELETE (152 lines)
690-752:  prompt_comprehensive() - DELETE (63+ lines)
```

**AFTER (164-170 lines):**
```
1-17:     Header and imports (17 lines)
19-27:    TableSchemaPrompts impl with simplified routing (9 lines)
29-37:    prompt_arguments() with updated description (9 lines)
39-43:    Closing and comments (5 lines)
45-47:    Comment section (3 lines)
49-164:   prompt_basic() (116 lines)
```

---

## Success Criteria

- ✓ **Line count**: 164-170 lines total (within 170-220 target)
- ✓ **Single scenario**: Only prompt_basic() function exists
- ✓ **No comprehensive scenario**: prompt_comprehensive() deleted
- ✓ **No use-case scenarios**: prompt_relationships(), prompt_query_building(), prompt_data_types() deleted
- ✓ **Updated routing**: match statement only handles "basic" scenario
- ✓ **Updated metadata**: prompt_arguments() only lists "basic"
- ✓ **Code compiles**: No syntax errors or missing function references
- ✓ **Default behavior**: Unknown scenarios default to basic for usability

---

## Implementation Notes

### Why This Approach?

1. **Educational Focus**: prompt_basic() teaches the core concept (table structure inspection)
2. **Scope Reduction**: Eliminates specialized use-case scenarios (relationships, query building, data types)
3. **Maintainability**: Fewer scenarios = fewer documentation paths to maintain
4. **Simplicity**: Single-scenario prompt is easier to understand and reason about

### No Breaking Changes

- The tool still functions the same way from user perspective
- Default behavior (None or unknown scenario) returns basic guidance
- No changes to Tool trait or public API
- No changes to other files needed (isolated to prompts.rs)

### Testing After Trim

After making these changes, the tool should still:
1. Respond to `db_table_schema()` calls without scenario parameter
2. Respond to `db_table_schema(..., scenario="basic")` calls
3. Default gracefully to basic for unknown scenario names
4. Build successfully with `cargo check` and `cargo build`
5. Pass clippy linting with no warnings
