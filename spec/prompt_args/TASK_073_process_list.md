# TASK 073: Trim process_list

**Tool**: `process_list`
**Complexity**: 2 (Simple)
**Current size**: 310 lines (5 scenarios)
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/process/process_list/prompts.rs`

---

## Context & Objective

The `process_list` tool lists running processes with 3 parameters: `filter`, `limit`, and implicit all-processes listing. The current 310-line file contains severe redundancy with a 104-line comprehensive scenario that duplicates all others, plus use-case scenarios (monitoring, workflows) that don't teach new tool features.

The goal is to trim to ~200 lines containing only 2 scenarios:
1. **Basic scenario** (~90 lines): Simple process listing with all core features
2. **Filtering scenario** (~90 lines): Filtering and limit parameters

This matches the Complexity 2 reference standard (PRECURSOR_02_fs_read_file.md).

---

## Current State Analysis

**File location**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/process/process_list/prompts.rs`

**Current line count**: 310 lines total

**Current scenarios** (5 functions):
1. `prompt_basic()` - lines 46-75 (30 lines) ← **KEEP, MINIMAL TRIM**
   - Teaches: List all processes, response structure, field meanings
   - Quality: Good, core teaching content

2. `prompt_filtering()` - lines 77-109 (33 lines) ← **KEEP, TRIM TO ~70-80 LINES**
   - Teaches: Filter by name, case-insensitive matching, partial matching
   - Currently: Concise, needs expansion to fill advanced scenario role
   - Add examples for limit parameter

3. `prompt_monitoring()` - lines 111-155 (45 lines) ← **DELETE (USE-CASE)**
   - Problem: Teaching system monitoring patterns, not tool features
   - "Find high CPU" and "Find memory hogs" are workflows, not tool parameters
   - No unique tool parameters demonstrated beyond basic/filtering
   - Decorative `═══` headers present

4. `prompt_workflows()` - lines 157-205 (49 lines) ← **DELETE (USE-CASE)**
   - Problem: Pure use-cases combining process_list with process_kill
   - Teaching workflow patterns, not process_list features
   - "Find and kill runaway process" is operational workflow
   - "Clean up zombie processes" is DevOps use-case
   - Zero new tool features vs basic/filtering scenarios

5. `prompt_comprehensive()` - lines 207-310 (104 lines) ← **DELETE (DUPLICATION)**
   - Problem: Massive duplication of basic + filtering + monitoring + workflows
   - Lines 215-229: Duplicates basic scenario introduction
   - Lines 231-259: Duplicates filtering examples
   - Lines 261-279: Repeats monitoring examples from prompt_monitoring()
   - Lines 281-307: Includes decorative `═══════` headers throughout
   - No unique content; pure concatenation with decoration

