# TASK 030: Trim git_remote_add

**Tool**: `git_remote_add`
**Complexity**: 2 (Simple)
**Current size**: 970 lines (5 scenarios)
**Target size**: 170-220 lines (1-2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_add/prompts.rs`

---

## Context

The git_remote_add tool is a simple tool with 3 required parameters: `path`, `name`, and `url`, plus one optional parameter: `fetch`. Currently, the prompts.rs file contains 970 lines with 5 distinct scenarios. The comprehensive scenario alone accounts for ~500 lines and duplicates content from the use-case scenarios. This task trims the file to the Complexity 2 standard (170-220 lines) by removing use-case scenarios and the comprehensive guide, keeping only the basic scenario.

**Current file**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_add/prompts.rs`

---

## Current Scenario Analysis

**Current scenarios** (970 lines total):

1. `prompt_basic()` - ~130 lines (lines 43-170) ← KEEP
2. `prompt_fork()` - ~150 lines (lines 190-340) ← DELETE (fork workflow use case)
3. `prompt_multiple()` - ~180 lines (lines 370-550) ← DELETE (multiple remotes use case)
4. `prompt_configure()` - ~300 lines (lines 580-880) ← DELETE (advanced config use case)
5. `prompt_comprehensive()` - ~500 lines (lines 910-970) ← DELETE (pure duplication)

**Classification:**
- **Essential**: basic scenario (teaches core tool functionality)
- **Use-case scenarios**: fork, multiple, configure (teach workflows, not tool features)
- **Duplication**: comprehensive (repeats all other scenarios)

---

## Trimming Instructions

### KEEP: `prompt_basic()` (Target: 130-150 lines)

The basic scenario already teaches:
- How to add a remote with 3 required parameters (path, name, url)
- URL format options (HTTPS, SSH, git protocol)
- Common remote names (origin, upstream, production, staging, backup)
- Typical workflow integration (after git_init and git_commit)
- Related tools (git_remote_list, git_fetch, git_push)
- Named remote conventions
- Error handling patterns

**Keep in basic:**
- Tool description (~5 lines)
- Parameter explanation (~10 lines)
- Basic examples (HTTPS, SSH, git protocol) (~15 lines)
- Response structure (~8 lines)
- Common naming conventions (~10 lines)
- Workflow integration (~15 lines)
- Error handling (~8 lines)
- After-adding operations (~10 lines)
- Quick reference (~10 lines)

**Output result**: prompt_basic should remain ~130-150 lines. Do NOT trim further.

### DELETE ENTIRELY

1. **`prompt_fork()`** (~150 lines):
   - This teaches a WORKFLOW (fork workflow), not a tool feature
   - Fork syncing uses git_fetch, git_merge, git_push - not git_remote_add
   - git_remote_add only adds the "upstream" remote name
   - No parameters specific to fork workflow
   - Example: "Fetch upstream changes" uses git_fetch, not git_remote_add
   - Readers should learn fork workflow from git_fetch/git_merge prompts, not here

2. **`prompt_multiple()`** (~180 lines):
   - This teaches using multiple remotes simultaneously (a workflow)
   - Not a tool feature - adding multiple remotes is just calling git_remote_add N times
   - Each call is identical: git_remote_add with different name/url
   - The "use cases" (backup, deployment, team collaboration) are workflows, not tool features
   - Readers can understand multiple remotes from the basic scenario + simple iteration

3. **`prompt_configure()`** (~300 lines):
   - This teaches advanced Git configuration via git_config_set (different tool)
   - Only ~20% demonstrates git_remote_add itself
   - ~80% demonstrates git_config_set, git_config_get, git_config_unset
   - Topics like pushurl, refspecs, proxy config are git_config_set features
   - Should be covered in git_config tool's prompts, not here

4. **`prompt_comprehensive()`** (~500 lines):
   - Pure duplication combining all scenarios
   - Line 596-700: duplicates basic scenario word-for-word
   - Line 720-800: duplicates fork workflow
   - Line 850-920: duplicates multiple remotes
   - Line 940-970: duplicates configure scenario
   - No unique content
   - Bloats file size without adding value

### Update Scenario Routing

**Before:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("fork") => prompt_fork(),
        Some("multiple") => prompt_multiple(),
        Some("configure") => prompt_configure(),
        _ => prompt_comprehensive(),
    }
}
```

**After:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    // Only one scenario - basic covers all essential git_remote_add usage
    match args.scenario.as_deref() {
        _ => prompt_basic(),
    }
}
```

