# TASK 079: Trim git_clone (Complexity 3)

**Tool**: `git_clone`
**Complexity**: 3 (Medium)
**Current size**: 839 lines (5 scenarios)
**Target size**: 300-360 lines (3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/clone/prompts.rs`

---

## Reference

See **PRECURSOR_03_git_branch_create.md** for Complexity 3 trimming template and success criteria.

---

## Current State Analysis

### Overview
The git_clone tool has 4 supported parameters only:
- `url` (required): Repository URL (HTTPS, SSH, git://, file://)
- `path` (required): Local destination path
- `branch` (optional): Specific branch or tag to checkout
- `depth` (optional): Shallow clone depth (number of commits)

### Scenario Analysis

**Current scenarios** (839 lines total):
1. `prompt_basic()` - ~140 lines (43-182) ← KEEP, TRIM TO ~100
   - Covers: HTTPS/SSH URLs, what gets cloned, response format, troubleshooting
   - Quality: Good foundational content, but verbose

2. `prompt_shallow()` - ~240 lines (185-425) ← KEEP, TRIM TO ~110
   - Covers: depth parameter benefits, depth meanings, realistic examples, limitations
   - Quality: Comprehensive depth explanation with real repository sizes
   - Contains some redundancy with decision trees

3. `prompt_branch()` - ~204 lines (428-632) ← KEEP, TRIM TO ~110
   - Covers: branch/tag cloning, branch naming conventions, use cases, branch vs tag
   - Quality: Good coverage of parameter, but some examples are excessive

4. `prompt_options()` - ~189 lines (635-824) ← DELETE ENTIRELY
   - Problem: Discusses bare repos, mirrors, submodules NOT in tool's API
   - Misleading: Claims features the tool doesn't support
   - Redundant: Most content duplicates scenarios 1-3
   - Decision: Remove this scenario completely (false advanced features)

5. `prompt_comprehensive()` - ~1000+ lines (827+) ← DELETE ENTIRELY
   - Pure duplication of scenarios 1-4
   - Classic "comprehensive scenario" pattern that creates redundancy
   - Decision: Delete all lines

---

## Trimming Instructions

### STEP 1: Keep & Trim `prompt_basic()` (Target: ~100 lines)

**Current**: ~140 lines covering basic repository cloning

**What to Keep:**
- URL format patterns (35 lines):
  - HTTPS: `git_clone({"url": "https://github.com/user/repo.git", "path": "/projects/repo"})`
  - SSH: `git_clone({"url": "git@github.com:user/repo.git", "path": "/projects/repo"})`
  - Git protocol and local paths (brief mention)
- Response format documentation (15 lines):
  - Fields: path, branch, success
  - Example JSON response
- What gets cloned (15 lines):
  - Branches, commit history, tags, configuration
  - Default branch checkout
- After cloning (10 lines):
  - Remote 'origin' configured
  - Related tools: git_status, git_log, git_branch_list, git_pull
- Basic workflow (15 lines):
  - Common use cases: public repo, personal project, specific directory
- Minimal troubleshooting (10 lines):
  - Most common errors only

**What to Remove:**
- PATH REQUIREMENTS section (lines ~120-126): Too verbose for basic scenario
- COMMON USE CASES detailed list (lines ~130-150): Keep examples brief inline instead
- Extended TROUBLESHOOTING (lines ~160-180): Reduce to common cases only
- Decorative headers: Simplify section markers

**Implementation:**
- Compress "RESPONSE FORMAT" into 5 lines
- Inline 2 best basic examples only
- Remove extended error list (save for advanced scenario)
- Keep flow focused on: URL formats → what happens → quick examples

---

### STEP 2: Keep & Trim `prompt_shallow()` (Target: ~110 lines)

**Current**: ~240 lines covering depth parameter for performance

**What to Keep:**
- Depth parameter examples (30 lines):
  - Depth 1 (latest only): `depth: 1`
  - Depth 10 (recent history): `depth: 10`
  - Full clone (no depth): explain default
  - Real world impact example
- Shallow benefits (15 lines):
  - 10x-100x faster, less disk, bandwidth savings
  - CI/CD ideal use cases
- When to use shallow (20 lines):
  - CI/CD pipelines: depth 1
  - Testing: depth 10-50
  - Development: full clone
  - Choose based on your use case
- One complete workflow (25 lines):
  - Clone shallow → build → deploy example
  - Show how it saves time/space in practice
- Limitations (15 lines):
  - Can't see full history
  - git log is limited
  - Some operations fail on old commits
- Best practices (5 lines):
  - Use depth 1 for builds, full for dev

**What to Remove:**
- CONVERTING SHALLOW TO FULL section (lines ~377-386): Out of scope
- Multiple depth strategy blocks (lines ~461-545): Consolidate into one table
- Excessive performance metrics (lines ~640-650): Keep one realistic example
- Verbose WHEN TO USE SHALLOW lists (lines ~656-674): Shorten to bullets
- Extended BEST PRACTICES (lines ~708-730): Keep to 2-3 key points

**Implementation:**
- Create one depth guidance table: depth → use case → size impact
- Keep ONE real example: Linux kernel (3.5GB full vs 200MB shallow)
- Move submodule discussion out (not in tool anyway)
- Simplify section structure

---

### STEP 3: Keep & Trim `prompt_branch()` (Target: ~110 lines)

**Current**: ~204 lines covering branch and tag parameter

**What to Keep:**
- Branch cloning patterns (35 lines):
  - Clone develop: `git_clone({"url": "...", "path": "...", "branch": "develop"})`
  - Clone tag: `git_clone({"url": "...", "path": "...", "branch": "v1.0.0"})`
  - Branch vs tag difference (5 lines)
  - Shallow + branch combo (5 lines)
- Branch behavior after clone (15 lines):
  - Specified branch is checked out
  - All branches available as remote refs
  - Can switch later with git_checkout
  - Branch doesn't exist = error
- Use cases (30 lines):
  - Feature branch work
  - Release tag building
  - PR branch review
  - Development branch testing
- Branch naming conventions (15 lines):
  - Prefixes: feature/, fix/, hotfix/, release/
  - Rules: lowercase, hyphens, descriptive
  - Examples: feature/user-auth, hotfix/crash-bug
- Workflow example (15 lines):
  - One complete fetch → clone → work → push flow

**What to Remove:**
- Extended BRANCH NAMING section (lines ~380-395): Integrate naming into basic examples
- Multiple workflow examples (lines ~398-430): Keep ONE representative workflow
- BRANCH vs TAG comparison table (lines ~454-467): Simplify to 2-3 line explanation
- Verbose best practices (lines ~530-540): Reduce to actionable bullets
- PATH REQUIREMENTS repeated (lines ~451-460): Already in basic scenario

**Implementation:**
- Show naming conventions with 2-3 concrete examples inline
- Focus on: why clone branch, how it works, 2-3 use cases
- One workflow from start to finish
- Brief explanation of branch vs tag

---

### STEP 4: Delete `prompt_options()` ENTIRELY

**Reason for Deletion:**
This scenario (189 lines) describes features NOT in git_clone tool:
- Bare repositories: `git clone --bare` (not supported)
- Mirror repositories: `git clone --mirror` (not supported)
- Submodule handling: `git submodule update` (not supported)
- Advanced options documented: NOT implemented in this tool

The git_clone tool ONLY supports: url, path, branch, depth

**Content Issues:**
- Lines 635-824: Misleading agents about tool capabilities
- All "advanced options" descriptions are false for this tool
- Submodule section (lines 744-760) not applicable

**Action:**
Delete entire `prompt_options()` function and remove it from match statement.

---

### STEP 5: Delete `prompt_comprehensive()` ENTIRELY

**Reason for Deletion:**
- Pure duplication of basic + shallow + branch scenarios
- Contains all content from 3 other scenarios reorganized
- Makes file maintenance harder (change in one place duplicated 5 ways)
- Violates Complexity 3 standard (no comprehensive scenario)

**Content breakdown:**
- BASIC USAGE (827-860): Duplicates prompt_basic()
- URL FORMATS (863-930): Same as prompt_basic()
- SHALLOW CLONES (933-1000): Same as prompt_shallow()
- BRANCH-SPECIFIC (1003-1080): Same as prompt_branch()
- DECISION TREE (1177-1210): Redundant, implicit in 3 scenarios

**Action:**
Delete entire function (lines 827-1282).

---

### STEP 6: Update Scenario Routing

**Current routing** (lines 18-24):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("shallow") => prompt_shallow(),
        Some("branch") => prompt_branch(),
        Some("options") => prompt_options(),      // DELETE
        _ => prompt_comprehensive(),               // DELETE
    }
}
```

**New routing** (keep first 3, delete options, change default to basic):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("shallow") => prompt_shallow(),
        Some("branch") => prompt_branch(),
        _ => prompt_basic(),                       // NEW default
    }
}
```

---

### STEP 7: Update `prompt_arguments()` Documentation

**Current** (lines 26-35):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, shallow, branch, options)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Change to**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show: basic (default), shallow, branch".to_string()),
            required: Some(false),
        }
    ]
}
```

