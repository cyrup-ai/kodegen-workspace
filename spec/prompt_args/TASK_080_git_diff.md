# TASK 080: Trim git_diff Prompts

**Tool**: `git_diff`
**Complexity**: 3 (Medium)
**Current size**: 1,153 lines (6 scenarios)
**Target size**: 320 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/diff/prompts.rs`

---

## Current State Analysis

The prompts.rs file is severely oversized with massive redundancy across scenarios. Each scenario function is 150-300+ lines with overlapping content.

### Current Structure (1,153 lines total)

**Scenario Functions:**
1. `prompt_working()` - Lines 43-250 (~207 lines)
   - View uncommitted changes in working directory
   - Contains: 8+ examples, extensive diff format explanation, multiple workflows, verbose best practices

2. `prompt_staged()` - Lines 253-650 (~397 lines)
   - Review changes to be committed
   - Contains: Staged vs unstaged comparison, complete commit workflow, verification checklist, partial staging workflow
   - Highly redundant with prompt_working() concept

3. `prompt_commits()` - Lines 653-1000 (~347 lines)
   - Compare two commits, tags, or branches
   - Contains: 6+ commit reference examples, 6 use cases, multiple workflows, extensive debugging section

4. `prompt_branches()` - Lines 1003-1400+ (~400+ lines, continues beyond first read)
   - Compare branches before merge
   - Contains: Multiple branch patterns, two-dot vs three-dot syntax, 5 scenarios, PR workflow, troubleshooting

5. `prompt_files()` - Lines 1400+-1750+ (~350+ lines)
   - Filter changes to specific files/directories
   - Contains: Multiple file filter examples, 6 use cases, directory patterns, performance optimization

6. `prompt_comprehensive()` - Final section (~150-200 lines)
   - Pure duplication of concepts from other scenarios
   - Headers, summary patterns, quick reference

**Redundancy Issues:**
- Response structure and diff format explained 5+ times
- Unified diff format (line prefixes, headers) explained 5+ times
- Workflow examples repeated across scenarios
- Best practices duplicated across all scenarios
- Sections like "READING DIFF OUTPUT" appear in multiple scenarios
- "COMMON PATTERNS" sections repeat same patterns

---

## Implementation Strategy

Reference: **PRECURSOR_03_git_branch_create.md** (Complexity 3 template)
- That tool: 1,282 → 310-330 lines (3 scenarios)
- This tool: 1,153 → 280-360 lines (2 scenarios target, 3 maximum)

### TRIM TO KEEP: `prompt_working()` (~130 lines)

**Current**: 207 lines of viewing working directory changes

**Keep these core elements (~130 lines total):**
1. **Basic Examples** (25 lines):
   ```rust
   git_diff({"path": "/project"})  // All changes
   git_diff({"path": "/project", "files": ["src/main.rs"]})  // Single file
   git_diff({"path": "/project", "files": ["src/"]})  // Directory
   git_diff({"path": "/project", "staged": true})  // Staged only
   git_diff({"path": "/project", "stat_only": true})  // Stats only
   ```

2. **Response Structure & Understanding** (20 lines):
   - Files changed, insertions, deletions explanation
   - Unified diff format line prefixes (+ - space @@)
   - Brief example of reading a diff hunk

3. **When to Check Working Directory** (10 lines):
   - Before staging
   - Before committing
   - Before switching branches
   - After modifying files

4. **One Core Workflow Example** (20 lines):
   - Simple: make changes → diff → stage → verify staged diff → commit
   - Shows the verify-then-stage-then-verify pattern

5. **File Filtering Brief** (15 lines):
   - How to use files parameter
   - Single file, multiple files, directory examples
   - When to use stat_only / name_only options

6. **Best Practices - Essential Only** (15 lines):
   - Review before staging
   - Check for debug code
   - Verify no sensitive data
   - Test updates with code changes
   - Documentation matches changes

7. **Parameters Documentation** (15 lines):
   - path: Repository path (required)
   - files: Array of file paths (optional)
   - staged: true/false (optional) - Key parameter showing both modes
   - stat_only: Show statistics only
   - name_only: Show file names only

**DELETE from prompt_working():**
- Extensive diff format explanation (lines 98-140+) → replaced with brief version
- "NO CHANGES SCENARIO" section → not essential
- "CHECKING SPECIFIC CHANGES" section (duplicate of files parameter)
- "DIFF OUTPUT ANALYSIS" (lines ~160-180) → consolidated into response structure
- Extended best practices (lines ~180-200) → keep only top 5
- Multiple workflow variations → keep ONE simple workflow
- Verbose "when to check" explanations → compress to bullets

---

### TRIM TO KEEP: `prompt_commits()` (~130 lines)

**Current**: 347 lines of comparing commits

**Keep these core elements (~130 lines total):**
1. **Commit Reference Formats** (30 lines):
   - Hash references: "abc1234", "abc1234^", "abc1234~5"
   - Branch references: "main", "feature/auth", "origin/main"
   - Tag references: "v1.0.0", "v2.1.3"
   - Relative references: "HEAD", "HEAD~1", "HEAD~5"
   - Merge commit parents: "HEAD^1", "HEAD^2"

2. **Core Commit Comparison Examples** (20 lines):
   ```rust
   git_diff({"path": "/project", "from": "abc1234", "to": "def5678"})
   git_diff({"path": "/project", "from": "HEAD~5", "to": "HEAD"})
   git_diff({"path": "/project", "from": "v1.0.0", "to": "v2.0.0"})
   git_diff({"path": "/project", "from": "HEAD^", "to": "HEAD"})
   ```

3. **Use Cases - Select 2-3 Most Common** (20 lines):
   - Release comparison (between version tags)
   - Bug investigation (good commit to bad commit)
   - Single commit analysis (commit^ to commit)

4. **One Complete Workflow Example** (25 lines):
   - Bug investigation workflow:
     1. Review history with git_log
     2. Find last known good commit
     3. Compare good to current: git_diff(from: last_good)
     4. Narrow to suspect file: git_diff(from: last_good, files: [suspect.rs])
     5. Identify problematic change

5. **Reading the Output** (15 lines):
   - Response structure: from, to, files_changed, insertions, deletions, diff
   - Interpretation: what each field means
   - Understanding insertions vs deletions ratio

6. **Parameters Documentation** (15 lines):
   - path: Repository path (required)
   - from: Starting revision (required for comparison)
   - to: Ending revision (optional, defaults to working directory)
   - files: Optional file filtering (same as working scenario)
   - stat_only / name_only: Performance options

7. **Brief Best Practices** (10 lines):
   - Use abbreviated hashes (7 chars)
   - Combine with git_log to find commit hashes
   - Focus on specific files for large diffs
   - Use tags for release comparisons
   - Save important commit hashes

**DELETE from prompt_commits():**
- Multiple detailed use case explanations (lines ~350-450) → keep only 2-3 use cases
- "COMMIT HASH DISCOVERY" section (lines ~500+) → brief mention only
- Multiple practical examples (4+ examples) → keep 1 workflow example
- Extensive debugging workflow → compress to 5-step workflow
- "READING THE OUTPUT" extended section → condense to 10 lines
- Redundant commit reference table → consolidate to 30 lines
- Best practices section (lines ~850+) → keep 5 essential items only
- Two-dot vs three-dot explanation (belongs in branches, not commits)

---

### DELETE ENTIRELY

**Delete these scenario functions completely:**

1. `prompt_staged()` - 397 lines
   - All staged vs unstaged verification is now in prompt_working() with staged parameter
   - No reason to have separate scenario for same fundamental operation

2. `prompt_branches()` - 400+ lines
   - Branch comparison uses same from/to parameters as commit comparison
   - Two-dot vs three-dot is advanced detail, not essential for Complexity 3
   - Can add brief note to prompt_commits() if needed

3. `prompt_files()` - 350+ lines
   - File filtering is a parameter available in both working and commits scenarios
   - No need for separate scenario - covered in both remaining scenarios
   - Detailed file patterns are over-engineering for medium tool

4. `prompt_comprehensive()` - 150-200 lines
   - Pure duplication of content from other scenarios
   - Comprehensive scenarios are always deleted per Complexity 3 template

---

## Routing Update

### Current Routing (6 scenarios)

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("working") => prompt_working(),
        Some("staged") => prompt_staged(),
        Some("commits") => prompt_commits(),
        Some("branches") => prompt_branches(),
        Some("files") => prompt_files(),
        _ => prompt_comprehensive(),
    }
}
```

