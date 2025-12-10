# TASK 012: Trim inspect_usage_stats

**Tool**: `inspect_usage_stats`
**Complexity**: 1 (Trivial)
**Current size**: 181 lines
**Target size**: 90-110 lines (1 scenario)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/introspection/usage_stats/prompts.rs`

---

## Current State Analysis

### File Structure (181 lines total)

The file contains the `InspectUsageStatsPrompts` struct implementing the `PromptProvider` trait.

**Current routing** (lines 13-20):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.focus_area.as_deref() {
        Some("performance") => prompt_performance_overview(),
        Some("failures") => prompt_failure_investigation(),
        Some("optimization") => prompt_optimization_insights(),
        Some("overview") => prompt_session_overview(),
        _ => prompt_health_check(),
    }
}
```

**Current scenarios** (5 total):
1. `prompt_performance_overview()` - lines 45-70 (26 lines)
2. `prompt_failure_investigation()` - lines 72-97 (26 lines)
3. `prompt_optimization_insights()` - lines 99-124 (26 lines)
4. `prompt_session_overview()` - lines 126-151 (26 lines)
5. `prompt_health_check()` - lines 153-181 (29 lines) **← KEEP THIS ONE**

Each scenario is identical in structure: user question + assistant response with field explanations.

**Current prompt_arguments** (lines 22-36):
- `focus_area`: Optional string describing focus area
- `show_examples`: Optional boolean flag
- Both are optional parameters

---

## Trimming Instructions

### STEP 1: Select and Enhance Single Scenario

**Decision**: Keep `prompt_health_check()` as the sole scenario because:
- It's the default scenario (catches wildcard match)
- Provides essential metrics for the most common use case
- Addresses the core purpose: monitoring session health
- Practical and widely applicable

**Current prompt_health_check content** (lines 153-181):
```rust
fn prompt_health_check() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Is this session healthy? Are tools succeeding or failing?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "```typescript\n\
                 // Quick health check\n\
                 {}\n\n\
                 // Returns: InspectUsageOutput\n\
                 // - success_rate: Key health metric (above 90% = healthy)\n\
                 // - successful_calls: Number of successful operations\n\
                 // - failed_calls: Number of failures (investigate if high)\n\
                 // - total_calls: Overall activity level\n\
                 // - tool_usage: Identify which tools are failing\n\
                 //   - Tools with many calls but contributing to failed_calls need attention\n\
                 //   - Check avg_duration_ms for performance issues\n\
                 ```"
            ),
        },
    ]
}
```

**Expand this to include**:
1. Tool description section (what inspect_usage_stats does)
2. Basic usage example
3. Response structure explanation
4. When to use (3-4 practical bullets)
5. Common pattern (workflow example)
6. Quick reference

Target: 95-105 lines of prompt code + 33 lines boilerplate = 128-138 lines total. Trim non-essential content to reach 90-110 total.

---

### STEP 2: Delete All Other Scenarios

**Delete entirely** (lines to remove):
- `prompt_performance_overview()` - lines 45-70 (DELETE)
- `prompt_failure_investigation()` - lines 72-97 (DELETE)
- `prompt_optimization_insights()` - lines 99-124 (DELETE)
- `prompt_session_overview()` - lines 126-151 (DELETE)

This removes approximately 104 lines.

---

### STEP 3: Simplify Routing

**Before** (lines 13-20):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.focus_area.as_deref() {
        Some("performance") => prompt_performance_overview(),
        Some("failures") => prompt_failure_investigation(),
        Some("optimization") => prompt_optimization_insights(),
        Some("overview") => prompt_session_overview(),
        _ => prompt_health_check(),
    }
}
```

**After** (replace with):
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_health_check()
}
```

This eliminates the match statement entirely and makes the parameter unused (prefix with underscore).

---

### STEP 4: Simplify prompt_arguments

The `prompt_arguments()` method (lines 22-36) defines optional arguments. For a single scenario:

**Before**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "focus_area".to_string(),
            title: None,
            description: Some("Focus area: performance, failures, optimization, overview, health_check".to_string()),
            required: Some(false),
        },
        PromptArgument {
            name: "show_examples".to_string(),
            title: None,
            description: Some("Show detailed usage examples".to_string()),
            required: Some(false),
        },
    ]
}
```

**After** (choose one):

Option A - Return empty vec (no parameters):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![]
}
```

