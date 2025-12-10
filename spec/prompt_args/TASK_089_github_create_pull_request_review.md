# TASK 089: Refactor github_create_pull_request_review into scenarios

**Tool**: `github_create_pull_request_review`
**Complexity**: 3 (Medium)
**Current size**: 106 lines (1 comprehensive response)
**Target size**: 280-360 lines (3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_pull_request_review/prompts.rs`

---

## Reference

See **PRECURSOR_03_git_branch_create.md** for Complexity 3 template and pattern.

---

## Current State Analysis

**Current structure** (106 total lines):
- Single User question (line 18): "How do I use github_create_pull_request_review to review pull requests?"
- Single Assistant response (lines 20-104): Comprehensive coverage of all aspects in one response
- No scenario routing logic
- No prompt_arguments definition for scenarios

**Current response content breakdown**:
- Basic usage examples (lines 20-26): 4 examples covering APPROVE, REQUEST_CHANGES, COMMENT, with inline comments
- Parameters section (lines 28-41): Detailed parameter documentation (owner, repo, pull_number, event, body, commit_id, comments)
- Authentication (lines 43-46): GITHUB_TOKEN requirements and scopes
- Response format (lines 48-53): Success response structure with fields
- Common workflows (lines 55-68): 3 workflow patterns (automated review, security review, documentation review)
- Rate limiting (lines 70-73): API rate limits and header checking
- Error scenarios (lines 75-82): 3 error cases (404, 422, 403) with fixes
- Best practices (lines 84-104): 8 best practice guidelines

**Key observation**: This is a LEAN comprehensive response. The current 106 lines contain valuable content that needs to be split into focused scenarios rather than condensed further.

---

## Refactoring Instructions

You will refactor the single comprehensive response into **THREE scenario functions** matching the create_pull_request pattern (see packages/kodegen-mcp-schema/src/github/create_pull_request/prompts.rs for reference structure).

### Step 1: Understand the Current Tool Parameters

The github_create_pull_request_review tool has these parameters:
- **Required**: owner, repo, pull_number, event
- **Optional**: body, commit_id, comments (array with path, line, body, side)

This is simpler than create_pull_request (fewer parameters), so 3 scenarios (not 5) is appropriate.

### Step 2: Create Routing Logic

Replace the current `generate_prompts()` method with scenario routing:

```rust
// BEFORE (current, lines 18-109):
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
        PromptMessage { role: PromptMessageRole::User, ... },
        PromptMessage { role: PromptMessageRole::Assistant, ... },
    ]
}

// AFTER (target structure):
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("inline_comments") => prompt_inline_comments(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_basic(),
    }
}

fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, inline_comments, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

### Step 3: Create prompt_basic() Function (Target: ~110 lines)

**Purpose**: Cover the three review event types (APPROVE, REQUEST_CHANGES, COMMENT) with examples and when to use each.

**Keep from current response**:
- The 4 basic usage examples (lines 20-26), reworked as 3 clear event-type examples:
  - APPROVE with body
  - REQUEST_CHANGES with body
  - COMMENT without expecting changes
- Parameters section (lines 28-41), but trim to just event, body, pull_number
- Response format (lines 48-53), keep as-is
- Best practices specific to event types (lines 84-104), extract 20 lines about when to use each event

**New content to add**:
- Clear explanation of event type semantics and differences
- Decision tree: when to use APPROVE vs REQUEST_CHANGES vs COMMENT
- Body parameter (optional but recommended) for context

**Remove**:
- Inline comments details (move to prompt_inline_comments)
- Workflows section (move to prompt_workflows)
- commit_id parameter (not central to basic usage)
- Rate limiting (reference info, not needed per-scenario)

**Structure**:
```
User: "How do I review pull requests with different approval statuses?"
Assistant:
- Intro to three event types
- APPROVE example (30 lines): when to use, example with body
- REQUEST_CHANGES example (30 lines): when to use, example with body, what body should contain
- COMMENT example (25 lines): when to use, example, difference from REQUEST_CHANGES
- Parameters: event, body, pull_number, owner, repo (brief)
- Response format
- Best practices for event selection (15 lines)
```

