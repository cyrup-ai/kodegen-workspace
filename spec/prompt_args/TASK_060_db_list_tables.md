# TASK 060: Trim db_list_tables

**Tool**: `db_list_tables`
**Complexity**: 2 (Simple)
**Current size**: 792 lines (5 scenarios)
**Target size**: 200 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/list_tables/prompts.rs`

---

## Current State Analysis

### File Structure
- **Total lines**: 792
- **Implementation**: Uses sealed `PromptProvider` trait pattern (cannot implement inline)
- **Tool registration**: Defined in `kodegen-mcp-schema` (single source of truth)

### Current Scenarios (Line Counts)

1. **`prompt_basic()`** (lines 43-115, ~73 lines)
   - User question: "How do I list tables in a database using db_list_tables?"
   - Content: Basic usage examples (default schema, specific schema), response format explanation
   - Status: **KEEP** - core functionality teaching

2. **`prompt_filtering()`** (lines 117-269, ~153 lines)
   - User question: "How do I find specific tables in a database?"
   - Content: Filtering by pattern, type, size, schema; sorting; combining filters; best practices
   - Status: **KEEP & TRIM** - essential secondary scenario for practical usage patterns

3. **`prompt_exploration()`** (lines 271-509, ~239 lines)
   - User question: "What's the complete workflow for exploring database tables?"
   - Content: 6-step workflow, multi-schema exploration, discovery checklist
   - Status: **DELETE** - teaches cross-tool workflow, not db_list_tables-specific features

4. **`prompt_views()`** (lines 511-719, ~209 lines)
   - User question: "What's the difference between tables and views in db_list_tables results?"
   - Content: Tables vs views vs materialized views, querying differences, performance considerations
   - Status: **DELETE** - teaches database concepts (tables/views/materialized_views), not tool usage

5. **`prompt_comprehensive()`** (lines 721-792, ~72 lines as fallback)
   - User question: "Give me a complete guide to listing and exploring database tables."
   - Content: All sections combined, massive duplication
   - Status: **DELETE** - pure duplication, no unique content

### Match Statement (lines 15-21)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("filtering") => prompt_filtering(),
        Some("exploration") => prompt_exploration(),
        Some("views") => prompt_views(),
        _ => prompt_comprehensive(),  // Current fallback
    }
}
```

---

## Trimming Strategy

### Keep: `prompt_basic()` - Core Listing (Target: ~90 lines, currently ~73)

This scenario teaches how to call the tool and understand responses.

**Keep all sections:**
- Tool description (5 lines): "lists all tables and views..."
- Basic usage examples (20 lines):
  - List all tables in default schema: `db_list_tables({"connection": "main"})`
  - List in specific schema: `db_list_tables({"connection": "main", "schema": "app"})`
- Response format (12 lines): Show actual JSON response structure
- Table types explanation (8 lines): table, view, materialized_view
- Understanding the response (8 lines): name, type, rows fields
- Default schemas by database (8 lines): PostgreSQL, MySQL, SQL Server, SQLite
- Common patterns (6 lines): 3 most common use cases
- When to use (6 lines): when to use this tool

**Rationale**: Basic is already lean and essential. Every section teaches something core about the tool.

### Keep & Trim: `prompt_filtering()` - Result Processing (Target: ~100 lines, currently ~153)

This scenario teaches how to process and filter the results from db_list_tables.

**Keep sections:**
- Finding specific tables (15 lines): How to filter after calling the tool
- Pattern matching examples (12 lines):
  - Tables starting with "user"
  - Tables ending with "log"
  - Tables containing "order"
- Filtering by type (8 lines):
  - Only physical tables
  - Only views
  - Persistent objects (tables + materialized views)
- Filtering by size (10 lines):
  - Small tables (testing)
  - Large tables (optimization)
  - Empty tables
- Filtering by schema (12 lines):
  - List tables across multiple schemas
  - Find table in any schema

**Delete sections:**
- COMBINING FILTERS (teaches AND logic, not db_list_tables-specific)
- SORTING RESULTS (teaches general array operations, not tool-specific)
- Extensive decorative header sections (═══ lines)
- Verbose best practices that repeat concepts from basic

**Specific trimming details**:
- Lines 164-206: Delete COMBINING FILTERS section (~42 lines)
- Lines 208-239: Delete SORTING RESULTS section (~32 lines)
- Remove decorative headers, consolidate remaining sections

