# TASK 062: Trim db_table_indexes

**Tool**: `db_table_indexes`
**Complexity**: 2 (Simple)
**Current size**: 632 lines (5 scenarios)
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/table_indexes/prompts.rs`

---

## Context

This tool retrieves database table indexes via the MCP PromptProvider pattern. The current 632 lines contain excessive scenario duplication and include educational content (types, composite, performance) that don't teach anything new about the tool itself. 

The tool is simple: given a schema and table name, it returns all indexes on that table. The parameters are basic (schema, table) and the output structure is straightforward (array of index objects with name, columns, type, uniqueness flags).

**Current file**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/table_indexes/prompts.rs`

---

## Current File Analysis

**Exact structure (632 lines total):**

1. Lines 1-7: Module header and imports
2. Lines 8-34: PromptProvider trait implementation and routing
3. Lines 35-106: `prompt_comprehensive()` function - 72 lines ← KEEP
4. Lines 107-220: `prompt_index_types()` function - 114 lines ← DELETE
5. Lines 221-392: `prompt_composite_indexes()` function - 172 lines ← DELETE
6. Lines 393-590: `prompt_performance_implications()` function - 198 lines ← DELETE
7. Lines 591-632: `prompt_usage_scenarios()` function - 42 lines ← KEEP

**Current scenarios breakdown:**

1. **`prompt_comprehensive()`** (lines 35-106, 72 lines) - KEEP
   - Demonstrates basic tool usage with simple query pattern
   - Shows response structure with 3 realistic index examples
   - Explains key properties (name, columns, index_type, is_unique, is_primary)
   - Explains why indexes matter (speed, uniqueness, write overhead, disk space)
   - Perfect for default/basic scenario
   - Good length already (~70 lines)

2. **`prompt_index_types()`** (lines 107-220, 114 lines) - DELETE
   - Educational content about PostgreSQL index types (btree, hash, gin, gist, brin)
   - Detailed explanations of when to use each type
   - NOT about the tool - this teaches database concepts
   - No special parameters for different index types
   - User can read PostgreSQL docs for this

3. **`prompt_composite_indexes()`** (lines 221-392, 172 lines) - DELETE
   - Educational content about multi-column indexes
   - Explains column ordering and left-most prefix rule
   - NOT about the tool - this teaches database concepts
   - Tool returns composite indexes the same way as single-column
   - No tool-specific knowledge here

4. **`prompt_performance_implications()`** (lines 393-590, 198 lines) - DELETE
   - Educational content about index performance trade-offs
   - Explains read vs write performance
   - NOT about the tool - this teaches database performance theory
   - No tool-specific parameters or features
   - Belongs in database tuning guides, not tool prompts

5. **`prompt_usage_scenarios()`** (lines 591-632, 42 lines) - KEEP
   - Shows 6 practical use cases (query optimization, foreign keys, uniqueness, analysis, planning, primary keys)
   - These are ACTUAL TOOL USAGE PATTERNS
   - Demonstrates how to use the tool in real workflows
   - Shows how to combine with other tools (db_table_schema, db_execute_sql)
   - This is the optional advanced scenario
   - Perfect complement to basic scenario

---

## Trimming Instructions

### Step 1: Update PromptProvider Routing (lines 15-24)

**Current routing** (lines 15-24):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("types") => prompt_index_types(),
        Some("composite") => prompt_composite_indexes(),
        Some("performance") => prompt_performance_implications(),
        Some("usage") => prompt_usage_scenarios(),
        _ => prompt_comprehensive(),
    }
}
```

**After trimming** (replace with):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("usage") => prompt_usage_scenarios(),
        _ => prompt_comprehensive(),
    }
}
```

**Rationale**: Keep only 2 scenarios. Default is basic/comprehensive. Optional advanced scenario is "usage" for practical patterns.

### Step 2: Update prompt_arguments() Description (lines 25-33)

**Current description** (line 31):
```rust
description: Some("Index inspection scenario: types, composite, performance, usage".to_string()),
```

**After trimming** (replace with):
```rust
description: Some("Index inspection scenario: usage (optional) for practical patterns".to_string()),
```

**Rationale**: List only the remaining scenarios. Make clear that "usage" is optional (non-default).

