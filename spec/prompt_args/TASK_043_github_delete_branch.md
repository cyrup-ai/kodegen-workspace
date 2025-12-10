# TASK 043: Trim github_delete_branch

**Tool**: `github_delete_branch`
**Complexity**: 2 (Simple)
**Current size**: 97 lines
**Target size**: 170-220 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/delete_branch/prompts.rs`

---

## Current State Analysis

### File Structure (97 lines total)
The current prompts.rs contains:
- Lines 1-6: Module documentation and imports
  - `use crate::tool::PromptProvider;`
  - `use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};`
  - `use super::prompt_args::DeleteBranchPromptArgs;`

- Lines 7-11: Struct and trait implementation
  - `pub struct DeleteBranchPrompts;`
  - `impl PromptProvider for DeleteBranchPrompts { type PromptArgs = DeleteBranchPromptArgs;`

- Lines 12-19: Single generate_prompts function (no scenario matching)
  - Currently: `fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> { vec![ ... ] }`
  - Always returns the same comprehensive response regardless of scenario

- Lines 20-97: One massive assistant response covering ALL topics
  - Basic usage examples (4 examples)
  - Parameters explanation
  - Authentication requirements
  - Response structure
  - Common workflows (3 scenarios: post-merge, abandoned, automated)
  - Rate limiting
  - Error scenarios
  - Best practices

- Lines 95-97: Empty prompt_arguments function
  - `fn prompt_arguments() -> Vec<PromptArgument> { vec![] }`

### Key Content to Preserve
The current comprehensive response contains valuable information about:
1. Basic deletion workflow (simple 1-2 sentence explanation)
2. Parameter details (owner, repo, branch)
3. Authentication and GITHUB_TOKEN requirements
4. Response structure (success/failure JSON)
5. Common error scenarios (404, 422, 403)
6. Best practices (verify before delete, never delete main/master, use list_branches to confirm)

---

## Step-by-Step Implementation

### Step 1: Modify generate_prompts Function (Lines 18-30)

Replace the current single-response function with scenario matching logic:

**FROM (current lines 18-19):**
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
```

**TO (new implementation):**
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("safety") => prompt_safety(),
        _ => prompt_basic(),
    }
}
```

**Explanation**: The match statement now checks if a scenario is provided. If "safety" is requested, call prompt_safety(). Otherwise default to prompt_basic(). This enables two focused teaching scenarios instead of one overwhelming comprehensive response.

### Step 2: Implement prompt_arguments Function (Lines 31-35)

Replace the empty prompt_arguments with the scenario parameter definition:

**FROM (current lines 95-97):**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![]
}
```