**Routing logic** (lines 15-27):
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("filtering") => prompt_filtering(),
    Some("monitoring") => prompt_monitoring(),        // DELETE
    Some("workflows") => prompt_workflows(),          // DELETE
    _ => prompt_comprehensive(),                      // DELETE
}
```

---

## Implementation Instructions

### Step 1: Keep and Trim `prompt_basic()` (Target: ~90 lines)

**Current state**: 30 lines (lines 46-75)
**Action**: EXPAND to ~90 lines

**Keep all current content:**
- Tool description (what it returns: PID, name, CPU%, memory)
- Basic usage example: `process_list({})`
- Response structure with actual JSON format
- Field descriptions (pid, name, cpu_percent, memory_mb)

**Add to basic scenario:**
- Add limit parameter example (5-10 lines):
  ```rust
  "By count:\n\
   process_list({ \"limit\": 10 })\n\
   // Returns top 10 processes only"
  ```
- Expand field descriptions (5 lines more):
  - Explain each response field clearly
  - When values might be zero or N/A

**Result**: 90-100 lines

---

### Step 2: Keep and Expand `prompt_filtering()` (Target: ~80-90 lines)

**Current state**: 33 lines (lines 77-109)
**Action**: EXPAND from 33 to ~80-90 lines

**Keep all current content:**
- Filter description (case-insensitive substring matching)
- Filter by name examples (node, postgres)
- Case-insensitive behavior
- Partial match examples (post→postgres, postfix)

**Add to filtering scenario:**
- **Limit parameter examples** (15 lines):
  ```rust
  "With limit:\n\
   process_list({ \"limit\": 5 })\n\
   // Top 5 processes only\n\n\
   process_list({ \"limit\": 50 })\n\
   // Top 50 processes for analysis\n\n\
   LIMIT PATTERNS:\n\
   - \"limit\": 10     // Small set\n\
   - \"limit\": 100    // Large set\n\
   - no limit         // All processes"
  ```

- **Combining filter + limit** (15 lines):
  ```rust
  "Combine filter and limit:\n\
   process_list({\n\
       \"filter\": \"node\",\n\
       \"limit\": 5\n\
   })\n\
   // Top 5 Node processes"
  ```

- **Common patterns** (20 lines):
  ```rust
  "COMMON FILTERING PATTERNS:\n\
   - Find specific service: filter=\"nginx\"\n\
   - Find all Python: filter=\"python\"\n\
   - Top 20 anything: limit=20\n\
   - High-resource subset: filter=\"name\" + limit=10"
  ```

**Result**: 80-90 lines

---

### Step 3: Delete `prompt_monitoring()` (lines 111-155)

This entire function is a USE-CASE scenario, not teaching tool features:
- Finding "high CPU processes" requires application logic outside the tool
- "Finding memory hogs" is operational concern, not tool feature
- Tool feature is "list all", agent applies business logic
- No new parameters or behaviors vs basic/filtering

**Action**: Delete lines 111-155 entirely

---

### Step 4: Delete `prompt_workflows()` (lines 157-205)

This entire function is USE-CASE workflow patterns:
- "Find and kill runaway process" = operational workflow
- Combines process_list + process_kill (tool collaboration, not feature teaching)
- "Clean up zombie processes" = DevOps concern
- "Port conflict resolution" = operational workflow
- No new process_list features demonstrated

**Action**: Delete lines 157-205 entirely

---

### Step 5: Delete `prompt_comprehensive()` (lines 207-310)

This entire function is duplication with decoration:
- Lines 215-229: Duplicates basic scenario content
- Lines 231-259: Repeats filtering examples
- Lines 261-279: Copies monitoring use-cases
- Lines 281-307: Includes decorative separators (`═══════`)
- No unique technical content

**Action**: Delete lines 207-310 entirely

**Verification**: After deletion, file will be ~110 lines + 80 lines expanded basic = ~190 lines. With small tweaks, should land in 170-220 range.

---

### Step 6: Update Routing Logic (lines 15-27)

**Before:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("filtering") => prompt_filtering(),
        Some("monitoring") => prompt_monitoring(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**After:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("filtering") => prompt_filtering(),
        _ => prompt_basic(),
    }
}
```

**Rationale**:
- Default to basic (most common use case)
- Filtering as explicit advanced scenario for complex filtering patterns
- Removed monitoring, workflows, comprehensive as USE-CASES not FEATURES

---

### Step 7: Update Prompt Arguments Description (line 29)

**Before:**
```rust
description: Some("Scenario to show (basic, filtering, monitoring, workflows)".to_string()),
```

**After:**
```rust
description: Some("Scenario to show (basic, filtering)".to_string()),
```

---

### Step 8: Remove Header Comments (Line 37-39)

**Before:**
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO LIST PROCESSES
// ============================================================================
```

**After:**
Delete this decorative header. Comments above functions are sufficient.

---

## Code Structure After Trimming

```rust
//! Prompt messages for process_list tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::ProcessListPromptArgs;

pub struct ProcessListPrompts;