### Step 3: Delete prompt_index_types() Function (lines 107-220)

**Exact range to delete**: Lines 107-220 (114 lines)

**Function signature**: `fn prompt_index_types() -> Vec<PromptMessage> {`

**Content to remove**: Entire function including closing brace. This includes:
- Educational explanation of 5 index types (btree, hash, gin, gist, brin)
- Use case for each type with SQL examples
- Comparison table (JSON format)
- "Choosing the right index type" guidance

**Why delete**: This is database education, not tool documentation. Users should know index types independently of this tool. The tool returns whatever indexes exist - it doesn't let you choose types.

### Step 4: Delete prompt_composite_indexes() Function (lines 221-392)

**Exact range to delete**: Lines 221-392 (172 lines)

**Function signature**: `fn prompt_composite_indexes() -> Vec<PromptMessage> {`

**Content to remove**: Entire function including closing brace. This includes:
- Explanation of multi-column indexes
- Column order importance (phone book analogy)
- Left-most prefix rule detailed breakdown
- SQL examples of what queries use/don't use index
- Comparison: composite vs multiple single-column indexes

**Why delete**: This is database education about composite indexes, not tool-specific. The tool returns composite indexes (with columns array) exactly the same way as single-column indexes. User education about prefix rules doesn't belong in tool prompts.

### Step 5: Delete prompt_performance_implications() Function (lines 393-590)

**Exact range to delete**: Lines 393-590 (198 lines)

**Function signature**: `fn prompt_performance_implications() -> Vec<PromptMessage> {`

**Content to remove**: Entire function including closing brace. This includes:
- Index performance trade-offs (reads vs writes)
- WHERE clause filtering examples
- JOIN performance
- ORDER BY performance
- Uniqueness check performance
- INSERT/UPDATE/DELETE overhead
- Disk space calculations
- Index bloat explanation
- Best practices for indexing

**Why delete**: This is database performance tuning education, not tool documentation. None of this content is specific to the db_table_indexes tool. The tool just returns what's there - it doesn't help with performance analysis directly. Move this to database tuning guides.

### Step 6: Keep prompt_comprehensive() Function (lines 35-106)

**Status**: KEEP unchanged

**This becomes the default scenario** returned when no scenario is specified. Already lean at 72 lines with:
- Basic input example (query pattern)
- Response structure with 3 realistic index examples  
- Key properties explained once
- Why indexes matter (brief, 4 bullets)

### Step 7: Keep prompt_usage_scenarios() Function (lines 591-632)

**Status**: KEEP unchanged

**This becomes the optional advanced scenario** for "usage" argument. Contains 6 practical use cases:
1. Query optimization (checking existing indexes, identifying missing ones)
2. Foreign key relationships (verifying indexes on FK columns)
3. Uniqueness constraints (understanding unique indexes)
4. Performance analysis (checking index count, identifying redundant indexes)
5. Query planning (checking indexes before writing complex queries)
6. Understanding primary keys (finding and explaining PK indexes)

All show real tool usage patterns with TypeScript examples integrating with other tools.

---

## Execution Checklist

Apply these edits in order:

1. **Edit 1: Update routing match statement**
   - File: `prompts.rs` lines 15-24
   - Replace the entire match block
   - Remove 3 arms (types, composite, performance)
   - Keep usage and default arms

2. **Edit 2: Update scenario description**
   - File: `prompts.rs` line 31
   - Replace description string
   - Simplify to "usage (optional)"

3. **Edit 3: Delete prompt_index_types()**
   - File: `prompts.rs` lines 107-220
   - Delete entire function (114 lines)

4. **Edit 4: Delete prompt_composite_indexes()**
   - File: `prompts.rs` lines 221-392
   - Delete entire function (172 lines)
   - NOTE: Line numbers shift down after Edit 3 (by 114 lines)
   - Adjusted range: lines 107-278 (221-114 to 392-114)

5. **Edit 5: Delete prompt_performance_implications()**
   - File: `prompts.rs` lines 393-590
   - Delete entire function (198 lines)
   - NOTE: Line numbers shift down further after Edits 3-4 (by 114+172=286 total)
   - Adjusted range: lines 107-292 (393-286 to 590-286)

---

## Expected Result After Trimming

