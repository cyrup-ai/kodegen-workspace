# TASK 027: Trim git_fetch Prompts to 2 Scenarios

**Tool**: `git_fetch`
**Complexity**: 2 (Simple)
**Current size**: 677 lines
**Target size**: 170-220 lines (exactly 2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/fetch/prompts.rs`

---

## Current State Analysis

### File Structure (677 lines total)
The file contains 5 scenario functions with the following breakdown:

| Component | Lines | Action |
|-----------|-------|--------|
| Module imports & FetchPrompts struct | 1-43 | KEEP (modify match statement) |
| Decorative header comment | 44-80 | DELETE |
| `prompt_basic()` | 82-113 (32 lines) | KEEP + EXPAND to ~100 lines |
| `prompt_remotes()` | 115-168 (54 lines) | DELETE (use-case scenario) |
| `prompt_prune()` | 170-252 (83 lines) | KEEP as-is |
| `prompt_workflows()` | 254-352 (99 lines) | DELETE (comprehensive workflows scenario) |
| `prompt_comprehensive()` | 354-677 (324 lines) | DELETE (explicitly excluded) |

### Scenarios to Keep (2 total)
1. **`prompt_basic()`** (currently 32 lines, expand to ~100): Teaches basic git_fetch usage
2. **`prompt_prune()`** (83 lines, fits 70-100 target): Teaches cleanup of stale remote branches

### Scenarios to Delete (3 total)
1. **`prompt_remotes()`**: Specialized use-case for multiple remotes
2. **`prompt_workflows()`**: Comprehensive workflow patterns
3. **`prompt_comprehensive()`**: Complete feature guide (explicitly excluded by task)

---

## Implementation Steps

### Step 1: Update PromptProvider match statement
**Location**: Lines 32-35 in the impl PromptProvider block

**BEFORE**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("remotes") => prompt_remotes(),
        Some("prune") => prompt_prune(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("prune") => prompt_prune(),
        _ => prompt_basic(),
    }
}
```

**Execution**: Replace the entire match expression (5 arms become 3 arms, default returns `prompt_basic()` instead of `prompt_comprehensive()`)

---

### Step 2: Update prompt_arguments description
**Location**: Line 29 in prompt_arguments() function

**BEFORE**:
```rust
description: Some("Scenario to show (basic, remotes, prune, workflows)".to_string()),
```

**AFTER**:
```rust
description: Some("Scenario to show (basic, prune)".to_string()),
```

**Execution**: Replace only the scenario list string, keep the rest of the PromptArgument struct unchanged

---

### Step 3: Delete decorative header comment section
**Location**: Lines 44-80 (entire block of comment lines)

**EXACT CONTENT TO DELETE**:
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT FETCH
// ============================================================================
```

This is the decorative section header between the prompt_arguments() function and prompt_basic() function.

**Execution**: Delete lines 44-80 completely (the multi-line comment with repeating equals signs)

---

### Step 4: Expand prompt_basic() content
**Location**: Lines 82-113 (the entire prompt_basic() function definition)

**Current length**: 32 lines
**Target length**: ~100 lines (to reach ~200 total with prune scenario)

**Strategy**: Add more detailed usage patterns and parameter documentation to the assistant response content. Do NOT change the function signature, only expand the text content within `PromptMessageContent::text(...)`.

**Content to add** (insert into the existing text content):
- 3-4 additional basic examples (e.g., fetch before merge, fetch specific branch)
- Expanded parameter documentation with more detail per parameter
- Additional "why use fetch" explanations
- More workflow examples (check updates, before rebasing, daily sync)

**Execution**: Replace the entire `prompt_basic()` function (lines 82-113) with an expanded version that reaches ~100 lines total. Preserve the function structure but expand the text within the message content.

---

### Step 5: Delete prompt_remotes() function entirely
**Location**: Lines 115-168 (entire function definition)

**EXACT CONTENT TO DELETE**: All 54 lines of the `fn prompt_remotes() -> Vec<PromptMessage>` function

This function describes fetching from specific remotes and fork workflows - it's a use-case scenario that should be removed.

**Execution**: Delete lines 115-168 completely, including the function signature and all content

---

### Step 6: Delete prompt_workflows() function entirely
**Location**: Lines 254-352 (entire function definition)

**EXACT CONTENT TO DELETE**: All 99 lines of the `fn prompt_workflows() -> Vec<PromptMessage>` function

This function provides comprehensive workflow patterns (CI/CD, fork sync, PR review, maintenance).

**Execution**: Delete lines 254-352 completely, including the function signature and all content

---

### Step 7: Delete prompt_comprehensive() function entirely
**Location**: Lines 354-677 (entire function definition)

**EXACT CONTENT TO DELETE**: All 324 lines of the `fn prompt_comprehensive() -> Vec<PromptMessage>` function

This is the comprehensive guide explicitly excluded by the task requirements.

**Execution**: Delete lines 354-677 completely, including the function signature and all content

---

## Code Patterns: Before & After Examples

### Pattern 1: Match Statement Update
The routing logic changes from 5 scenarios to 2:

```rust
// BEFORE: 5 branches with comprehensive fallback
Some("basic") => prompt_basic(),
Some("remotes") => prompt_remotes(),
Some("prune") => prompt_prune(),
Some("workflows") => prompt_workflows(),
_ => prompt_comprehensive(),

// AFTER: 2 branches with basic fallback
Some("basic") => prompt_basic(),
Some("prune") => prompt_prune(),
_ => prompt_basic(),
```

### Pattern 2: prompt_basic() Expansion
The function keeps the same structure but content grows from 32 to ~100 lines:

```rust
// BEFORE structure
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage { role: User, content: "How do I fetch changes?" },
        PromptMessage { 
            role: Assistant, 
            content: PromptMessageContent::text(
                "Use git_fetch to download objects and refs. Here's how:\n\n\
                 [CURRENT: ~300 chars of text content]"
            )
        }
    ]
}

