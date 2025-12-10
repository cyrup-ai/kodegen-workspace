# TASK 022: Trim git_branch_delete Prompts

**Tool**: `git_branch_delete`  
**Complexity**: 2 (Simple)  
**Current size**: 902 lines  
**Target size**: 170-220 lines  
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/branch_delete/prompts.rs`

---

## Current State Analysis

### File Composition (902 total lines)

The prompts.rs file contains a `BranchDeletePrompts` struct implementing the `PromptProvider` trait with the following structure:

- **Lines 1-47**: Imports, struct definition, and trait implementation header
- **Lines 48-80**: `generate_prompts()` match statement routing 5 scenarios to functions
- **Lines 81-90**: `prompt_arguments()` defining the "scenario" argument
- **Lines 91-94**: Decorator comment header
- **Lines 95-269**: `prompt_local()` function (~175 lines) - safe vs force deletion for local branches
- **Lines 270-469**: `prompt_remote()` function (~200 lines) - remote branch deletion workflows
- **Lines 470-699**: `prompt_cleanup()` function (~230 lines) - cleanup strategies (USE-CASE)
- **Lines 700-899**: `prompt_safety()` function (~200 lines) - safety practices (USE-CASE)
- **Lines 900-902**: `prompt_comprehensive()` function (~335 lines, truncated in typical view) - complete guide (COMPREHENSIVE)

### Scenario Classification

| Scenario | Type | Lines | Decision |
|----------|------|-------|----------|
| `prompt_local()` | Core/Basic | ~175 | KEEP |
| `prompt_remote()` | Core/Basic | ~200 | KEEP |
| `prompt_cleanup()` | Use-case | ~230 | DELETE |
| `prompt_safety()` | Use-case | ~200 | DELETE |
| `prompt_comprehensive()` | Comprehensive | ~335 | DELETE |

### Current Routing Logic (Lines 23-29)

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("local") => prompt_local(),
        Some("remote") => prompt_remote(),
        Some("cleanup") => prompt_cleanup(),
        Some("safety") => prompt_safety(),
        _ => prompt_comprehensive(),  // DEFAULT
    }
}
```

### Current Argument Definition (Lines 32-40)

```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (local, remote, cleanup, safety)".to_string()),
            required: Some(false),
        }
    ]
}
```

---

## Implementation Instructions

### Step 1: Delete Three Scenario Functions

Delete the following three functions entirely (including their doc comments and decorative content):

**A. Delete `prompt_cleanup()` function**
- Location: Lines 470-699 (entire function)
- Reason: Use-case scenario describing cleanup strategies
- Impact: Removes ~230 lines

**B. Delete `prompt_safety()` function**
- Location: Lines 700-899 (entire function)
- Reason: Use-case scenario describing safety practices and recovery
- Impact: Removes ~200 lines

**C. Delete `prompt_comprehensive()` function**
- Location: Lines 900-EOF (entire function to end of file)
- Reason: Comprehensive scenario, not a core teaching scenario
- Impact: Removes ~335 lines

### Step 2: Update the Match Statement in `generate_prompts()`

**Before (Lines 23-29):**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("local") => prompt_local(),
        Some("remote") => prompt_remote(),
        Some("cleanup") => prompt_cleanup(),
        Some("safety") => prompt_safety(),
        _ => prompt_comprehensive(),
    }
}
```

**After (Lines 23-29, UNCHANGED IN FORM BUT BEHAVIOR CHANGES):**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("local") => prompt_local(),
        Some("remote") => prompt_remote(),
        _ => prompt_local(),  // Change: Default to local instead of comprehensive
    }
}
```

**Why this change**: The default should be `prompt_local()` because it covers the most common use case (safe vs force deletion of local branches). This is more helpful to users than defaulting to a comprehensive guide.

### Step 3: Update the `prompt_arguments()` Function

**Before (Lines 32-40):**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (local, remote, cleanup, safety)".to_string()),
            required: Some(false),
        }
    ]
}
```

**After (Lines 32-40):**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (local, remote)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Why this change**: The description now accurately reflects only the two remaining scenarios.

### Step 4: Remove Decorative Header Comment

**Before (Lines 91-94):**
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO DELETE GIT BRANCHES SAFELY
// ============================================================================
```

**After (DELETE ENTIRELY):**
This decorative header is removed. The function comments on each scenario function remain as they are descriptive.

### Step 5: Execution Order

Perform the edits in this exact order to minimize confusion:

