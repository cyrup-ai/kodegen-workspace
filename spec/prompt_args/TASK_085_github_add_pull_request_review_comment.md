# TASK 085: Trim github_add_pull_request_review_comment

**Tool**: `github_add_pull_request_review_comment`
**Complexity**: 3 (Medium)
**Current size**: 101 lines (1 monolithic scenario)
**Target size**: 310 lines (3 focused scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/add_pull_request_review_comment/prompts.rs`

---

## Reference

This task follows the **Complexity 3 pattern** established in **PRECURSOR_03_git_branch_create.md**.

---

## Current State Analysis

The file contains 101 lines with a single monolithic user-assistant conversation pair that covers ALL scenarios (basic single-line comments, multi-line spanning comments, LEFT/RIGHT side comments, and threaded replies) in one massive assistant response.

### Current Structure (101 lines)

```
Lines 1-8:    Module documentation and imports
Lines 9-14:   AddPullRequestReviewCommentPrompts struct and PromptProvider impl
Lines 15-17:  generate_prompts() function start and vec![ macro
Lines 18-22:  User question (single line)
Lines 23-101: Assistant response (ONE MONOLITHIC RESPONSE covering everything):
  - Basic usage examples (single line, multi-line, LEFT side, reply)
  - Parameters documentation
  - Authentication notes
  - Response format
  - Common workflows
  - Rate limiting
  - Error scenarios
  - Best practices
```

**Problem**: This single 78-line assistant response tries to cover 4 distinct use cases in one block, making it hard to navigate and overly verbose.

---

## Trimming Instructions

### Step 1: Create Three Scenario Functions

You will refactor the single `generate_prompts()` function into THREE separate scenario functions, each returning its own `Vec<PromptMessage>`:

#### Scenario 1: `prompt_basic()` (TARGET: ~110 lines)

**Purpose**: Basic single-line comment workflow (the most common use case)

**Structure**: 
```rust
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage { role: User, content: "How do I use github_add_pull_request_review_comment to add inline code review comments?" },
        PromptMessage { role: Assistant, content: "The github_add_pull_request_review_comment tool adds inline comments to specific lines in a pull request's diff for detailed code review.\n\nBASIC USAGE:\n1. Single line comment:\n   github_add_pull_request_review_comment({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"pull_number\": 5678, \"body\": \"Consider using `match` here\", \"commit_id\": \"abc123\", \"path\": \"src/lib.rs\", \"line\": 42})\n\n2. Comment on deletion (LEFT side):\n   github_add_pull_request_review_comment({\"owner\": \"actix\", \"repo\": \"actix-web\", \"pull_number\": 999, \"body\": \"Why remove this?\", \"commit_id\": \"ghi789\", \"path\": \"src/handler.rs\", \"line\": 20, \"side\": \"LEFT\"})\n\nPARAMETERS:\n- owner (required): Repository owner (username or organization)\n- repo (required): Repository name\n- pull_number (required): Pull request number\n- body (required): Comment text (Markdown supported)\n- commit_id (required for new comments): Commit SHA\n- path (required for new comments): File path in repository\n- line (required for new comments): Line number in diff\n- side (optional): \"LEFT\" (deletion) or \"RIGHT\" (addition, default)\n\nAUTHENTICATION:\nRequires GITHUB_TOKEN environment variable with scopes:\n- repo (for private repositories)\n- public_repo (for public repositories only)\n\nRESPONSE:\nReturns JSON with:\n- success: true/false\n- owner, repo: Repository identifiers\n- pr_number: Pull request number\n- comment_id: Created comment ID\n- message: Status message\n\nCOMMON WORKFLOW:\n1. Automated code review:\n   - Analyze PR diff for patterns\n   - Add inline suggestions on specific lines\n   - Focus on security, performance, style issues\n\nRATE LIMITING:\n- Authenticated: 5,000 requests/hour\n- Unauthenticated: 60 requests/hour\n\nERROR SCENARIOS:\n1. 404 Not Found: PR, commit, or file path doesn't exist\n   Fix: Verify pull_number and commit_id are valid\n2. 422 Unprocessable: Invalid line number or path\n   Fix: Line must exist in commit's diff, not entire file\n3. 403 Forbidden: Token lacks required scopes\n   Fix: Generate new token with 'repo' scope\n\nBEST PRACTICES:\n- Use commit_id from latest PR commit for accuracy\n- Line numbers are relative to diff, not file line numbers\n- Use RIGHT side for new/modified code (default)\n- Use LEFT side for deleted code or original version\n- Keep comments constructive and specific\n- Use code suggestions (```suggestion) when possible" }
    ]
}
```

**Keep from current content**:
- Single-line comment example (line 30-31 area)
- LEFT side deletion example (line 35-36 area)
- Basic parameter list (owner, repo, pull_number, body, commit_id, path, line, side)
- Authentication section (GITHUB_TOKEN requirements)
- Response format (success, owner, repo, pr_number, comment_id, message)
- Automated code review workflow (from COMMON WORKFLOWS)
- Rate limiting info
- Error scenarios 1, 2, 3 (404, 422, 403)
- Best practices about commit_id, line numbers, LEFT/RIGHT usage, constructive feedback

**Delete**:
- Multi-line examples (moved to prompt_multiline)
- Reply to existing comment example (moved to prompt_reply)
- Documentation review workflow (for clarity, can be inferred)
- Test coverage feedback workflow (for clarity, can be inferred)
- start_line parameter documentation (for multiline scenario)
- start_side parameter documentation (for multiline scenario)
- in_reply_to parameter documentation (for reply scenario)

---

#### Scenario 2: `prompt_multiline()` (TARGET: ~110 lines)

**Purpose**: Multi-line spanning comments and side parameter nuances

**Structure**:
```rust
fn prompt_multiline() -> Vec<PromptMessage> {
    vec![
        PromptMessage { role: User, content: "How do I use github_add_pull_request_review_comment for multi-line comments?" },
        PromptMessage { role: Assistant, content: "The github_add_pull_request_review_comment tool supports multi-line comments that span multiple lines in a pull request diff using the start_line parameter.\n\nMULTI-LINE USAGE:\n1. Multi-line comment spanning lines:\n   github_add_pull_request_review_comment({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"pull_number\": 123, \"body\": \"This entire block could be simplified\", \"commit_id\": \"def456\", \"path\": \"compiler/rustc/src/main.rs\", \"start_line\": 100, \"line\": 105})\n\n2. Multi-line deletion (LEFT side):\n   github_add_pull_request_review_comment({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"pull_number\": 999, \"body\": \"This logic was complex, good to remove\", \"commit_id\": \"xyz789\", \"path\": \"src/runtime.rs\", \"start_line\": 45, \"line\": 52, \"side\": \"LEFT\"})\n\nMULTI-LINE PARAMETERS:\n- start_line (optional): Start line for multi-line comments (must be < line)\n- line (required): End line number for comment range\n- start_side (optional): Side of start_line (\"LEFT\" or \"RIGHT\")\n- side (optional): Side of end line (\"LEFT\" or \"RIGHT\")\n\nWHEN TO USE EACH SIDE:\n- RIGHT side (default): For added or modified code in the diff\n- LEFT side: For deleted code or original version before changes\n\nLINE NUMBERING:\n- Line numbers are relative to the diff, NOT file line numbers\n- For multi-line comments: start_line must be less than line\n- Both lines must exist in the commit's diff context\n\nCOMMON MULTI-LINE WORKFLOW:\n1. Identify block of code to review (e.g., lines 45-52)\n2. Create multi-line comment:\n   - start_line: 45\n   - line: 52\n   - body: Explain the issue with the entire block\n3. Include specific suggestions for refactoring\n\nSIDE PARAMETER EXPLAINED:\n- RIGHT (addition): Use for new code or modified sections\n  - Default side if not specified\n  - Use for feature additions, bug fixes\n- LEFT (deletion): Use for removed code\n  - Explain why removal was good\n  - Or comment on what was removed in context\n\nBEST PRACTICES:\n- Multi-line comments must have start_line < line\n- Keep multi-line comments focused (max 5-7 lines)\n- Use single-line comments for small fixes\n- Use multi-line only when commenting on cohesive blocks\n- Ensure all lines in range are relevant to the comment\n\nERROR SCENARIOS:\n1. 422 Unprocessable: start_line >= line\n   Fix: Ensure start_line is less than line\n2. 422 Unprocessable: Lines don't exist in diff\n   Fix: Lines must be in the commit's diff context, not just the file" }
    ]
}
```

**Keep from current content**:
- Multi-line example (line 32-33 area)
- Multi-line deletion example (synthesize from LEFT side discussion)
- start_line, line, start_side, side parameters
- Explanation of LEFT vs RIGHT sides
- Line numbering relative to diff (not file)
- Documentation review workflow (rephrased as "identify block of code")
- Multi-line best practices
- Error scenarios specific to multi-line (422 for start_line >= line)

**Delete**:
- Single-line basic example (in prompt_basic)
- Reply example (in prompt_reply)
- AUTHENTICATION section (duplicate, only in prompt_basic)
- RESPONSE format (duplicate, only in prompt_basic)
- RATE LIMITING (duplicate, only in prompt_basic)
- Automated code review workflow (in prompt_basic)

---

#### Scenario 3: `prompt_reply()` (TARGET: ~90 lines)

**Purpose**: Threaded discussions by replying to existing comments

**Structure**:
```rust
fn prompt_reply() -> Vec<PromptMessage> {
    vec![
        PromptMessage { role: User, content: "How do I reply to existing code review comments in github_add_pull_request_review_comment?" },
        PromptMessage { role: Assistant, content: "The github_add_pull_request_review_comment tool supports replying to existing comments using the in_reply_to parameter to create threaded discussions.\n\nREPLY USAGE:\n1. Reply to existing comment:\n   github_add_pull_request_review_comment({\"owner\": \"serde-rs\", \"repo\": \"serde\", \"pull_number\": 456, \"body\": \"Good point! Fixed in latest commit\", \"in_reply_to\": 987654321})\n\nREPLY PARAMETERS:\n- in_reply_to (required for replies): Comment ID to reply to\n  - Do NOT include: commit_id, path, line when using in_reply_to\n  - These are automatically inherited from parent comment\n- body (required): Reply text (Markdown supported)\n- owner, repo, pull_number (required): Repository context\n\nWHEN TO USE REPLY MODE:\n- Author acknowledges feedback: \"Good point!\"\n- Explain implementation: \"This was fixed in abc1234\"\n- Ask clarifying questions: \"Did you mean X or Y?\"\n- Continue discussion threads without new diff comments\n\nTHREADED DISCUSSION WORKFLOW:\n1. Reviewer adds inline comment on line 50\n2. Author responds: \"Good catch, but can you clarify?\"\n3. Reviewer replies: \"Yes, the issue is that...\"\n4. Author replies: \"Got it, fixed in latest commit\"\n\nOBTAINING COMMENT IDS:\n- From github_search_issues or previous github_add_pull_request_review_comment calls\n- Comment ID returned in response.comment_id field\n- Use grep or parse PR discussion to find existing comment IDs\n\nREPLY vs NEW COMMENT:\n- Use in_reply_to for threaded discussions (keeps context)\n- Use commit_id+path+line for new inline comments on diff\n- Can't reply to arbitrary comments (must be on same PR)\n\nBEST PRACTICES:\n- Keep replies brief and focused\n- Reference specific lines if clarifying\n- Use thread conversations to keep PR discussion organized\n- Don't create new comment when reply would suffice\n- Include implementation details (commit SHA) when applicable\n\nERROR SCENARIOS:\n1. 404 Not Found: Comment ID doesn't exist\n   Fix: Verify in_reply_to comment ID is valid and on same PR\n2. 422 Unprocessable: Trying to reply with commit_id/path/line\n   Fix: When using in_reply_to, do NOT include commit_id, path, or line\n3. 403 Forbidden: Token lacks permissions\n   Fix: Use GITHUB_TOKEN with 'repo' scope" }
    ]
}
```

**Keep from current content**:
- Reply to existing comment example (line 37-38 area)
- in_reply_to parameter documentation
- Explanation that commit_id, path, line are NOT needed for replies
- Discussion/reply workflow concept
- Threaded conversation benefits
- Error handling specific to replies (404 for comment ID, 422 for mixing parameters)

**Delete**:
- All single-line examples (in prompt_basic)
- Multi-line examples (in prompt_multiline)
- AUTHENTICATION section (in prompt_basic)
- RESPONSE format (in prompt_basic)
- RATE LIMITING (in prompt_basic)
- Code review workflow (in prompt_basic)

---

### Step 2: Update the PromptProvider Implementation

**Current routing** (lines ~15-101):
```rust
impl PromptProvider for AddPullRequestReviewCommentPrompts {
    type PromptArgs = AddPullRequestReviewCommentPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
        PromptMessage { role: PromptMessageRole::User, content: PromptMessageContent::text(...) },
        PromptMessage { role: PromptMessageRole::Assistant, content: PromptMessageContent::text(...) },
    ]
    }
