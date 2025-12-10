# TASK 010: Trim git_worktree_prune

**Tool**: `git_worktree_prune`
**Complexity**: 1 (Trivial)
**Current size**: 107 lines (3 scenarios)
**Target size**: 100-110 lines (1 scenario)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/worktree_prune/prompts.rs`

---

## Reference Standards

See **PRECURSOR_01_memory_list_libraries.md** and **TOOL_COMPLEXITY_RATING.md** for Complexity 1 template guidelines.

**Complexity 1 Target**: 80-120 lines, 1 scenario, prescriptive structure:
- Tool description (5-10 lines)
- Basic usage example (5-10 lines)
- Response structure (5-10 lines)
- When to use (3-5 bullet points, 10 lines)
- Common pattern or workflow (10-15 lines)
- Quick reference (5 lines)

---

## Current State Analysis

The current `prompts.rs` file has **3 redundant scenarios**:

1. **prompt_basic()** (17 lines) - Minimal explanation, covers only basic usage
2. **prompt_cleanup()** (19 lines) - Real-world scenario focus, shows when/why to prune
3. **prompt_comprehensive()** (42 lines) - Over-detailed, covers same concepts 3 times

All three explain the same operation from slightly different angles. For a Complexity 1 trivial tool, this is excessive.

### Current Routing:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("cleanup") => prompt_cleanup(),
    _ => prompt_comprehensive(),  // default case
}
```

---

## Implementation Plan

### Step 1: Consolidate prompt_basic() - Single Scenario (65-75 lines)

Create ONE comprehensive `prompt_basic()` that merges the best elements:

```rust
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I clean up stale git worktrees?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Remove stale worktree administrative files:\n\n\
                 WHAT IT DOES:\n\
                 Prunes orphaned worktree metadata when actual directories have been manually \
                 deleted. Git maintains .git/worktrees/ references. This tool removes those stale \
                 references without deleting actual directories.\n\n\
                 BASIC USAGE:\n\
                 ```json\n\
                 {\"path\": \"./repo\"}\n\
                 ```\n\n\
                 RESPONSE STRUCTURE:\n\
                 The tool returns three fields:\n\
                 - success (bool): Whether pruning completed\n\
                 - pruned_count (usize): Number of stale references removed\n\
                 - message (String): Status description\n\n\
                 Example response: {\"success\": true, \"pruned_count\": 2, \
                 \"message\": \"Pruned 2 stale worktrees\"}\n\n\
                 WHEN TO USE:\n\
                 - After manually deleting a worktree directory with rm -rf\n\
                 - Before git_worktree_list to get clean, accurate listings\n\
                 - When 'git worktree list' shows missing or broken entries\n\
                 - After disk cleanup that removed worktree directories\n\
                 - Before adding a new worktree with a previously used name\n\n\
                 COMMON WORKFLOW:\n\
                 1. Check current worktrees: git_worktree_list\n\
                 2. Manually delete directory: rm -rf /repo-feature\n\
                 3. Run prune to clean metadata: git_worktree_prune {\"path\": \"./repo\"}\n\
                 4. Verify it's gone: git_worktree_list (should be removed)\n\
                 5. Now safe to reuse that worktree name\n\n\
                 IMPORTANT NOTES:\n\
                 - Locked worktrees are NOT pruned (they're protected)\n\
                 - To force-remove locked worktrees, use git_worktree_remove first\n\
                 - Safe to run even if no stale worktrees exist (pruned_count will be 0)\n\n\
                 RELATED TOOLS:\n\
                 - git_worktree_list: View all worktrees and their status\n\
                 - git_worktree_add: Create new worktree\n\
                 - git_worktree_lock: Protect worktree from pruning\n\
                 - git_worktree_remove: Safely remove worktree and its metadata"
            ),
        },
    ]
}
```

### Step 2: Simplify PromptProvider Implementation (10-12 lines)

Update the impl block to remove pattern matching since there's only one scenario:

```rust
impl PromptProvider for WorktreePrunePrompts {
    type PromptArgs = GitWorktreePrunePromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]  // No scenario parameter needed anymore
    }
}
```

### Step 3: Update prompt_args.rs (Optional - for full cleanup)

The `GitWorktreePrunePromptArgs` can either be:
- **Option A** (Recommended): Keep struct but make scenario field always ignored
  - Add comment: "scenario field is ignored; single prompt style is always used"
  - Maintains backward compatibility if any caller passes it

- **Option B** (Simplify): Replace struct with unit type or empty struct
  - `pub struct GitWorktreePrunePromptArgs;`
  - Cleaner but may break existing callers

**Decision**: Use Option A for backward compatibility.

### Step 4: Delete Redundant Functions

Remove these three functions entirely:
```rust
// DELETE prompt_cleanup() - 19 lines
// DELETE prompt_comprehensive() - 42 lines
```

---

## Target Structure After Trim

```
// File header and imports (10 lines)

pub struct WorktreePrunePrompts;

impl PromptProvider for WorktreePrunePrompts {
    // ...impl (12 lines)
}

fn prompt_basic() -> Vec<PromptMessage> {
    // Single consolidated prompt (65-75 lines)
}
```

**Total: ~95-105 lines** ✓ Within 80-120 line target

---

## Verification Checklist

- [ ] Only ONE prompt function exists: `prompt_basic()`
- [ ] Line count is 90-110 lines total (run `wc -l prompts.rs`)
- [ ] No `prompt_cleanup()` or `prompt_comprehensive()` functions remain
- [ ] `generate_prompts()` directly calls `prompt_basic()` without match statement
- [ ] `prompt_arguments()` returns empty vec or minimal args
- [ ] Content covers all 6 sections: description, usage, response, when-to-use, workflow, related tools
- [ ] No decorative headers (═══, ───, etc.)
- [ ] No redundant explanations (each concept explained exactly once)
- [ ] Concise and readable in under 2 minutes

---

## Success Criteria

✓ File is 90-110 lines total
✓ ONE scenario function (`prompt_basic()`) only
✓ No comprehensive scenario
✓ No cleanup scenario (merged into basic)
✓ No decorative headers
✓ Tool can be understood and used in 60 seconds
✓ All important context included (what, how, when, workflow, related tools)
✓ Backward compatible routing (single path, no pattern matching needed)

---

## Related Files

- **Schema**: `/packages/kodegen-mcp-schema/src/git/worktree_prune/schema.rs` (no changes needed)
- **Prompt Args**: `/packages/kodegen-mcp-schema/src/git/worktree_prune/prompt_args.rs` (optional simplification)
- **Module**: `/packages/kodegen-mcp-schema/src/git/worktree_prune/mod.rs` (no changes)

---

## Important Notes

- **git_worktree_prune** is a trivial, read-only operation with one required parameter (path)
- It has no variants or complex use cases - just "prune stale worktrees in this repository"
- Multiple scenarios are completely unnecessary for such a simple tool
- The consolidated prompt should be a single clear explanation, not a menu of options

---

## Implementation Reference

The Complexity 1 standard (per TOOL_COMPLEXITY_RATING.md) requires:
- **Format**: Single comprehensive scenario, ~100 lines
- **Content**: Tool description → basic usage → response structure → when to use → workflow → related tools
- **Style**: Prescriptive (not options), concise, one explanation per concept
- **Pattern**: This follows the same structure as `memory_list_libraries` reference template