### New Routing (2 scenarios)

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("commits") => prompt_commits(),
        _ => prompt_working(),  // Default: working directory changes
    }
}
```

### Prompt Arguments Update

**Current**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (working, staged, commits, branches, files)".to_string()),
            required: Some(false),
        }
    ]
}
```

**New**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (working, commits). Default: working directory changes".to_string()),
            required: Some(false),
        }
    ]
}
```

---

## Step-by-Step Implementation

### Step 1: Trim prompt_working()
1. Open the file at `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/diff/prompts.rs`
2. Locate `fn prompt_working()` function
3. Keep only the essential sections listed in "TRIM TO KEEP" section above
4. Delete verbose explanations, duplicate examples, and extended best practices
5. Verify function is ~130 lines (count lines from `vec![` to closing `]`)

**Specific deletions in prompt_working():**
- Extensive "READING DIFF OUTPUT" section (30+ lines) → replace with 5-line summary
- "CHECKING SPECIFIC CHANGES" subsection → merge into "Basic Examples" section
- "DIFF OUTPUT ANALYSIS" section (20+ lines) → condense to 10 lines in response structure
- "BEST PRACTICES" list (lines beyond 200) → keep only 5 bullet points
- "NO CHANGES SCENARIO" section (10+ lines) → delete entirely

### Step 2: Trim prompt_commits()
1. Locate `fn prompt_commits()` function
2. Keep only the essential sections listed in "TRIM TO KEEP" section above
3. Delete use cases beyond 2-3, keep only workflows
4. Reduce multiple examples to single workflow pattern
5. Verify function is ~130 lines

**Specific deletions in prompt_commits():**
- "COMMIT HASH DISCOVERY" section (30+ lines) → replace with brief mention
- 4-6 "USE CASES" subsections (200+ lines) → keep only 2-3 most common
- "PRACTICAL EXAMPLES" section (50+ lines) → consolidate to 1 workflow
- "DEBUGGING WORKFLOW" section (30+ lines) → condense to 5-step list
- Extended "BEST PRACTICES" → keep top 5
- All "SECTION X" headers with === lines → remove decorative headers, use simple comments instead

### Step 3: Delete Four Scenario Functions

Delete completely (remove entire function definitions):

1. **Delete prompt_staged()** - Lines ~253-650
   - Remove entire function
   - Content is redundant with prompt_working() + staged parameter

2. **Delete prompt_branches()** - Lines ~1003-1400+
   - Remove entire function
   - Branch comparison uses same from/to parameters as commits scenario

3. **Delete prompt_files()** - Lines ~1400+-1750+
   - Remove entire function
   - File filtering is a parameter, not a scenario

4. **Delete prompt_comprehensive()** - Last section
   - Remove entire function
   - Pure duplication per Complexity 3 standard

### Step 4: Update the routing match statement

Replace the `generate_prompts()` match statement as shown in "Routing Update" section above:
- Remove cases for "staged", "branches", "files"
- Change default from `prompt_comprehensive()` to `prompt_working()`

### Step 5: Update prompt_arguments()

Update the description string to reflect new scenarios: "(working, commits)"

---

## Definition of Done - Measurable Criteria

All criteria must be satisfied to complete this task:

1. **File Size**: 280-360 lines total
   - Command: `wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/diff/prompts.rs`
   - Expected output: 280-360 lines

2. **Scenario Function Count**: Exactly 2 functions
   - Command: `grep -c "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/diff/prompts.rs`
   - Expected output: 2

