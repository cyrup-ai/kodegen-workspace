# TASK 050: Trim github_get_pull_request_reviews

**Tool**: `github_get_pull_request_reviews`
**Complexity**: 2 (Simple)
**Current size**: 107 lines (1 comprehensive scenario)
**Target size**: 170-220 lines (1-2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_pull_request_reviews/prompts.rs`
**Reference**: See PRECURSOR_02_fs_read_file.md for Complexity 2 template pattern

---

## Current State Analysis

### File Location
`/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_pull_request_reviews/prompts.rs`

### Current Structure (107 lines)
The file currently contains:
- Lines 1-11: Imports and struct definition
- Lines 13-19: PromptProvider trait implementation header
- Lines 20-104: Single comprehensive prompt (User/Assistant pair)
- Lines 105-107: prompt_arguments() returning empty vec

### Current Scenario Content
**Single comprehensive scenario** (lines 20-104):
- User asks: "How do I use github_get_pull_request_reviews to retrieve PR reviews?"
- Assistant response is a monolithic block covering:
  1. Tool description (1 paragraph)
  2. BASIC USAGE section (4 example calls: all reviews, paginated, approval status, review history)
  3. PARAMETERS section (owner, repo, pull_number, page, per_page)
  4. AUTHENTICATION section (GITHUB_TOKEN requirement, scopes)
  5. RESPONSE section (success, owner, repo, pr_number, count, reviews array with fields)
  6. COMMON WORKFLOWS section (merge readiness, review tracking, compliance verification)
  7. RATE LIMITING section (authenticated/unauthenticated limits, header info)
  8. ERROR SCENARIOS section (404, 403, 422 with fixes)
  9. BEST PRACTICES section (13 bullet points on filtering, checking commit_id, counting states, etc.)

### PromptArgs Structure
Located in `prompt_args.rs`: `GetPullRequestReviewsPromptArgs` is an **empty struct** with no customization arguments. This means the tool has no scenario variants.

### No Scenario Enum
Unlike fs_read_file which has `FsReadFileScenario` enum with variants (Basic, LargeFiles, etc.), this tool has no enum. The routing simply generates one static prompt.

---

## Trimming Strategy

### Analysis: Why Trim?

The current 107-line comprehensive prompt violates Complexity 2 principles:
- **Single massive block**: All information crammed into one Assistant message
- **Redundant sections**: "RESPONSE section" repeats what "BASIC USAGE" examples show
- **Use-case workflows**: COMMON WORKFLOWS section teaches workflows, not the tool itself
- **Excessive detail**: BEST PRACTICES lists 13 distinct approaches; most are workflows not tool features

### Trim to 2 Scenarios

**KEEP - Scenario 1: Basic Review Retrieval** (~85-100 lines)
- Tool description and core purpose
- Basic usage examples (all reviews, paginated, approval status)
- Parameters explanation (required: owner, repo, pull_number; optional: page, per_page)
- Response structure (success, count, reviews array with key fields)
- Common use-case: checking if PR is approved
- Authentication requirement (GITHUB_TOKEN)
- One error scenario example (404 Not Found)

**KEEP - Scenario 2: Advanced Review Analysis** (~85-100 lines)
- Purpose: filtering and analyzing review states for automation
- When to use this scenario: building workflows that depend on review states
- Examples showing:
  - Filtering by state (APPROVED, CHANGES_REQUESTED, COMMENTED, DISMISSED)
  - Checking commit_id for currency (outdated reviews)
  - Counting states for merge eligibility
  - Using submitted_at for timeline tracking
- Workflow example: merge readiness check (count approvals, check for blocking changes)
- Rate limiting information (5,000/hour authenticated, 60 unauthenticated)
- Error handling (403 Forbidden, 422 Unprocessable)

### Delete from Current Comprehensive Prompt

These sections stay in the **overview** but are removed from detailed scenarios:
- "Review history" example (duplicate of "all reviews" with different repo)
- Entire "COMMON WORKFLOWS" section → condense to 2-3 specific examples in scenarios
- "BEST PRACTICES" list of 13 items → keep only 3-4 essential ones in advanced scenario
- Verbose explanations of obvious concepts

---

## Step-by-Step Implementation

### Step 1: Create Two Scenario Functions

Replace the single prompt with two functions. Structure:

```rust
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_get_pull_request_reviews to check PR reviews?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                // ~85-100 lines of focused basic usage
            ),
        },
    ]
}

fn prompt_review_analysis() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I analyze PR reviews programmatically?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                // ~85-100 lines on filtering, states, automation
            ),
        },
    ]
}
```

### Step 2: Write prompt_basic() Content

Include these sections (total: ~85-100 lines):

**Opening (8 lines)**
```
The github_get_pull_request_reviews tool retrieves all reviews submitted on a pull request.