Or simplified:
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

### Update prompt_arguments() Description

**Before:**
```rust
description: Some("Scenario to show (basic, fork, multiple, configure)".to_string()),
```

**After:**
```rust
description: Some("Scenario to show (basic)".to_string()),
```

Or change to:
```rust
required: Some(true), // Only basic is available
```

### Update Comments

Remove the decorative section header:
```rust
// DELETE THIS:
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT_REMOTE_ADD
// ============================================================================
```

Replace with:
```rust
// Single scenario: basic git_remote_add usage
```

---

## Specific Line Deletions

1. Delete lines ~190-340: Entire `prompt_fork()` function
2. Delete lines ~370-550: Entire `prompt_multiple()` function
3. Delete lines ~580-880: Entire `prompt_configure()` function
4. Delete lines ~910-970: Entire `prompt_comprehensive()` function
5. Update match statement (lines ~15-25) to only return prompt_basic()
6. Update PromptArgument description (line ~28) to reflect only "basic" scenario

---

## Success Criteria

- ✓ File is 160-200 lines total (down from 970)
- ✓ ONE scenario function exists: `prompt_basic()`
- ✓ No use-case scenarios (fork, multiple)
- ✓ No advanced configuration scenario (configure)
- ✓ No comprehensive scenario
- ✓ Match statement returns only `prompt_basic()`
- ✓ PromptArgument updated to reflect single scenario
- ✓ All essential git_remote_add features covered in basic scenario:
  - Three required parameters (path, name, url)
  - Optional fetch parameter
  - All URL formats (HTTPS, SSH, git protocol)
  - Common naming conventions
  - Integration with other git tools
  - Error handling patterns

---

## Validation

After trimming:

```bash
# Line count check
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_add/prompts.rs
# Expected output: 160-200 lines

# Scenario count check
grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_add/prompts.rs
# Expected output: only 1 result (fn prompt_basic)

# Verify no deleted scenarios remain
grep -E "prompt_(fork|multiple|configure|comprehensive)" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_add/prompts.rs
# Expected output: 0 results

# Verify match statement is simplified
grep -A5 "fn generate_prompts" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/remote_add/prompts.rs
# Expected output: Shows match returning only prompt_basic() or direct call

# Verify file compiles
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
cargo check
# Expected: SUCCESS
```

---

## Reference Template

This follows the Complexity 2 template established by PRECURSOR_02_fs_read_file:
- Simple tools with few parameters (git_remote_add has 3 required + 1 optional)
- One comprehensive basic scenario (~130 lines)
- No use-case scenarios (workflows belong elsewhere)
- No comprehensive scenario (duplication)
- Total: 160-200 lines
- Goal: Teach the tool itself, not workflows

---

## Implementation Notes

1. **Order of deletions**: Delete functions from bottom to top (comprehensive → configure → multiple → fork) to avoid line number shifting during edits
2. **Preserve imports**: Keep all use statements at the top (they remain valid)
3. **Preserve struct and trait impl**: Only modify the match statement inside generate_prompts
4. **Test with cargo check**: After all deletions, run cargo check to ensure no compilation errors
5. **Comment updates**: Remove or update the "HELPER FUNCTIONS" comment section to reflect single scenario

---

## After Completion

Once trimmed:
- This file becomes a reference for all Complexity 2 git tools
- git_remote_remove, git_remote_rename, git_branch_create, git_tag, etc. should follow same structure
- Each tool's prompts.rs should be 160-200 lines with one focused scenario
- Use-case workflows move to separate documentation or composite scenarios at a higher level