1. First, delete the entire `prompt_cleanup()` function
2. Then, delete the entire `prompt_safety()` function
3. Then, delete the entire `prompt_comprehensive()` function
4. Update the match statement default case (one line change)
5. Update the `prompt_arguments()` description string (one line change)
6. Remove the decorative header comment (3 lines)

---

## Code Patterns: Before and After

### Complete File Structure After Trimming

```
File header/imports (lines 1-90)
├── Lines 1-7: Language comments and use statements
├── Lines 8-15: Struct definition and impl block start
├── Lines 16-30: generate_prompts() [MODIFIED]
├── Lines 31-40: prompt_arguments() [MODIFIED]
├── Lines 41-42: Closing brace

Scenario functions (lines 43-N where N ≈ 470)
├── prompt_local() (~175 lines) [KEPT]
└── prompt_remote() (~200 lines) [KEPT]

End of file
```

### Specific Changes Summary

| Element | Current | After Change | Lines Affected |
|---------|---------|---------------|-----------------|
| generate_prompts default | `prompt_comprehensive()` | `prompt_local()` | Line ~28 |
| prompt_arguments description | "local, remote, cleanup, safety" | "local, remote" | Line ~37 |
| Decorative header | Present | Deleted | Lines ~91-94 |
| prompt_cleanup() | Present (~230 lines) | Deleted | ~230 lines removed |
| prompt_safety() | Present (~200 lines) | Deleted | ~200 lines removed |
| prompt_comprehensive() | Present (~335 lines) | Deleted | ~335 lines removed |

---

## Definition of Done

The task is complete when ALL the following criteria are met:

**File Size Requirement:**
- File total line count: **170-220 lines** (from current 902)
- Actual: Should be approximately 465-480 lines = initial 95 lines + 175 (local) + 200 (remote) + decorative removal = ~462 lines

**Scenario Requirements:**
- Exactly 2 scenarios remain: `prompt_local()` and `prompt_remote()`
- No `prompt_cleanup()` function exists
- No `prompt_safety()` function exists
- No `prompt_comprehensive()` function exists
- All function bodies are removed, not just gutted

**Routing Requirements:**
- `generate_prompts()` match statement handles: `Some("local")`, `Some("remote")`, and `_ => prompt_local()`
- Default case routes to `prompt_local()`, not `prompt_comprehensive()`
- No dead code or unreachable match arms

**Documentation Requirements:**
- `prompt_arguments()` correctly describes only "local, remote" scenarios
- No decorative header comments (like "HELPER FUNCTIONS - TEACH AI AGENTS...")
- Doc comments on the 2 remaining functions are preserved

**Code Quality Requirements:**
- File compiles with `cargo check` without errors
- No clippy warnings related to unused functions
- No syntax errors in Rust code
- Proper formatting maintained

**Verification Steps:**

Run these commands to verify completion:

```bash
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
wc -l src/git/branch_delete/prompts.rs  # Should show ~462-480 lines

cargo check  # Should compile without errors
cargo clippy  # Should show no warnings about the file
```

Search verification (from repo root):
```bash
grep -n "prompt_cleanup" packages/kodegen-mcp-schema/src/git/branch_delete/prompts.rs  # Should show 0 results
grep -n "prompt_safety" packages/kodegen-mcp-schema/src/git/branch_delete/prompts.rs   # Should show 0 results
grep -n "prompt_comprehensive" packages/kodegen-mcp-schema/src/git/branch_delete/prompts.rs  # Should show 0 results
```

File structure verification:
```bash
grep -n "^fn prompt_" packages/kodegen-mcp-schema/src/git/branch_delete/prompts.rs
# Should output exactly 2 lines:
# Line ~X: fn prompt_local() -> Vec<PromptMessage> {
# Line ~Y: fn prompt_remote() -> Vec<PromptMessage> {
```

---

## Key Points for Execution

1. **Do NOT modify the content of `prompt_local()` or `prompt_remote()` functions** - keep their complete user/assistant message pairs
2. **Do change the routing logic** - update the default case and scenario list
3. **Order matters** - delete functions bottom-to-top to avoid line number confusion
4. **Verify compilation** - the file must compile and pass clippy before considering the task done
5. **No test/documentation creation** - this is a trimming task only, no additions

---

## Success Indicators

After completing this task, the file will:
- Be approximately 462-480 lines (down from 902)
- Contain only 2 scenario functions instead of 5
- Route unknown scenarios to `prompt_local()` as the safe default
- Compile without errors or clippy warnings
- Still provide essential teaching for both local and remote branch deletion
