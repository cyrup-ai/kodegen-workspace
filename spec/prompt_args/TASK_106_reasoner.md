# TASK 106: Trim reasoner

**Tool**: `reasoner`
**Complexity**: 5 (Very Complex)
**Current size**: 559 lines
**Target size**: 420 lines (5 scenarios, no comprehensive)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/reasoning/reasoner/prompts.rs`

---

## Reference

**terminal** at 591 lines is the GOLD STANDARD for Complexity 5. The reasoner tool will be trimmed to 420 lines by removing the redundant comprehensive scenario, achieving better efficiency while maintaining all essential documentation.

---

## Current State Analysis

### File Structure
The prompts.rs file is located at `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/reasoning/reasoner/prompts.rs`.

### Line Count Breakdown (Current: 559 lines)
- **Lines 1-41**: Header comment, imports, ReasonerPrompts struct, PromptProvider trait implementation
  - Line 3: `use crate::tool::PromptProvider`
  - Line 13: `impl PromptProvider for ReasonerPrompts`
  - Lines 16-25: `generate_prompts()` method with match statement routing to 6 scenarios
  - Lines 27-36: `prompt_arguments()` method defining scenario parameter

- **Lines 44-104**: `prompt_basic()` function (61 lines)
  - Content: STRUCTURED REASONING explanation with 4-step example workflow
  - Covers: thought_number, total_thoughts, next_thought_needed parameters
  - Documents response fields: node_id, score, depth, is_complete, best_score

- **Lines 107-174**: `prompt_beam_search()` function (68 lines)
  - Content: Beam search strategy with 3-step database optimization example
  - Covers: beam_width parameter (1-10 range) with guidance for different widths
  - Documents advantages: parallel path exploration, prevents suboptimal paths

- **Lines 177-259**: `prompt_mcts()` function (83 lines)
  - Content: Monte Carlo Tree Search with 3-step payment module refactoring example
  - Covers: num_simulations parameter (1-150 range)
  - Documents 3 MCTS variants:
    - mcts: Standard with UCB1 (balanced exploration/exploitation)
    - mcts_002_alpha: High exploration (increased coefficient, novel solutions)
    - mcts_002alt_alpha: Length-rewarding (thorough analysis, comprehensive solutions)

- **Lines 262-344**: `prompt_branching()` function (83 lines)
  - Content: Branching and revision with 5-step caching strategy example
  - Covers: parent_id parameter for tree navigation
  - Documents tree structure visualization and branching use cases

- **Lines 347-418**: `prompt_strategies()` function (72 lines)
  - Content: Strategy comparison reference guide with decorative headers
  - Lists: All 4 strategies (beam_search, mcts, mcts_002_alpha, mcts_002alt_alpha)
  - Includes: Decision guide: problem type → recommended strategy

- **Lines 421-558**: `prompt_comprehensive()` function (138 lines) **REDUNDANT**
  - Problem: Duplicates all content from basic, beam_search, mcts, branching, strategies
  - Decorative headers: Multiple "═════════════════════════" separator lines
  - Inefficient: Contains full examples and explanations already in other scenarios
  - Action: DELETE THIS ENTIRE FUNCTION

### Routing Logic (Lines 16-25)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("beam_search") => prompt_beam_search(),
        Some("mcts") => prompt_mcts(),
        Some("branching") => prompt_branching(),
        Some("strategies") => prompt_strategies(),
        _ => prompt_comprehensive(),  // <-- REMOVE THIS LINE
    }
}
```

After trim, change to:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("beam_search") => prompt_beam_search(),
        Some("mcts") => prompt_mcts(),
        Some("branching") => prompt_branching(),
        Some("strategies") => prompt_strategies(),
        _ => prompt_basic(),  // <-- DEFAULT TO BASIC INSTEAD
    }
}
```

### Prompt Arguments (Lines 27-36)
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![PromptArgument {
        name: "scenario".to_string(),
        title: None,
        description: Some(
            "Scenario to show (basic, beam_search, mcts, branching, strategies)".to_string(),
            // Current: includes "strategies)" but file has 6 scenarios
            // After: will list all 5 remaining scenarios (no comprehensive)
        ),
        required: Some(false),
    }]
}
```

### Strategy Coverage Analysis
All four reasoner strategies ARE properly documented across the 5 scenarios:

1. **beam_search** - Dedicated scenario at lines 107-174
2. **mcts** - Dedicated scenario at lines 177-259 (includes this variant)
3. **mcts_002_alpha** - Documented in MCTS scenario at lines 229-232
4. **mcts_002alt_alpha** - Documented in MCTS scenario at lines 234-237

After trim, strategy coverage remains complete and distributed across scenarios.

---

## Implementation Instructions

### Step 1: Update the Routing Logic

