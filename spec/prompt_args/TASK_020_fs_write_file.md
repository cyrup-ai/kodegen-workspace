# TASK 020: Trim fs_write_file

**Tool**: `fs_write_file`
**Complexity**: 2 (Simple)
**Current size**: 836 lines (6 scenarios)
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/write_file/prompts.rs`

---

## Current State Analysis

### File Overview

The prompts.rs file currently contains:
- **Total lines**: 836
- **Scenarios**: 6 (basic, append, code_files, config_files, workflows, comprehensive)
- **Routing logic**: Lines 16-23 in `generate_prompts()` match statement
- **Helper functions**: Lines 44-836 contain scenario implementations

### Current Scenario Breakdown

1. **`prompt_basic()`** - Lines 44-117 (74 lines)
   - Describes basic file write and overwrite operations
   - Examples: new file, overwrite, explicit mode parameter
   - Response structure explained
   - Common patterns listed

2. **`prompt_append()`** - Lines 118-205 (88 lines)
   - Demonstrates append mode with mode: "append" parameter
   - Examples: log entries, data files, multiple lines
   - Newline handling explained (no automatic newline in append)
   - Comparison: append vs rewrite modes

3. **`prompt_code_files()`** - Lines 206-306 (101 lines)
   - Use case scenario (NOT a core feature)
   - Shows Rust, Python, JavaScript, TypeScript examples
   - Teaches nothing new about the tool itself

4. **`prompt_config_files()`** - Lines 307-416 (110 lines)
   - Use case scenario (NOT a core feature)
   - Shows JSON, TOML, YAML, INI, Env files
   - Teaches nothing new about the tool itself

5. **`prompt_workflows()`** - Lines 417-591 (175 lines)
   - Use case scenario (NOT a core feature)
   - Complete workflows combining fs_write_file with other operations
   - Extensive examples but no unique tool features

6. **`prompt_comprehensive()`** - Lines 592-836 (245 lines)
   - Pure duplication with decorative section headers (=== separators)
   - Repeats content from basic and append
   - No unique content whatsoever

---

## Trimming Instructions

### STEP 1: Keep and Trim `prompt_basic()` (Target: 80-90 lines)

**Current state**: Lines 44-117 (74 lines)

**Keep these elements**:
1. User question (1-2 lines)
2. Assistant introduction (1-2 lines)
3. Writing files section header (1 line)
4. Basic examples (keep 3, trim explanatory text):
   - Write new file
   - Overwrite existing
   - Write with explicit mode
5. Write modes explanation (4-6 lines) - keep concise
6. Response structure (4-6 lines)
7. When to use (4-6 lines)
8. Parameters description (6-8 lines)
9. Path handling (4-6 lines)
10. Content handling (6-8 lines)
11. Common patterns (8-12 lines) - keep essential patterns only
12. Error handling (4-6 lines)

**Remove from basic**:
- Decorative headers (not present, but remove if found)
- Verbose workflow examples (save for other scenarios)
- Extended use cases (code_files, config_files handled separately)
- Repeated "Success field" explanations (mention once)

**Target composition**:
- Description: 2 lines
- Examples: 20 lines (3 examples with brief explanations)
- Modes: 5 lines
- Response: 5 lines
- When to use: 5 lines
- Parameters: 8 lines
- Path/Content handling: 15 lines
- Common patterns: 12 lines
- Error handling: 6 lines
- Total: 78-85 lines

### STEP 2: Keep and Trim `prompt_append()` (Target: 85-95 lines)

**Current state**: Lines 118-205 (88 lines)

**Keep these elements**:
1. User question (1-2 lines)
2. Assistant introduction (1-2 lines)
3. Appending section header (1 line)
4. Append examples (keep 3-4 clear examples):
   - Add log entry
   - Add to data file
   - Continuous logging
5. Append patterns explanation (6-8 lines)
6. When to append vs when not to (8-10 lines)
7. Newline handling (important!) (6-8 lines)
8. Response structure (4-6 lines) - can cross-reference basic if needed
9. Comparison: append vs rewrite (8-10 lines)
10. Best practices (8-10 lines)

**Remove from append**:
- Decorative separators
- Redundant response explanations (already in basic)
- Excessive use-case examples

**Target composition**:
- Description: 2 lines
- Examples: 20 lines (3-4 examples)
- Patterns: 8 lines
- When to append/not: 8 lines
- Newline handling: 8 lines (critical section)
- Comparison: 10 lines
- Best practices: 10 lines
- Total: 84-94 lines

### STEP 3: Delete Entirely

**Delete `prompt_code_files()` (Lines 206-306, 101 lines)**
- This teaches a USE CASE (how to write code), not a tool feature
- Reading code files is identical to reading any other file
- No code_files-specific parameters in fs_write_file
- DELETE everything in this function

**Delete `prompt_config_files()` (Lines 307-416, 110 lines)**
- This teaches a USE CASE (how to write configs), not a tool feature
- Writing config files is identical to writing any other file
- No config-specific parameters in fs_write_file
- DELETE everything in this function

**Delete `prompt_workflows()` (Lines 417-591, 175 lines)**
- This teaches USE CASES and complete workflows
- Examples like "create project structure" are workflow patterns, not tool features
- All features shown are covered by basic + append scenarios
- DELETE everything in this function

**Delete `prompt_comprehensive()` (Lines 592-836, 245 lines)**
- This is pure duplication with decorative `===` headers
- Repeats basic scenario content (lines correspond to basic concepts)
- Contains the same "MODE: REWRITE" and "MODE: APPEND" sections verbatim
- Decorative section headers add no value
- DELETE everything in this function

---

## Implementation Steps

### Step 1: Update Routing Logic (Lines 16-23)

**Current routing** (6 scenarios):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("append") => prompt_append(),
        Some("code_files") => prompt_code_files(),      // DELETE
        Some("config_files") => prompt_config_files(),  // DELETE
        Some("workflows") => prompt_workflows(),        // DELETE
        _ => prompt_comprehensive(),                    // DELETE
    }
}
```

