# TASK 003: Trim db_pool_stats

**Tool**: `db_pool_stats`
**Complexity**: 1 (Trivial)
**Current size**: 710 lines
**Target size**: 100 lines (1 scenario)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/pool_stats/prompts.rs`

---

## Core Objective

Aggressively trim the `prompts.rs` file from 710 lines down to approximately 100 lines by eliminating redundant prompt scenarios and consolidating to a single, focused "basic" usage scenario. This aligns with Complexity 1 standards where tools provide minimal, essential guidance rather than comprehensive multi-scenario documentation.

---

## Current State Analysis

The file currently contains **4 prompt scenarios**:

| Scenario | Lines | Purpose | Keep? |
|----------|-------|---------|-------|
| `prompt_basic()` | ~150 | Core usage, health checks | ✓ YES |
| `prompt_troubleshooting()` | ~200 | Diagnostic guide, 5 failure scenarios | DELETE |
| `prompt_monitoring()` | ~200 | Monitoring patterns, alerting thresholds | DELETE |
| `prompt_comprehensive()` | ~160 | Complete reference with decorative headers | DELETE |

The `PromptProvider` implementation currently routes to these scenarios via a match statement in `generate_prompts()`, with conditional logic that supports 3 different scenario parameters.

**Redundancy Identified**:
- Sections repeated across all scenarios: basic usage, response structure, key metrics
- Extensive decorative section headers (====== lines)
- Multiple pool state examples (healthy, busy, exhausted) shown in each scenario
- Best practices, monitoring patterns, capacity planning duplicated across scenarios

---

## Implementation Strategy

### 1. Simplify PromptProvider Implementation

**Current code pattern**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("troubleshooting") => prompt_troubleshooting(),
        Some("monitoring") => prompt_monitoring(),
        _ => prompt_comprehensive(),
    }
}
```

**New code pattern**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

**Rationale**: Eliminate dead code paths. Since only one scenario exists, remove conditional routing entirely. This makes it clear that `db_pool_stats` is a basic usage tool.

### 2. Update prompt_arguments()

**Current**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, troubleshooting, monitoring)".to_string()),
            required: Some(false),
        }
    ]
}
```

**New** (Option A - Remove parameter):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![]  // No arguments - always returns basic scenario
}
```

**New** (Option B - Keep parameter but document default):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario parameter (currently only 'basic' supported)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Recommendation**: Use **Option A** (remove parameter) for truest code simplification and to prevent confusion about non-existent scenarios.

### 3. Optimize prompt_basic()

The basic prompt currently spans ~150 lines with these sections:

1. **Tool description** (1 line)
2. **Basic usage example** (8 lines)
3. **Response structure** (8 lines)
4. **Key metrics explained** (25 lines)
5. **Interpreting results** (30 lines) - 3 pool states with detailed explanations
6. **Quick health check formula** (6 lines)
7. **Common patterns** (12 lines)
8. **Parameters** (5 lines)
9. **When to use** (8 lines)

**Compression strategy**:
- **Keep**: Sections 1-4 (core content is essential, 42 lines total)
- **Compress**: Section 5 - reduce from 30 lines to 10 lines by using a single table-like format for pool states
- **Keep**: Sections 6-9 (actionable content, 31 lines total)
- **Remove**: Decorative headers, excessive blank lines between sections, repeated metric definitions

**Target structure**:
```
BASIC USAGE (8 lines)
RESPONSE (8 lines)
KEY METRICS (20 lines - compressed)
POOL STATE REFERENCE (8 lines - quick table format)
QUICK HEALTH CHECK (6 lines)
COMMON PATTERNS (12 lines)
PARAMETERS (5 lines)
WHEN TO USE (8 lines)
────────────────────────
Total: ~75 lines
```

### 4. Delete Functions

Remove entirely:
- `prompt_troubleshooting()` (200 lines) - Scenario 2
- `prompt_monitoring()` (200 lines) - Scenario 3
- `prompt_comprehensive()` (160 lines) - Scenario 4

These functions are completely replaced by the optimized `prompt_basic()`.

---

## Exact Code Changes Required

### File: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/database/pool_stats/prompts.rs`

**Step 1: Modify PromptProvider impl block**

Replace the entire `generate_prompts` function:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