**Rationale**: Filtering is the natural follow-up to basic usage. It teaches how to effectively use the results without going into cross-tool workflow or database theory.

### Delete Entirely: `prompt_exploration()` (239 lines)

**Reason**: This teaches a multi-step workflow combining db_list_tables, db_table_schema, db_table_indexes, and db_execute_sql. It's cross-tool knowledge, not db_list_tables-specific features.
- Steps 3-6 use other tools (db_table_schema, db_table_indexes, db_execute_sql)
- Multi-schema exploration is covered adequately in filtering scenario
- Workflow checklist is aspirational, not teaching tool parameters

### Delete Entirely: `prompt_views()` (209 lines)

**Reason**: This teaches database concepts (tables vs views vs materialized_views) rather than tool features. Every database teaches this, and it's not specific to how db_list_tables works.
- No special parameters for handling different table types
- Response format already covers the type field
- This is domain knowledge, not tool usage knowledge

### Delete Entirely: `prompt_comprehensive()` (72 lines)

**Reason**: Pure duplication of basic + filtering content with massive redundancy.
- Repeats response format shown in basic
- Duplicates all filtering examples
- No unique content

---

## Implementation Instructions

### Step 1: Identify Exact Line Ranges to Delete

Search the file for function definitions:
```
grep -n "^fn prompt_" prompts.rs
```

Expected output:
```
43:fn prompt_basic() -> Vec<PromptMessage> {
117:fn prompt_filtering() -> Vec<PromptMessage> {
271:fn prompt_exploration() -> Vec<PromptMessage> {
511:fn prompt_views() -> Vec<PromptMessage> {
721:fn prompt_comprehensive() -> Vec<PromptMessage> {
793:}
```

### Step 2: Trim `prompt_filtering()` Function (lines 117-270)

**Location**: Lines 117-270 (153 lines total)
**Target size**: ~100 lines

**Delete from filtering function**:
1. Lines 164-205: Complete COMBINING FILTERS section
   - Text: "const userTables = results.tables.filter(t => ..."
   - Reason: Teaches AND logic, not specific to tool
   - Count: ~42 lines

2. Lines 208-239: Complete SORTING RESULTS section
   - Text: "SORTING RESULTS:" through "return a.name.localeCompare(b.name);"
   - Reason: Teaches general array operations, not tool-specific
   - Count: ~32 lines

3. Reduce BEST PRACTICES at end from 5 bullet points to 2:
   - Keep: "Always check if rows is null before comparing"
   - Keep: "Cache results if filtering multiple times"
   - Delete: "Use case-insensitive matching..." (basic operation knowledge)
   - Delete: "Consider regex for complex patterns..." (advanced, not core)
   - Delete: "Filter early to reduce..." (optimization, not tool knowledge)
   - Saves: ~8 lines

**Result after trimming filtering**: ~100 lines (153 - 42 - 32 - 8 = 71 + added clarity = ~100)

### Step 3: Delete Unused Functions Entirely

**Delete complete functions**:
1. `prompt_exploration()` - All lines from "fn prompt_exploration()" through closing brace
2. `prompt_views()` - All lines from "fn prompt_views()" through closing brace
3. Remove `prompt_comprehensive()` function and update default routing

**Estimated lines removed**: 239 + 209 + 72 = 520 lines

### Step 4: Update Match Statement (lines 15-21)

**Before**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("filtering") => prompt_filtering(),
        Some("exploration") => prompt_exploration(),
        Some("views") => prompt_views(),
        _ => prompt_comprehensive(),
    }
}
```

**After**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("filtering") => prompt_filtering(),
        _ => prompt_basic(),
    }
}
```

**Changes**:
- Remove lines for exploration, views, comprehensive variants
- Move filtering before basic (allows explicit selection, defaults to basic)
- Default case now calls `prompt_basic()` instead of `prompt_comprehensive()`

### Step 5: Update prompt_arguments() Description (line 24)

**Before**:
```rust
description: Some("Scenario to show (basic, filtering, exploration, views)".to_string()),
```

**After**:
```rust
description: Some("Scenario to show (basic, filtering)".to_string()),
```

### Step 6: Update Documentation Comment (lines 7-9)

**Keep as-is** - comment accurately describes the pattern and is still valid.

### Step 7: Clean Up Decorative Headers (Throughout)

If `prompt_filtering()` contains decorative headers like:
- `// ============================================================================`
- `=============================================================================`

**Action**: Remove them. Replace section breaks with single blank lines.

