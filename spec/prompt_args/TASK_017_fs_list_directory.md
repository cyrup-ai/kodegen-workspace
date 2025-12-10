# TASK 017: Trim fs_list_directory Prompts

**Tool**: `fs_list_directory`
**Complexity**: 2 (Simple)
**Current size**: 814 lines
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/list_directory/prompts.rs`

---

## Current State Analysis

### File Structure
The prompts.rs file currently contains:
- **Header comments & imports** (lines 1-11): Rust module documentation and use statements
- **ListDirectoryPrompts struct** (lines 13-34): Public struct and PromptProvider implementation with routing logic
- **Five scenario functions** (lines 36-814): Functions that generate PromptMessage vectors for different use cases

### Current Scenarios (5 total)

| Scenario | Lines | Purpose | Status |
|----------|-------|---------|--------|
| `prompt_basic()` | ~95 | Core listing operation and response structure | KEEP |
| `prompt_hidden()` | ~210 | Hidden file parameter (include_hidden) handling | KEEP |
| `prompt_exploration()` | ~205 | Navigation patterns through directory trees | DELETE |
| `prompt_verification()` | ~220 | Before/after verification workflows | DELETE |
| `prompt_comprehensive()` | ~160+ | Complete guide with all sections and headers | DELETE |

### Routing Logic (Lines 17-23)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("hidden") => prompt_hidden(),
        Some("exploration") => prompt_exploration(),      // DELETE THIS CASE
        Some("verification") => prompt_verification(),    // DELETE THIS CASE
        _ => prompt_comprehensive(),                       // DELETE THIS CASE
    }
}
```

### Prompt Arguments (Lines 25-34)
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, hidden, exploration, verification)".to_string()),
            // DESCRIPTION NEEDS UPDATE TO ONLY: "basic, hidden"
            required: Some(false),
        }
    ]
}
```

---

## Implementation Plan

### Step 1: Update Routing Logic (Lines 17-23)

**BEFORE:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("hidden") => prompt_hidden(),
        Some("exploration") => prompt_exploration(),
        Some("verification") => prompt_verification(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("hidden") => prompt_hidden(),
        _ => prompt_basic(),  // Default to basic instead of comprehensive
    }
}
```

**Actions:**
- Delete 2 scenario match cases: `Some("exploration")` and `Some("verification")`
- Change default case `_ => prompt_comprehensive()` to `_ => prompt_basic()`
- This reduces routing from 5 cases to 3 cases (2 explicit + 1 default)

### Step 2: Update Prompt Arguments Description (Line 31)

**BEFORE:**
```rust
description: Some("Scenario to show (basic, hidden, exploration, verification)".to_string()),
```

**AFTER:**
```rust
description: Some("Scenario to show (basic, hidden)".to_string()),
```

**Actions:**
- Remove `exploration, verification` from the description
- Keep only `basic, hidden` as valid options

### Step 3: Delete prompt_exploration() Function

**Location**: After `prompt_hidden()` function ends, approximately line 265
**Size**: ~205 lines including function header and closing brace
**Content to delete**: Entire function starting with `/// Navigation and exploration patterns` comment through the closing `}` of `prompt_exploration()`

**Identify by markers:**
- Starts with: `/// Navigation and exploration patterns`
- Ends with: `fn prompt_exploration() -> Vec<PromptMessage> { ... }`
- Look for: The function contains text about "NAVIGATION PATTERN", "EXPLORATION WORKFLOW", "PRACTICAL EXAMPLE - Finding Configuration", etc.

### Step 4: Delete prompt_verification() Function

**Location**: After deleted `prompt_exploration()`, approximately line 470
**Size**: ~220 lines including function header and closing brace
**Content to delete**: Entire function starting with `/// Verification workflows` comment through the closing `}` of `prompt_verification()`

**Identify by markers:**
- Starts with: `/// Verification workflows`
- Ends with: `fn prompt_verification() -> Vec<PromptMessage> { ... }`
- Look for: The function contains text about "VERIFICATION PATTERN", "VERIFICATION WORKFLOW 1", "Before operations:", etc.

### Step 5: Delete prompt_comprehensive() Function

**Location**: After deleted `prompt_verification()`, approximately line 690
**Size**: ~160+ lines (the remainder of the file)
**Content to delete**: Entire function starting with `/// Comprehensive guide covering all scenarios` comment through the final closing `}` at end of file

**Identify by markers:**
- Starts with: `/// Comprehensive guide covering all scenarios`
- Ends with: Final `}` closing the function (last line of file)
- Look for: The function contains text about "BASIC USAGE", "PARAMETERS", "HIDDEN FILES", "INTERPRETING RESULTS", "NAVIGATION & EXPLORATION", "VERIFICATION WORKFLOWS", "COMMON PATTERNS", "COMBINING WITH OTHER TOOLS", "ERROR HANDLING", "BEST PRACTICES", "QUICK REFERENCE"

---

## Exact Implementation Steps

### Using fs_edit_block / Manual Deletion

Execute these changes in order:

1. **Edit routing function** (replace lines 17-23):
   - Find the `fn generate_prompts()` function
   - Replace the entire match statement with the 3-case version shown above