```

**New routing pattern** (follows git_branch_create model):
```rust
impl PromptProvider for AddPullRequestReviewCommentPrompts {
    type PromptArgs = AddPullRequestReviewCommentPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario {
            Some("multiline") => prompt_multiline(),
            Some("reply") => prompt_reply(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}
```

**Default scenario**: When no scenario specified, return `prompt_basic()` (single-line comments - most common use case)

**Named scenarios**:
- `"multiline"` → `prompt_multiline()`
- `"reply"` → `prompt_reply()`
- Default (None) → `prompt_basic()`

---

### Step 3: Validate Line Counts

After creating the three functions, verify:

```bash
# Total lines should be 310-330
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/add_pull_request_review_comment/prompts.rs

# Breakdown (approximate):
# Lines 1-8:     Module header, imports (8 lines)
# Lines 9-14:    Struct definition (6 lines)
# Lines 15-100:  prompt_basic() function (~86 lines)
# Lines 101-185: prompt_multiline() function (~85 lines)
# Lines 186-260: prompt_reply() function (~75 lines)
# Lines 261-280: PromptProvider impl with routing (~20 lines)
# Lines 281-310: prompt_arguments() and closing braces (~30 lines)
```

---

## Success Criteria

- ✓ **Total file size**: 310-330 lines
- ✓ **Three scenario functions**: `prompt_basic()`, `prompt_multiline()`, `prompt_reply()`
- ✓ **Each function returns**: `Vec<PromptMessage>` with exactly one user-assistant pair
- ✓ **Routing match statement**: Handles scenarios "multiline", "reply", with default to "basic"
- ✓ **Scenario sizes**:
  - `prompt_basic()`: ~110 lines
  - `prompt_multiline()`: ~110 lines
  - `prompt_reply()`: ~90 lines
- ✓ **No monolithic scenario**: Each PromptMessage pair is focused on one use case
- ✓ **Content split correctly**:
  - Basic: single-line comments, LEFT side, authentication, response format, rate limiting, basic error scenarios
  - Multiline: multi-line spanning, start_line parameter, both sides, diff line numbering
  - Reply: in_reply_to parameter, threaded discussions, no commit/path/line needed
- ✓ **Content removed**: No duplication across scenarios (each appears in only the most relevant scenario)
- ✓ **Routing works**: `match args.scenario` with three branches plus default

---

## Validation Steps

After implementation, verify:

1. **Line count**: `wc -l prompts.rs` → 310-330 lines
2. **Function count**: `grep "^fn prompt_" prompts.rs | wc -l` → should output 3
3. **Function names**: 
   ```bash
   grep "^fn prompt_" prompts.rs
   # Should output:
   # fn prompt_basic()
   # fn prompt_multiline()
   # fn prompt_reply()
   ```
4. **Routing present**: `grep "match args.scenario" prompts.rs` → 1 match (in generate_prompts)
5. **Routing branches**: Should have 3 branches in match:
   - `Some("multiline")` → `prompt_multiline()`
   - `Some("reply")` → `prompt_reply()`
   - `_` → `prompt_basic()`
6. **No duplicate content**: Grep for unique concepts (e.g., "authentication" should appear only in prompt_basic)
7. **Compilation**: Run `cargo check` in the github tools package
8. **Understanding**: Read through all three functions and verify you can explain each in one sentence

---

## Implementation Notes

- Each scenario function is INDEPENDENT - can be read and understood separately
- Each scenario answers its own User question fully
- No scenario should reference "see scenario X" or cross-reference other scenarios
- Keep parameter documentation brief in each scenario (only parameters used in that scenario)
- Authentication/response format/rate limiting only needed once (in prompt_basic)
- Error scenarios should be specific to that scenario's use case
- Follow the exact structure: `PromptMessage { role: PromptMessageRole::*, content: PromptMessageContent::text(...) }`
