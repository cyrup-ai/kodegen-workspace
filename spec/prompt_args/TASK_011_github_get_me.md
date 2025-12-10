# TASK 011: Trim github_get_me Prompts

**Tool**: `github_get_me`
**Complexity**: 1 (Trivial)
**Current size**: 788 lines (5 scenarios + routing logic)
**Target size**: 100 lines (1 scenario only)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_me/prompts.rs`

---

## Current State Analysis

The file currently contains:
- Module-level imports and documentation (lines 1-45)
- GetMePrompts struct implementing PromptProvider (lines 14-34)
- Router with 5 scenario matches: basic, verification, workflows, integration, comprehensive (lines 17-24)
- prompt_arguments() declaring scenario parameter (lines 26-32)
- Helper comment block (lines 44-45)
- 4 scenario functions to DELETE:
  - `prompt_verification()` (~120 lines): Authentication verification patterns
  - `prompt_workflows()` (~210 lines): Identity workflows and username usage
  - `prompt_integration()` (~180 lines): Integration with other GitHub operations
  - `prompt_comprehensive()` (~170 lines): Comprehensive guide with decorative headers (====)
- 1 scenario function to KEEP:
  - `prompt_basic()` (~60 lines): Basic user info retrieval with usage, response structure, fields, auth, when to use

**Line count breakdown:**
- Module header + imports: ~10 lines
- GetMePrompts impl: ~18 lines
- Helper comment: ~2 lines
- prompt_basic(): ~60 lines
- **Target total: ~100 lines**

---

## Implementation Steps

### Step 1: Simplify the Router (Lines 17-24)

**BEFORE:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("verification") => prompt_verification(),
        Some("workflows") => prompt_workflows(),
        Some("integration") => prompt_integration(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER:**
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

**Why:** Single scenario means no branching logic needed. The underscore prefix on args indicates the parameter is intentionally unused but kept for trait compatibility.

---

### Step 2: Simplify prompt_arguments() (Lines 26-32)

**BEFORE:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, verification, workflows, integration)".to_string()),
            required: Some(false),
        }
    ]
}
```

**AFTER:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![]
}
```

**Why:** With only one scenario, no arguments are needed. Return empty vector to indicate no parameters are accepted.

---

### Step 3: Trim Helper Comment (Lines 44-45)

**BEFORE:**
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GITHUB_GET_ME
// ============================================================================
```

**AFTER:**
```rust
/// Basic user info retrieval
```

**Why:** Remove decorative headers. The doc comment on the function itself is sufficient.

---

### Step 4: Delete 4 Functions (Lines 108-788)

**DELETE ENTIRELY:**
- `fn prompt_verification()` at ~line 108
- `fn prompt_workflows()` at ~line 228
- `fn prompt_integration()` at ~line 441
- `fn prompt_comprehensive()` at ~line 621

These functions contain:
- Extra scenarios not needed
- Redundant information
- Decorative headers (========= separators)
- Best practices sections (covered in basic)
- Multi-workflow examples (not in basic scenario)

**Result after deletion:** File ends immediately after prompt_basic() function closes, at approximately line 110.

---

## Final File Structure

The trimmed file will contain (in order):

1. **Module header** (~10 lines)
   - File docstring
   - Use statements for imports
   - Brief documentation

2. **GetMePrompts struct** (~2 lines)
   - Pub struct definition only

3. **PromptProvider impl** (~18 lines)
   - type PromptArgs declaration
   - generate_prompts() returning prompt_basic() directly
   - prompt_arguments() returning empty Vec

4. **Concise helper comment** (~1 line)
   - "/// Basic user info retrieval"

5. **prompt_basic() function** (~60 lines)
   - User-Assistant message pair
   - Content includes:
     - BASIC USAGE
     - RESPONSE STRUCTURE (JSON example)
     - KEY FIELDS explanation
     - NO PARAMETERS REQUIRED
     - AUTHENTICATION details
     - COMMON USE CASES
     - RATE LIMITING info
     - WHEN TO USE section

---

## Success Criteria

The completed file MUST satisfy ALL criteria:

