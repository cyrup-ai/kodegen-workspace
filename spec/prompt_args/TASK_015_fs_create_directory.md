# TASK 015: Trim fs_create_directory

**Tool**: `fs_create_directory`
**Complexity**: 2 (Simple)
**Current size**: 565 lines
**Target size**: 195-210 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/create_directory/prompts.rs`

---

## Template Reference

This task follows the **Complexity 2 template** established in **PRECURSOR_02_fs_read_file.md**:
- Keep 2 scenarios: basic + one advanced scenario for special parameter/behavior
- Delete use-case scenarios (they don't teach the tool)
- Delete comprehensive scenario (pure duplication)
- Total: 190-220 lines

---

## Current State Analysis

### File Structure (565 total lines)

**Header & Struct** (lines 1-43):
- Module documentation (lines 1-2)
- Imports (lines 4-6)
- CreateDirectoryPrompts struct and PromptProvider impl (lines 8-43)
- Contains: generate_prompts() function with match statement
- Contains: prompt_arguments() function

**Scenario Functions** (lines 44-565, 5 functions):

1. **prompt_basic()** (lines 44-117, ~74 lines) ← **KEEP & TRIM**
   - Teaches core tool usage: single directory creation with path parameter
   - Shows response structure and field meanings
   - Shows when to use (before writing files, organizing data)
   - Shows path types (absolute vs relative)
   - Shows 3 common patterns
   - Shows best practices
   - Essential for understanding basic tool operation

2. **prompt_nested()** (lines 119-258, ~140 lines) ← **KEEP & TRIM**
   - Teaches auto-parent-directory creation (mkdir -p behavior)
   - Shows do-this-not-that comparison (manual vs single call)
   - Shows 4 practical examples (logs, features, tenants, builds)
   - Shows benefits (simpler code, atomic operation, no race conditions)
   - Shows error handling
   - Essential for understanding the special nested-path behavior

3. **prompt_project_setup()** (lines 260-402, ~143 lines) ← **DELETE (USE CASE)**
   - Shows example project structures (standard, web, Rust, Python, full-stack)
   - Shows specialized structures (microservices, data science, static site)
   - Shows project setup workflow (create dirs, write files, initialize git)
   - **Problem**: This is purely USE CASE content - shows what different projects look like, not how fs_create_directory works
   - Calling fs_create_directory multiple times with different paths teaches nothing new
   - No new parameters or behaviors demonstrated

4. **prompt_idempotent()** (lines 404-542, ~139 lines) ← **DELETE (REDUNDANT)**
   - Teaches idempotency: safe to call multiple times
   - Shows created field behavior (true for new, false for existing)
   - Shows why idempotent matters (scripts can run multiple times, no existence checks needed, safe in concurrent ops)
   - Shows practical examples (daily logs, user directories, cache, tests)
   - **Problem**: Idempotency is a tool BEHAVIOR, not a parameter. Response field (created: true/false) already shown in basic scenario
   - Can merge critical points (no need to check existence first) into basic "when to use" section

5. **prompt_comprehensive()** (lines 544-565, ~220+ lines) ← **DELETE (DUPLICATION)**
   - Pure duplication: combines all basic + nested + project_setup content
   - Headers repeated multiple times
   - Examples repeated across scenarios
   - No unique content beyond other scenarios

### Routing Logic (lines 16-25)

Current match statement:
```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("nested") => prompt_nested(),
    Some("project_setup") => prompt_project_setup(),
    Some("idempotent") => prompt_idempotent(),
    _ => prompt_comprehensive(),
}
```

**Change to** (2-scenario routing):
```rust
match args.scenario.as_deref() {
    Some("nested") => prompt_nested(),
    _ => prompt_basic(),
}
```

### Prompt Arguments (lines 27-35)

Current description lists all 4 scenarios:
```
"Scenario to show examples for (basic, nested, project_setup, idempotent)"
```

**Change to**:
```
"Scenario to show examples for (basic, nested)"
```

---

## Implementation Steps

### STEP 1: Update Routing Logic (lines 16-25)

Replace the entire match statement in generate_prompts() function:

**BEFORE**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("nested") => prompt_nested(),
        Some("project_setup") => prompt_project_setup(),
        Some("idempotent") => prompt_idempotent(),
        _ => prompt_comprehensive(),
    }
}
```