2. **Edit argument description** (replace line 31):
   - Find `description: Some("Scenario to show...`
   - Change the string to only list `"basic, hidden"`

3. **Delete exploration scenario**:
   - Find the line `/// Navigation and exploration patterns` comment
   - Delete from that line through the entire `prompt_exploration()` function
   - This removes approximately 205 lines

4. **Delete verification scenario**:
   - Find the line `/// Verification workflows` comment
   - Delete from that line through the entire `prompt_verification()` function
   - This removes approximately 220 lines

5. **Delete comprehensive scenario**:
   - Find the line `/// Comprehensive guide covering all scenarios` comment
   - Delete from that line through the final closing brace `}` of the function
   - This removes approximately 160+ lines (rest of file)

---

## Code Patterns: Before and After

### Before (Routing - 5 scenarios)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("hidden") => prompt_hidden(),
        Some("exploration") => prompt_exploration(),
        Some("verification") => prompt_verification(),
        _ => prompt_comprehensive(),
    }
}
```

### After (Routing - 2 scenarios)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("hidden") => prompt_hidden(),
        _ => prompt_basic(),
    }
}
```

### Before (Arguments - 4 scenario options)
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, hidden, exploration, verification)".to_string()),
            required: Some(false),
        }
    ]
}
```

### After (Arguments - 2 scenario options)
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, hidden)".to_string()),
            required: Some(false),
        }
    ]
}
```

---

## Remaining File Structure After Trim

After all deletions, the file will contain:

1. **Lines 1-11**: Module header and imports (unchanged)
2. **Lines 13-34**: ListDirectoryPrompts struct and PromptProvider impl
   - Updated routing logic in generate_prompts()
   - Updated description in prompt_arguments()
3. **Lines 36-130**: prompt_basic() function
   - User/Assistant messages about basic directory listing
   - RESPONSE STRUCTURE JSON example
   - INTERPRETING RESULTS explanation
   - ENTRY TYPES description
   - COMMON PATTERNS (4 patterns)
   - WHEN TO USE section
   - DEFAULT BEHAVIOR explanation
   - ERROR CASES section
4. **Lines 131-350**: prompt_hidden() function
   - User/Assistant messages about hidden files
   - HIDDEN FILES EXPLAINED section
   - DEFAULT vs INCLUDING HIDDEN comparison with examples
   - COMMON HIDDEN FILES BY PURPOSE (5 categories)
   - WHEN TO INCLUDE HIDDEN (5 scenarios)
   - WHEN NOT TO INCLUDE HIDDEN section
   - WORKFLOW EXAMPLE (2 steps)
   - IMPORTANT NOTES (6 points)

**DELETED FUNCTIONS:**
- ~~prompt_exploration()~~ - Navigation patterns (~205 lines)
- ~~prompt_verification()~~ - Verification workflows (~220 lines)
- ~~prompt_comprehensive()~~ - Complete guide (~160+ lines)

---

## Success Criteria (Definition of Done)

The task is complete when ALL of the following are true:

1. **Line count**: Final file is **170-220 lines total**
   - Current: 814 lines
   - Target: ~200 lines (approximately)
   - Tolerance: 170-220 lines acceptable

2. **Scenario count**: Exactly **2 scenarios remain**
   - `prompt_basic()` - core operation
   - `prompt_hidden()` - include_hidden parameter
   - All others deleted

3. **No comprehensive scenario**: 
   - Comprehensive function completely removed
   - No references to comprehensive in routing or arguments
   - No decorative headers like "=============" sections remain

4. **Routing updated**:
   - Match statement has only 3 cases (basic, hidden, default)
   - Default case returns `prompt_basic()` not `prompt_comprehensive()`
   - No exploration or verification cases

5. **Arguments updated**:
   - Description string lists only "(basic, hidden)"
   - No mention of exploration or verification

6. **Code compiles**:
   - File is valid Rust syntax
   - No orphaned function references
   - PromptProvider trait still properly implemented

7. **No decorative elements**:
   - All "=============" section headers removed
   - All "BOLD SECTION:" style formatting removed from main content
   - Markdown-style ## headers replaced with inline labels if necessary
   - But keep legitimate prompt content (JSON examples, explanations)

---

## Verification Checklist

After implementation, verify:

- [ ] File loads without syntax errors
- [ ] `cargo check` passes in kodegen-mcp-schema package
- [ ] File is 170-220 lines (count with `wc -l` or equivalent)
- [ ] Only prompt_basic() and prompt_hidden() functions remain
- [ ] Match statement in generate_prompts() has 3 cases total
- [ ] prompt_arguments() description mentions only "basic, hidden"
- [ ] No references to "exploration" in entire file
- [ ] No references to "verification" in entire file
- [ ] No references to "comprehensive" in entire file
- [ ] File ends with closing brace of prompt_hidden() function

---

## Reference Materials

See **PRECURSOR_02_fs_read_file.md** for Complexity 2 template patterns.

The Complexity 2 approach for this file:
- Keep only essential scenarios (basic + one parameter variation)
- Remove use-case scenarios (exploration, verification, comprehensive)
- Simplify routing logic from 5 options to 2 options
- Reduce file from ~800 lines to ~200 lines
- Maintain core educational value about the tool
