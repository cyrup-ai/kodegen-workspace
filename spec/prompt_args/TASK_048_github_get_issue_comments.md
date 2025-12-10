# TASK 048: Trim github_get_issue_comments

**Tool**: `github_get_issue_comments`
**Complexity**: 2 (Simple)
**Current size**: 108 lines (1 scenario)
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_issue_comments/prompts.rs`

---

## Current State Analysis

**Current prompts.rs structure** (108 lines):
- Lines 1-7: File header and imports
- Lines 9-14: Imports and struct definition
- Lines 16-21: impl PromptProvider trait header
- Lines 23-108: Single User-Assistant pair in generate_prompts() function

**Current single scenario breakdown**:
The current implementation has ONE monolithic prompt (lines 23-105) that combines:
1. Basic usage examples (lines 32-45): 4 examples covering owner/repo/issue_number
2. Parameters documentation (lines 46-58): All 8 parameters listed
3. Authentication section (lines 59-65): Required GITHUB_TOKEN info
4. Response structure (lines 66-75): JSON response format example
5. Common workflows (lines 76-95): 3 different workflow patterns
6. Rate limiting info (lines 96-101): Authenticated vs unauthenticated limits
7. Error scenarios (lines 102-108): 3 error cases
8. Best practices (lines 109-119): 10 best practice items

**Key observation**: This is a RETRIEVAL TOOL, not a modification tool. It has:
- 3 required parameters: owner, repo, issue_number
- 5 optional parameters: since, sort, direction, page, per_page
- The pagination parameters (page, per_page) are the DIFFERENTIATOR for a second scenario
- No use-case scenarios (unlike fs_read_file which had code_files, config_files)

---

## Refactoring Instructions

### Step 1: Analyze Parameter Groups

**Group A - Core Parameters (Always used)**:
- owner (required)
- repo (required)
- issue_number (required)

**Group B - Filtering/Sorting (Optional, for basic scenario)**:
- since (ISO 8601 timestamp)
- sort ("created" or "updated")
- direction ("asc" or "desc")

**Group C - Pagination (Optional, for pagination scenario)**:
- page (page number)
- per_page (results per page, max 100)

**Decision**: The presence of pagination-specific parameters (page, per_page) naturally creates two distinct scenarios.

### Step 2: Create Scenario Routing Structure

Update generate_prompts() function (lines 23-25):

**BEFORE** (no routing, single function):
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_get_issue_comments to retrieve issue comments?",
            ),
        },
        PromptMessage {
            // ... massive 80+ line assistant response
        },
    ]
}
```

**AFTER** (with routing to two scenarios):
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match _args.scenario.as_deref() {
        Some("pagination") => prompt_pagination(),
        _ => prompt_basic(),
    }
}
```

Also update prompt_arguments() to document the scenario option:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, pagination)".to_string()),
            required: Some(false),
        }
    ]
}
```

### Step 3: Create prompt_basic() Function (~100-110 lines)

**Purpose**: Teach simple issue comment retrieval

**Structure**:
- User-Assistant pair
- Question: "How do I retrieve comments from a GitHub issue?"
- Assistant response includes:

1. **Quick examples** (15-20 lines):
   - Get all comments: `github_get_issue_comments({owner, repo, issue_number})`
   - Get recent comments: with `since` parameter
   - Get sorted comments: with `sort` and `direction`

2. **Core parameters** (10 lines):
   - owner (required): Repository owner
   - repo (required): Repository name
   - issue_number (required): Issue number
   - Brief mention of optional params

3. **Response structure** (15 lines):
   - Basic JSON response example
   - Key fields: id, body, author, created_at, updated_at, html_url

4. **Authentication** (5 lines):
   - Requires GITHUB_TOKEN
   - Scopes needed

5. **Common use cases** (20-25 lines):
   - Issue analysis: get issue details, retrieve comments, analyze discussion
   - Sentiment analysis: fetch comments, analyze tone
   - Extract decisions: get comments, parse for key decisions

6. **Error scenarios** (10 lines):
   - 404: Issue or repo doesn't exist
   - 403: No access to private repo
   - 410: Issue deleted/transferred

7. **Best practices** (8-10 lines):
   - Check issue.comments_count before fetching
   - Parse comment body for mentions, links, code blocks
   - Use timestamps to build timeline
   - Combine with github_add_issue_comment for full workflow