**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/reasoning/reasoner/prompts.rs`

**Location**: Lines 16-25 (the `generate_prompts` method)

**Action**: Replace the default case from `prompt_comprehensive()` to `prompt_basic()`

**Before** (line 23):
```rust
        _ => prompt_comprehensive(),
```

**After** (line 23):
```rust
        _ => prompt_basic(),
```

**Rationale**: When an unknown scenario is requested, default to basic reasoning examples instead of comprehensive guide. This is consistent with the terminal tool pattern.

---

### Step 2: Update Prompt Arguments Description

**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/reasoning/reasoner/prompts.rs`

**Location**: Lines 27-36 (the `prompt_arguments` method)

**Action**: Update the description string to reflect 5 scenarios only

**Before** (line 32):
```rust
                "Scenario to show (basic, beam_search, mcts, branching, strategies)".to_string(),
```

**After** (line 32):
```rust
                "Scenario to show (basic, beam_search, mcts, branching, strategies)".to_string(),
```

**Note**: The description is already correct! It already lists 5 scenarios without comprehensive. No change needed.

---

### Step 3: Delete the Comprehensive Scenario Function

**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/reasoning/reasoner/prompts.rs`

**Location**: Lines 421-558 (the entire `prompt_comprehensive()` function)

**Action**: Delete all 138 lines of this function

**Before** (lines 420-422):
```rust

/// Comprehensive guide covering all scenarios
fn prompt_comprehensive() -> Vec<PromptMessage> {
```

**Delete these sections entirely**:
- Line 420: Blank line
- Lines 421-558: Entire `fn prompt_comprehensive()` function including:
  - Function definition (line 421)
  - All PromptMessage::User examples
  - All PromptMessage::Assistant responses with decorative headers
  - Closing brace (line 557)
- Line 558: Final blank line (if present)

**Verification**: After deletion, line 419 should be the final content line (closing brace of strategies function).

---

## Expected Results After Trim

### File Metrics
- **New total lines**: 420 lines (from 559)
- **Lines removed**: 139 lines (1 function + blank lines)
- **Reduction**: 25% smaller, meeting efficiency goals

### Scenario Count
- **Total scenarios**: 5 (from 6)
- **Remaining scenarios**:
  1. `basic` - Lines 44-104 (61 lines) - Thought progression fundamentals
  2. `beam_search` - Lines 107-174 (68 lines) - Multi-path exploration with beam_width
  3. `mcts` - Lines 177-259 (83 lines) - Complex decisions with 3 MCTS variants
  4. `branching` - Lines 262-344 (83 lines) - Tree exploration and parent_id usage
  5. `strategies` - Lines 347-418 (72 lines) - Strategy comparison reference

### Strategy Coverage (All Maintained)
- ✓ **beam_search** - Dedicated scenario with examples, beam_width guidance (1-10 range)
- ✓ **mcts** - Dedicated scenario with standard UCB1 explanation
- ✓ **mcts_002_alpha** - MCTS scenario lines 229-232, high exploration documentation
- ✓ **mcts_002alt_alpha** - MCTS scenario lines 234-237, length-rewarding documentation

### Quality Improvements
- No content loss (comprehensive was redundant)
- All essential patterns documented
- Cleaner routing logic with consistent defaults
- Matches terminal.rs pattern (keep comprehensive only if unique value)

---

## Definition of Done

This task is complete when ALL of the following criteria are met:

### Code Changes
- [ ] Line 23 in `generate_prompts()` changed from `prompt_comprehensive()` to `prompt_basic()`
- [ ] Lines 420-558 (entire `prompt_comprehensive()` function) are deleted
- [ ] No other functions, imports, or structures are modified

### File Validation
- [ ] File compiles with `cargo build` in kodegen-mcp-schema package
- [ ] File passes `cargo clippy` with no warnings
- [ ] Line count is approximately 420 lines (420-425 acceptable for blank lines)
- [ ] Exactly 5 scenario routes exist in the match statement

### Scenario Verification
- [ ] `basic` scenario still accessible and functional
- [ ] `beam_search` scenario still accessible and functional
- [ ] `mcts` scenario still accessible and functional
- [ ] `branching` scenario still accessible and functional
- [ ] `strategies` scenario still accessible and functional
- [ ] Default case (unknown scenario) routes to `prompt_basic()`

### Strategy Documentation Verification
- [ ] `beam_search` strategy documented with beam_width parameter guidance
- [ ] `mcts` standard variant documented with UCB1 explanation
- [ ] `mcts_002_alpha` variant documented with high-exploration explanation
- [ ] `mcts_002alt_alpha` variant documented with length-rewarding explanation
- [ ] All 4 strategies appear in the `strategies` scenario comparison section

### Final Checklist
- [ ] Git diff shows ONLY the removal of `prompt_comprehensive()` function
- [ ] Git diff shows ONLY the one-line change to `generate_prompts()` default case
- [ ] No accidental whitespace or formatting changes
- [ ] File is ready for integration without additional cleanup