// AFTER structure (same)
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage { role: User, content: "How do I fetch changes?" },
        PromptMessage { 
            role: Assistant, 
            content: PromptMessageContent::text(
                "Use git_fetch to download objects and refs. Here's how:\n\n\
                 [EXPANDED: ~1200-1500 chars of text content with more examples]"
            )
        }
    ]
}
```

The function signature stays identical. Only the text passed to `PromptMessageContent::text()` grows.

---

## Execution Order

1. **First**: Update match statement (Step 1) - removes the 4 deleted functions from routing
2. **Second**: Update prompt_arguments (Step 2) - documents the available scenarios
3. **Third**: Delete decorative header (Step 3) - cleans up comments
4. **Fourth**: Delete prompt_remotes (Step 5) - removes first unused scenario
5. **Fifth**: Delete prompt_workflows (Step 6) - removes second unused scenario
6. **Sixth**: Delete prompt_comprehensive (Step 7) - removes comprehensive scenario
7. **Last**: Expand prompt_basic (Step 4) - adds content to reach target line count

This order ensures the match statement is updated before functions are deleted, preventing compile errors.

---

## Definition of Done

### Success Criteria (ALL must pass)

- **File compiles**: No Rust compilation errors when running `cargo check` in `packages/kodegen-mcp-schema`
- **Line count**: Exactly 170-220 lines total (measure with `wc -l prompts.rs`)
- **Scenarios**: Exactly 2 scenario functions exist (`prompt_basic()` and `prompt_prune()`)
- **No compile errors**: No references to deleted functions in code
- **Match statement**: Has exactly 3 arms (basic, prune, default)
- **No decorative headers**: All `// ====...====` comment lines removed
- **prompt_basic expansion**: ~95-110 lines (expanded from original 32)
- **prompt_prune unchanged**: Still 82-85 lines (original content intact)

### Measurable Targets

After implementation, the file structure must be:
```
Lines 1-43:   FetchPrompts struct + impl (with updated match)
Lines 44-145: prompt_basic() function (~100 lines)
Lines 146-227: prompt_prune() function (~82 lines)
TOTAL: 227 lines maximum
```

### Verification Steps

1. Run `wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/fetch/prompts.rs`
   - Must report: 170-220 lines

2. Run `cargo check` in `packages/kodegen-mcp-schema`
   - Must complete with zero errors

3. Verify with grep:
   ```bash
   grep -c "^fn prompt_" packages/kodegen-mcp-schema/src/git/fetch/prompts.rs
   ```
   - Must report: 2 (only `prompt_basic` and `prompt_prune`)

4. Verify match statement with grep:
   ```bash
   grep -A 5 "fn generate_prompts" packages/kodegen-mcp-schema/src/git/fetch/prompts.rs
   ```
   - Must show exactly 3 match arms, no references to remotes/workflows/comprehensive

---

## Implementation Notes

### Expanding prompt_basic() Content

The expansion should maintain the same teaching structure but add more practical details:

**Content to add** (integrate into the existing text):
1. Additional examples showing different parameter combinations
2. More detailed explanation of each parameter (path, remote, all, tags, prune, refspec)
3. Expanded "COMMON WORKFLOW" section with 4-5 real-world usage patterns
4. More "why fetch is safe" reassurances
5. Additional context on when to use fetch vs pull

This keeps the function focused on "basic usage" while providing the depth that justifies ~100 lines.

### Why These 2 Scenarios?

- **`prompt_basic()`**: Teaches the essential git_fetch operation (download from remote)
- **`prompt_prune()`**: Teaches the optional cleanup feature (remove stale refs)

Together they cover:
- Primary use case (basic fetch)
- Common maintenance task (pruning)
- All essential parameters and behaviors

Deleted scenarios were specialized use-cases (remotes) or comprehensive guides (workflows/comprehensive) that exceed Complexity 2 scope.

### Testing After Implementation

After making changes:
1. `cargo check` in `packages/kodegen-mcp-schema` must pass
2. Build the full project: `just check`
3. No other packages should be affected (git_fetch is isolated)

---

## Files to Modify

Only 1 file requires modification:
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/fetch/prompts.rs`

No other files reference these specific scenario functions by name.

---

## References

- **Complexity 2 template**: See PRECURSOR_02_fs_read_file.md
- **Target size**: 200 lines (acceptable range: 170-220)
- **Current size**: 677 lines (reduction: ~74%)
- **Scenario count**: 5 to 2 (removal: 3 scenarios)
