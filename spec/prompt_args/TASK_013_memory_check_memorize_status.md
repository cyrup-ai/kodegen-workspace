# TASK 013: Trim memory_check_memorize_status

**Tool**: `memory_check_memorize_status`
**Complexity**: 1 (Trivial)
**Current size**: 243 lines
**Target size**: 90-110 lines (1 scenario)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/memory/check_memorize_status/prompts.rs`

---

## Executive Summary

The `prompts.rs` file currently contains one scenario function (`prompt_comprehensive()`) that is 214 lines of excessive documentation. It uses decorative ASCII headers, redundant explanations, too many workflow patterns, and entire sections (best practices, error handling) unnecessary for a Complexity 1 tool. This task reduces it to a single concise `prompt_basic()` function of approximately 100 lines.

---

## Current File Analysis

**File Location**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/memory/check_memorize_status/prompts.rs`

**Current Metrics**:
- Total lines: 243
- Scenario functions: 1 (prompt_comprehensive)
- File structure:
  - Lines 1-23: File header, imports, CheckMemorizeStatusPrompts struct
  - Lines 24-29: PromptProvider trait implementation with routing
  - Lines 30-243: Single prompt_comprehensive() function (214 lines)

**Current prompt_comprehensive() Structure**:
- Decorative headers: ~40 lines of "=====" separators
- Tool description & when to use: ~20 lines
- Basic usage examples: ~30 lines
- Status values section: ~20 lines
- Progress stages list: ~8 lines
- Workflow patterns: 4 patterns with ~70 lines total
  - PATTERN 1: FIRE AND FORGET (~15 lines)
  - PATTERN 2: VERIFY BEFORE RECALL (~20 lines)
  - PATTERN 3: POLL FOR LARGE CONTENT (~18 lines)
  - PATTERN 4: BATCH VERIFICATION (~17 lines)
- Error handling section: ~30 lines
- Response fields section: ~15 lines
- Best practices section: ~12 lines
- Quick reference: ~3 lines

---

## Reference Implementation Pattern

Follow the Complexity 1 template from `PRECURSOR_01_memory_list_libraries.md`. The target structure is:

```
File header + imports (23 lines unchanged)
↓
CheckMemorizeStatusPrompts struct with modified generate_prompts (9 lines)
↓
prompt_basic() function containing:
  - User question (1-2 lines)
  - Assistant answer (85-95 lines):
    • Tool description (10 lines)
    • Basic usage (10 lines)
    • Response structure (10 lines)
    • When to use (15 lines)
    • Common pattern (25 lines)
    • Quick reference (10 lines)
```

---

## Core Implementation Instructions

### Step 1: Update the Routing (Lines 13-23)

The `CheckMemorizeStatusPrompts` struct and `PromptProvider` implementation must call `prompt_basic()` instead of `prompt_comprehensive()`.