**Estimated lines**: 100-115

### Step 4: Create prompt_pagination() Function (~80-110 lines)

**Purpose**: Teach advanced retrieval with pagination and filtering

**Structure**:
- User-Assistant pair
- Question: "How do I handle pagination and large comment threads?"
- Assistant response includes:

1. **Why pagination matters** (10 lines):
   - Issues can have hundreds of comments
   - GitHub limits to 30 per page by default
   - Max per_page is 100

2. **Pagination examples** (20-25 lines):
   - Get page 2: `per_page: 50, page: 2`
   - Get all pages iteratively
   - Build pagination loop example

3. **All parameters explained** (15-20 lines):
   - since: Filter by timestamp (ISO 8601)
   - sort: "created" vs "updated"
   - direction: "asc" vs "desc"
   - page: Which page to fetch
   - per_page: Results per page (max 100, default 30)

4. **Pagination workflows** (20-25 lines):
   - Fetch first 100 recent comments
   - Build comment archive (iterate through all pages)
   - Get comments since last check
   - Get latest X comments (sort by updated, desc)

5. **Rate limiting** (10 lines):
   - Authenticated: 5,000 requests/hour
   - Each page counts as 1 request
   - Check X-RateLimit-Remaining header
   - Calculate requests needed for full thread

6. **Memory efficiency** (10 lines):
   - Paginate instead of fetching all at once
   - Use since parameter to get only new comments
   - Cache comments locally, update incrementally

7. **Best practices** (8-10 lines):
   - Use pagination for issues with many comments
   - Filter by since to get only recent updates
   - Sort by updated to see recent activity first
   - Respect rate limits for bulk retrieval
   - Combine pagination with since for incremental updates

**Estimated lines**: 90-115

### Step 5: Consolidate and Remove Duplication

**Remove from both scenarios**:
- Decorative headers (none currently present, good)
- Repeated "best practices" sections (consolidate into each scenario)
- Verbose explanations of obvious concepts
- Redundant error handling (each scenario shows relevant errors only)

**Content audit**:
- Basic: Errors 404, 403, 410
- Pagination: Rate limiting errors, pagination-specific errors
- No overlap of core examples
- Each scenario teaches different aspects

---

## Code Implementation Guide

### File: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_issue_comments/prompts.rs`

**Step 1: Keep imports unchanged** (Lines 1-7)
```rust
//! Prompt messages for github_get_issue_comments tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetIssueCommentsPromptArgs;
```

**Step 2: Replace impl PromptProvider block** (Lines 9-31)

BEFORE:
```rust
pub struct GetIssueCommentsPrompts;

impl PromptProvider for GetIssueCommentsPrompts {
    type PromptArgs = GetIssueCommentsPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_get_issue_comments to retrieve issue comments?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            // ... 80+ lines of content
        },
    ]
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}
```

AFTER:
```rust
/// Prompt provider for get_issue_comments tool
pub struct GetIssueCommentsPrompts;

impl PromptProvider for GetIssueCommentsPrompts {
    type PromptArgs = GetIssueCommentsPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("pagination") => prompt_pagination(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, pagination)".to_string()),
                required: Some(false),
            }
        ]
    }
}
```

**Step 3: Add helper comment and function stubs**

Add after impl block (before scenario functions):
```rust
// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GITHUB_GET_ISSUE_COMMENTS
// ============================================================================
```

**Step 4: Add prompt_basic() function** (~100-115 lines)