**After trimming** (2 scenarios):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("append") => prompt_append(),
        _ => prompt_basic(),
    }
}
```

**Changes**:
- Remove 3 match arms (code_files, config_files, workflows)
- Change default from `prompt_comprehensive()` to `prompt_basic()`
- Keep only 2 scenario branches

### Step 2: Update Prompt Arguments Descriptor (Lines 25-31)

**Current** (6 scenarios listed):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, append, code_files, config_files, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

**After trimming** (2 scenarios listed):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, append)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Changes**:
- Update description string to list only "basic, append"
- Remove mention of code_files, config_files, workflows

### Step 3: Trim `prompt_basic()` Function (Lines 44-117)

**Current**: 74 lines
**Target**: 85-90 lines (may slightly expand due to better organization)

**Trim strategy**:
- Keep the user question and core explanation
- Consolidate "WRITING FILES" examples (reduce from current structure)
- Keep exact code examples but reduce surrounding explanation
- Consolidate "WRITE MODES" to 4-5 lines maximum
- Keep "PARAMETERS" section intact (clear and concise already)
- Keep "PATH HANDLING" and "CONTENT HANDLING" (essential)
- For "COMMON PATTERNS", keep 3 key patterns maximum
- Keep "ERROR HANDLING" section

**What NOT to trim**:
- Code examples (essential for understanding)
- The WHEN TO USE section (important context)
- Parameter documentation

### Step 4: Trim `prompt_append()` Function (Lines 118-205)

**Current**: 88 lines
**Target**: 85-95 lines (approximately same size)

**Trim strategy**:
- Keep the user question (asking about append without overwriting)
- Keep the "Use mode: append" introduction
- Reduce examples from 4 to 3 (keep log, data, continuous logging)
- Consolidate "APPEND PATTERNS" explanation
- Keep "WHEN TO APPEND" and "WHEN NOT TO APPEND" (crucial for understanding)
- Keep "NEWLINE HANDLING" (absolutely critical - this is where users fail)
- Condense "RESPONSE" section (can reference basic for structure)
- Keep "COMPARISON: APPEND vs REWRITE" (essential differentiation)
- Trim "BEST PRACTICES" to 5 items maximum

**What NOT to trim**:
- Newline handling explanation (this is the key learning point)
- The append vs rewrite comparison (essential understanding)
- All code examples

### Step 5: Delete Scenario Functions

**Delete these entire functions** (replace with nothing):
1. `prompt_code_files()` - Lines 206-306 (101 lines)
2. `prompt_config_files()` - Lines 307-416 (110 lines)
3. `prompt_workflows()` - Lines 417-591 (175 lines)
4. `prompt_comprehensive()` - Lines 592-836 (245 lines)

**Method**: Use `fs_edit_block` to remove each function in order, or manually delete these line ranges.

---

## Code Examples: Before and After

### Before: Routing (6 scenarios)

```rust
impl PromptProvider for WriteFilePrompts {
    type PromptArgs = FsWriteFilePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("append") => prompt_append(),
            Some("code_files") => prompt_code_files(),
            Some("config_files") => prompt_config_files(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }
}
```

### After: Routing (2 scenarios)

```rust
impl PromptProvider for WriteFilePrompts {
    type PromptArgs = FsWriteFilePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("append") => prompt_append(),
            _ => prompt_basic(),
        }
    }
}
```

---

## Success Criteria

All of these criteria MUST be met:

1. **File size**: 170-220 lines total (measured with `wc -l`)
2. **Scenario functions**: Exactly 2 exist (prompt_basic and prompt_append)
3. **Deleted scenarios**: No code_files, config_files, workflows, or comprehensive functions
4. **No use-case scenarios**: File teaches tool FEATURES, not application examples
5. **No decorative headers**: No `═══` or `---` decorative separators
6. **Routing updated**: match statement has only 2 arms (append, default)
7. **Scenario list updated**: "Scenario to show (basic, append)" in prompt_arguments
8. **Parameter documentation**: All 3 parameters (path, content, mode) clearly explained
9. **Mode explanation**: Both rewrite and append modes explicitly covered
10. **Newline handling**: Append newline behavior explained clearly (no auto-newline)
11. **Response structure**: Success/path/bytes_written fields documented
12. **Error handling**: Error cases mentioned
13. **Common patterns**: 3-5 essential patterns per scenario
14. **Readable in 3 minutes**: All core concepts understood in quick read

---

## Validation Checklist

After completing edits, verify these measurements:

1. **Line count**:
   ```bash
   wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/write_file/prompts.rs
   # Should output: 180-220 (approximately)
   ```

2. **Function count**:
   ```bash
   grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/write_file/prompts.rs | wc -l
   # Should output: 2
   ```

3. **Deleted scenarios**:
   ```bash
   grep -E "(prompt_code_files|prompt_config_files|prompt_workflows|prompt_comprehensive)" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/write_file/prompts.rs
   # Should output: (no results)
   ```

4. **Routing updated**:
   ```bash
   sed -n '16,23p' /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/write_file/prompts.rs
   # Should show only 2 scenario matches
   ```

5. **No decorative headers**:
   ```bash
   grep "═══\|^---$" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/write_file/prompts.rs
   # Should output: (no results)
   ```

---

## Reference Standard

This task follows the **Complexity 2 template** established in PRECURSOR_02_fs_read_file. The resulting file serves as the reference standard for all simple CRUD tool prompts across the 64+ Complexity 2 tools. The pattern is:

- **1 basic scenario** (~80-95 lines) showing core functionality
- **1 optional advanced scenario** (~80-95 lines) showing special modes or parameters
- **No use-case scenarios** (those are taught by examples, not separate prompts)
- **No comprehensive scenario** (prevents duplication)
- **Total**: 150-220 lines, readable end-to-end in 3-5 minutes

This task is complete when the fs_write_file file matches this pattern.