Remove "options" from list, add "(default)" marker.

---

## Exact Line-by-Line Changes

### Phase 1: Trim prompt_basic() [lines 43-182]

Reduce from ~140 lines to ~100 lines by:
- Lines 43-50: KEEP (User question)
- Lines 52-108: KEEP basic patterns (HTTPS, SSH, git://, local), trim verbose explanations
- Lines 109-133: DELETE extended path handling, integrate into examples
- Lines 134-150: TRIM COMMON USE CASES to 2 inline examples
- Lines 151-180: TRIM TROUBLESHOOTING to 3 most common errors
- Lines 181-182: KEEP (closing)

**Target output**: 95-105 lines

---

### Phase 2: Trim prompt_shallow() [lines 185-425]

Reduce from ~240 lines to ~110 lines by:
- Lines 185-195: KEEP (User question)
- Lines 197-210: KEEP depth examples (1, 10, 50, 100)
- Lines 211-280: TRIM verbose explanations, create condensed benefit list
- Lines 281-320: TRIM REAL-WORLD EXAMPLES to one (Linux kernel)
- Lines 321-360: TRIM LIMITATIONS section (keep concise)
- Lines 361-380: DELETE CONVERTING SHALLOW TO FULL (not applicable)
- Lines 390-425: TRIM best practices, keep 3 bullets

**Target output**: 105-115 lines

---

### Phase 3: Trim prompt_branch() [lines 428-632]

Reduce from ~204 lines to ~110 lines by:
- Lines 428-435: KEEP (User question)
- Lines 437-500: KEEP clone examples but trim descriptions
- Lines 501-530: TRIM BRANCH NAMING (move 15 lines into examples)
- Lines 531-590: TRIM USE CASES (keep 3 concrete examples)
- Lines 591-620: DELETE redundant BRANCH vs TAG comparison (explain inline)
- Lines 621-632: TRIM best practices

**Target output**: 105-115 lines

---

### Phase 4: Delete prompt_options() [lines 635-824]

**Action**: Delete entire function.

This removes 189 lines describing unsupported features.

---

### Phase 5: Delete prompt_comprehensive() [lines 827-1282+]

**Action**: Delete entire function.

This removes ~450+ lines of duplication.

---

### Phase 6: Update match statement [lines 18-24]

**Current**:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("shallow") => prompt_shallow(),
    Some("branch") => prompt_branch(),
    Some("options") => prompt_options(),
    _ => prompt_comprehensive(),
}
```

**Replace with**:
```rust
match args.scenario.as_deref() {
    Some("shallow") => prompt_shallow(),
    Some("branch") => prompt_branch(),
    _ => prompt_basic(),
}
```

---

## Success Criteria

After trimming, file MUST satisfy:

**Line Count:**
- ✓ Total file 300-360 lines (down from 839)
- ✓ Header/imports/impl: ~40 lines
- ✓ Three scenarios: ~100 lines each (basic, shallow, branch)

**Scenario Functions:**
- ✓ `prompt_basic()`: ~100 lines - URL formats, response, quick examples
- ✓ `prompt_shallow()`: ~110 lines - depth parameter, performance, CI/CD use
- ✓ `prompt_branch()`: ~110 lines - branch/tag cloning, naming, workflows
- ✓ No `prompt_options()` function (deleted)
- ✓ No `prompt_comprehensive()` function (deleted)

**Routing:**
- ✓ Match statement has exactly 3 arms (basic default, shallow, branch)
- ✓ Default case (`_`) returns `prompt_basic()`
- ✓ No references to deleted scenarios

**Content Quality:**
- ✓ No false claims about tool features (bare repos, mirrors, submodules removed)
- ✓ Parameter documentation accurate (url, path, branch, depth only)
- ✓ One workflow example per scenario (not multiple)
- ✓ Brief parameter explanations (not exhaustive)
- ✓ Practical examples with real repositories
- ✓ Each scenario covers distinct use case/parameter

**No Redundancy:**
- ✓ No duplicate content between scenarios
- ✓ No comprehensive scenario
- ✓ Each scenario focused on one concept

**Validation Checklist:**
```bash
# Line count
wc -l prompts.rs         # Should be 300-360