Start with structure:
```rust
/// Basic issue comment retrieval
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I retrieve comments from a GitHub issue?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_issue_comments tool retrieves all comments on a specific GitHub issue...\n\n\
                 BASIC USAGE:\n\
                 1. Get all comments:\n\
                    github_get_issue_comments({\"owner\": \"user\", \"repo\": \"project\", \"issue_number\": 123})\n\n\
                 2. Get recent comments:\n\
                    github_get_issue_comments({\"owner\": \"user\", \"repo\": \"project\", \"issue_number\": 123, \"since\": \"2024-01-01T00:00:00Z\"})\n\n\
                 3. Get sorted comments:\n\
                    github_get_issue_comments({\"owner\": \"user\", \"repo\": \"project\", \"issue_number\": 123, \"sort\": \"created\", \"direction\": \"desc\"})\n\n\
                 CORE PARAMETERS:\n\
                 - owner (required): Repository owner\n\
                 - repo (required): Repository name\n\
                 - issue_number (required): Issue number\n\n\
                 OPTIONAL PARAMETERS:\n\
                 - since: Filter comments updated after this timestamp (ISO 8601)\n\
                 - sort: \"created\" or \"updated\" (default: \"created\")\n\
                 - direction: \"asc\" or \"desc\" (default: \"asc\")\n\n\
                 RESPONSE:\n\
                 Returns JSON with:\n\
                 - count: Number of comments returned\n\
                 - comments: Array of comment objects with id, body, author, created_at, updated_at, html_url\n\n\
                 COMMON USE CASES:\n\
                 1. Issue analysis:\n\
                    - Get issue details first\n\
                    - Retrieve comments to understand discussion\n\
                    - Extract key decisions from conversation\n\n\
                 2. Sentiment analysis:\n\
                    - Fetch all comments on issue\n\
                    - Analyze tone and feedback\n\
                    - Identify concerns or suggestions\n\n\
                 3. Response tracking:\n\
                    - Get recent comments (since parameter)\n\
                    - Check for team member responses\n\
                    - Identify unanswered questions\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found: Issue doesn't exist\n\
                    Fix: Verify owner/repo/issue_number\n\
                 2. 403 Forbidden: No access to private repository\n\
                    Fix: Verify GITHUB_TOKEN has repo access\n\
                 3. 410 Gone: Issue was deleted or transferred\n\
                    Fix: Issue no longer accessible\n\n\
                 BEST PRACTICES:\n\
                 - Check issue.comments_count before fetching\n\
                 - Use since parameter to get only recent updates\n\
                 - Parse comment body for mentions, links, code blocks\n\
                 - Use timestamps to build timeline of discussion\n\
                 - Combine with github_add_issue_comment for full workflow",
            ),
        },
    ]
}
```

**Step 5: Add prompt_pagination() function** (~90-110 lines)

```rust
/// Issue comment pagination and advanced filtering
fn prompt_pagination() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I handle pagination for issues with many comments?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Large GitHub issues can have hundreds of comments. The github_get_issue_comments tool supports pagination to efficiently retrieve them.\n\n\
                 WHY PAGINATION:\n\
                 - GitHub returns max 100 results per page\n\
                 - Default is 30 comments per page\n\
                 - Issues can have thousands of comments\n\
                 - Use page and per_page parameters\n\n\
                 PAGINATION EXAMPLES:\n\n\
                 1. Get second page of 50 results:\n\
                    github_get_issue_comments({\n\
                        \"owner\": \"rust-lang\",\n\
                        \"repo\": \"rust\",\n\
                        \"issue_number\": 98765,\n\
                        \"per_page\": 50,\n\
                        \"page\": 2\n\
                    })\n\n\
                 2. Iterate through all pages:\n\
                    for page in 1..max_pages {\n\
                      const comments = github_get_issue_comments({\n\
                        \"owner\": \"rust-lang\",\n\
                        \"repo\": \"rust\",\n\
                        \"issue_number\": 98765,\n\
                        \"per_page\": 100,\n\
                        \"page\": page\n\
                      });\n\
                      if comments.count < 100 break;  // Last page\n\
                    }\n\n\
                 3. Get latest 100 comments (most recent):\n\
                    github_get_issue_comments({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"issue_number\": 123,\n\
                        \"sort\": \"updated\",\n\
                        \"direction\": \"desc\",\n\
                        \"per_page\": 100\n\
                    })\n\n\
                 ALL PARAMETERS:\n\
                 - owner (required): Repository owner\n\
                 - repo (required): Repository name\n\
                 - issue_number (required): Issue number\n\
                 - since (optional): Only comments after timestamp (ISO 8601)\n\
                 - sort (optional): \"created\" or \"updated\" (default: \"created\")\n\
                 - direction (optional): \"asc\" or \"desc\" (default: \"asc\")\n\
                 - page (optional): Page number for pagination (default: 1)\n\
                 - per_page (optional): Results per page, max 100 (default: 30)\n\n\
                 PAGINATION WORKFLOWS:\n\n\
                 1. Archive all comments:\n\
                    - Start with page 1, per_page 100\n\
                    - Save all comments\n\
                    - Increment page until fewer than 100 results\n\
                    - Combine into single archive\n\n\
                 2. Incremental updates:\n\
                    - Get last stored comment timestamp\n\
                    - Use since parameter to fetch only new\n\
                    - Merge with existing comments\n\
                    - Save new timestamp for next update\n\n\
                 3. Recent activity focus:\n\
                    - Sort by \"updated\" descending\n\
                    - Get first 100 (most recent changes)\n\
                    - Identify what's being discussed now\n\n\
                 RATE LIMITING:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Each page = 1 API request\n\
                 - Issue with 10,000 comments = 100 requests at per_page=100\n\
                 - Check X-RateLimit-Remaining header\n\
                 - Plan pagination strategy to avoid hitting limits\n\n\
                 MEMORY EFFICIENCY:\n\
                 - Use per_page=100 to minimize request count\n\
                 - Don't fetch all at once for huge threads\n\
                 - Use since parameter to fetch only updates\n\
                 - Process pages incrementally instead of accumulating\n\
                 - Cache results locally, update on demand\n\n\
                 BEST PRACTICES:\n\
                 - Use per_page=100 for most scenarios\n\
                 - Combine since with pagination for incremental fetching\n\
                 - Sort by updated to see recent activity first\n\
                 - Calculate total requests needed before bulk retrieval\n\
                 - Respect rate limits and implement backoff\n\
                 - Use caching to avoid re-fetching same comments\n\
                 - Consider storing locally to reduce future API calls",
            ),
        },
    ]
}
```