**TO (new implementation):**
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![PromptArgument {
        name: "scenario".to_string(),
        title: None,
        description: Some(
            "Scenario to show (basic, safety)".to_string(),
        ),
        required: Some(false),
    }]
}
```

**Explanation**: This tells users they can request specific scenarios via the `scenario` parameter. When no parameter is provided, prompt_basic() is the default. This follows the exact same pattern as create_branch.

### Step 3: Create prompt_basic Function (Lines 36-115, approximately 80 lines)

This function teaches how to delete branches in straightforward scenarios.

**Structure**:
```rust
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I delete branches from GitHub repositories?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_delete_branch tool removes a branch reference from a GitHub repository via the API.\n\n\
                 BASIC DELETION:\n\
                 1. Delete merged feature branch:\n\
                    github_delete_branch({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"branch_name\": \"feature/new-api\"\n\
                    })\n\n\
                 2. Delete stale branch:\n\
                    github_delete_branch({\n\
                      \"owner\": \"company\",\n\
                      \"repo\": \"backend\",\n\
                      \"branch_name\": \"fix/old-bug\"\n\
                    })\n\n\
                 3. Clean up after PR:\n\
                    github_delete_branch({\n\
                      \"owner\": \"team\",\n\
                      \"repo\": \"frontend\",\n\
                      \"branch_name\": \"feature/login\"\n\
                    })\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - branch_name (required): Exact name of branch to delete\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"branch_name\": \"feature/new-api\",\n\
                   \"message\": \"Branch deleted successfully\"\n\
                 }\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with:\n\
                 - repo scope (for private repositories)\n\
                 - public_repo scope (for public repositories)\n\n\
                 COMMON ERRORS:\n\
                 1. 404 Not Found: Branch doesn't exist\n\
                    Fix: Verify branch name is correct using github_list_branches\n\n\
                 2. 422 Unprocessable: Branch is protected or default\n\
                    Fix: Cannot delete protected branches via API; remove protection in GitHub UI\n\n\
                 3. 403 Forbidden: Token lacks write access\n\
                    Fix: Generate new token with 'repo' scope\n\n\
                 BASIC WORKFLOW:\n\
                 1. Merge pull request on GitHub\n\
                 2. Delete source branch using github_delete_branch\n\
                 3. Keep repository clean and organized\n\n\
                 KEY POINTS:\n\
                 - Branches are remote references; local copies must be deleted separately\n\
                 - Deletion is immediate and cannot be undone from API\n\
                 - Use github_list_branches to verify branch exists first\n\
                 - Document retention policy in CONTRIBUTING.md",
            ),
        },
    ]
}
```

**Line count target**: Approximately 75-85 lines of code

**Content extracted from**: Current comprehensive response, focusing on:
- Basic usage examples (examples 1-3, simplified)
- Parameter descriptions
- Response structure
- Authentication
- Three most common errors with fixes
- Basic workflow summary
- Key points about the operation

### Step 4: Create prompt_safety Function (Lines 116-200, approximately 85 lines)

This function teaches about protected branches, verification, and best practices.

**Structure**:
```rust
fn prompt_safety() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I safely delete branches without breaking the repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Safe branch deletion requires verification and understanding protection rules.\n\n\
                 PROTECTED BRANCHES:\n\
                 Cannot delete these via API:\n\
                 - Default branch (usually main/master)\n\
                 - Branches with protection rules enabled\n\
                 - Branches with active status checks\n\n\
                 Error response (422 Unprocessable):\n\
                 {\n\
                   \"success\": false,\n\
                   \"message\": \"Reference cannot be deleted\"\n\
                 }\n\n\
                 Solution: Remove protection rules in GitHub UI (Settings > Branches > Branch protection rules)\n\n\
                 VERIFICATION BEFORE DELETION:\n\
                 Always verify branch exists and merged before deleting:\n\
                 1. List all branches:\n\
                    github_list_branches({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\"\n\
                    })\n\n\
                 2. Check branch details:\n\
                    github_get_pull_request_status({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"branch\": \"feature/xyz\"\n\
                    })\n\n\
                 3. Only delete if merged:\n\
                    github_delete_branch({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"branch_name\": \"feature/xyz\"\n\
                    })\n\n\
                 BRANCHES NEVER TO DELETE:\n\
                 - main (primary production branch)\n\
                 - master (legacy default branch)\n\
                 - develop (integration branch in Git Flow)\n\
                 - release/* (unless specifically archiving)\n\
                 - Any branch with active pull requests\n\n\
                 RECOVERY AFTER ACCIDENTAL DELETION:\n\
                 - Branch references can be recovered if commits still exist\n\
                 - Use github_create_branch to recreate from same commit SHA\n\
                 - GitHub retains commit history (GitHub Support can recover)\n\
                 - Best protection: Require PR reviews and protection rules\n\n\
                 BEST PRACTICES:\n\
                 1. Always verify with github_list_branches first\n\
                 2. Confirm branch is merged before deletion\n\
                 3. Use branch protection for critical branches\n\
                 4. Document retention policy for release branches\n\
                 5. Delete only feature/hotfix branches after merge\n\
                 6. Require approval before deleting any branch\n\
                 7. Maintain branch naming convention to identify protection needs\n\
                 8. Regular cleanup after sprint/release cycles\n\n\
                 PERMISSION REQUIREMENTS:\n\
                 - GITHUB_TOKEN must have write access to repository\n\
                 - For organizations: ensure token has access to org\n\
                 - For private repos: token needs 'repo' scope\n\
                 - For public repos: token needs 'public_repo' scope\n\n\
                 CLEANUP WORKFLOW:\n\
                 1. Merge pull request (GitHub UI)\n\
                 2. Delete source branch (GitHub PR option, OR)\n\
                 3. Delete with tool:\n\
                    github_delete_branch({\"owner\": \"...\", \"repo\": \"...\", \"branch_name\": \"...\"})\n\
                 4. Verify deletion:\n\
                    github_list_branches({\"owner\": \"...\", \"repo\": \"...\"})\n\
                 5. Keep repository uncluttered",
            ),
        },
    ]
}
```

**Line count target**: Approximately 85-95 lines of code

**Content extracted from**: Current comprehensive response, focusing on:
- Protected branch restrictions (NEW emphasis, critical)
- Verification workflow before deletion
- Branches that should never be deleted
- Recovery procedures
- Best practices expanded
- Permission requirements
- Cleanup workflow

---

## Success Criteria

Verify the completed prompts.rs file meets ALL these requirements:

- **Line count**: Total file should be 170-220 lines (including imports and trait definition)
  - Measure with: `wc -l packages/kodegen-mcp-schema/src/github/delete_branch/prompts.rs`

- **Two scenarios implemented**:
  - `Some("basic")` returns prompt_basic()
  - `Some("safety")` returns prompt_safety()
  - Default (no scenario specified) returns prompt_basic()

- **prompt_arguments() returns scenario parameter**:
  - Defines "scenario" as an optional parameter
  - Description mentions "(basic, safety)"
  - Matches create_branch pattern exactly

- **No decorative headers**:
  - Remove all `===...===` separator lines
  - Remove section headers like "OVERVIEW", "PARAMETERS", "AUTHENTICATION"
  - Content flows naturally with minimal formatting

- **Both scenarios compile**:
  - Run: `cd /Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema && cargo check`
  - Should complete without errors or warnings

- **Scenarios are focused**:
  - prompt_basic(): Teaches straightforward deletion (75-85 lines)
  - prompt_safety(): Teaches protection, verification, best practices (85-95 lines)
  - No overlap or duplication between them

- **Content accuracy preserved**:
  - All parameters correctly described
  - Error codes (404, 422, 403) still documented
  - Authentication requirements still clear
  - Best practices from original preserved

---

## Implementation Pattern Reference

This follows the exact pattern from `create_branch/prompts.rs`:
- Generate_prompts uses match statement on args.scenario
- prompt_arguments returns PromptArgument vec with "scenario" name
- Helper functions (prompt_basic, prompt_safety) return Vec<PromptMessage>
- Each function has User question and Assistant answer PromptMessages
- Default scenario (no match) uses prompt_basic()

---

## No Tests, Benchmarks, or Documentation Changes

- Do NOT modify or create test files
- Do NOT update README or other documentation
- Do NOT add benchmarks
- Only modify: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/delete_branch/prompts.rs`
