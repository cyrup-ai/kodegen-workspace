# TASK 026: Trim git_commit

**Tool**: `git_commit`
**Complexity**: 2 (Simple)
**Current size**: 607 lines (5 scenarios)
**Target size**: 200 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/commit/prompts.rs`

---

## Reference

See **PRECURSOR_02_fs_read_file.md** for Complexity 2 template. This task follows the exact same pattern.

---

## Current State Analysis

### File Structure

**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/commit/prompts.rs`
**Total lines**: 607
**Implementation framework**: Rust with rmcp MCP protocol

### Current Scenarios (607 lines total)

| Scenario | Lines | Status | Reason |
|----------|-------|--------|--------|
| `prompt_basic()` | 1-104 | KEEP, TRIM | Core tool usage |
| `prompt_messages()` | 107-194 | DELETE | Git conventions, not tool feature |
| `prompt_amend()` | 196-278 | KEEP, TRIM | Special parameter (amend) |
| `prompt_workflows()` | 280-410 | DELETE | Use-case scenarios, not features |
| `prompt_comprehensive()` | 412-607 | DELETE | Duplication of other scenarios |

### PromptProvider Implementation (lines 12-34)

Current routing:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),          // KEEP
        Some("messages") => prompt_messages(),    // DELETE
        Some("amend") => prompt_amend(),          // KEEP
        Some("workflows") => prompt_workflows(),  // DELETE
        _ => prompt_comprehensive(),              // DELETE
    }
}

fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, messages, amend, workflows)".to_string()),
            // UPDATE TO: "Scenario to show (basic, amend)"
            required: Some(false),
        }
    ]
}
```

---

## Trimming Instructions

### STEP 1: Analyze Current Scenarios

**`prompt_basic()` (lines 43-106, ~64 lines)**
- Teaches how to create basic commits
- Shows: simple commit, multi-line message, body parameter
- Lists required/optional parameters
- Shows typical workflow sequence
- Shows response structure
- Shows error handling
- This is CORE FUNCTIONALITY - KEEP but trim

**`prompt_messages()` (lines 107-195, ~89 lines)**
- Teaches Git commit message conventions
- Shows commit types (feat, fix, docs, refactor, test, chore)
- Shows message examples
- Shows imperative mood rules
- This is a STYLE GUIDE, NOT a tool feature
- **DELETE ENTIRELY** - teaches Git conventions, not git_commit tool

**`prompt_amend()` (lines 196-279, ~84 lines)**
- Teaches how to modify last commit
- Shows amend use cases: fix message, add forgotten file, fix entirely
- Shows amend parameter behavior
- Lists SAFE vs UNSAFE to amend
- Shows amend workflow
- This teaches a SPECIAL PARAMETER (amend) - KEEP but trim

**`prompt_workflows()` (lines 280-411, ~132 lines)**
- Shows 10 different commit scenarios: standard feature, atomic, fix-up, co-authored, bug fix, review, quick all, forgot file, multi-file, breaking change
- Lists best practices
- This is SHOWING USE-CASES, not tool features
- **DELETE ENTIRELY** - tools don't teach use cases, only features

**`prompt_comprehensive()` (lines 412-607, ~196 lines)**
- Duplicates all content from basic + messages + amend + workflows
- Adds 6 decorative section headers with `=============================================================================`
- Has repeated explanations (basic usage, message guidelines, workflows, etc.)
- **DELETE ENTIRELY** - pure duplication

### STEP 2: Trim `prompt_basic()` to ~90 lines

**Current content to KEEP:**
- Tool description (4 lines): "The git_commit tool records changes..."
- Creating commits section (22 lines):
  - Simple commit example
  - Multi-line message example
  - With body example
- Response structure (8 lines): Shows commit hash, message, files_changed, insertions, deletions
- Commit requires section (4 lines): Staged changes, non-empty message, valid path
- Typical sequence (6 lines): Make changes, git_status, git_add, git_commit, git_push
- Parameters section (8 lines): List path, message, body, all, amend, no_edit (brief descriptions)
- Error handling (5 lines): nothing to commit, empty message, not a repository

**Current content to REMOVE:**
- None - this scenario is already fairly focused
- Just compact it by removing extra newlines and verbose explanations

**Target line count**: 80-90 lines

### STEP 3: Trim `prompt_amend()` to ~90 lines

**Current content to KEEP:**
- How to amend description (2 lines)
- Amending commits section (18 lines):
  - Fix commit message example
  - Add forgotten file example
  - Fix last commit entirely example
  - Amend with no changes example
- Amend warnings (6 lines): Only amend unpushed, rewrites history, changes hash, don't amend shared, breaks others
- Safe to amend section (5 lines): Local commits, private branches, before PR review
- Unsafe to amend section (4 lines): Already pushed, on main/master, others based work, merged
- Amend workflow (12 lines): Check status, stage changes, amend, force push
- Alternatives to amending (4 lines): New commit, revert, reset
- Best practices (8 lines): Only amend local, check status, review diff, use no_edit, careful with force push

**Current content to REMOVE:**
- None - this scenario teaches a specific parameter
- Compact by removing extra context and verbose warnings
- Merge "Alternatives" into best practices

**Target line count**: 80-95 lines

### STEP 4: Update PromptProvider Implementation

Change the match statement (lines 15-21):

```rust
// OLD
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("messages") => prompt_messages(),
        Some("amend") => prompt_amend(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}

