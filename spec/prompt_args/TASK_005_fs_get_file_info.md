# TASK 005: Trim fs_get_file_info Prompts to Single Scenario

## Core Objective

Reduce the `fs_get_file_info` prompt file from 718 lines to a single, focused scenario of 90-110 lines total. This eliminates redundant prompt variants and creates a streamlined, AI-friendly prompt that covers essential file metadata retrieval patterns without unnecessary extensions.

---

## Current State Analysis

**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/get_file_info/prompts.rs`

**Current Structure**:
- Total lines: 718
- Scenarios implemented: 5
  - `prompt_basic()` (~120 lines): Core usage, response structure, field explanations, when to use, common patterns
  - `prompt_size_check()` (~170 lines): File size strategies and thresholds
  - `prompt_timestamps()` (~180 lines): Timestamp comparisons and cache invalidation
  - `prompt_verification()` (~200 lines): Pre-operation file verification workflows
  - `prompt_comprehensive()` (~440 lines): Complete reference guide (all of above combined)

**Problem**: Multiple overlapping scenarios create redundancy and bloat. The `prompt_basic()` contains the essential material needed to teach the AI agent how to use the tool effectively.

---

## Implementation Strategy

### Step 1: Identify the Target Scenario

**Keep**: `prompt_basic()` - This scenario is optimal because:
- Covers fundamental file metadata retrieval (the core use case)
- Explains response structure and all critical fields
- Provides "when to use" context
- Includes essential common patterns (file existence, size checks, type verification)
- Approximately 120 lines (can be condensed to 90-110)

**Delete Entirely**:
- `prompt_size_check()` - Specialized extension, move critical info to basic
- `prompt_timestamps()` - Specialized extension, covered in basic patterns
- `prompt_verification()` - Specialized extension, covered in basic patterns
- `prompt_comprehensive()` - Redundant aggregation of all above

### Step 2: Condense prompt_basic() to Target Size (90-110 Lines)

The condensed version must retain:
- Tool description and basic usage example (2-3 lines)
- Response structure JSON example (8-10 lines)
- Field explanations (20-25 lines)
- When to use section (5-7 lines)
- Common patterns (4-5 simple examples, 30-40 lines)

**Remove from prompt_basic()**:
- Decorative section separators (remove "============" lines)
- Redundant field descriptions (consolidate to concise explanations)
- Verbose pattern examples (keep only 4 essential patterns)
- Extended "best practices" sections (only keep essential guidance in each section)
- Duplicate explanations of concepts

### Step 3: Simplify PromptProvider Implementation

**Current** (lines 15-33):
```rust
impl PromptProvider for GetFileInfoPrompts {
    type PromptArgs = FsGetFileInfoPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("size_check") => prompt_size_check(),
            Some("timestamps") => prompt_timestamps(),
            Some("verification") => prompt_verification(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, size_check, timestamps, verification)".to_string()),
                required: Some(false),
            }
        ]
    }
}
```

**After** (simplified routing):
```rust
impl PromptProvider for GetFileInfoPrompts {
    type PromptArgs = FsGetFileInfoPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        // All scenarios route to prompt_basic as single source of truth
        let _ = args;
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![] // No scenario selection needed, single scenario
    }
}
```

**Rationale**: Since there is only one scenario, the match statement becomes unnecessary. The function simply ignores the args and returns the single prompt. The prompt_arguments() becomes empty because there are no arguments to configure.

### Step 4: Clean Up Module Structure

**Remove**:
- `fn prompt_size_check()` - DELETE ENTIRE FUNCTION (lines ~220-390)
- `fn prompt_timestamps()` - DELETE ENTIRE FUNCTION (lines ~392-570)
- `fn prompt_verification()` - DELETE ENTIRE FUNCTION (lines ~572-690)
- `fn prompt_comprehensive()` - DELETE ENTIRE FUNCTION (lines ~692-718)

**Keep**:
- Module documentation and use statements (lines 1-6)
- Struct documentation (lines 8-11)
- PromptProvider implementation (lines 13-37)
- Comment header for helper functions (lines 39-42)
- `fn prompt_basic()` - CONDENSE FROM 120 LINES TO 90-110 LINES (lines 44-onwards)

### Step 5: Condense prompt_basic() Content

**Pattern for condensed content**:

```
User Question (1-2 lines)
↓
Assistant Response with:
  1. Tool Description (2-3 lines)
  2. Basic Usage Example (1-2 lines)
  3. Response Structure (8-10 lines of JSON)
  4. Quick Field Explanations (15-20 lines)
  5. File vs Directory Difference (5-8 lines)
  6. When to Use (5-7 lines)
  7. Common Patterns (4-5 examples, ~30 lines)
