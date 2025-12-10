# TASK 098: Trim db_execute_sql Prompts

**Tool**: `db_execute_sql`
**Complexity**: 3 (Medium)
**Current size**: 1,057 lines (6 scenarios)
**Target size**: 280-360 lines (3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/execute_sql/prompts.rs`

---

## Reference

This task follows the **Complexity 3 template** established by PRECURSOR_03_git_branch_create.md. The db_execute_sql tool has legitimate complexity (multiple execution modes, safety considerations, parameter handling) but suffers from scenario redundancy and overly exhaustive documentation.

---

## Current State Analysis

### File Overview
- **Location**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/execute_sql/prompts.rs`
- **Current structure**: 1,057 total lines
- **Impl block**: Lines 1-43 (struct + impl PromptProvider + match statement)

### Current Scenarios (6 total)

1. **prompt_select()** - ~146 lines (lines 45-190)
   - Safe SELECT query patterns with LIMIT, WHERE, joins, aggregations
   - Includes parameter placeholders for SQLite/MySQL/PostgreSQL
   - Best practices for read operations

2. **prompt_modification()** - ~280 lines (lines 193-472)
   - INSERT, UPDATE, DELETE query patterns
   - Modification workflow (verify first with SELECT)
   - Checklist for safe modifications
   - Common modification patterns

3. **prompt_safety()** - ~280 lines (lines 475-754)
   - SQL injection prevention (primary focus)
   - Parameterized query patterns
   - Database-specific placeholder syntax
   - Real-world injection examples
   - What parameters do/don't protect

4. **prompt_transactions()** - ~290 lines (lines 757-1046)
   - BEGIN/COMMIT/ROLLBACK patterns
   - When to use transactions
   - Transaction workflow examples
   - Isolation levels (READ COMMITTED, REPEATABLE READ, SERIALIZABLE)
   - ACID properties

5. **prompt_troubleshooting()** - ~220+ lines
   - Common query issues: syntax errors, no results, too many results, performance
   - Debugging strategies (COUNT, test simpler queries, EXPLAIN)
   - Performance analysis tips
   - Data type mismatches

6. **prompt_comprehensive()** - ~380+ lines
   - **PURE DUPLICATION** of all 5 focused scenarios
   - Combines SELECT, modification, safety, transactions, and troubleshooting
   - Includes redundant decision tree and best practices checklist
   - **This must be deleted entirely**

### Routing Structure (current - lines 20-27)

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("select") => prompt_select(),
        Some("modification") => prompt_modification(),
        Some("safety") => prompt_safety(),
        Some("transactions") => prompt_transactions(),
        Some("troubleshooting") => prompt_troubleshooting(),
        _ => prompt_comprehensive(),
    }
}
```

### Problem Analysis

**Redundancy issues**:
- `prompt_comprehensive()` duplicates all content from other 5 scenarios (380 lines of pure duplication)
- `prompt_modification()` repeats safety concepts already in `prompt_safety()`
- `prompt_transactions()` includes verbose isolation level explanations not essential for medium complexity
- `prompt_troubleshooting()` mixes pattern documentation with debugging (should be separated)

**Scope creep**:
- Isolation levels (SERIALIZABLE, REPEATABLE READ) are advanced topics for Complexity 5+ tools
- Extensive troubleshooting guidance (10+ distinct issue types) is too detailed
- Multiple workflow examples for each operation type
- Redundant best practices checklists

---

## Trimming Strategy

### Target Structure: 3 Scenarios

Following the Complexity 3 template, consolidate into focused scenarios:

1. **prompt_basic()** [RENAME from "select"] - Target: 110-120 lines
   - Basic SELECT patterns with LIMIT, WHERE, parameters
   - Keep: Join patterns, aggregations, common query types
   - Remove: Extended performance tips, pagination details

2. **prompt_safety()** [CONSOLIDATE] - Target: 110-120 lines
   - Merge: current safety + modification + transactions
   - Keep: SQL injection prevention, parameterized queries, modification workflow, basic transactions
   - Remove: Isolation levels, verbose examples, isolation level comparison table

3. **prompt_patterns()** [EXTRACT from troubleshooting + consolidate] - Target: 100-110 lines
   - Common query patterns and troubleshooting
   - Keep: Common patterns, basic debugging, one error example per type
   - Remove: Extensive issue-by-issue debugging, verbose workarounds

### Detailed Scenario Trimming

#### SCENARIO 1: Keep & Trim prompt_select() → prompt_basic()

**Current**: 146 lines
**Target**: 110-120 lines

**What to Keep** (90 lines):
- User/Assistant message pair opening (4 lines)
- Basic SELECT syntax with LIMIT (8 lines)
- Filtered query example (8 lines)
- Parameters for SQLite/MySQL/PostgreSQL (8 lines)
- Aggregation examples (10 lines)
- JOIN examples (12 lines)
- Pattern matching with LIKE (8 lines)
- Read query best practices bullet list (15 lines)
- Common read patterns bullet list (10 lines)

**What to Remove** (30+ lines):
- Decorative line 44: "How do I execute safe SELECT queries using db_execute_sql?" → Keep (part of message)
- Lines with "PERFORMANCE TIPS:" section → Move general index discussion to patterns, keep only "Use LIMIT" and "Avoid SELECT *"
- Redundant best practices (some duplicate with safety scenario)

**Exact changes**:
- Keep lines 46-85 (all examples): Feature branch creation patterns, all 7 SELECT patterns
- Keep lines 87-104 (READ QUERY BEST PRACTICES): All 7 bullet points
- Keep lines 106-114 (COMMON READ PATTERNS): All 5 patterns
- DELETE lines 116-146 (PERFORMANCE TIPS extended section)
- NEW: Add brief intro: "Read operations are the safest database operations."

#### SCENARIO 2: Consolidate → prompt_safety()

**Sources**:
- Current safety section: 280 lines
- Current modification section: 280 lines
- Current transactions section: 290 lines
- **Combined**: 850 lines
**Target**: 110-120 lines

**New prompt_safety() structure** (120 lines):

1. **Opening message** (2 lines): User question about safety

2. **SQL Injection Prevention** (35 lines)
   - Why it matters: "SQL injection is the most dangerous security vulnerability"
   - Parameterized query definition (8 lines)
   - Dangerous vs safe example (4 lines)
   - Parameter syntax by database (10 lines)
   - One real-world example (5 lines)

3. **Modification Workflow** (40 lines)
   - Before any UPDATE/DELETE, follow these steps: (2 lines)
   - Step 1: SELECT to preview (5 lines)
   - Step 2: COUNT affected rows (5 lines)
   - Step 3: Execute modification (5 lines)
   - Step 4: Verify the change (5 lines)
   - Critical warnings: NEVER omit WHERE clause (10 lines)
   - One INSERT, UPDATE, DELETE example (3 lines)

4. **Basic Transactions** (35 lines)
   - When to use: multiple related operations (5 lines)
   - Basic structure: BEGIN/operation/COMMIT or ROLLBACK (8 lines)
   - One money transfer example (15 lines)
   - Key point: "Use transactions when all operations must succeed or all fail" (2 lines)

5. **Closing** (8 lines): Summary of critical rules (parameterized queries, WHERE clauses, verification)

**What to delete entirely**:
- Lines from current safety: Isolation level table (30+ lines), extended parameter examples (30 lines), what parameters don't protect (40 lines)
- Lines from current modification: Multiple patterns for each operation type (100+ lines), extensive checklist (15+ items)
- Lines from current transactions: Isolation level definitions (80+ lines), ACID properties explanation (15+ lines), multiple workflow examples (100+ lines)

**Critical**: Remove all isolation level discussion (SERIALIZABLE, REPEATABLE READ, READ COMMITTED). Save for Complexity 5+ tools.

#### SCENARIO 3: Extract & Create → prompt_patterns()

**Sources**:
- Current troubleshooting: 220+ lines (patterns + debugging)
- Extract patterns from other scenarios: 30+ lines
**Target**: 100-110 lines

**New prompt_patterns() structure** (110 lines):

1. **Opening message** (2 lines): User asking for common patterns

2. **Common SELECT Patterns** (25 lines)
   - Explore table: `SELECT * FROM table LIMIT 10` (3 lines)
   - Count records: `SELECT COUNT(*) FROM table WHERE ...` (3 lines)
   - Find specific record: `SELECT * FROM table WHERE id = ?` (3 lines)
   - Recent records: `SELECT * FROM table ORDER BY created_at DESC LIMIT 20` (3 lines)
   - Range query: `SELECT * FROM table WHERE created_at BETWEEN ? AND ?` (3 lines)
   - Page pagination: LIMIT with OFFSET (3 lines)
   - Distinct values: `SELECT DISTINCT column FROM table` (2 lines)

3. **Common Modification Patterns** (20 lines)
   - INSERT single record (3 lines)
   - UPDATE single column with WHERE (3 lines)
   - UPDATE multiple columns (3 lines)
   - DELETE with conditions (3 lines)
   - Batch operations (3 lines)
   - Using RETURNING (PostgreSQL) (2 lines)

4. **Quick Troubleshooting** (30 lines)
   - No results: Check table has data, remove WHERE, check case sensitivity (10 lines)
   - Too many results: Add LIMIT, more specific WHERE, use date ranges (10 lines)
   - Syntax error: Check quotes, verify keywords, test simpler query (8 lines)
   - Performance: Use EXPLAIN, check indexes exist (2 lines)

5. **Quick Debug Checklist** (15 lines)
   - Start with simple query (2 lines)
   - Add complexity gradually (2 lines)
   - Test each JOIN separately (2 lines)
   - Use COUNT before LIMIT (2 lines)
   - Use EXPLAIN for performance (2 lines)
   - Check schema with db_table_schema tool (2 lines)
   - Test on small dataset first (1 line)

6. **Closing** (8 lines): Summary pointing to other scenarios for deep dives

**What to delete**:
- Extensive issue-by-issue analysis (90+ lines)
- Multiple resolution approaches per issue (50+ lines)
- Verbose debugging workflows
- Extended performance analysis (move essentials only to this scenario)
- Data type mismatch examples (too detailed)

### Routing Update

**Current routing** (lines 20-27):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("select") => prompt_select(),
        Some("modification") => prompt_modification(),
        Some("safety") => prompt_safety(),
        Some("transactions") => prompt_transactions(),
        Some("troubleshooting") => prompt_troubleshooting(),
        _ => prompt_comprehensive(),
    }
}
```