// NEW
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("amend") => prompt_amend(),
        _ => prompt_basic(),
    }
}
```

Change prompt_arguments() (lines 25-33):

```rust
// OLD
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, messages, amend, workflows)".to_string()),
            required: Some(false),
        }
    ]
}

// NEW
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, amend)".to_string()),
            required: Some(false),
        }
    ]
}
```

### STEP 5: Delete Functions

**Delete entire functions:**
1. `prompt_messages()` - lines 107-195
2. `prompt_workflows()` - lines 280-411
3. `prompt_comprehensive()` - lines 412-607

**Keep functions:**
1. `prompt_basic()` - lines 43-106 (trim for compactness)
2. `prompt_amend()` - lines 196-279 (trim for compactness)

---

## Implementation Strategy

### Execution Order

1. **Delete `prompt_comprehensive()`** first (lines 412-607, 196 lines)
   - Biggest, cleanest deletion
   - No dependencies on other deletions
   
2. **Delete `prompt_workflows()`** (lines 280-411, 132 lines)
   - Middle section, straightforward deletion
   
3. **Delete `prompt_messages()`** (lines 107-195, 89 lines)
   - Smallest deletion
   - Happens early in file
   
4. **Update PromptProvider implementation** (lines 12-34)
   - Simplify match statement
   - Update description in prompt_arguments
   
5. **Trim `prompt_basic()`** (lines 43-106 after deletions)
   - Remove verbose explanations
   - Consolidate sections
   - Target: 85-95 lines
   
6. **Trim `prompt_amend()`** (new line numbers after deletions)
   - Remove redundant warnings
   - Consolidate best practices
   - Target: 80-90 lines

### Code Changes by Location

**Location 1**: Lines 12-34 (PromptProvider implementation)
- Change match statement to only handle "amend" or default
- Update prompt_arguments description

**Location 2**: Lines 43-106 (prompt_basic function) 
- Keep structure but remove verbose padding
- Ensure all 4 key usage patterns are shown
- Keep response structure and error handling

**Location 3**: Lines 107-195 (prompt_messages)
- DELETE ENTIRELY - not needed for git_commit tool

**Location 4**: Lines 196-279 (prompt_amend function)
- Keep structure with trimmed explanations
- Ensure safe/unsafe amending is clear
- Keep best practices

**Location 5**: Lines 280-411 (prompt_workflows)
- DELETE ENTIRELY - these are use-cases, not tool features

**Location 6**: Lines 412-607 (prompt_comprehensive)
- DELETE ENTIRELY - pure duplication

---

## Before/After Code Patterns

### PromptProvider Match Statement

```rust
// BEFORE
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("messages") => prompt_messages(),
    Some("amend") => prompt_amend(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}

// AFTER
match args.scenario.as_deref() {
    Some("amend") => prompt_amend(),
    _ => prompt_basic(),
}
```

### Scenario Functions Count

```
BEFORE: 5 functions
- prompt_basic()           (64 lines)
- prompt_messages()        (89 lines)
- prompt_amend()           (84 lines)
- prompt_workflows()       (132 lines)
- prompt_comprehensive()   (196 lines)
Total: 607 lines

AFTER: 2 functions
- prompt_basic()           (85-95 lines, trimmed)
- prompt_amend()           (80-90 lines, trimmed)
Total: 165-185 lines
```

### prompt_basic() Trimming Example

```rust
// BEFORE (verbose sections)
"CREATING COMMITS:\n\n\
 1. Simple commit:\n\
    git_commit({\n\
        \"path\": \"/project\",\n\
        \"message\": \"Add user authentication\"\n\
    })\n\n\
 2. Multi-line message:\n\
    ... [verbose multi-line example] ...\n\n\
 3. With body:\n\
    ... [verbose body example] ...\n\n\
 RESPONSE:\n\
 {\n\
   \"commit\": \"abc1234\",\n\
   \"message\": \"Add user authentication\",\n\
   \"files_changed\": 5,\n\
   \"insertions\": 120,\n\
   \"deletions\": 15\n\
 }\n\n\
 COMMIT REQUIRES:\n\
 - Staged changes (use git_add first)\n\
 - Non-empty message\n\
 - Valid repository path\n\n\
 TYPICAL SEQUENCE:\n\
 1. Make changes to files\n\
 2. git_status - Check what changed\n\
 3. git_add - Stage files\n\
 4. git_commit - Record changes\n\
 5. git_push - Upload to remote\n\n\
 PARAMETERS:\n\
 - path (required): Repository directory path\n\
 - message (required): Commit message\n\
 - body (optional): Extended commit description\n\
 - all (optional): Auto-stage all tracked changes\n\
 - amend (optional): Modify last commit\n\
 - no_edit (optional): Keep message when amending\n\n\
 ERROR HANDLING:\n\
 - No staged changes: \"nothing to commit\"\n\
 - Empty message: \"commit message cannot be empty\"\n\
 - Not a repository: \"not a git repository\""