```

**Keep these essential common patterns**:
1. Verify file exists
2. Check file size
3. Compare modification times
4. Verify file type (file vs directory)

**Example condensed pattern** (keep this level of detail):
```
COMMON PATTERNS:
1. Verify file exists:
   info = fs_get_file_info({"path": "/config.json"})
   // If successful, file exists

2. Check file size:
   info = fs_get_file_info({"path": "/data.csv"})
   if info.size_bytes < 1048576:  // < 1 MB
       process_file()
   else:
       handle_large_file()
```

---

## File Changes Required

### PRIMARY FILE: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/get_file_info/prompts.rs`

**Changes**:
1. Keep lines 1-42 (module docs, imports, struct definition, beginning of PromptProvider impl)
2. Replace PromptProvider::generate_prompts() match statement to simple routing (line 22-27)
3. Replace PromptProvider::prompt_arguments() to return empty vec (lines 30-38)
4. Keep lines 40-42 (comment header)
5. Condense prompt_basic() from ~120 lines to exactly 80-100 lines
6. Delete prompt_size_check() entirely
7. Delete prompt_timestamps() entirely
8. Delete prompt_verification() entirely
9. Delete prompt_comprehensive() entirely

**Expected Result**:
- Total file size: 90-110 lines
- Single prompt function: `prompt_basic()`
- Simple, non-routable implementation

### NO OTHER FILES CHANGE

The prompt_args.rs file does not need changes (it still defines FsGetFileInfoPromptArgs, but the args are no longer used).

---

## Code Pattern: Condensed prompt_basic() Structure

Here is the structural pattern for the condensed version. Line count guidance is in parentheses:

```rust
/// Basic file metadata retrieval (1 line)
fn prompt_basic() -> Vec<PromptMessage> { (1 line)
    vec![ (1 line)
        PromptMessage { (1 line)
            role: PromptMessageRole::User, (1 line)
            content: PromptMessageContent::text( (1 line)
                "How do I get file metadata using fs_get_file_info?", (1 line)
            ), (1 line)
        }, (1 line)
        PromptMessage { (1 line)
            role: PromptMessageRole::Assistant, (1 line)
            content: PromptMessageContent::text( (1 line)
                "The fs_get_file_info tool retrieves file metadata. (50-70 lines of condensed content)
                ...[CONTENT BELOW]...
                " (1 line)
            ), (1 line)
        }, (1 line)
    ] (1 line)
} (1 line)
```

**Content breakdown** (within the text string):
- Lines 1-3: Tool purpose and basic usage
- Lines 4-12: Response JSON structure example
- Lines 13-25: Field explanations (concise, one sentence each)
- Lines 26-30: File vs directory differences
- Lines 31-35: When to use bullet points
- Lines 36-80: Common patterns (4-5 examples)

---

## Definition of Done

The task is complete when:

1. **File Size**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/get_file_info/prompts.rs` is between 90-110 total lines
2. **Single Scenario**: Only one prompt function (`prompt_basic()`) exists in the file
3. **Routing**: `PromptProvider::generate_prompts()` routes to single function without match statement
4. **Arguments**: `PromptProvider::prompt_arguments()` returns empty Vec
5. **Content Coverage**: prompt_basic() includes:
   - ✓ Tool description and basic usage
   - ✓ Response structure (JSON example)
   - ✓ Field explanations (all 8-9 fields briefly explained)
   - ✓ File vs directory differences
   - ✓ When to use (5-7 reasons)
   - ✓ 4-5 essential common patterns with code examples
6. **Deleted Functions**: All of these are completely removed:
   - ✓ `prompt_size_check()`
   - ✓ `prompt_timestamps()`
   - ✓ `prompt_verification()`
   - ✓ `prompt_comprehensive()`
7. **No Regressions**: Code compiles with `cargo check` in the kodegen-mcp-schema package

---

## Key Principles

- **No Options**: This is a prescriptive task. Keep `prompt_basic()`, delete the others. No alternatives.
- **Preserve Essentials**: The condensed content must still teach the core use cases.
- **Ruthless Pruning**: Remove all decorative headers, extensive examples, and "best practices" lists.
- **Clarity Over Breadth**: One focused, high-quality prompt beats five scattered prompts.
- **Single Source of Truth**: `prompt_basic()` becomes THE way to learn fs_get_file_info.
