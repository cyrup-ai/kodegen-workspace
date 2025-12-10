# TASK 007: Trim git_open Prompts

**Tool**: `git_open`
**Complexity**: 1 (Trivial)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/open/prompts.rs`
**Current Size**: 221 lines (4 scenarios)
**Target Size**: 90-110 lines (1 scenario)

---

## Objective

Reduce the `git_open` prompt module from 4 scenarios to 1 focused scenario. This improves code maintainability while preserving essential functionality for users opening existing Git repositories. The remaining scenario must comprehensively cover tool usage patterns without redundancy.

---

## Core Analysis

The current implementation in `prompts.rs` contains four prompt scenarios:

1. **prompt_basic()** (~55 lines) - How to open an existing repository
2. **prompt_status()** (~65 lines) - Repository state information after opening
3. **prompt_vs_discover()** (~95 lines) - Differentiation between git_open vs git_discover tools
4. **prompt_comprehensive()** (~95 lines) - Full feature coverage with examples and troubleshooting

### Why Keep `prompt_basic`

The `prompt_basic()` function is selected as the single retained scenario because:

- **Complete Coverage**: Addresses the primary user need (opening a repository)
- **Lean Structure**: Avoids redundancy found in comprehensive scenario
- **User-Centric**: Directly answers the core question without tangential comparisons
- **Scalability**: Can serve as foundation for future scenario expansion

The other scenarios are removed because:
- `prompt_status`: Overlaps with git_status tool (separate responsibility)
- `prompt_vs_discover`: Tool comparison belongs in discovery/help documentation, not this prompt
- `prompt_comprehensive`: Bloated with troubleshooting and edge cases outside Complexity 1 scope

---

## Implementation Path

### Step 1: Simplify PromptProvider Implementation

**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/open/prompts.rs`

**Current Code** (lines 12-21):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("status") => prompt_status(),
        Some("vs_discover") => prompt_vs_discover(),
        _ => prompt_comprehensive(),
    }
}

fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario: basic, status, vs_discover".to_string()),
            required: Some(false),
        }
    ]
}
```

**Required Change**:

Replace match statement with direct call to `prompt_basic()`:

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

Remove scenario argument since no routing is needed:

```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![]
}
```

**Rationale**: With a single scenario, the `PromptProvider` trait methods become simplified. The unused `args` parameter and scenario matching are eliminated. The `prompt_arguments()` method returns an empty vector since no arguments affect prompt generation.

### Step 2: Delete Redundant Scenario Functions

Remove these functions entirely from the file:

- `prompt_status()` (lines 101-150)
- `prompt_vs_discover()` (lines 152-212)  
- `prompt_comprehensive()` (lines 214-221)

**Deletion Impact**: These account for ~165 lines. After deletion, file will be ~55 lines (well under the 110 line target).

### Step 3: Preserve prompt_basic()

**Retain the entire `prompt_basic()` function** (current lines 33-100):

```rust
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I open an existing Git repository?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Open an existing Git repository with git_open:\n\n\
                 ```json\n\
                 {\"path\": \"/path/to/repository\"}\n\
                 ```\n\n\
                 This connects to a repository that already exists on disk (has a .git directory).\n\n\
                 Example:\n\
                 ```json\n\
                 {\"path\": \"/home/user/myproject\"}\n\
                 ```\n\n\
                 WHAT HAPPENS:\n\
                 - Verifies .git directory exists\n\
                 - Loads repository configuration\n\
                 - Returns current branch information\n\
                 - Checks if working directory is clean\n\n\
                 USE git_open WHEN:\n\
                 - You know the exact repository path\n\
                 - Repository was previously cloned or initialized\n\
                 - Need to perform git operations on existing repo"
            ),
        },
    ]
}
```

This function is complete and requires no modifications. The content structure naturally aligns with Complexity 1 requirements:
- **Tool Description**: "How do I open..." question and explanation of what it does
- **Basic Usage**: JSON syntax with `/path/to/repository` pattern
- **Response Structure** (implicit): Bullet points explaining what happens
- **When to Use**: Three concrete scenarios for appropriate usage

### Step 4: Final File Structure

After implementation, the file will have this structure:

```
1. File header comment (2 lines)
2. Module imports (3 lines)
3. OpenPrompts struct definition (1 line)
4. PromptProvider implementation (10 lines):
   - impl block (1 line)
   - generate_prompts method (2 lines)
   - prompt_arguments method (2 lines)
   - closing brace (1 line)
5. prompt_basic function (35-40 lines)

Total: ~54 lines
```

---

## Validation Checklist

After implementing changes, verify:

- [x] File compiles with `cargo check` from `packages/kodegen-mcp-schema/`
- [x] Line count is 45-65 lines (well within 90-110 target)
- [x] Only `prompt_basic()` function remains
- [x] `generate_prompts()` calls `prompt_basic()` directly without match
- [x] `prompt_arguments()` returns empty vector `vec![]`
- [x] No compilation warnings or errors
- [x] Tool routes to correct prompt via stdio routing (no changes needed to routing logic)

---

## Files Modified

- **`/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/git/open/prompts.rs`**
  - Lines deleted: 167 (old prompt_status, prompt_vs_discover, prompt_comprehensive functions)
  - Lines modified: 9 (generate_prompts and prompt_arguments methods)
  - Lines retained: 38 (imports, struct, prompt_basic, trait impl skeleton)

---

## Related Files (No Changes Required)

These files reference git_open prompts but need NO modifications:

- `prompt_args.rs`: GitOpenPromptArgs struct still works (scenario field remains optional but unused)
- `mod.rs`: Module re-exports remain unchanged
- `schema.rs`: Response schema unaffected
- Routing in `packages/kodegen/src/stdio/metadata/`: git_open category routing continues to work

---

## Success Criteria

- **Line Count**: Final file is 90-110 lines (will be ~54 lines)
- **Single Scenario**: Only `prompt_basic()` function exists
- **Clean Routing**: `generate_prompts()` contains no match statement
- **No Redundancy**: Removed overlapping prompts (status, vs_discover, comprehensive)
- **Code Quality**: File compiles without warnings

---

## Implementation Notes

### Why This Approach Works

1. **Minimal Coupling**: Removing scenario routing simplifies the implementation without breaking the trait interface
2. **Backward Compatibility**: External callers still get valid prompts; the `GitOpenPromptArgs` struct remains unchanged
3. **Focused Design**: A single well-designed scenario serves 95% of user needs without unnecessary context switching
4. **Future Flexibility**: If scenarios are needed later, the pattern can be re-introduced without structural changes

### Compiler Expectations

The Rust compiler will verify:
- `OpenPrompts` still implements `PromptProvider` trait correctly
- `prompt_basic()` returns `Vec<PromptMessage>` as required
- Empty `prompt_arguments()` is valid (satisfies Vec return type)
- No unused imports or dead code warnings

---

## Complexity Assessment

**Why Complexity 1 (Trivial)**:
- Single file modification
- No API changes (trait interface preserved)
- No new dependencies
- Straightforward deletion of redundant code
- Compiler validates correctness automatically
- No algorithmic changes or novel patterns

Total implementation time: < 5 minutes