### Step 4: Create prompt_inline_comments() Function (Target: ~110 lines)

**Purpose**: Teach how to add comments to specific lines in the PR diff.

**Keep from current response**:
- The one inline comments example (lines 24-26)
- The comments parameter documentation (partial from lines 28-41): path, line, body, side
- Relevant error scenarios around comment validation

**New content to add**:
- Expanded inline comments example (40 lines) showing:
  - Single inline comment
  - Multiple comments on different files
  - Comments on LEFT vs RIGHT side (diff sides)
  - Using commit_id to target specific commit
- Line numbering explanation (15 lines):
  - What "line" means (position in diff, not file)
  - How to find correct line numbers
  - Common mistakes with line numbering
- Comment best practices (20 lines):
  - Specific, actionable feedback
  - Line-specific issues only (use body for general)
  - Code example in comments
  - Tone and constructiveness
- Workflow: how to use comments in reviews (15 lines)

**Structure**:
```
User: "How do I add inline comments to specific lines in a pull request review?"
Assistant:
- Intro to inline review comments
- Single inline comment example (25 lines)
- Multiple file comments example (30 lines)
- LEFT/RIGHT side explanation (15 lines)
- Line number basics and common mistakes (20 lines)
- Comment structure (path, line, body, side) (15 lines)
- Best practices for constructive feedback (20 lines)
- Complete workflow example (10 lines)
```

### Step 5: Create prompt_workflows() Function (Target: ~110 lines)

**Purpose**: Show automated and integration patterns for code reviews.

**Keep from current response**:
- The 3 workflow examples (lines 55-68): automated code review, security review, documentation review

**New content to add**:
- Expand each workflow to 20-25 lines with more detail and examples
- Automated review integration (20 lines):
  - Analyze PR changes → add comments → approve or request changes
  - Using commit_id to target reviews
- Security review workflow (20 lines):
  - Check for vulnerabilities
  - Add security comments
  - REQUEST_CHANGES for issues
- Documentation review workflow (20 lines):
  - Verify API docs
  - Comment on unclear sections
  - APPROVE when complete
- Integration with other tools (10 lines):
  - Chain with github_request_reviews
  - Link to github_get_pull_request_files
  - Use in approval automation
- Error handling in workflows (15 lines):
  - Already reviewed PR
  - Invalid event transitions
  - Comment failures

**Remove**:
- Duplicate best practices (covered in basic scenario)
- Parameter documentation (covered in basic scenario)

**Structure**:
```
User: "What are common workflows for automating pull request reviews?"
Assistant:
- Intro to review automation
- Automated code review pattern (25 lines):
  - Get PR files → analyze → add comments → decide approval
  - Example with multiple comments
- Security review pattern (25 lines):
  - Check for vulnerabilities
  - Add security-focused comments
  - Example REQUEST_CHANGES
- Documentation review pattern (25 lines):
  - Verify documentation completeness
  - Comment on unclear docs
  - Example approval
- Integration with other tools (15 lines):
  - github_get_pull_request_files
  - github_request_reviews
  - Sequential pattern examples
- Common workflow mistakes (10 lines)
- Best practices for automated reviews (10 lines)
```

### Step 6: Update Scenario Routing

In `impl PromptProvider for CreatePullRequestReviewPrompts`:

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("inline_comments") => prompt_inline_comments(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_basic(),  // default to basic
    }
}

fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some(
                "Scenario to show (basic, inline_comments, workflows)".to_string()
            ),
            required: Some(false),
        }
    ]
}
```

---

## Implementation Checklist

### Phase 1: Planning & Structure
- Read current prompts.rs (DONE by executor)
- Identify content blocks in current response (DONE by executor)
- Sketch 3 scenario functions with line targets (DONE by executor)

### Phase 2: Create prompt_basic()
1. Add `fn prompt_basic() -> Vec<PromptMessage>` function
2. Include User question: "How do I review pull requests with different approval statuses?"
3. Add 3 event-type focused examples (APPROVE, REQUEST_CHANGES, COMMENT)
4. Include decision tree for event selection
5. Document body parameter
6. Include response format
7. Add event-specific best practices
8. Target: 100-120 lines

### Phase 3: Create prompt_inline_comments()
1. Add `fn prompt_inline_comments() -> Vec<PromptMessage>` function
2. Include User question: "How do I add inline comments to specific lines?"
3. Add single and multiple file comment examples
4. Explain LEFT/RIGHT side parameter
5. Document line numbering (diff lines, not file lines)
6. Show complete examples with 2-3 files
7. Include best practices section
8. Target: 100-120 lines

### Phase 4: Create prompt_workflows()
1. Add `fn prompt_workflows() -> Vec<PromptMessage>` function
2. Include User question: "What are common review workflows?"
3. Expand 3 workflow patterns from current response
4. Show integration with other tools
5. Include error handling scenarios
6. Add automation best practices
7. Target: 100-120 lines

### Phase 5: Update generate_prompts() & Add prompt_arguments()
1. Replace current method body with match statement
2. Add three scenario cases + default to basic
3. Implement new `prompt_arguments()` method
4. Verify match arms cover all three scenarios

### Phase 6: Verification
1. Line count: `wc -l prompts.rs` → 280-360 lines
2. Scenario functions: `grep "^fn prompt_" prompts.rs` → exactly 3 results
3. Routing logic: Verify match statement has 3 arms + default
4. Argument definition: Verify prompt_arguments() returns scenario descriptions
5. Read through: Verify each scenario is self-contained (can understand without others)

---

## Success Criteria

- ✓ **Total file size**: 280-360 lines (measure: `wc -l prompts.rs`)
- ✓ **Scenario count**: Exactly 3 functions (measure: `grep "^fn prompt_" prompts.rs`)
- ✓ **Function names**: prompt_basic, prompt_inline_comments, prompt_workflows
- ✓ **Routing logic**: match statement with 3 Some arms + default case
- ✓ **Prompt arguments**: defined with scenario names and descriptions
- ✓ **Line distribution**: ~110 lines per scenario, ~30 lines for infrastructure
- ✓ **No comprehensive function**: Verify `grep "comprehensive"` returns 0 results
- ✓ **Self-contained scenarios**: Each scenario covers complete use case
- ✓ **Indentation consistency**: Match create_pull_request.rs style
- ✓ **Code quality**: Compile with `cargo check` in kodegen-mcp-schema package
- ✓ **No breaking changes**: PromptProvider trait implementation unchanged

---

## Code Structure Reference

Follow the exact pattern from `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_pull_request/prompts.rs`:

1. Helper functions (`fn prompt_*()`) that return `Vec<PromptMessage>`
2. Each helper creates User message + Assistant message pair
3. Match statement routes to helpers based on scenario
4. prompt_arguments() returns Vec of PromptArgument
5. Comments above helper functions (e.g., `/// Basic PR creation examples`)

---

## Validation Steps After Implementation

1. **Compilation**: `cd packages/kodegen-mcp-schema && cargo check`
2. **Size verification**: `wc -l src/github/create_pull_request_review/prompts.rs`
3. **Function count**: `grep -c "^fn prompt_" src/github/create_pull_request_review/prompts.rs` (expect: 3)
4. **No comprehensive**: `grep "comprehensive" src/github/create_pull_request_review/prompts.rs` (expect: empty)
5. **Routing coverage**: All three scenarios (basic, inline_comments, workflows) in match statement
6. **Line distribution**: Each scenario approximately 100-120 lines

---

## Definition of Done

The task is complete when:

1. File compiles without errors (`cargo check`)
2. All three scenario functions exist with correct signatures
3. Routing logic properly dispatches based on scenario argument
4. Total file is 280-360 lines
5. Each scenario is self-contained and covers its use case fully
6. Scenarios follow the exact pattern of create_pull_request.rs
7. No comprehensive function exists
8. prompt_arguments() properly documents the three scenarios
9. No functionality is lost compared to original response
10. Code review: can understand all review workflows in under 10 minutes
