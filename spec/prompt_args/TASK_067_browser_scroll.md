# TASK 067: Trim browser_scroll

**Tool**: `browser_scroll`
**Complexity**: 2 (Simple)
**Current size**: 879 lines
**Target size**: ~170-220 lines (1 scenario kept)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/scroll/prompts.rs`

---

## Current State Analysis

### File Structure (879 lines total)

The `prompts.rs` file contains the `ScrollPrompts` struct and 5 scenario prompt functions organized as follows:

**Lines 1-37: Module header and ScrollPrompts implementation**
- Imports: crate::tool::PromptProvider, rmcp types
- ScrollPrompts struct definition with PromptProvider trait implementation
- Routing logic in generate_prompts() method

**Lines 40-44: Decorative header comment block** (TO DELETE)
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO SCROLL PAGES
// ============================================================================
```

**Lines 43-124: prompt_basic() function (~82 lines) - KEEP**
- Docstring: "Basic page scrolling with x/y deltas"
- Teaches pixel-based scrolling with x/y parameters
- Content: scroll directions, amounts, common patterns, output structure, troubleshooting
- Classification: FOUNDATIONAL (not a use-case)
- This is the core teaching scenario required to keep

**Lines 125-241: prompt_element_into_view() function (~117 lines) - DELETE**
- Docstring: "Scrolling elements into view using selectors"
- Teaches selector-based scrolling for specific elements
- Content: ID/class/attribute/complex selectors, workflow patterns
- Classification: USE-CASE (specific technique, not foundational)

**Lines 242-406: prompt_infinite_scroll() function (~165 lines) - DELETE**
- Docstring: "Infinite scroll patterns for loading dynamic content"
- Teaches patterns for sites like Twitter/Instagram/news feeds
- Content: detection methods, scroll strategies, timing, real-world examples
- Classification: USE-CASE (specific pattern for dynamic content)

**Lines 407-583: prompt_element_containers() function (~177 lines) - DELETE**
- Docstring: "Scrolling within scrollable element containers"
- Teaches limitations and workarounds for container scrolling
- Content: modal dialogs, chat windows, code editors, limitations explanation
- Classification: USE-CASE (specific limitation scenario)

**Lines 584-879: prompt_comprehensive() function (~296 lines) - DELETE**
- Docstring: "Comprehensive guide covering all scroll patterns"
- Covers all modes, workflows, patterns, troubleshooting in one large response
- Serves as default catch-all when no scenario specified
- Classification: COMPREHENSIVE/CATCH-ALL (not a focused scenario)

### Current Routing Logic (lines 18-23)

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("element_into_view") => prompt_element_into_view(),
        Some("infinite_scroll") => prompt_infinite_scroll(),
        Some("element_containers") => prompt_element_containers(),
        _ => prompt_comprehensive(),
    }
}
```

### Current Prompt Arguments (lines 26-33)

```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, element_into_view, infinite_scroll, element_containers)".to_string()),
            required: Some(false),
        }
    ]
}
```

---

## Step 1: Update Routing Logic (lines 18-23)

Simplify the match statement to only handle the "basic" scenario. All other scenarios should fall back to "basic" as well.

**Location**: Lines 18-23

**Old code**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("element_into_view") => prompt_element_into_view(),
        Some("infinite_scroll") => prompt_infinite_scroll(),
        Some("element_containers") => prompt_element_containers(),
        _ => prompt_comprehensive(),
    }
}
```

**New code**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    // Always return basic scenario - all others have been trimmed
    prompt_basic()
}
```

**Why**: Eliminates references to deleted functions and simplifies routing to only support the one remaining scenario.

---

## Step 2: Update Prompt Arguments Description (line 30)

Update the description to reflect that only "basic" scenario is available.

**Location**: Line 30

**Old string**:
```
"Scenario to show (basic, element_into_view, infinite_scroll, element_containers)"
```

**New string**:
```
"Scenario to show (basic)"
```

**Why**: Accurate documentation of available scenarios after trimming.

---

## Step 3: Delete Decorative Header Comment (lines 40-42)

Remove the decorative "====" comment block above the first function.

**Location**: Lines 40-42

**Content to delete**:
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO SCROLL PAGES
// ============================================================================
```

**Why**: Task requirement specifies deleting decorative headers. This comment is purely visual and not needed.

---

## Step 4: Delete Use-Case Scenario Functions (lines 125-583)

Remove all three use-case scenario functions completely.

**Location**: Lines 125-583

**Functions to delete**:
1. `prompt_element_into_view()` - lines 125-241 (~117 lines)
2. `prompt_infinite_scroll()` - lines 242-406 (~165 lines)
3. `prompt_element_containers()` - lines 407-583 (~177 lines)

**Why**: Task specifies deleting all use-case scenarios. These three functions are specific techniques/patterns, not foundational teaching material.

---

## Step 5: Delete Comprehensive Scenario Function (lines 584-879)

Remove the comprehensive catch-all scenario function entirely.

**Location**: Lines 584-879

**Content to delete**:
```rust
/// Comprehensive guide covering all scroll patterns
fn prompt_comprehensive() -> Vec<PromptMessage> {
    // ... ~296 lines of detailed content ...
}
```

**Why**: Task specifies deleting comprehensive scenario. This is a catch-all fallback that should be replaced with just the basic scenario.

---

## Execution Order

Execute edits in this order to avoid line number shifts:

1. **First**: Delete comprehensive scenario (lines 584-879) - largest deletion, won't affect earlier line numbers
2. **Second**: Delete use-case scenarios (lines 125-583)
3. **Third**: Delete decorative header (lines 40-42)
4. **Fourth**: Update prompt_arguments (line 30)
5. **Fifth**: Update routing logic (lines 18-23)

---

## Expected Result

**After all edits complete:**

- File size: approximately 119 lines (879 - 760 deleted lines)
- Scenario count: 1 (only "basic")
- Available scenarios: basic (always returned, no routing needed)
- Routing logic: Simple direct call to prompt_basic()

**File structure will be:**
```rust
// Lines 1-37: Imports and ScrollPrompts struct
impl PromptProvider for ScrollPrompts {
    type PromptArgs = BrowserScrollPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// Lines 38-119: prompt_basic() function unchanged
```

---

## Verification Steps

After completing all edits:

1. **Verify file compiles**:
   ```bash
   cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
   cargo check
   ```

2. **Verify no warnings**:
   ```bash
   cargo clippy
   ```

3. **Verify line count**:
   ```bash
   wc -l src/browser/scroll/prompts.rs
   # Should output approximately 119
   ```

4. **Verify routing**:
   - Open the file
   - Confirm generate_prompts() simply calls prompt_basic()
   - Confirm no other function names are referenced

5. **Verify scenario list**:
   - Check prompt_arguments() description
   - Should only list "basic"

---

## Success Criteria

- ✓ File reduced from 879 to ~119 lines (86% reduction)
- ✓ Only 1 scenario kept (basic)
- ✓ All 3 use-case scenarios removed (element_into_view, infinite_scroll, element_containers)
- ✓ Comprehensive scenario removed
- ✓ Decorative headers removed
- ✓ Routing logic simplified to direct function call
- ✓ prompt_arguments description updated
- ✓ File compiles without errors
- ✓ No clippy warnings