Replace the entire `prompt_arguments` function:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![]
}
```

**Step 2: Keep prompt_basic() but optimize content**

The function signature remains unchanged:
```rust
fn prompt_basic() -> Vec<PromptMessage> {
```

But the assistant response text needs compression:
- Remove section headers (====, ----, ····)
- Compress "INTERPRETING RESULTS" from 3 full examples to 1 example + reference format
- Consolidate repeated explanations
- Keep JSON examples clear and minimal
- Inline "PARAMETERS" content into usage examples
- Compress "WHEN TO USE" to 3-4 lines

Target: ~75 lines for prompt_basic content.

**Step 3: Delete functions**

Remove these four function definitions entirely:
- `prompt_troubleshooting() -> Vec<PromptMessage>` (delete all 200 lines)
- `prompt_monitoring() -> Vec<PromptMessage>` (delete all 200 lines)
- `prompt_comprehensive() -> Vec<PromptMessage>` (delete all 160 lines)

**Step 4: Update file documentation comment** (optional)

Change top-level comment from:
```rust
//! Prompt messages for db_pool_stats tool
```

To:
```rust
//! Basic prompt message for db_pool_stats tool
//!
//! Provides single scenario covering core usage patterns,
//! pool health assessment, and troubleshooting basics.
```

---

## Concrete Content Example

Here's what the optimized prompt_basic() response should contain (compressed format):

```
How do I check database connection pool statistics?

Use db_pool_stats to monitor real-time connection pool health.

BASIC USAGE:
db_pool_stats({"connection": "main"})

RESPONSE:
{
  "pool_size": 10,
  "active_connections": 3,
  "idle_connections": 7,
  "waiting_requests": 0,
  "max_lifetime_ms": 1800000,
  "idle_timeout_ms": 600000
}

KEY METRICS:
- pool_size: Maximum connections (10 connections total)
- active_connections: Currently executing queries (3 active)
- idle_connections: Ready for immediate use (7 idle)
- waiting_requests: Queries waiting for connection (0 waiting)
- max_lifetime_ms: Max age before connection refresh (30 minutes)
- idle_timeout_ms: How long idle connections stay alive (10 minutes)

QUICK REFERENCE - Pool Health:
HEALTHY: idle > 0, waiting = 0, utilization < 80%
BUSY: idle = 1-2, waiting = 0, needs monitoring
EXHAUSTED: idle = 0, waiting > 0, increase pool_size immediately

COMMON PATTERNS:
// Before heavy operation - verify idle connections exist
db_pool_stats({"connection": "main"})  // Check idle_connections > 0

// Monitor during load - watch for waiting requests
db_pool_stats({"connection": "main"})  // Check waiting_requests = 0

// Diagnose slow queries - check active connection count
db_pool_stats({"connection": "main"})  // Sustained high active = slow queries

PARAMETERS:
- connection: Named connection to check (required)
  Examples: "main", "replica", "analytics"

WHEN TO USE:
- Regular health checks
- Before/after major operations
- Investigating performance issues
- Capacity planning
```

This structure is ~95-110 lines when formatted as PromptMessage content.

---

## Success Criteria

**Line count validation**:
- ✓ Total file size: 90-110 lines (down from 710)
- ✓ Header/impl: ~30 lines
- ✓ prompt_basic content: ~75 lines
- ✓ No other prompt functions

**Functionality validation**:
- ✓ PromptProvider trait still implemented
- ✓ generate_prompts() always returns basic scenario
- ✓ prompt_arguments() returns empty vec or simple parameter
- ✓ Tool compiles without warnings
- ✓ Tool behavior unchanged from user perspective (basic scenario was default)

**Code quality validation**:
- ✓ No dead code paths (match statements removed)
- ✓ No redundant sections or decorative headers
- ✓ Clear, actionable content in single scenario
- ✓ Essential information preserved (metrics, examples, usage)

---

## Related Documentation

- **Reference pattern**: See `PRECURSOR_01_memory_list_libraries.md` for Complexity 1 single-scenario structure
- **Similar trimming**: Other db tools in `packages/kodegen-mcp-schema/src/database/` may follow this same pattern
- **Prompt provider trait**: `packages/kodegen-mcp-schema/src/tool/mod.rs` defines PromptProvider interface

---

## No Additional Requirements

- ✗ No unit tests needed (prompt generation is deterministic)
- ✗ No benchmarks needed (file size reduction only)
- ✗ No documentation changes needed (this task file is the guide)
- ✗ No tests to verify behavior (behavior unchanged, only code removed)
