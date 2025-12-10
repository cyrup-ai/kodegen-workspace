# TASK 075: Trim prompt_add

**Tool**: `prompt_add`
**Complexity**: 2 (Simple)
**Current size**: 347 lines with 5 scenarios
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/prompt/prompt_add/prompts.rs`

---

## Current State Analysis

### File Structure
The prompts.rs file is organized as follows:
- **Lines 1-36**: Module documentation, imports, and PromptAddPrompts struct definition
- **Lines 13-33**: PromptProvider trait implementation with match statement
- **Lines 37-38**: Helper function comment header
- **Lines 41-83**: `prompt_basic()` scenario function (43 lines)
- **Lines 85-154**: `prompt_templating()` scenario function (70 lines)
- **Lines 156-212**: `prompt_organization()` scenario function (57 lines) - DELETE
- **Lines 214-269**: `prompt_workflows()` scenario function (56 lines) - DELETE
- **Lines 271-347**: `prompt_comprehensive()` scenario function (77 lines with decorative box headers) - DELETE

### Routing Logic
The `PromptProvider::generate_prompts()` method (lines 18-24) contains a match statement that routes scenarios:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("templating") => prompt_templating(),
    Some("organization") => prompt_organization(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

The `prompt_arguments()` method (lines 26-32) describes available scenarios to the user.

---

## Implementation Instructions

### Step 1: Update Match Statement (Lines 18-24)
Replace the full match statement with only basic and templating scenarios:

**CURRENT CODE:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("templating") => prompt_templating(),
        Some("organization") => prompt_organization(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**NEW CODE:**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        _ => prompt_templating(),
    }
}
```

This keeps the "basic" scenario explicitly callable while making templating the default (when no scenario is specified or unrecognized scenario is requested).

### Step 2: Update prompt_arguments() Method (Lines 26-32)
Update the description to reflect only 2 available scenarios:

**CURRENT CODE:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, templating, organization, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

**NEW CODE:**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, templating)".to_string()),
            required: Some(false),
        }
    ]
}
```

### Step 3: Delete prompt_organization() Function
Delete lines 156-212 entirely. This 57-line function covers prompt organization strategies and naming conventions. This content is less critical than basic usage and templating syntax.

### Step 4: Delete prompt_workflows() Function
Delete lines 214-269 entirely. This 56-line function covers common workflows for creating and using prompts. While useful, it is covered by basic and templating examples.

### Step 5: Delete prompt_comprehensive() Function
Delete lines 271-347 entirely. This 77-line function with decorative box-drawing headers (═══════════════════════════════════════════════════════════════) is the most verbose and comprehensive guide. This is explicitly mentioned as a "use-case scenario" to be deleted per task instructions.

### Step 6: Verify Final Structure
After all deletions, the file structure should be:
- Lines 1-36: Module header, imports, struct, impl block start
- Lines 37-38: Helper comment header
- Lines 39-83: `prompt_basic()` function (approximately 43 lines)
- Lines 84-153: `prompt_templating()` function (approximately 70 lines)

**Final line count**: Approximately 153 lines

---

## Detailed Code Changes

### Complete Implementation

Execute these changes in order using `fs_edit_block`:

**EDIT 1: Update generate_prompts match statement**
- Find: The match statement at lines 18-24
- Replace with the new 4-line match statement that only handles "basic" and uses templating as default

**EDIT 2: Update prompt_arguments description**
- Find: The description string "Scenario to show (basic, templating, organization, workflows)"
- Replace with: "Scenario to show (basic, templating)"

**EDIT 3: Delete prompt_organization function**
- Find: Everything from `/// Prompt organization strategies` (line 156) through the closing brace of `prompt_organization()` (line 212)
- Replace with: Empty string (deletion)

**EDIT 4: Delete prompt_workflows function**
- Find: Everything from `/// Prompt creation workflows` (line 214) through the closing brace of `prompt_workflows()` (line 269)
- Replace with: Empty string (deletion)

**EDIT 5: Delete prompt_comprehensive function**
- Find: Everything from `/// Comprehensive guide covering all aspects` (line 271) through the closing brace of `prompt_comprehensive()` (line 347)
- Replace with: Empty string (deletion)

---

## Functional Impact

After trimming:

1. **Basic Scenario** (`scenario="basic"` or unrecognized)
   - Teaches fundamental prompt creation with name, content, and description fields
   - Shows simple examples of code-review and explain-error prompts
   - Includes field documentation (name, content, description)

2. **Templating Scenario** (default when no scenario specified)
   - Teaches Jinja2 templating syntax: variable substitution, conditionals, loops, filters, defaults
   - Shows 5 example patterns: variables, conditionals, loops, filters, default values
   - Covers 7 key template features essential for dynamic prompt creation

3. **Removed**
   - Organization: prompt naming conventions, namespacing, versioning strategies
   - Workflows: create-test-iterate patterns, library building, updating patterns
   - Comprehensive: full guide with decorative headers covering all aspects in one massive response

---

## Success Criteria

After implementation, verify:

- ✓ File size is between 170-220 lines (target: ~153 lines after deletions, acceptable if within range)
- ✓ Exactly 2 scenarios remain: `prompt_basic` and `prompt_templating`
- ✓ Match statement in `generate_prompts()` routes only "basic" to `prompt_basic()` and defaults to `prompt_templating()`
- ✓ `prompt_arguments()` description updated to list only "(basic, templating)"
- ✓ No decorative headers or comprehensive scenario present
- ✓ File compiles without errors: `cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check`

---

## Testing Verification

Run after edits to ensure correctness:

```bash
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
cargo check --lib
```

This verifies the Rust code compiles and all imports are satisfied. The prompts can be functionally tested via the add_prompt tool by calling it with `scenario="basic"` and `scenario="templating"`.

---

## Reference Information

The `PromptProvider` trait (defined in `kodegen-mcp-tool` crate) is sealed and can only be implemented in kodegen-mcp-schema. The `generate_prompts()` method returns `Vec<PromptMessage>` containing user/assistant message pairs that teach the LLM how to use the add_prompt tool.

Key message structure:
- User role: Question about how to use the tool
- Assistant role: Educational answer with code examples and explanations

Both remaining scenarios follow this exact pattern with practical examples that agents can reference.