**File structure** (~190-210 lines):
- Lines 1-7: Module header and imports (unchanged)
- Lines 8-25: PromptProvider trait with simplified routing
- Lines 26-98: prompt_comprehensive() function (70 lines) - default scenario
- Lines 99-210: prompt_usage_scenarios() function (112 lines) - optional scenario

**Final line count**: 210 lines (target: 170-220) ✓

**Scenario count**: 2 functions (target: 1-2) ✓

---

## Success Criteria

Verify completion with these checks:

1. **Line count**: 
   ```bash
   wc -l packages/kodegen-mcp-schema/src/database/table_indexes/prompts.rs
   # Expected: 210 lines (range 170-220 acceptable)
   ```

2. **Scenario functions**: 
   ```bash
   grep "^fn prompt_" packages/kodegen-mcp-schema/src/database/table_indexes/prompts.rs
   # Expected output:
   # fn prompt_comprehensive() -> Vec<PromptMessage> {
   # fn prompt_usage_scenarios() -> Vec<PromptMessage> {
   ```

3. **Routing simplicity**: 
   ```bash
   grep -A 5 "fn generate_prompts" packages/kodegen-mcp-schema/src/database/table_indexes/prompts.rs
   # Expected: Only 2 match arms (usage and _ default)
   ```

4. **No educational content**: 
   ```bash
   grep -i "performance\|trade-off\|overhead\|left-most prefix\|btree\|gin\|gist" \
     packages/kodegen-mcp-schema/src/database/table_indexes/prompts.rs | wc -l
   # Expected: 0-2 lines maximum (performance mentioned in basic context only)
   ```

5. **File compiles**: 
   ```bash
   cd packages/kodegen-mcp-schema && cargo check
   # Expected: No errors, no warnings about unused functions
   ```

---

## Code Pattern Examples

### Before: Over-engineered with multiple scenarios

```rust
match args.scenario.as_deref() {
    Some("types") => prompt_index_types(),        // ❌ Educational (remove)
    Some("composite") => prompt_composite_indexes(), // ❌ Educational (remove)
    Some("performance") => prompt_performance_implications(), // ❌ Educational (remove)
    Some("usage") => prompt_usage_scenarios(),    // ✅ Tool usage (keep)
    _ => prompt_comprehensive(),                   // ✅ Basic (keep)
}
```

### After: Focused on actual tool usage

```rust
match args.scenario.as_deref() {
    Some("usage") => prompt_usage_scenarios(),    // ✅ Practical patterns
    _ => prompt_comprehensive(),                   // ✅ Default/basic
}
```

---

## Why This Trimming Is Correct

**What we're keeping:**
1. **`prompt_comprehensive()`** - Shows the basic tool: input, output, properties. Answers "What does this tool do?"
2. **`prompt_usage_scenarios()`** - Shows practical patterns: foreign keys, uniqueness, optimization, analysis. Answers "When/how do I use this?"

**What we're deleting:**
1. **`prompt_index_types()`** - Database education (btree vs gin vs gist). Belongs in database docs, not tool prompts.
2. **`prompt_composite_indexes()`** - Database education (column ordering, prefix rules). Belongs in database docs.
3. **`prompt_performance_implications()`** - Database performance theory (reads vs writes, disk space). Belongs in tuning guides.

**Philosophy**: Tool prompts should teach tool usage, not database theory. Educational content distracts from learning the actual API.

---

## Validation After Completion

After applying all edits:

1. Open the file in editor
2. Verify you can read entire file in <2 minutes
3. Understand all 2 scenarios and their purposes
4. See that every line teaches something about db_table_indexes tool usage
5. No line teaches database theory unrelated to the tool
6. Run `cargo check` to ensure no compilation errors
7. File should be in the 170-220 line range (target met if 190-210 lines)

---

## This Demonstrates the Complexity 2 Standard

After trimming, this tool exemplifies the **Complexity 2 standard**:
- **1 basic/default scenario** (~70 lines): What does the tool do?
- **1 optional advanced scenario** (~110 lines): When/how to use it?
- **Clear, focused prompts** that teach the tool, not database theory
- **No decorative headers or redundancy**
- **Total: 170-220 lines** for complete tool education

Apply this pattern to all 64+ Complexity 2 tools in the codebase.