### Step 8: Verify Line Count and Structure

After all deletions and trims, file should be:
- **Total lines**: 170-220 (target: ~200)
- **Functions**: Exactly 2 prompt functions (basic + filtering)
- **Comment header**: Still present and accurate
- **PromptProvider impl**: Still present and updated

---

## Success Criteria (Definition of Done)

The task is complete when ALL of the following are true:

### Size Criteria
- **File is exactly 170-220 lines** (verify: `wc -l prompts.rs`)
- Ideally ~200 lines (the target in original task)

### Scenario Criteria
- **Exactly 2 scenario functions exist** (verify: `grep "^fn prompt_" prompts.rs | wc -l`)
  - `prompt_basic()` function exists
  - `prompt_filtering()` function exists
  - NO `prompt_exploration()` function
  - NO `prompt_views()` function
  - NO `prompt_comprehensive()` function

### Code Structure Criteria
- **Match statement has exactly 2 branches**:
  ```rust
  match args.scenario.as_deref() {
      Some("filtering") => prompt_filtering(),
      _ => prompt_basic(),
  }
  ```
- **prompt_arguments() description shows**: "Scenario to show (basic, filtering)"
- **No dangling references** to deleted functions
- **All decorative header lines removed**: `grep "═══" prompts.rs` returns 0 results

### Content Criteria
- **basic scenario (~90 lines)**:
  - Has tool description
  - Shows response format with JSON structure
  - Explains table types (table, view, materialized_view)
  - Lists default schemas by database
  - Has 3+ common pattern examples
  - Mentions "when to use"

- **filtering scenario (~100 lines)**:
  - Teaches filtering by pattern (startsWith, endsWith, includes)
  - Teaches filtering by type (table, view, materialized_view)
  - Teaches filtering by size (row counts)
  - Teaches filtering by schema
  - No COMBINING FILTERS section
  - No SORTING RESULTS section
  - Best practices reduced to 2 essential items

### Validation Checks

Run these commands to verify completion:

1. **Line count check**:
   ```bash
   wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/list_tables/prompts.rs
   # Expected: 170-220 lines
   ```

2. **Function count check**:
   ```bash
   grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/list_tables/prompts.rs
   # Expected: exactly 2 functions (prompt_basic, prompt_filtering)
   ```

3. **Deleted content verification**:
   ```bash
   grep "prompt_exploration\|prompt_views\|prompt_comprehensive" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/list_tables/prompts.rs
   # Expected: 0 results (except in comments)
   ```

4. **Decorative header check**:
   ```bash
   grep "════" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/list_tables/prompts.rs
   # Expected: 0 results
   ```

5. **Match statement check**:
   ```bash
   grep -A 3 "fn generate_prompts" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/list_tables/prompts.rs
   # Expected: Shows "Some(\"filtering\") => prompt_filtering()," and "_ => prompt_basic(),"
   ```

6. **Rust compilation check**:
   ```bash
   cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
   cargo check 2>&1 | head -20
   # Expected: No errors related to this file
   ```

---

## Reference: Complexity 2 Pattern

This task follows the established Complexity 2 standard (from PRECURSOR_02_fs_read_file.md):

- **1 basic scenario** (~90 lines): How to use the tool
- **1 advanced/optional scenario** (~100 lines): How to use special parameters/patterns
- **0 use-case scenarios**: Not teaching domain knowledge or cross-tool workflows
- **0 comprehensive scenarios**: No duplication
- **Total: 170-220 lines** for the complete file

All Complexity 2 tools should match this structure.

---

## Execution Checklist

- [ ] Read the current prompts.rs file (already done)
- [ ] Identify exact line numbers for deletion
- [ ] Delete `prompt_exploration()` function entirely
- [ ] Delete `prompt_views()` function entirely
- [ ] Delete `prompt_comprehensive()` function entirely
- [ ] Trim `prompt_filtering()` by removing COMBINING FILTERS and SORTING RESULTS sections
- [ ] Reduce BEST PRACTICES in filtering to 2 essential items
- [ ] Update match statement to remove 4 scenario arms
- [ ] Update match statement default to `prompt_basic()`
- [ ] Update `prompt_arguments()` description string
- [ ] Remove decorative header lines (═══)
- [ ] Verify file compiles: `cargo check`
- [ ] Verify line count: 170-220 lines
- [ ] Verify scenario count: exactly 2 functions
- [ ] Verify no deleted function references remain
- [ ] Verify no decorative headers remain