# Scenario count
grep "^fn prompt_" prompts.rs    # Should show 3 lines exactly:
                                 # fn prompt_basic()
                                 # fn prompt_shallow()
                                 # fn prompt_branch()

# Deleted scenarios
grep "prompt_options\|prompt_comprehensive" prompts.rs  # Should be 0 results

# Match arms
grep -A10 "fn generate_prompts" prompts.rs  # Should show exactly 3 arms

# Parameter accuracy
grep -i "bare\|mirror\|submodule" prompts.rs  # Should be 0 results
```

---

## Scope & Definition of Done

**This task ONLY edits one file:**
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/clone/prompts.rs`

**This task DOES NOT:**
- Modify tests
- Change tool implementation or API
- Update documentation outside this file
- Modify prompt_args.rs or schema.rs
- Create new test cases

**File is complete when:**
1. Trimming instructions fully applied
2. All success criteria met
3. No orphaned scenario references
4. Match statement correctly updated
5. File compiles (no Rust syntax errors)
6. git_clone scenarios work as documented

---

## Trimming Workflow

1. Read and copy entire prompts.rs
2. Delete prompt_options() function completely
3. Delete prompt_comprehensive() function completely
4. Trim prompt_basic() to ~100 lines
5. Trim prompt_shallow() to ~110 lines
6. Trim prompt_branch() to ~110 lines
7. Update match statement (remove 2 arms)
8. Update prompt_arguments() description
9. Verify line count: 300-360
10. Validate: no orphaned references, proper Rust syntax
11. Spot-check: scenarios are readable 5-minute explanations

---

## Why This Structure

This trimming follows the **Complexity 3 standard** from PRECURSOR_03:
- 3 focused scenarios (not 5, not 1 comprehensive)
- Each scenario covers one key use case or parameter
- Total file size matches medium complexity tools
- No advanced features that aren't in the API
- Maintainable: changes in one place, not duplicated 5 ways
- Readable: agents understand tool in 5 minutes
- Actionable: real examples with concrete parameters