// AFTER (keep all examples but remove padding)
"The git_commit tool records changes to the repository. Use after staging files with git_add.\n\n\
 BASIC USAGE:\n\
 1. Simple: git_commit({\"path\": \"/repo\", \"message\": \"Add feature\"})\n\
 2. Multi-line: git_commit({\"path\": \"/repo\", \"message\": \"Add feature\\n\\nDescription here\"})\n\
 3. With body: git_commit({\"path\": \"/repo\", \"message\": \"...\", \"body\": \"...\"})\n\n\
 REQUIRED PARAMETERS:\n\
 - path: Repository directory path\n\
 - message: Commit message (non-empty)\n\n\
 OPTIONAL PARAMETERS:\n\
 - body: Extended description\n\
 - all: Auto-stage all tracked files\n\
 - amend: Modify last commit (see amend scenario)\n\
 - no_edit: Keep message when amending\n\n\
 RESPONSE:\n\
 {\"commit\": \"abc1234\", \"message\": \"...\", \"files_changed\": 5, \"insertions\": 120, \"deletions\": 15}\n\n\
 WORKFLOW:\n\
 1. git_status - See what changed\n\
 2. git_add - Stage files\n\
 3. git_commit - Create commit\n\n\
 ERRORS:\n\
 - \"nothing to commit\": No staged changes\n\
 - \"commit message cannot be empty\": Message required\n\
 - \"not a git repository\": Invalid path"
```

---

## Success Criteria

All must be satisfied:

- ✓ Total file is 170-220 lines (measured: `wc -l prompts.rs`)
- ✓ Exactly TWO scenario functions exist (grep "^fn prompt_" shows exactly 2)
- ✓ Functions are: `prompt_basic()` and `prompt_amend()`
- ✓ No use-case scenarios: NO `prompt_messages()`, NO `prompt_workflows()`
- ✓ No comprehensive scenario: NO `prompt_comprehensive()`
- ✓ PromptProvider match statement handles only "amend" or default
- ✓ prompt_arguments description is: "Scenario to show (basic, amend)"
- ✓ No decorative `===` headers
- ✓ Both scenarios teach core git_commit functionality
- ✓ prompt_basic: 85-95 lines, covers simple/multi-line/body usage
- ✓ prompt_amend: 80-90 lines, covers when/why/how to amend
- ✓ All required parameters clearly shown in basic scenario
- ✓ All special parameters (amend, no_edit) clearly explained
- ✓ Response structure shown in basic scenario
- ✓ Error cases shown in basic scenario
- ✓ Each scenario teachable in under 2 minutes

---

## Validation Steps

After completing all changes:

1. **Line count**: Run `wc -l prompts.rs` → expect 170-220 lines
2. **Scenario count**: Run `grep "^fn prompt_" prompts.rs` → expect 2 matches (basic, amend)
3. **Comprehensive deleted**: Run `grep -c "prompt_comprehensive" prompts.rs` → expect 0
4. **Messages deleted**: Run `grep -c "prompt_messages" prompts.rs` → expect 0
5. **Workflows deleted**: Run `grep -c "prompt_workflows" prompts.rs` → expect 0
6. **Decorative headers removed**: Run `grep -c "=====" prompts.rs` → expect 0
7. **Match statement correct**: Grep for "match args.scenario" section, verify only "amend" case exists
8. **Description updated**: Grep prompt_arguments description, verify "(basic, amend)" text

---

## This Follows the Template

This task follows the exact same pattern as PRECURSOR_02_fs_read_file.md:
- Current: 607 lines (5 scenarios)
- Target: 200 lines (2 scenarios)
- Keep: Core functionality scenarios only
- Delete: Use-case scenarios
- Delete: Comprehensive scenario (duplication)
- Delete: Decorative headers
- Result: ~200 line file with 2 focused scenarios matching Complexity 2 standard