**Change Line 18 from:**
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_comprehensive()
}
```

**Change Line 18 to:**
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

This is a simple single-line change in the function body.

---

### Step 2: Delete prompt_comprehensive() Entirely

**Delete lines 30-243** (the entire `prompt_comprehensive()` function).

This removes all decorative headers, extended examples, best practices, and verbose explanations.

---

### Step 3: Create prompt_basic() Function

Add a new `prompt_basic()` function after line 29 (after the comment separator). The function must contain exactly 2 PromptMessages: one User query and one Assistant response.

**Structure Overview**:
```rust
/// Basic usage guide for checking memorization status
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I check if a memorization operation has completed?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The memory_check_memorize_status tool checks the progress of asynchronous memorization operations.\n\n\
                 DESCRIPTION\n\n\
                 [10 lines of tool description]\n\n\
                 BASIC USAGE\n\n\
                 [10 lines with parameter, example, response sample]\n\n\
                 RESPONSE STRUCTURE\n\n\
                 [10 lines explaining fields]\n\n\
                 WHEN TO USE\n\n\
                 [15 lines with 4 bullet points]\n\n\
                 COMMON PATTERN\n\n\
                 [25 lines with one complete workflow example]\n\n\
                 QUICK REFERENCE\n\n\
                 [10 lines]\n\
                 ",
            ),
        },
    ]
}
```

---

### Step 4: Content Specifications for prompt_basic()

The Assistant response must contain these sections in order. Line counts are approximate and flexible within +/- 2 lines:

#### Section 1: Tool Description (10 lines)
**Content**: 
- What it does: Checks progress of asynchronous memorization operations
- What parameter is required: session_id (returned from memory_memorize)
- Three possible status values: COMPLETED, IN_PROGRESS, FAILED
- When to use: After memory_memorize call, before memory_recall, for large content operations

**Keep from original**: Yes, condensed from lines 50-65

#### Section 2: Basic Usage (10 lines)
**Content**:
- Show the call: `memory_check_memorize_status({"session_id": "mem_abc123"})`
- Show one COMPLETED response example with JSON (5 lines)
- Show one IN_PROGRESS response example with JSON (3 lines)

**Keep from original**: Yes, from lines 68-100, but remove FAILED example

#### Section 3: Response Structure (10 lines)
**Content**:
- Explain each response field: session_id, status, memory_id, library, progress, error, runtime_ms
- Explain the progress object: stage, percent_complete, files_loaded, total_size_bytes
- Note which fields appear in which scenarios

**Keep from original**: Yes, from lines 123-165, but condense heavily

#### Section 4: When To Use (15 lines)
**Content**: 4 bullet points with brief explanations
- To verify completion before recalling
- To monitor large content being embedded
- To debug failed operations
- To track batch operations

**Keep from original**: Yes, from lines 53-66, reformatted as bullets

#### Section 5: Common Pattern (25 lines)
**Content**: ONE complete workflow example showing:
1. Call memory_memorize and capture session_id
2. Call memory_check_memorize_status with that session_id
3. Check if status is COMPLETED
4. If yes, proceed with memory_recall
5. If no/failed, handle appropriately

**Select from original**: Use PATTERN 2 (VERIFY BEFORE RECALL) from lines 168-191, but reduce to 25 lines max

#### Section 6: Quick Reference (10 lines)
**Content**:
- Command syntax: `memory_check_memorize_status({"session_id": "..."})`
- Status values: COMPLETED, IN_PROGRESS, FAILED
- Progress stages: validating, generating_embedding, storing, indexing, embedding_complete
- Key fact: Most memorizations complete in <100ms

**Keep from original**: Yes, from lines 227-240

---

### Step 5: Critical Deletions

You MUST delete these sections entirely from the original:

1. **All decorative header lines** (lines with "=====" or similar):
   - Line 45: WHEN TO USE header
   - Line 49: closing separator
   - Line 52: BASIC USAGE header
   - Line 102: closing separator
   - Line 104: STATUS VALUES header
   - Line 125: closing separator
   - Line 150: PROGRESS STAGES header
   - Line 161: closing separator
   - Line 163: WORKFLOW PATTERNS header
   - Line 211: closing separator
   - Line 213: ERROR HANDLING header
   - Line 225: closing separator
   - Line 227: RESPONSE FIELDS header
   - Line 241: closing separator
   - Line 243: BEST PRACTICES header
   - Line 253: closing separator
   - Line 255: QUICK REFERENCE header
   - Line 261: closing separator

2. **Entire ERROR HANDLING section** (lines 213-225):
   - Remove COMMON ERRORS subsection
   - Remove RETRY STRATEGY subsection
   - Keep only error information if absolutely critical

3. **Entire BEST PRACTICES section** (lines 243-253):
   - All 6 best practice items
   - Rationale: Too verbose for a Complexity 1 tool; patterns are in the workflow section

4. **PATTERN 1, 3, and 4** from workflow patterns (keep only PATTERN 2):
   - PATTERN 1: FIRE AND FORGET (delete entirely)
   - PATTERN 3: POLL FOR LARGE CONTENT (delete entirely)
   - PATTERN 4: BATCH VERIFICATION (delete entirely)
   - Rationale: One pattern is sufficient for basic usage

5. **Response fields section extended explanation** (reduce from 15 to 5 lines):
   - Keep field names and essential descriptions
   - Remove nested explanations

---

## Validation Steps

After completing the edits, perform these validations:

### Line Count Validation
```bash
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/memory/check_memorize_status/prompts.rs
# Expected output: 90-110 lines
```

### Function Existence Validation
```bash
grep -n "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/memory/check_memorize_status/prompts.rs
# Expected output: Only one line matching "prompt_basic"
# Should NOT match "prompt_comprehensive"
```

### Decorative Header Validation
```bash
grep "=====" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/memory/check_memorize_status/prompts.rs
# Expected output: No matches (empty result)
```

### Routing Validation
```bash
grep -A2 "fn generate_prompts" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/memory/check_memorize_status/prompts.rs
# Expected output: Contains "prompt_basic()" not "prompt_comprehensive()"
```

### Compilation Check
```bash
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
cargo check
# Must compile without errors or warnings
```

---

## Success Criteria

The task is complete when ALL of these conditions are met:

- [ ] File has 90-110 lines total (verified with `wc -l`)
- [ ] Only ONE scenario function exists: `prompt_basic()`
- [ ] Function `prompt_comprehensive()` is completely deleted
- [ ] `generate_prompts()` routing calls `prompt_basic()` not `prompt_comprehensive()`
- [ ] NO decorative header lines (no "=====" patterns)
- [ ] NO "BEST PRACTICES" section
- [ ] NO "ERROR HANDLING" section
- [ ] Only ONE workflow pattern example (PATTERN 2: VERIFY BEFORE RECALL)
- [ ] Tool description is clear and concise (one paragraph)
- [ ] One basic usage example with 2 response examples
- [ ] One response structure explanation
- [ ] 4 bullet points for "when to use"
- [ ] One complete workflow pattern example (25 lines max)
- [ ] Quick reference section present
- [ ] Code compiles: `cargo check` passes with no errors
- [ ] Code passes linting: `cargo clippy` shows no warnings

---

## Implementation Timeline

1. **Read and understand** (5 min): Re-read the current prompts.rs file to locate exact lines
2. **Modify routing** (2 min): Change generate_prompts() to call prompt_basic()
3. **Delete old function** (2 min): Remove prompt_comprehensive() entirely
4. **Write new function** (15 min): Create prompt_basic() with 6 content sections
5. **Validate** (5 min): Run all validation checks and verify success criteria
6. **Compile and test** (3 min): Ensure cargo check and cargo clippy pass

**Total estimated time**: 30 minutes

---

## Common Pitfalls

1. **Forgetting to update routing**: If you delete prompt_comprehensive() but forget to change the generate_prompts() call, compilation will fail.

2. **Adding back decorative headers**: The new prompt_basic() must NOT include "=====" separators. Use plain text section titles.

3. **Keeping all workflow patterns**: You must delete PATTERN 1, 3, and 4. Keep ONLY PATTERN 2 (VERIFY BEFORE RECALL).

4. **Line count creep**: Watch for accidental additions. If content exceeds 110 lines, remove less critical details from response explanations.

5. **Breaking the JSON string**: The assistant response content is one long string with `\n\` line continuations. Ensure all backslashes and quote escaping are correct.

6. **Missing response examples**: The basic usage section MUST include at least 2 sample JSON responses (COMPLETED and IN_PROGRESS).

---

## Related Files

- Template reference: `/Volumes/samsung_t9/kodegen-workspace/task/PRECURSOR_01_memory_list_libraries.md`
- Tool definition: Check `memory_check_memorize_status` definition in the memory tool module
- Prompt args: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/memory/check_memorize_status/prompt_args.rs`
