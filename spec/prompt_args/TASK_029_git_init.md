# TASK 029: Trim git_init Prompts

**Tool**: `git_init`  
**Complexity**: 2 (Simple)  
**Current size**: 673 lines  
**Target size**: 170-220 lines (2 scenarios)  
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/init/prompts.rs`

---

## Current State Analysis

The prompts.rs file contains **5 prompt scenarios**:

| Scenario | Lines | Type | Action |
|----------|-------|------|--------|
| `prompt_basic()` | ~44 | Core usage | KEEP |
| `prompt_options()` | ~85 | Feature details | DELETE |
| `prompt_bare()` | ~118 | Specialized use case | KEEP |
| `prompt_workflows()` | ~233 | Complete workflows (use-case examples) | DELETE |
| `prompt_comprehensive()` | ~186 | Comprehensive guide (default fallback) | DELETE |
| **Total** | **673** | | |

### Routing Logic (Lines 18-25)

The `generate_prompts()` method currently routes scenarios:

```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("options") => prompt_options(),
    Some("bare") => prompt_bare(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

The `prompt_arguments()` method (Lines 27-37) documents available scenarios:

```rust
description: Some("Scenario to show (basic, options, bare, workflows)".to_string()),
```

---

## Step-by-Step Implementation

### Step 1: Delete Unnecessary Scenarios

Delete the following function definitions entirely (do NOT leave stubs):

**DELETE `prompt_options()` function** (approx. lines 132-220):
- Covers init parameter options like `initial_branch`, `template`, `git_dir`, `quiet`
- This is a feature-level explanation that belongs in comprehensive reference
- Not a core workflow

**DELETE `prompt_workflows()` function** (approx. lines 349-552):
- Contains 5 complete workflows: new project, existing project, remote setup, monorepo, template-based
- These are use-case examples that make the file bloated
- Basic and bare scenarios provide sufficient coverage

**DELETE `prompt_comprehensive()` function** (approx. lines 554-673):
- The default/fallback scenario when no scenario is specified
- Duplicate/redundant content already covered in basic + bare
- Will be replaced with prompt_basic as the new default

### Step 2: Update Routing Logic

Replace the match statement in `generate_prompts()` (lines 18-25):

**FROM:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("options") => prompt_options(),
        Some("bare") => prompt_bare(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**TO:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("bare") => prompt_bare(),
        _ => prompt_basic(),
    }
}
```

**Why:** Default fallback now points to `prompt_basic()` instead of the deleted `prompt_comprehensive()`. This ensures AI agents get core usage documentation when no scenario is specified.

### Step 3: Update Scenario Documentation

Replace the `prompt_arguments()` function (lines 27-37):

**FROM:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, options, bare, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

**TO:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, bare)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Why:** The description string documents available scenario options. Must match the scenarios kept in the routing match statement.

### Step 4: Verify Kept Scenarios

The following two scenarios should remain UNCHANGED:

**`prompt_basic()`** (lines 40-105):
- Covers: Creating new Git repository, initializing in directory, response structure
- Key concepts: .git/ directory structure, initial branch, verification with git_status
- Importance: Core workflow for any agent using git_init
- Line count: ~65 lines

**`prompt_bare()`** (lines 107-222):
- Covers: What bare repositories are, when to use them, naming conventions, team workflows
- Key concepts: Bare vs normal repos, use cases (server, collaboration, backup), setup patterns
- Importance: Critical for understanding server-side repository setup
- Line count: ~115 lines

**Keep these functions exactly as they are.** Do not modify their content, structure, or line counts.

---

## Expected Result

After completing all steps, the file should have:

- **Lines 1-6**: Module documentation and imports (unchanged)
- **Lines 8-11**: `InitPrompts` struct definition (unchanged)
- **Lines 13-32**: Updated `impl PromptProvider` block with new `generate_prompts()` routing
- **Lines 34-44**: Updated `prompt_arguments()` function
- **Lines 46-52**: Comment header for prompt functions
- **Lines 54-120**: `prompt_basic()` function (unchanged)
- **Lines 122-237**: `prompt_bare()` function (unchanged)

**Total: Approximately 200-210 lines**

This falls within the target range of 170-220 lines.

---

## Verification Checklist

Before considering the task complete, verify ALL of the following:

- [ ] File compiles without errors: `cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Exactly 2 scenarios remain (basic and bare)
- [ ] No function definitions for `prompt_options()`, `prompt_workflows()`, or `prompt_comprehensive()`
- [ ] Match statement in `generate_prompts()` has exactly 3 arms: Some("basic"), Some("bare"), and _ (default)
- [ ] Default case (_ => prompt_basic()) correctly set
- [ ] `prompt_arguments()` description string says "Scenario to show (basic, bare)"
- [ ] Total line count: 170-220 lines (use `wc -l prompts.rs` to verify)
- [ ] File ends cleanly with no dangling braces or incomplete code

---

## Success Criteria

The task is complete when:

1. **Size**: File is exactly 170-220 lines (verified with `wc -l`)
2. **Scenarios**: Only 2 scenarios remain (basic and bare)
3. **Routing**: Match statement in `generate_prompts()` routes correctly
4. **Documentation**: `prompt_arguments()` accurately lists available scenarios
5. **Code quality**: File compiles and passes clippy linting
6. **No deleted code remains**: All references to deleted scenarios removed from routing