**AFTER**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("nested") => prompt_nested(),
        _ => prompt_basic(),
    }
}
```

This removes 3 match arms and changes default from comprehensive to basic.

### STEP 2: Update Prompt Arguments (lines 27-35)

Replace the description string in prompt_arguments() function:

**BEFORE**:
```
"Scenario to show examples for (basic, nested, project_setup, idempotent)"
```

**AFTER**:
```
"Scenario to show examples for (basic, nested)"
```

### STEP 3: Trim prompt_basic() to ~85 lines (currently ~74 lines)

**Location**: Lines 44-117

**Keep everything, but** ADD a note about idempotency in the response/when-to-use section.

After the "BEST PRACTICES" section, add (before closing the PromptMessage):
```
- It is safe to call multiple times with the same path (idempotent operation)
- If directory already exists, response returns created: false, existing contents preserved
- No need to check if directory exists before calling - just create it
```

This moves critical idempotency knowledge from prompt_idempotent into basic without duplication.

**Result**: prompt_basic stays ~85-90 lines (added idempotency note)

### STEP 4: Trim prompt_nested() to ~85 lines (currently ~140 lines)

**Location**: Lines 119-258

Keep the structure but condense verbose sections:

**Keep sections** (essential for understanding nested paths):
- User question (2 lines)
- Intro explanation (2 lines)
- Description of auto-creation (5 lines)
- Examples showing nested creation (3 examples, ~10 lines)
- Do-this-not-that comparison (most important, ~12 lines) - this teaches why nested is valuable
- Practical examples (reduce from 4 to 3 examples, ~12 lines): logs, features, builds (remove tenants as redundant)
- Benefits summary (condense to ~5 lines): simpler code, atomic, no race conditions
- Error handling (keep brief, ~4 lines)

**Remove sections**:
- Decorative headers like "═════════════════"
- Verbose "HOW IT WORKS" explanation (can be condensed to 2-3 lines)
- Redundant explanations already in basic scenario

**Result**: prompt_nested reduces to ~85 lines

### STEP 5: Delete Entire Functions

**Delete lines 260-402**: prompt_project_setup() function
- 143 lines of USE CASE examples (different project structures)
- Teaches nothing new about fs_create_directory functionality
- Just shows example paths, not tool behavior

**Delete lines 404-542**: prompt_idempotent() function
- 139 lines of response field behavior already covered in basic
- Idempotency is now mentioned in basic scenario

**Delete lines 544-565+**: prompt_comprehensive() function
- 220+ lines of pure duplication
- Combines basic + nested + project_setup content
- No unique or necessary content

### STEP 6: Remove Decorative Headers

Throughout the file, remove lines containing only ASCII decoration like:
- `// ============================================================================`
- `// HELPER FUNCTIONS - TEACH AI AGENTS...` (keep brief module comment)

Replace with simple comments when needed (lines 44-45):
```
/// Basic directory creation (core tool usage)
fn prompt_basic() -> Vec<PromptMessage> {
```

---

## Implementation Order

Execute changes in this order:

1. **First**: Delete prompt_project_setup(), prompt_idempotent(), prompt_comprehensive() functions (removes ~502 lines)
2. **Second**: Update routing logic to 2-scenario match (removes 3 arms)
3. **Third**: Update prompt_arguments description string
4. **Fourth**: Add idempotency note to prompt_basic response/best-practices section
5. **Fifth**: Trim verbose sections in prompt_nested (remove decorative headers, condense explanations)
6. **Sixth**: Verify line count and content

---

## Code Patterns: Before & After

### Before: generate_prompts() routing (5 scenarios, 10 match arms)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("nested") => prompt_nested(),
        Some("project_setup") => prompt_project_setup(),
        Some("idempotent") => prompt_idempotent(),
        _ => prompt_comprehensive(),
    }
}
```

### After: generate_prompts() routing (2 scenarios, 2 match arms)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("nested") => prompt_nested(),
        _ => prompt_basic(),
    }
}
```

### Before: Idempotency not mentioned in basic scenario

### After: Idempotency note in basic scenario best practices
```rust
- It is safe to call multiple times with the same path (idempotent)
- If directory already exists, response returns created: false
- No need to check if directory exists before calling
```

---

## Success Criteria (Measurable Definition of Done)

Execute this validation after making changes:

```bash
# 1. Line count check
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/create_directory/prompts.rs
# Must be between 195-210 lines (down from 565)

# 2. Scenario count check
grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/create_directory/prompts.rs
# Must show exactly 2 functions: prompt_basic and prompt_nested

# 3. No deleted scenarios still present
grep -c "prompt_project_setup\|prompt_idempotent\|prompt_comprehensive" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/create_directory/prompts.rs
# Must return 0 (no matches)

# 4. Routing logic check
grep -A 5 "match args.scenario" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/create_directory/prompts.rs
# Must show only 2 match arms (nested and default)

# 5. Prompt arguments check
grep "basic, nested" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/create_directory/prompts.rs
# Must find the updated description string

# 6. No decorative headers
grep "════" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/create_directory/prompts.rs
# Must return 0 (no decorative lines)
```

**All 6 checks must pass.**

---

## Why This Approach (Complexity 2 Template)

Following **PRECURSOR_02_fs_read_file.md** template:

- **Scenario 1 (basic)**: Shows core tool usage with the single "path" parameter
- **Scenario 2 (nested)**: Shows advanced behavior - how nested paths are handled specially (parent auto-creation)
- **Deleted (project_setup)**: USE CASE content (what projects look like) not TOOL FEATURE
- **Deleted (idempotent)**: Response field behavior, now folded into basic scenario
- **Deleted (comprehensive)**: Pure duplication of other scenarios

This maintains consistency with the Complexity 2 standard: 2 focused scenarios teaching core + special parameter behavior, no use cases, no duplication, 190-220 lines total.

---

## Files Modified

- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/filesystem/create_directory/prompts.rs`

No other files modified. The prompt_args.rs file does not define an enum (uses string matching), so no changes needed there.

---

## Testing the Result

After completing all steps, the file should:
1. Be readable in 2 minutes
2. Teach all essential fs_create_directory usage patterns
3. Show both basic single-path and nested-path behaviors
4. Provide clear response structure examples
5. Demonstrate practical patterns without duplication
6. Pass all 6 validation checks above