3. **Deleted Functions**: All removed
   - Command: `grep "fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/diff/prompts.rs`
   - Expected output: only `prompt_working()` and `prompt_commits()`
   - Should NOT contain:
     - `fn prompt_staged()`
     - `fn prompt_branches()`
     - `fn prompt_files()`
     - `fn prompt_comprehensive()`

4. **Routing Simplification**: Match statement has 2 branches
   - Command: `grep -A 5 "fn generate_prompts" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/diff/prompts.rs`
   - Should contain:
     - `Some("commits") => prompt_commits(),`
     - `_ => prompt_working(),`
   - Should NOT contain scenario cases for "staged", "branches", "files", or call to `prompt_comprehensive()`

5. **Prompt Arguments**: Description updated
   - Command: `grep -A 3 "fn prompt_arguments" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/diff/prompts.rs`
   - Description should mention "(working, commits)" not the old 5 scenarios

6. **Code Quality**: No compilation errors
   - Command: `cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check`
   - Should complete successfully with no errors

7. **Content Retention**: Essential information preserved
   - prompt_working() must include:
     - Basic diff examples (uncommitted changes)
     - Response structure explanation
     - Line prefix meanings (+, -, space, @@)
     - When to check working directory
     - File filtering examples (files, stat_only, name_only)
     - One workflow example
   - prompt_commits() must include:
     - Commit reference formats (HEAD, HEAD~N, hashes, tags, branches)
     - Basic comparison examples (two commits, release tags, HEAD^ to HEAD)
     - One workflow example (recommended: bug investigation)
     - Response interpretation
     - How to find commits (git_log reference)

---

## Validation Checklist

After implementation, verify each criterion:

- [ ] File is 280-360 lines (use: `wc -l prompts.rs`)
- [ ] Exactly 2 scenario functions (use: `grep "^fn prompt_"`)
- [ ] prompt_working() is ~130 lines (count from vec![ to ])
- [ ] prompt_commits() is ~130 lines (count from vec![ to ])
- [ ] No prompt_staged() function (search file, should not exist)
- [ ] No prompt_branches() function (search file, should not exist)
- [ ] No prompt_files() function (search file, should not exist)
- [ ] No prompt_comprehensive() function (search file, should not exist)
- [ ] Routing has exactly 2 match arms (working and commits)
- [ ] Default case calls prompt_working()
- [ ] Prompt arguments description updated
- [ ] Cargo check passes (no compilation errors)
- [ ] Can read through entire file in 5 minutes
- [ ] All core git_diff concepts covered in remaining 2 scenarios

---

## Success Definition

This task is complete when:
- The prompts.rs file is 280-360 lines (reduced from 1,153)
- Contains exactly 2 scenario functions (reduced from 6)
- No redundancy between scenarios
- All essential git_diff knowledge is preserved
- Each scenario fits on screen and is skimmable in 2-3 minutes
- Matches the Complexity 3 template from PRECURSOR_03_git_branch_create.md