Option B - Keep one optional parameter for compatibility:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "show_examples".to_string(),
            title: None,
            description: Some("Show detailed usage examples".to_string()),
            required: Some(false),
        },
    ]
}
```

**Recommendation**: Use Option A (empty vec) since there's only one scenario.

---

### STEP 5: Expand prompt_health_check with Required Content

The expanded `prompt_health_check()` function must include:

1. **Tool Description** (8-10 lines)
   - Purpose: "Inspects session usage statistics"
   - Takes no required parameters
   - Returns InspectUsageOutput with metrics

2. **Basic Usage** (8-10 lines)
   - Show: `inspect_usage_stats({})`
   - Show response structure

3. **Response Structure** (12-15 lines)
   - Key fields: success_rate, successful_calls, failed_calls, total_calls, session_duration_ms, tool_usage array
   - Brief explanation of each field

4. **When to Use** (15-20 lines)
   - Check session health after running multiple tools
   - Identify which tools are problematic
   - Monitor success rates
   - Track cumulative performance

5. **Common Pattern** (25-30 lines)
   - Workflow example:
     ```
     1. Run multiple tools in a session
     2. Call inspect_usage_stats({})
     3. Check success_rate (healthy if > 90%)
     4. If low success_rate, inspect tool_usage array
     5. Identify tools with high failures
     6. Use inspect_tool_calls for detailed failure info
     ```

6. **Quick Reference** (10-12 lines)
   - Command: `inspect_usage_stats({})`
   - No parameters required
   - Related tools: inspect_tool_calls, get_events

**Content constraint**: The entire prompt message content must be under 95 lines (roughly 70-80 lines in the text string, considering escape sequences).

---

## Complete Implementation Path

### File Structure After Trim

```
// Lines 1-5: File header comment
//! Prompt messages for inspect_usage_stats tool

// Lines 6-10: Imports
use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::InspectUsageStatsPromptArgs;

// Lines 11-15: Struct definition
/// Prompt provider for inspect_usage_stats tool
pub struct InspectUsageStatsPrompts;

// Lines 16-40: PromptProvider implementation
impl PromptProvider for InspectUsageStatsPrompts {
    type PromptArgs = InspectUsageStatsPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_health_check()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

// Lines 41-108: SINGLE scenario function (expanded prompt_health_check)
fn prompt_health_check() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Is this session healthy? Show me the usage statistics."
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "// [Expanded content: 65-75 lines]
                 // Tool description
                 // Basic usage
                 // Response structure
                 // When to use (bullets)
                 // Common pattern
                 // Quick reference"
            ),
        },
    ]
}
```

Final line count target: 90-110 lines total

---

## Success Criteria

- ✓ **File line count**: 90-110 lines (verify with `wc -l prompts.rs`)
- ✓ **Scenario count**: Only ONE scenario function exists (`prompt_health_check`)
- ✓ **Deleted functions**: All four other scenario functions removed entirely
- ✓ **Routing simplified**: `generate_prompts()` calls `prompt_health_check()` directly with no match statement
- ✓ **Tool description**: Clear 1-sentence description of what inspect_usage_stats does
- ✓ **Usage example**: Shows `inspect_usage_stats({})` and basic response
- ✓ **Response structure**: Explains success_rate, successful_calls, failed_calls, total_calls, tool_usage
- ✓ **When to use**: 3-4 practical bullet points with brief examples
- ✓ **Common pattern**: One complete workflow showing health check investigation
- ✓ **Quick reference**: Command signature and related tools
- ✓ **No duplication**: Each concept explained exactly once
- ✓ **No decorative headers**: No `═══` or `---` lines in the code
- ✓ **No comprehensive scenario**: Only basic/health-check scenario remains

---

## Validation Checklist

After completing changes:

1. **Line count validation**:
   ```bash
   wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/introspection/inspect_usage_stats/prompts.rs
   # Should output: 90-110 (plus a few lines)
   ```

2. **Function validation**:
   - Search for `fn prompt_` in file - should find only `prompt_health_check()`
   - Search for `Some("` in generate_prompts() - should find nothing (no match arms)
   - Verify no compilation errors with `cargo check` in the package

3. **Content validation**:
   - Expand content in editor and read through in ~60 seconds
   - Verify can answer: "What does inspect_usage_stats do?" from the prompt alone
   - Verify can answer: "When should I use it?" from the bullets
   - Verify can answer: "How do I call it?" from the usage example

4. **Structure validation**:
   - Exactly 2 PromptMessage entries (User + Assistant)
   - Assistant message uses text() content
   - No decorative markdown headers in output
   - Escaping is correct for multi-line strings

---

## Related Files

- **Source file**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/introspection/inspect_usage_stats/prompts.rs`
- **Prompt args**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/introspection/inspect_usage_stats/prompt_args.rs` (no changes needed)
- **Schema**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/introspection/inspect_usage_stats/schema.rs` (no changes needed)
- **Reference template**: `/Volumes/samsung_t9/kodegen-workspace/task/PRECURSOR_01_memory_list_libraries.md`

---

## Summary of Changes

| Aspect | Before | After |
|--------|--------|-------|
| Total lines | 181 | 90-110 |
| Scenarios | 5 | 1 |
| Routing logic | 8-line match statement | Direct function call |
| Prompt arguments | 2 arguments | 0 arguments |
| Deleted code | - | ~110 lines (4 scenario functions) |
| Key scenario | Multiple options | prompt_health_check |
| Content density | Distributed across 5 prompts | Consolidated in 1 prompt |
