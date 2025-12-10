# TASK 056: Trim github_request_copilot_review

**Tool**: `github_request_copilot_review`
**Complexity**: 2 (Simple)
**Current size**: 101 lines (1 scenario)
**Target size**: 170-220 lines (1-2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/request_copilot_review/prompts.rs`

---

## Current State Analysis

### File Structure (101 lines total)

**Structure breakdown:**
- Lines 1-5: Module header comment and imports (`use` statements)
- Lines 6-10: Doc comment for RequestCopilotReviewPrompts struct definition
- Lines 11-15: `impl PromptProvider` opening and type alias
- Lines 16-95: Single scenario (User question + Assistant response pair)
- Lines 96-101: `prompt_arguments()` method returning empty vec and closing brace

### Scenario Content Breakdown

**Single scenario structure:**
- User question (1 line): "How do I use github_request_copilot_review to get AI code reviews?"
- Assistant response (66 lines) containing:
  - Tool description (2 lines)
  - BASIC USAGE section (28 lines): 4 numbered examples showing different review request patterns
  - PARAMETERS section (7 lines): Describes owner, repo, pull_number, context
  - AUTHENTICATION section (6 lines): Token requirements and scopes
  - RESPONSE section (6 lines): JSON output fields (success, owner, repo, pr_number, review_id, status, message)
  - COMMON WORKFLOWS section (13 lines): 3 workflows (automated initial, security-focused, best practices)
  - RATE LIMITING section (4 lines): Request limits and header checks
  - ERROR SCENARIOS section (8 lines): 3 error cases with fixes (404, 403, 422)
  - BEST PRACTICES section (10 lines): 10 best practice bullets

### Available Tool Parameters

From `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/request_copilot_review/prompt_args.rs`:

```rust
pub struct RequestCopilotReviewPromptArgs {
    pub focus_area: Option<String>,  // Optional focus area for targeted reviews
    pub depth: Option<String>,       // Optional depth level (quick/standard/deep)
}
```

**Note:** Current implementation does NOT use these parameters in routing - there is no scenario selection logic. The `generate_prompts()` method ignores `_args` and returns a single vec.

---

## Implementation Strategy

Unlike PRECURSOR_02 (fs_read_file), this tool already has minimal structure with only 1 scenario. **The tool is already lean.** This is not a "trim overgrown file" task - it's a "verify the existing scenario is comprehensive" task.

### Decision Tree

**Option A (Recommended): Keep Single Scenario**
- Current 101 lines already covers all essential information
- The single scenario is comprehensive for both basic and advanced usage
- Meets target range (100-220 lines)
- No routing logic needed - simple and clean

**Option B: Expand to Two Scenarios (if focus_area/depth should be demonstrated)**
- Scenario 1: `prompt_basic()` (~90 lines) - current content
- Scenario 2: `prompt_focused_review()` (~100 lines) - using focus_area parameter for targeted reviews
- Would require:
  - Adding scenario routing logic to `generate_prompts()`
  - Creating RequestCopilotReviewScenario enum in prompt_args.rs
  - Updating prompt_arguments() to expose scenario selection

---

## Step-by-Step Instructions

### Step 1: Analyze Current Content (Complete)

Verify the current scenario covers:
- ✓ BASIC USAGE with concrete examples (lines 28-36)
- ✓ All required parameters (owner, repo, pull_number)
- ✓ Optional context parameter usage (line 41)
- ✓ Authentication requirements (lines 42-46)
- ✓ Response structure (lines 47-54)
- ✓ Common workflows (lines 55-70)
- ✓ Error handling (lines 75-81)
- ✓ Best practices (lines 82-92)

### Step 2: Decision - Single vs. Two Scenarios

The tool has focus_area and depth parameters available but the current prompt doesn't demonstrate them. Choose approach:

**IF keeping single scenario (Option A):**
- Add brief mention of focus_area parameter in PARAMETERS section
- Add example showing context parameter with security focus
- Verify all content fits in 100-220 line range
- Ensure no decorative headers (═══ style)
- Total: ~110-130 lines (acceptable)

**IF expanding to two scenarios (Option B):**
- Create scenario selection in RequestCopilotReviewPromptArgs or new enum
- Write second scenario focused on `focus_area` usage patterns
- Add routing logic to generate_prompts() method
- Update PromptProvider impl to handle both scenarios
- Target: 200-220 lines total

### Step 3: Content Review (Single Scenario Approach)

Read lines 27-92 (the assistant response) and verify:
- [ ] BASIC USAGE examples are clear and correct (lines 28-36)
  - Example 1: Basic PR review
  - Example 2: Review with custom context
  - Example 3: Security-specific review (shows context parameter)
  - Example 4: General review
- [ ] PARAMETERS section accurate (lines 37-41)
  - All required params documented
  - Optional context param explained
- [ ] AUTHENTICATION section correct (lines 42-46)
  - Token requirement stated
  - Scopes listed
  - Copilot enablement requirement mentioned
- [ ] RESPONSE section complete (lines 47-54)
  - All output fields listed
  - Field descriptions accurate
- [ ] WORKFLOWS realistic (lines 55-70)
  - Automated initial review (PRs → Copilot → human)
  - Security-focused (check vulnerabilities)
  - Best practices (style/patterns)
- [ ] ERROR SCENARIOS actionable (lines 75-81)
  - 404: PR not found - fix provided
  - 403: Copilot disabled - fix provided
  - 422: Review pending - fix provided
- [ ] BEST PRACTICES useful (lines 82-92)
  - Early in PR lifecycle
  - Don't replace human review
  - Combine with tests/linters
  - Review critically

### Step 4: Remove Decorative Elements (if present)

Search for and remove:
- [ ] Decorative `═══` or `─────` headers
- [ ] Excessive blank lines between sections
- [ ] Repeated explanations (e.g., "always use with" mentioned multiple times)
- [ ] Verbose introductions before examples

Examples of what to remove:
- `═════════════════` decorative lines
- Multiple "IMPORTANT:" prefixes (keep max 1-2)
- Redundant parameter explanations

### Step 5: Validate Against Success Criteria

Ensure:
- [ ] File is 100-220 lines total (check: `wc -l prompts.rs`)
- [ ] Exactly 1 scenario (or 2 if choosing Option B)
- [ ] No "comprehensive" scenario (there isn't one)
- [ ] No use-case scenarios (there aren't any)
- [ ] All parameters documented (owner, repo, pull_number, context at minimum)
- [ ] Response structure shown once (not repeated)
- [ ] 3 error scenarios with fixes included
- [ ] Workflows practical and distinct (automated, security, best practices)
- [ ] Best practices actionable and not obvious

---

## Code Pattern: Current Implementation

### Current (Lines 16-95)

```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_request_copilot_review to get AI code reviews?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_request_copilot_review tool requests an AI-powered code review from GitHub Copilot for a pull request.\n\n\
                 BASIC USAGE:\n\
                 ... (content continues for 66 lines) ...\
                 - Supplement with human expertise for architecture decisions",
            ),
        },
    ]
}
```

### Pattern: If Expanding to Two Scenarios

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    // Add enum or parameter check to determine which scenario to return
    if args.focus_area.is_some() {
        Self::prompt_focused_review()
    } else {
        Self::prompt_basic()
    }
}

fn prompt_basic() -> Vec<PromptMessage> {
    // Current content (lines 17-95)
}

fn prompt_focused_review() -> Vec<PromptMessage> {
    // New scenario showing focus_area usage
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I request a security-focused Copilot review?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "When you need targeted reviews using the focus_area parameter:\n\n\
                 SECURITY FOCUS:\n\
                 github_request_copilot_review({...\"context\": \"Check for memory safety, buffer overflows, SQL injection patterns\"})\n\n\
                 ... (approximately 90 more lines covering performance, architecture, standards, etc.) ...",
            ),
        },
    ]
}
```