impl PromptProvider for ProcessListPrompts {
    type PromptArgs = ProcessListPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("filtering") => prompt_filtering(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, filtering)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic process listing
fn prompt_basic() -> Vec<PromptMessage> {
    // [EXISTING 30 lines PLUS 60 new lines = ~90 lines]
}

/// Filtering processes by name and limiting results
fn prompt_filtering() -> Vec<PromptMessage> {
    // [EXISTING 33 lines PLUS 50 new lines = ~80-90 lines]
}

// [NO MORE FUNCTIONS]
```

**Total expected lines**: 45 (header/impl) + 90 (basic) + 80 (filtering) = **215 lines** ✓ (within 170-220 range)

---

## Implementation Checklist

- [ ] **Delete monitoring scenario**: Remove lines 111-155 and `Some("monitoring") =>` from match
- [ ] **Delete workflows scenario**: Remove lines 157-205 and `Some("workflows") =>` from match
- [ ] **Delete comprehensive scenario**: Remove lines 207-310 and `_ => prompt_comprehensive()` reference
- [ ] **Expand prompt_basic()**: Add 60 lines of content about limit parameter and expanded explanations
- [ ] **Expand prompt_filtering()**: Add 50 lines covering limit parameter, combining filter+limit, common patterns
- [ ] **Update routing**: Change match to default to basic, keep filtering explicit
- [ ] **Update description**: Change scenario options to "(basic, filtering)"
- [ ] **Remove decoration**: Delete `// ============================================================================` header
- [ ] **Verify line count**: `wc -l prompts.rs` should show 170-220 lines
- [ ] **Verify functions**: `grep "^fn prompt_" | wc -l` should show exactly 2
- [ ] **Verify no duplication**: `grep -c "Find high CPU"` should be 0 (monitoring deleted)
- [ ] **Verify routing**: Confirm match statement has exactly 2 arms (filtering + default)

---

## Success Criteria

- ✓ File is 170-220 lines total (expect ~215)
- ✓ Exactly 2 scenario functions: `prompt_basic()` and `prompt_filtering()`
- ✓ No use-case scenarios: monitoring and workflows completely removed
- ✓ No comprehensive scenario with decorative headers
- ✓ No duplication of content across scenarios
- ✓ Limit parameter taught in filtering scenario (not basic)
- ✓ All three tool features covered:
  - Basic: List all processes, response structure
  - Filtering: Name matching, case-insensitive, partial match
  - Filtering (advanced): Limit parameter, combining filter+limit
- ✓ Match statement has 2 arms: filtering explicit, basic as default
- ✓ Prompt arguments description lists only "basic, filtering"
- ✓ No decorative `═══` or `─────` headers
- ✓ Readable by AI in 2-3 minutes

---

## Validation After Completion

Run these commands to verify success:

```bash
# Check line count
wc -l packages/kodegen-mcp-schema/src/process/process_list/prompts.rs
# Expected: 170-220

# Count scenario functions
grep "^fn prompt_" packages/kodegen-mcp-schema/src/process/process_list/prompts.rs | wc -l
# Expected: 2

# Verify monitoring deleted
grep -i "monitoring" packages/kodegen-mcp-schema/src/process/process_list/prompts.rs
# Expected: 0 results

# Verify workflows deleted
grep -i "workflows" packages/kodegen-mcp-schema/src/process/process_list/prompts.rs
# Expected: 0 results

# Verify comprehensive deleted
grep "comprehensive" packages/kodegen-mcp-schema/src/process/process_list/prompts.rs
# Expected: 0 results

# Verify no decorative headers
grep "═══" packages/kodegen-mcp-schema/src/process/process_list/prompts.rs
# Expected: 0 results

# Verify limit parameter is taught
grep -i "limit" packages/kodegen-mcp-schema/src/process/process_list/prompts.rs
# Expected: 3+ results in filtering scenario
```

---

## Reference

This task follows the Complexity 2 template from **PRECURSOR_02_fs_read_file.md**:
- Delete use-case scenarios (monitoring, workflows)
- Delete comprehensive duplication
- Keep and expand 2 focused teaching scenarios
- Total 170-220 lines
- Each parameter clearly demonstrated
- No decorative elements