BASIC USAGE:
1. Get all reviews:
   github_get_pull_request_reviews({"owner": "tokio-rs", "repo": "tokio", "pull_number": 5678})

2. Paginated reviews (50 per page):
   github_get_pull_request_reviews({"owner": "rust-lang", "repo": "rust", "pull_number": 123, "per_page": 50, "page": 1})

3. Check if PR is approved:
   github_get_pull_request_reviews({"owner": "actix", "repo": "actix-web", "pull_number": 999})
```

**Parameters (8 lines)**
```
PARAMETERS:
- owner (required): Repository owner (username or organization)
- repo (required): Repository name  
- pull_number (required): Pull request number
- page (optional): Page number for pagination
- per_page (optional): Results per page (max 100, default 30)
```

**Response Structure (15 lines)**
```
RESPONSE:
Returns JSON with:
- success: true/false
- count: Number of reviews returned
- reviews: Array of review objects:
  - id: Review ID
  - user: Reviewer username
  - body: Review comment text
  - state: "APPROVED", "CHANGES_REQUESTED", "COMMENTED", "DISMISSED", "PENDING"
  - commit_id: Commit SHA reviewed
  - submitted_at: Review submission timestamp
  - html_url: Link to review on GitHub
```

**Authentication (6 lines)**
```
AUTHENTICATION:
Requires GITHUB_TOKEN environment variable with scopes:
- repo (for private repositories)
- public_repo (for public repositories only)
```

**Common Pattern (12 lines)**
```
COMMON PATTERN - Check if PR has approvals:
- Call github_get_pull_request_reviews with owner/repo/pull_number
- Count reviews with state="APPROVED"
- If count >= required_approvals, PR is approvable
- Always use this tool before merging to verify approval status
```

**Error Handling (10 lines)**
```
ERROR HANDLING:
1. 404 Not Found: PR or repository doesn't exist
   Fix: Verify owner/repo/pull_number are correct

2. 403 Forbidden: No access to private repository
   Fix: Verify GITHUB_TOKEN has repo access
```

**Line Total**: 8 + 8 + 15 + 6 + 12 + 10 = 59 lines (add connector sentences to reach 85-100)

### Step 3: Write prompt_review_analysis() Content

Include these sections (total: ~85-100 lines):

**Opening (6 lines)**
```
Analyzing pull request reviews helps automate approval workflows and compliance checks.

ANALYZING REVIEW STATES:
The 'state' field determines the review's impact on merge eligibility.
```

**State Filtering Examples (20 lines)**
```
FILTER BY STATE:
1. Find blocking changes:
   Filter reviews where state="CHANGES_REQUESTED"
   Example: github_get_pull_request_reviews({...}) then filter by state

2. Count approvals:
   Filter reviews where state="APPROVED"
   Count results to determine if PR meets approval requirements

3. Identify pending reviews:
   Filter reviews where state="PENDING" or state="COMMENTED"
   These do not block merging

4. Find dismissed reviews:
   Filter state="DISMISSED" (reviews marked obsolete)
   These should be ignored in automation logic
```

**Commit Freshness (10 lines)**
```
CHECK REVIEW FRESHNESS:
Reviews are stale if their commit_id does not match the PR's latest commit.
- Get reviews with github_get_pull_request_reviews
- Compare review's commit_id to PR's latest commit
- Outdated reviews should not count toward approval
- Use with git_get_pull_request_status to get latest commit
```

**Automation Workflow (18 lines)**
```
MERGE READINESS AUTOMATION:
1. Call github_get_pull_request_reviews({owner, repo, pull_number})
2. For each review:
   - Check state (must not include CHANGES_REQUESTED for merge)
   - Check commit_id (discard if not latest commit)
   - Check submitted_at (track review recency)
3. Count states:
   - APPROVED count >= branch_protection_required_approvals? ✓ ready
   - Any CHANGES_REQUESTED? ✗ blocked
   - All reviews are current commits? ✓ approved
4. Return merge eligibility decision
```

**Rate Limiting (8 lines)**
```
RATE LIMITING:
- Authenticated requests: 5,000 per hour
- Unauthenticated: 60 per hour
- Check X-RateLimit-Remaining header in response
- Batch review requests to avoid rate limit exhaustion
```

**Advanced Errors (12 lines)**
```
ERROR SCENARIOS:
1. 403 Forbidden: Token lacks repo access
   Fix: Add 'repo' scope to GITHUB_TOKEN

2. 422 Unprocessable: Invalid pagination
   Fix: Ensure page >= 1, per_page <= 100

3. Rate limit exceeded
   Fix: Wait for reset time in X-RateLimit-Reset header
```

**Line Total**: 6 + 20 + 10 + 18 + 8 + 12 = 74 lines (add connectors to reach 85-100)

### Step 4: Update generate_prompts() Routing

Since there's no scenario enum in GetPullRequestReviewsPromptArgs, modify the implementation:

**Before**:
```rust
impl PromptProvider for GetPullRequestReviewsPrompts {
    type PromptArgs = GetPullRequestReviewsPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        vec![
            // Single comprehensive prompt
        ]
    }