---

## Line Count Validation

**Before refactoring**: 108 lines

**After refactoring target breakdown**:
- Lines 1-14: Imports + struct definition + doc comment (unchanged)
- Lines 16-35: impl PromptProvider block with routing (20 lines)
- Lines 37-40: Helper comment
- Lines 42-145: prompt_basic() function (~105 lines)
- Lines 147-260: prompt_pagination() function (~115 lines)

**Total expected**: ~260 lines (but will compress to ~200-220 with proper formatting)

Actually targeting 170-220 total lines means consolidating content further.

---

## Success Criteria

- ✓ **File size**: 170-220 lines total (currently 108 → add 62-112 lines)
- ✓ **Scenario count**: Exactly 2 functions (prompt_basic, prompt_pagination)
- ✓ **Routing**: Match statement routes "pagination" → prompt_pagination(), else → prompt_basic()
- ✓ **Parameters demonstrated**: All 8 parameters taught (owner, repo, issue_number, since, sort, direction, page, per_page)
- ✓ **No duplication**: Each scenario teaches different aspect (basic vs pagination)
- ✓ **No comprehensive scenario**: Unlike larger tools, keep it focused
- ✓ **Pagination well-covered**: Second scenario thoroughly explains page/per_page logic
- ✓ **Real examples**: Code examples show actual usage patterns
- ✓ **Error handling**: Each scenario covers relevant errors
- ✓ **Best practices**: Consolidated into each scenario appropriately

---

## Validation Checklist

After implementation, verify:

1. **Line count**: `wc -l prompts.rs` → 170-220 lines ✓
2. **Scenario functions**: `grep "^fn prompt_" prompts.rs` → exactly 2 lines (prompt_basic, prompt_pagination) ✓
3. **Routing logic**: `grep -A3 "fn generate_prompts"` → shows match statement ✓
4. **Parameter coverage**: `grep -E "per_page|page|since|sort|direction"` → all appear in pagination scenario ✓
5. **No comprehensive**: `grep "comprehensive"` → 0 results ✓
6. **Duplicate check**: No section appears in both scenarios ✓
7. **Readability**: Can understand pagination vs basic difference in <2 minutes ✓

---

## Notes for Executor

- The current prompt_args.rs has empty GetIssueCommentsPromptArgs struct. NO changes needed there - the scenario routing uses a string field.
- The tool itself (in github tools package) doesn't need changes, only the prompts.
- This tool is already mature and doesn't have use-case scenarios to remove
- Focus is on SPLITTING existing content into two focused scenarios
- Use prompt_arguments() to document that scenario parameter exists