---

## Success Criteria

**Objective**: This tool is ALREADY TRIMMED. Verify it meets Complexity 2 standards.

- ✓ File is 100-220 lines total
- ✓ Exactly 1 scenario (or 2 if expanding parameters)
- ✓ No comprehensive scenario (check: grep "comprehensive" → 0 results)
- ✓ No use-case scenarios (there aren't code-specific or security-specific separate files)
- ✓ All required parameters documented (owner, repo, pull_number)
- ✓ Context parameter clearly shown with examples
- ✓ Response structure shown once
- ✓ 3+ error scenarios with fixes
- ✓ No decorative `═══` headers
- ✓ Best practices mentioned but not excessive

---

## Validation Commands

After completing the task:

```bash
# Verify line count
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/request_copilot_review/prompts.rs
# Expected: 100-220 lines

# Check for decorative headers
grep -E "^[═─=\-]{5,}" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/request_copilot_review/prompts.rs
# Expected: 0 results

# Count scenario functions
grep "^    fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/request_copilot_review/prompts.rs
# Expected: 1-2 functions

# Check error scenarios are present
grep -c "404\|403\|422" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/request_copilot_review/prompts.rs
# Expected: at least 1 (ideally 3)
```

---

## References

- **Template**: PRECURSOR_02_fs_read_file.md (reference for Complexity 2 standard)
- **Current tool**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/request_copilot_review/prompts.rs`
- **Parameters**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/request_copilot_review/prompt_args.rs`
- **Schema**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/request_copilot_review/schema.rs`

---

## Key Difference from PRECURSOR_02

**fs_read_file (PRECURSOR_02)**: 804 lines with 6 scenarios → trim to 180 lines with 2 scenarios

**github_request_copilot_review (THIS TASK)**: 101 lines with 1 scenario → verify it's optimal at 100-220 lines with 1-2 scenarios

This tool is a **reference case of "already optimal"** - not bloated like fs_read_file.
