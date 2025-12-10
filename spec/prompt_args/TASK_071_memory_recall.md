# TASK 071: Trim memory_recall

**Tool**: `memory_recall`
**Complexity**: 2 (Simple)
**Current size**: 713 lines (5 scenarios)
**Target size**: 200 lines (1-2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/memory/recall/prompts.rs`

---

## Current State Analysis

### File Structure (713 lines total)
- Lines 1-13: Module imports and MemoryRecallPrompts struct definition
- Lines 15-31: PromptProvider implementation with match statement routing
- Lines 40-141: `prompt_basic()` - 102 lines - Basic retrieval + semantic search fundamentals
- Lines 143-288: `prompt_semantic()` - 146 lines - How semantic search works + similarity scoring
- Lines 290-454: `prompt_workflows()` - 165 lines - 10 workflow scenarios (DELETE)
- Lines 456-631: `prompt_advanced()` - 176 lines - 10 advanced techniques (DELETE)
- Lines 633-713: `prompt_comprehensive()` - 81 lines - Comprehensive guide combining all topics (DELETE)

### Current Routing (Lines 20-26)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("semantic") => prompt_semantic(),
        Some("workflows") => prompt_workflows(),
        Some("advanced") => prompt_advanced(),
        _ => prompt_comprehensive(),
    }
}
```

### Scenarios to Keep
1. **prompt_basic()** (102 lines) - KEEP FULL
   - Covers basic memory retrieval using semantic search
   - Includes 4 example use cases and best practices
   - Aligns with "Basic scenario (80-120 lines): semantic search and recall"

2. **prompt_semantic()** (146 lines) - TRIM TO ~80 LINES
   - Currently covers semantic matching examples, similarity scoring, and query optimization
   - Aligns with "Optional relevance scenario (70-100 lines) for scoring/limits"
   - Must remove sections: query variations (10 lines), understanding results detail (15 lines), semantic concepts detailed (10 lines)
   - Keep: embedding generation, query embedding, similarity matching, similarity scores table, optimizing queries, query strategies

### Scenarios to Delete
- `prompt_workflows()` - Use-case scenario, 165 lines, DELETE ENTIRELY
- `prompt_advanced()` - Use-case scenario, 176 lines, DELETE ENTIRELY
- `prompt_comprehensive()` - Comprehensive scenario, DELETE ENTIRELY per success criteria "No comprehensive"

---

## Reference

See **PRECURSOR_02_fs_read_file.md** for Complexity 2 template.

---

## Instructions

### Step 1: Trim prompt_semantic() to ~80 lines
The current prompt_semantic() spans lines 143-288 (146 lines). Reduce to ~80 lines by:

**KEEP these sections (highest value for scoring/relevance):**
- Lines 147-149: User question about semantic search
- Lines 150-164: Opening explanation of how it works (embedding generation, query embedding, similarity matching)
- Lines 166-184: Semantic matching examples showing stored memory and query results with similarity scores
- Lines 186-197: Similarity scores table (0.90-1.00 = excellent through below 0.60 = poor)
- Lines 199-214: "OPTIMIZING QUERIES" section showing vague vs specific vs natural language
- Lines 216-244: "QUERY STRATEGIES" section with 4 strategies

**DELETE these sections (lower value, covered in basic):**
- Lines 245-253: "UNDERSTANDING RESULTS" detail example
- Lines 255-265: "WHY SEMANTIC > KEYWORD" comparative section (redundant with strategies)
- Lines 267-276: "SEMANTIC CONCEPTS" section with relationships explanation (too detailed)

**Implementation approach:**
In the PromptMessage Assistant content, remove the "WHY SEMANTIC > KEYWORD" section entirely. This frees up ~20 lines. Remove the detailed "UNDERSTANDING RESULTS" example, freeing ~15 lines. Keep the core concepts: how embeddings work, similarity matching, similarity scores table, query optimization, and the 4 strategies. This brings the function to approximately 80 lines.

### Step 2: Delete Functions and Clean Decorative Headers
**Delete the following function definitions entirely:**
- `fn prompt_workflows()` (lines 290-454) - DELETE ALL 165 LINES
- `fn prompt_advanced()` (lines 456-631) - DELETE ALL 176 LINES  
- `fn prompt_comprehensive()` (lines 633-713) - DELETE ALL 81 LINES

This removes 422 lines of content.

**Remove decorative headers:**
- Remove line 35: `// ============================================================================`
- Remove line 36: `// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE MEMORY_RECALL`
- Remove line 37: `// ============================================================================`

These headers are redundant with only 2 scenarios remaining.

### Step 3: Update PromptProvider Implementation Routing
The match statement in `generate_prompts()` must be updated to handle only 2 scenarios.

**BEFORE (lines 20-26):**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("semantic") => prompt_semantic(),
        Some("workflows") => prompt_workflows(),
        Some("advanced") => prompt_advanced(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("semantic") => prompt_semantic(),
        _ => prompt_basic(),
    }
}
```

**Changes:**
- Remove line routing to `prompt_workflows()`
- Remove line routing to `prompt_advanced()`
- Replace default `_ => prompt_comprehensive()` with `_ => prompt_basic()`
- Keep only "basic" and "semantic" scenarios

### Step 4: Update PromptArgument Description
The `prompt_arguments()` description must reflect only 2 scenarios available.

**BEFORE (line 30):**
```rust
description: Some("Scenario to show (basic, semantic, workflows, advanced)".to_string()),
```

**AFTER:**
```rust
description: Some("Scenario to show (basic, semantic)".to_string()),
```

---

## Success Criteria

The task IS COMPLETE when:
- ✓ Total file size is 170-220 lines
- ✓ Exactly 1-2 scenario functions exist (prompt_basic and prompt_semantic)
- ✓ NO prompt_comprehensive function exists
- ✓ NO prompt_workflows function exists  
- ✓ NO prompt_advanced function exists
- ✓ Match statement in generate_prompts() routes only "basic" and "semantic"
- ✓ Default match case falls back to prompt_basic (not comprehensive)
- ✓ prompt_arguments() description shows only "basic, semantic"
- ✓ Decorative header comment lines are removed
- ✓ File compiles with `cargo check` in kodegen-mcp-schema package

---

## Code Patterns: Before and After

### Before (Current - 713 lines, 5 scenarios)
```
File structure:
├── Imports (13 lines)
├── MemoryRecallPrompts struct + PromptProvider impl (17 lines)
├── prompt_basic() (102 lines)
├── prompt_semantic() (146 lines)
├── prompt_workflows() (165 lines)  ← DELETE
├── prompt_advanced() (176 lines)   ← DELETE
└── prompt_comprehensive() (81 lines) ← DELETE
```

### After (Target - 195-205 lines, 2 scenarios)
```
File structure:
├── Imports (13 lines)
├── MemoryRecallPrompts struct + PromptProvider impl (17 lines)
├── prompt_basic() (102 lines)
├── prompt_semantic() (80 lines, trimmed)
└── [END OF FILE]

Total: ~212 lines (within 170-220 target)
```

### Routing Changes Pattern

**Before:**
- 5 match arms for 5 scenarios
- Default fallback to comprehensive view (all features)
- Arguments describe 4 scenarios

**After:**
- 2 match arms for 2 scenarios
- Default fallback to basic (fundamental usage)
- Arguments describe 2 scenarios
- Cleaner, simpler routing logic

---

## Verification Steps

After making all changes, verify by:

1. **Line count**: Run `wc -l packages/kodegen-mcp-schema/src/memory/recall/prompts.rs` → expect 195-210 lines
2. **Function count**: Search for `fn prompt_` in file → expect exactly 2 occurrences (basic, semantic)
3. **No references to deleted**: Search for "workflows", "advanced", "comprehensive" → expect 0 results in function definitions
4. **Compile check**: Run `cargo check` in packages/kodegen-mcp-schema → must pass without errors
5. **Manual review**: Verify prompt_arguments() description matches the 2 remaining scenarios
