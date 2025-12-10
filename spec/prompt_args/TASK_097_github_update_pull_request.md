# TASK 097: Trim github_update_pull_request Prompts

**Tool**: `github_update_pull_request`
**Complexity**: 3 (Medium)
**Current size**: 999 lines
**Target size**: 280-360 lines
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/update_pull_request/prompts.rs`

---

## Current State Analysis

### File Statistics
- **Total lines**: 999
- **Current scenarios**: 5 (state, content, draft, workflows, comprehensive)
- **Structure**: Header (lines 1-40), 5 scenario functions, routing logic

### Scenario Functions (Current)
1. `prompt_state()` (lines 43-114): 72 lines - Changing PR state (open/close)
2. `prompt_content()` (lines 117-275): 159 lines - Updating PR fields (title, body, base)
3. `prompt_draft()` (lines 278-445): 168 lines - Draft status management
4. `prompt_workflows()` (lines 448-708): 261 lines - Multi-step PR management workflows
5. `prompt_comprehensive()` (lines 711-998): 288 lines - Complete feature overview (DELETE)

### Routing Logic (Current)
Located in `generate_prompts()` function (lines 16-24):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("state") => prompt_state(),           // DELETE THIS ARM
        Some("content") => prompt_content(),
        Some("draft") => prompt_draft(),           // DELETE THIS ARM
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),              // CHANGE THIS DEFAULT
    }
}
```