- **Line count**: 90-110 lines total (count every line including blanks)
- **Scenario count**: EXACTLY 1 scenario (prompt_basic only)
- **Routing logic**: Single direct call to prompt_basic(), no match statement
- **No comprehensive**: The decorative header scenario is completely removed
- **No decorative headers**: No ===== separators or section dividers
- **No best practices sections**: These are in comprehensive, remove them
- **No multi-workflow examples**: Workflows scenario is deleted
- **No verification patterns**: Verification scenario is deleted
- **No integration patterns**: Integration scenario is deleted
- **prompt_arguments() empty**: Returns Vec::new() or vec![]
- **Trait implementation intact**: PromptProvider trait still correctly implemented
- **File compiles**: Run `cargo check` in packages/kodegen-mcp-schema

---

## Execution Checklist

1. **OPEN** file at `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_me/prompts.rs`

2. **FIND** and REPLACE generate_prompts method:
   - Search for: `fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {`
   - Find the entire match block
   - Replace with single-line implementation calling prompt_basic()

3. **REPLACE** prompt_arguments method:
   - Find: Current implementation with vec![PromptArgument {...}]
   - Replace with: vec![]

4. **DELETE** helper comment block:
   - Locate: Lines starting with `// ============================================================================`
   - Delete: All 3 lines of the comment block
   - Replace with: Single line `/// Basic user info retrieval`

5. **DELETE** prompt_verification() function:
   - Locate: `fn prompt_verification() -> Vec<PromptMessage> {`
   - Delete: Entire function from fn to closing brace

6. **DELETE** prompt_workflows() function:
   - Locate: `fn prompt_workflows() -> Vec<PromptMessage> {`
   - Delete: Entire function from fn to closing brace

7. **DELETE** prompt_integration() function:
   - Locate: `fn prompt_integration() -> Vec<PromptMessage> {`
   - Delete: Entire function from fn to closing brace

8. **DELETE** prompt_comprehensive() function:
   - Locate: `fn prompt_comprehensive() -> Vec<PromptMessage> {`
   - Delete: Entire function from fn to closing brace
   - This is the largest function (nearly 170 lines)

9. **VERIFY** file structure:
   - Count total lines (should be 90-110)
   - Verify prompt_basic() is last function in file
   - No blank lines after final closing brace

10. **TEST** compilation:
    ```bash
    cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
    cargo check
    ```

---

## Code Pattern Reference

### Routing Logic Transformation

The entire routing logic compresses from a match statement to a direct function call:

**OLD PATTERN (7 lines):**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("verification") => prompt_verification(),
        Some("workflows") => prompt_workflows(),
        Some("integration") => prompt_integration(),
        _ => prompt_comprehensive(),
    }
}
```

**NEW PATTERN (2 lines):**
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

### Arguments Transformation

**OLD PATTERN (8 lines):**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, verification, workflows, integration)".to_string()),
            required: Some(false),
        }
    ]
}
```

**NEW PATTERN (1 line in function body):**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![]
}
```

---

## Definition of Done

Task is complete when:

1. File compiles without errors: `cargo check` passes
2. File compiles without clippy warnings: `cargo clippy` passes with no warnings
3. Line count is 90-110 (verify with `wc -l`)
4. grep confirms single scenario: `grep -c "fn prompt_" prompts.rs` returns 1
5. grep confirms no comprehensive: `grep "comprehensive" prompts.rs` returns 0
6. grep confirms no decorative headers: `grep "=====" prompts.rs` returns 0
7. grep confirms prompt_arguments is empty: `grep -A 1 "fn prompt_arguments" prompts.rs` shows vec![]
8. grep confirms generate_prompts calls basic: `grep -A 1 "fn generate_prompts" prompts.rs` shows prompt_basic()

---

## Notes

- **Preserve the basic scenario content exactly as-is** - it already meets quality standards
- **Do NOT shorten prompt_basic()** - it's already appropriately sized
- The 100-line target includes ALL lines: module header, imports, blank lines, etc.
- Keep all use statements and module documentation intact
- The PromptProvider trait must remain fully implemented (all required methods present)