**New routing** (lines 20-24):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("safety") => prompt_safety(),
        Some("patterns") => prompt_patterns(),
        _ => prompt_basic(),
    }
}
```

**Changes**:
- Rename "select" → keep as default case (not a match arm)
- Merge "modification", "transactions" into "safety" scenario
- Delete "troubleshooting" → extract to "patterns"
- Delete "comprehensive" → no fallback needed
- Update prompt_arguments() description: "Scenario to show (safety, patterns)" → only list remaining 2 scenarios

### Prompt Arguments Update

**Current** (lines 29-35):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (select, modification, safety, transactions, troubleshooting)".to_string()),
            required: Some(false),
        }
    ]
}
```

**New** (lines 29-35):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (default: basic SQL queries, safety: parameterized queries and transactions, patterns: common patterns and troubleshooting)".to_string()),
            required: Some(false),
        }
    ]
}
```

### Function Deletions

**Delete entirely** (don't just rename):
1. `fn prompt_modification()` - Content merged into prompt_safety()
2. `fn prompt_transactions()` - Content merged into prompt_safety()
3. `fn prompt_troubleshooting()` - Content extracted to prompt_patterns()
4. `fn prompt_comprehensive()` - Pure duplication, 380+ lines

**Rename**:
1. `fn prompt_select()` → `fn prompt_basic()` (reflects broader scope)

**Keep unchanged**:
1. `fn prompt_safety()` - Restructured with consolidated content

**Create new**:
1. `fn prompt_patterns()` - Extracted and created from troubleshooting + consolidation

---

## Implementation Workflow

### Step 1: Plan the deletions
1. Remove `fn prompt_comprehensive()` entirely (verify it starts at correct line)
2. Remove `fn prompt_troubleshooting()` entirely
3. Remove `fn prompt_transactions()` entirely
4. Remove `fn prompt_modification()` entirely

### Step 2: Rename prompt_select to prompt_basic
- Change `fn prompt_select()` to `fn prompt_basic()`
- Update first message to broader context

### Step 3: Restructure prompt_safety()
- Keep opening message structure
- Delete isolation level section (80+ lines)
- Delete "What parameters don't protect" section (40 lines)
- Delete "REAL-WORLD EXAMPLES" section (consolidate to 1-2 examples)
- Keep SQL injection prevention (core)
- Extract modification workflow from prompt_modification() (20-30 lines)
- Extract transaction basics from prompt_transactions() (15-20 lines)
- Consolidate to 110-120 lines total

### Step 4: Create prompt_patterns()
- Extract pattern documentation from other scenarios
- Extract quick troubleshooting from current troubleshooting scenario
- Organize by: SELECT patterns, modification patterns, quick troubleshooting, debug checklist
- Target: 100-110 lines

### Step 5: Update routing
- Update match statement to 3 arms (safety, patterns, default)
- Update prompt_arguments() description
- Verify all scenarios are accessible

### Step 6: Verify line counts
- Use `wc -l prompts.rs` to verify final count is 280-360 lines
- Use `grep "^fn prompt_" prompts.rs` to verify exactly 3 functions
- Use `grep "fn prompt_modification\|fn prompt_transactions\|fn prompt_troubleshooting\|fn prompt_comprehensive\|fn prompt_select" prompts.rs` to verify deletions/renames

---

## Success Criteria

**Line Count**: `wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/execute_sql/prompts.rs`
- ✓ Result: 280-360 lines total (including impl, struct, routing, all scenarios)

**Scenario Functions**: `grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/execute_sql/prompts.rs`
- ✓ Result: Exactly 3 functions
  - `fn prompt_basic()`
  - `fn prompt_safety()`
  - `fn prompt_patterns()`

**Deleted Functions**: `grep -c "fn prompt_select\|fn prompt_modification\|fn prompt_transactions\|fn prompt_troubleshooting\|fn prompt_comprehensive" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/execute_sql/prompts.rs`
- ✓ Result: 0 (all old names removed, only prompt_basic remains)

**Routing Updated**: Check lines 20-27
- ✓ Match statement has exactly 3 arms: safety, patterns, default
- ✓ Default case calls `prompt_basic()`
- ✓ prompt_arguments() lists only 2 scenarios: safety, patterns

**Content Verification**:
- ✓ prompt_basic() is 110-120 lines: SELECT examples, parameters, best practices
- ✓ prompt_safety() is 110-120 lines: SQL injection + modification workflow + basic transactions
- ✓ prompt_patterns() is 100-110 lines: Common patterns + quick troubleshooting

**No Test/Benchmark/Documentation**:
- ✓ File contains only prompts (no test content)
- ✓ No Markdown documentation added to this file
- ✓ Focus is implementation guidance only

---

## Validation Commands

Run these after completion to verify success:

```bash
# Verify final line count
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/execute_sql/prompts.rs
# Expected: 280-360

# Verify scenario function count
grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/execute_sql/prompts.rs | wc -l
# Expected: 3

# List the scenario functions
grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/execute_sql/prompts.rs
# Expected: prompt_basic, prompt_safety, prompt_patterns

# Verify old function names are gone
grep "fn prompt_select\|fn prompt_modification\|fn prompt_transactions\|fn prompt_troubleshooting\|fn prompt_comprehensive" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/execute_sql/prompts.rs
# Expected: (empty - no results)

# Verify routing is updated
sed -n '20,27p' /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/execute_sql/prompts.rs
# Expected: 3 match arms (safety, patterns, default)

# Compile check
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check
# Expected: No errors
```

---

## Notes for Execution

- This is a **prescriptive task**: all guidance IS how to do it, not optional suggestions
- Focus on **consolidation, not creation**: reuse existing content, reorganize to be concise
- **Isolation levels are out of scope**: Do not include SERIALIZABLE, REPEATABLE READ, READ COMMITTED level details
- **One example per pattern**: Use the best, most instructive example for each pattern type
- **Safety first mentality**: Keep SQL injection prevention as central theme through all 3 scenarios
- **Quick reference style**: Patterns scenario should be scannable, not narrative prose