```

**After**:
```rust
impl PromptProvider for GetPullRequestReviewsPrompts {
    type PromptArgs = GetPullRequestReviewsPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        // Return basic scenario by default
        Self::prompt_basic()
    }

    fn generate_alternative_prompts(_args: &Self::PromptArgs) -> Vec<Vec<PromptMessage>> {
        vec![
            Self::prompt_basic(),
            Self::prompt_review_analysis(),
        ]
    }
```

Or if alternatives aren't supported, simply have generate_prompts return prompt_basic and add a comment about prompt_review_analysis for completeness.

### Step 5: Validation

After trimming:
1. **Line count check**: `wc -l prompts.rs` should return 170-220 lines
2. **Function count**: `grep "^fn prompt_" prompts.rs` should return exactly 2 functions
3. **No comprehensive**: `grep -i "comprehensive" prompts.rs` should return 0 results
4. **No decorative headers**: `grep "═══" prompts.rs` should return 0 results
5. **Redundancy check**: `grep -i "best practice" prompts.rs` should appear 0 or 1 times
6. **Readability**: Reading both scenarios should take 2-3 minutes total and teach all parameters

---

## Code Pattern Reference

### Before (Current - 107 lines)
```rust
impl PromptProvider for GetPullRequestReviewsPrompts {
    type PromptArgs = GetPullRequestReviewsPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        vec![
            PromptMessage {
                role: PromptMessageRole::User,
                content: PromptMessageContent::text(
                    "How do I use github_get_pull_request_reviews to retrieve PR reviews?",
                ),
            },
            PromptMessage {
                role: PromptMessageRole::Assistant,
                content: PromptMessageContent::text(
                    "The github_get_pull_request_reviews tool retrieves all reviews...
                     [MASSIVE 80+ LINE BLOCK WITH 9 SECTIONS]
                     - Use pagination for PRs with many reviews",
                ),
            },
        ]
    }
    // ... rest unchanged
}
```

### After (Target - 170-220 lines)
```rust
impl PromptProvider for GetPullRequestReviewsPrompts {
    type PromptArgs = GetPullRequestReviewsPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        Self::prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

// Scenario 1: Basic Usage
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage { /* user message */ },
        PromptMessage { /* assistant response: ~90 lines */ },
    ]
}

// Scenario 2: Advanced Analysis
fn prompt_review_analysis() -> Vec<PromptMessage> {
    vec![
        PromptMessage { /* user message */ },
        PromptMessage { /* assistant response: ~90 lines */ },
    ]
}
```

---

## Success Criteria

Your trimming is complete when ALL of these are true:

- ✓ File is 170-220 lines total
- ✓ Exactly TWO scenario functions: `prompt_basic()` and `prompt_review_analysis()`
- ✓ No "comprehensive" scenario
- ✓ No decorative headers (═══, ###, etc. used for decoration only)
- ✓ Each parameter explained once in basic scenario
- ✓ Response structure shown once (in basic)
- ✓ "approval" / "merge readiness" mentioned in both scenarios but in different contexts (basic: checking, advanced: automating)
- ✓ COMMON WORKFLOWS reduced from separate section to embedded in scenario examples
- ✓ BEST PRACTICES reduced from 13 bullets to 3-4 essential ones in advanced scenario
- ✓ Both scenarios together fit 170-220 lines
- ✓ generate_prompts() calls prompt_basic()
- ✓ File compiles and has no clippy warnings

---

## Validation Checklist

Run these commands after completing the task:

```bash
# Check line count
wc -l /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_pull_request_reviews/prompts.rs
# Should output: 170-220

# Check function definitions
grep "^fn prompt_" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_pull_request_reviews/prompts.rs
# Should output exactly 2 lines

# Verify no comprehensive
grep -i "comprehensive" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_pull_request_reviews/prompts.rs
# Should output 0 matches

# Verify no decorative headers
grep "═══" /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/get_pull_request_reviews/prompts.rs
# Should output 0 matches

# Verify compilation
cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema
cargo check --lib
# Should pass with no errors
```

---

## Definition of Done

This task is complete when:

1. The prompts.rs file has been modified to contain exactly 2 prompt scenarios
2. Total line count is between 170-220 lines
3. The file compiles without errors
4. No use-case sections or comprehensive scenarios remain
5. Both scenarios are functionally independent and teach different aspects of the tool
6. prompt_basic() covers parameters, response, authentication, basic errors
7. prompt_review_analysis() covers state filtering, freshness checks, automation, rate limits
8. All decorative formatting is removed
9. No clippy warnings are introduced
10. The tool can be demonstrated in 3-5 minutes using just these two scenarios