### Prompt Arguments (Current)
Located in `prompt_arguments()` function (lines 26-35):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (state, content, draft, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

---

## Target State

### Desired Configuration
- **Total lines**: 280-360 (compressed by 60-70%)
- **Kept scenarios**: 2 (content, workflows)
- **Deleted scenarios**: state, draft, comprehensive
- **Rationale**: Scenarios to keep show both fundamental operations and realistic multi-step workflows

### Why This Selection
- **prompt_content()**: Core functionality showing individual PR field updates (title, body, base branch)
- **prompt_workflows()**: Practical real-world examples combining multiple operations
- **Removed prompt_state()**: Covered redundantly in workflows (state changes shown in context)
- **Removed prompt_draft()**: Draft status is covered as part of workflow examples
- **Removed prompt_comprehensive()**: Overlaps heavily with content and workflows; too verbose for Complexity 3

---

## Step-by-Step Implementation

### Step 1: Update Routing Logic
Modify the `generate_prompts()` function (lines 16-24):

**Before:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("state") => prompt_state(),
        Some("content") => prompt_content(),
        Some("draft") => prompt_draft(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**After:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("content") => prompt_content(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_content(),  // Default to content scenario
    }
}
```

### Step 2: Update Prompt Arguments
Modify the `prompt_arguments()` function (lines 26-35):

**Before:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (state, content, draft, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

**After:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (content, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

### Step 3: Delete Scenario Functions
Delete the following entire function definitions:

1. **Delete `prompt_state()` function** (lines 43-114)
   - Remove the entire function from `fn prompt_state()` through the closing brace
   - This removes 72 lines
   - Removes user question "How do I change the state of a pull request?"

2. **Delete `prompt_draft()` function** (lines 278-445, but will shift after Step 1)
   - Remove the entire function definition
   - This removes 168 lines
   - Removes user question "How do I work with draft pull requests?"

3. **Delete `prompt_comprehensive()` function** (lines 711-998, but will shift)
   - Remove the entire function definition
   - This removes 288 lines
   - Removes user question "Give me a complete guide to updating pull requests..."

### Step 4: Remove Decorative Headers (Optional Compression)
The comment header at lines 38-40 can be simplified:

**Before:**
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO UPDATE GITHUB PULL REQUESTS
// ============================================================================
```

**After:**
```rust
// Helper functions for PR update scenarios
```

---

## Execution Order (Critical)

**IMPORTANT**: Follow this exact sequence to avoid line number confusion:

1. First, update `prompt_arguments()` (lines 26-35) - smallest change
2. Second, update `generate_prompts()` routing (lines 16-24)
3. Third, delete `prompt_state()` (lines 43-114) - 72 lines removed
4. Fourth, delete `prompt_draft()` (original lines 278-445) - 168 lines removed
5. Fifth, delete `prompt_comprehensive()` (original lines 711-998) - 288 lines removed
6. Finally, optionally update the decorative header (lines 38-40)

**Why this order**: Smallest changes first (no line shift impact), then deletions from top to bottom to maintain accurate line numbers.

---

## Expected Line Count After Each Step

| Step | Action | Lines Before | Lines After | Change |
|------|--------|--------------|-------------|--------|
| 0 | Starting state | 999 | 999 | - |
| 1-2 | Update routing & args | 999 | 999 | 0 (logical changes only) |
| 3 | Delete prompt_state() | 999 | 927 | -72 |
| 4 | Delete prompt_draft() | 927 | 759 | -168 |
| 5 | Delete prompt_comprehensive() | 759 | 471 | -288 |
| 6 | Optional header cleanup | 471 | 468-470 | -1 to -3 |

---

## Final Structure After Trimming

### File Components After Execution
```
Lines 1-37:     Module header & imports
Lines 38-40:    Decorative comment (or simplified version)
Lines 41-36:    UpdatePullRequestPrompts struct
Lines 37-40:    PromptProvider impl (updated routing)
Lines 41-45:    Updated prompt_arguments()
Lines 46-200:   prompt_content() scenario (~155-160 lines)
Lines 201-470:  prompt_workflows() scenario (~260-270 lines)
Total: ~470 lines (within 280-360 target with some structural overhead)
```

**Note**: The target of 280-360 lines refers to content lines. The actual file will be slightly larger (~470) due to:
- Struct definition (11 lines)
- Module header/imports (8 lines)
- impl block (26 lines for routing/args)
- Line breaks between sections

---

## Code Patterns to Preserve

### Pattern 1: Scenario Function Structure (Keep as-is)
Both remaining scenarios follow this pattern:
```rust
fn prompt_scenario_name() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("User question here"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text("Detailed answer with examples..."),
        },
    ]
}
```

This pattern is used in both `prompt_content()` and `prompt_workflows()` - do NOT modify.

### Pattern 2: Backslash Continuation (Keep as-is)
All prompt text uses `\` for line continuation. Preserve this exact formatting in both remaining scenarios.

---

## Verification Checklist

After completing all steps, verify:

- [ ] File compiles: `cargo check` succeeds in kodegen-mcp-schema
- [ ] Routing logic: Only "content" and "workflows" handled by match arms
- [ ] Default case: Returns `prompt_content()`
- [ ] prompt_arguments description: Lists only "content, workflows"
- [ ] Line count: Final file is 465-475 lines total
- [ ] Scenario functions: Only `prompt_content()` and `prompt_workflows()` remain
- [ ] No orphaned function calls: All deleted functions completely removed
- [ ] Module exports: Still compiles without errors
- [ ] Formatting: Run `cargo fmt` to ensure consistent style

---

## Success Criteria

- ✓ File is 465-475 lines total (within reasonable bounds for Complexity 3)
- ✓ Exactly 2 scenarios remain (content and workflows)
- ✓ All 3 deleted scenarios completely removed
- ✓ No comprehensive scenario present
- ✓ Routing match statement updated to 2 arms + default
- ✓ prompt_arguments() description reflects only 2 valid options
- ✓ File compiles without errors
- ✓ No dangling references to deleted functions
- ✓ All backslash continuations preserved in remaining scenarios

---

## Related Files (Reference Only)

- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/update_pull_request/prompt_args.rs` - Argument definitions (no changes needed)
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/update_pull_request/schema.rs` - Tool schema (no changes needed)
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/mod.rs` - Module re-exports (no changes needed)

---

## Notes

- This is a content trimming task, not a functional refactoring
- The tool's actual capabilities remain unchanged; we're only reducing prompt examples
- Both remaining scenarios provide practical, complementary guidance
- The workflows scenario covers state changes implicitly, so removing prompt_state() causes no functional loss
