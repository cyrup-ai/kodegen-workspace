# TASK 090: Trim github_create_repository

**Tool**: `github_create_repository`
**Complexity**: 3 (Medium)
**Current size**: 669 lines (5 scenarios + comprehensive)
**Target size**: 320 lines (3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/create_repository/prompts.rs`

---

## Reference

See **PRECURSOR_03_git_branch_create.md** for Complexity 3 template structure and philosophy.

---

## Context

The `github_create_repository` tool creates GitHub repositories with extensive configuration options:

**Key Parameters**:
- **Required**: `name` (repository name string)
- **Optional**: `description`, `private`, `auto_init`, `gitignore_template`, `license_template`, `organization`, `team_id`
- **Optional**: `has_issues`, `has_wiki`, `has_projects`, `allow_squash_merge`, `allow_merge_commit`, `allow_rebase_merge`, `delete_branch_on_merge`

The tool has legitimate complexity with multiple parameter interactions but currently includes significant redundancy and a massive comprehensive scenario that merely duplicates focused scenarios.

**Current file structure**:
```
Lines 1-40:      Module header, imports, struct CreateRepositoryPrompts
Lines 41-42:     impl PromptProvider trait
Lines 43-98:     prompt_basic() function (56 lines)
Lines 101-250:   prompt_options() function (149 lines)
Lines 253-439:   prompt_organization() function (186 lines)
Lines 442-632:   prompt_workflows() function (190 lines)
Lines 635-1026:  prompt_comprehensive() function (390+ lines) ← MUST DELETE
```

---

## Current Scenario Analysis

**Five scenarios, 669 lines total:**

1. **`prompt_basic()`** (56 lines) ← KEEP & TRIM
   - Simple repo creation, private repo, public repo examples
   - Required/optional parameter documentation
   - Response structure (success, owner, name, full_name, html_url, clone_url, message)
   - Naming rules and common errors (422, 401, 400)

2. **`prompt_options()`** (149 lines) ← KEEP & TRIM
   - With README, gitignore templates, license templates
   - Full configuration example with all options
   - Disable features example
   - Initialization, feature, and merge options documentation

3. **`prompt_organization()`** (186 lines) ← KEEP & TRIM
   - Basic org repo, private org repo, org repo with team
   - Full org repo setup with configuration
   - Permissions, team assignment, personal vs org comparison
   - Common org workflows and error handling

4. **`prompt_workflows()`** (190 lines) ← CONSOLIDATE INTO OTHERS
   - New project from scratch workflow
   - Push existing local project workflow
   - Create from template workflow
   - Organization team project workflow
   - Monorepo and fork examples
   - Workflow decision guide and best practices

5. **`prompt_comprehensive()`** (390+ lines) ← DELETE ENTIRELY
   - Pure duplication with decorative headers (=== lines)
   - Exhaustive reference duplicating all 4 focused scenarios
   - No new information beyond focused scenarios

---

## Trimming Instructions

### KEEP & TRIM: `prompt_basic()` (Target: ~110 lines)

**Current**: 56 lines covering basic repository creation

**Keep**:
- Basic creation examples (25 lines):
  - Simple repo: `github_create_repository({"name": "my-project", "description": "..."})`
  - Private repo: Add `"private": true`
  - Public repo: Explicit `"private": false`
- Required and optional parameters (20 lines):
  - name: Repository name (alphanumeric, hyphens, underscores)
  - description: Short description
  - private: true/false (default false)
- Repository naming rules (15 lines):
  - Alphanumeric characters, hyphens, underscores only
  - Cannot start with hyphen or underscore
  - Must be unique within account/organization
  - Use lowercase, descriptive names
- Response structure (15 lines):
  - success, owner, name, full_name, html_url, clone_url, message
- Common errors (15 lines):
  - 422: Name already exists
  - 401: Authentication failed
  - 400: Invalid name format
- After creation workflow (20 lines):
  - Clone: `git clone https://github.com/username/my-project.git`
  - Or push existing: git remote add origin + git push

**Total target: ~110 lines**

### KEEP & TRIM: `prompt_options()` (Target: ~110 lines)

**Current**: 149 lines covering repository configuration

**Keep**:
- Initialization examples (30 lines):
  - With README: `"auto_init": true`
  - With gitignore: `"gitignore_template": "Rust"` (requires auto_init)
  - With license: `"license_template": "mit"` (requires auto_init)
  - Full configuration example
- Gitignore templates list (10 lines):
  - Common: Rust, Python, Node, Go, Java, C++, Ruby
- License templates list (10 lines):
  - mit (permissive), apache-2.0, gpl-3.0, bsd-3-clause
- Feature configuration (20 lines):
  - has_issues, has_wiki, has_projects (with defaults)
  - When to use each
- Merge strategy options (20 lines):
  - allow_squash_merge, allow_merge_commit, allow_rebase_merge
  - delete_branch_on_merge
  - Decision guidance
- One complete workflow example (10 lines):
  - Create with options example
- Best practices (10 lines):
  - Use auto_init when starting fresh
  - Choose gitignore for your language
  - Add license for open source
  - Match team merge strategy preference

**Total target: ~110 lines**

### KEEP & TRIM: `prompt_organization()` (Target: ~110 lines)

**Current**: 186 lines covering organization-specific creation

**Keep**:
- Organization repo examples (30 lines):
  - Basic org repo: Add `"organization": "my-org"`
  - Private org repo with team: Add `"team_id": 12345`
  - Full org setup example
- Organization parameters (10 lines):
  - organization: Target org name
  - team_id: Numeric team identifier
- Permissions required (15 lines):
  - Organization membership
  - Repo creation permissions
  - Team admin for assignment
- Team assignment (10 lines):
  - Numeric identifier from GitHub
  - Grants team access
  - Can assign multiple teams after
- Personal vs Organization (15 lines):
  - Personal: Under account, you control, limited quota
  - Organization: Under org namespace, org controls, org quota, team permissions
- One complete workflow (20 lines):
  - Create org repo → clone → set up teams
- Error handling (10 lines):
  - 404: Organization not found
  - 403: No permission to create
  - 422: Name conflict

**Total target: ~110 lines**

### DELETE ENTIRELY: `prompt_workflows()` (190 lines)

Consolidate workflow examples into appropriate focused scenarios:
- "New project from scratch" → Integrate into basic scenario
- "Push existing local project" → Integrate into basic scenario
- "Organization team project" → Integrate into organization scenario
- Delete: "Fork and customize", "Monorepo setup", "Create from template" (not core to this tool)
- Consolidate: Workflow decision guide principles → integrate into basic scenario

### DELETE ENTIRELY: `prompt_comprehensive()` (390+ lines)

This is 100% duplication with decorative headers (lines of "=" characters).

Contains only:
- Basic usage (already in basic scenario)
- All parameters (already documented in focused scenarios)
- All workflows (being consolidated into focused scenarios)
- All best practices (already distributed across scenarios)
- Error handling (already in each scenario)

**Delete all content.**

---

## Update Scenario Routing

**In `generate_prompts()` match statement**, change from:

```rust
match args.scenario.as_deref() {
    Some("basic") => prompt_basic(),
    Some("options") => prompt_options(),
    Some("organization") => prompt_organization(),
    Some("workflows") => prompt_workflows(),
    _ => prompt_comprehensive(),
}
```

**To**:

```rust
match args.scenario.as_deref() {
    Some("options") => prompt_options(),
    Some("organization") => prompt_organization(),
    _ => prompt_basic(),
}
```

**In `prompt_arguments()` description**, change from:

```rust
description: Some("Scenario to show (basic, options, organization, workflows)".to_string()),
```

**To**:

```rust
description: Some("Scenario to show (basic, options, organization)".to_string()),
```

---

## Exact Implementation Steps

1. **Delete `prompt_workflows()` function entirely** (lines 442-632 in current file)
2. **Delete `prompt_comprehensive()` function entirely** (lines 635-1026+ in current file)
3. **Edit `prompt_basic()` function**:
   - Keep all current content
   - Add consolidated workflow examples from prompt_workflows (3-5 lines):
     - "New project from scratch" example
     - "Push existing local project" example
   - Total after edit: ~110 lines
4. **Edit `prompt_options()` function**:
   - Trim verbose template descriptions
   - Condense redundant explanations
   - Remove excessive examples of the same pattern
   - Keep essential configuration guidance
   - Total after edit: ~110 lines
5. **Edit `prompt_organization()` function**:
   - Add one workflow example from prompt_workflows (organization team project setup)
   - Remove any redundant feature/merge strategy discussion (refer to options scenario)
   - Total after edit: ~110 lines
6. **Update `generate_prompts()` routing** as shown above
7. **Update `prompt_arguments()` description** as shown above

---

## Success Criteria

- ✓ **Total lines**: 310-340 lines (measured with wc -l)
- ✓ **Scenario functions**: Exactly 3 (prompt_basic, prompt_options, prompt_organization)
- ✓ **No workflows scenario**: grep "fn prompt_workflows" returns 0 matches
- ✓ **No comprehensive scenario**: grep "fn prompt_comprehensive" returns 0 matches
- ✓ **No decorative headers**: grep "^=====" returns 0 matches
- ✓ **Each scenario**: 100-120 lines
- ✓ **Workflows integrated**: Each scenario contains 1-2 practical examples
- ✓ **Parameters documented**: Brief and focused, not exhaustive
- ✓ **Routing statements**: Only 3 Some("...") patterns in match, basic as default
- ✓ **Prompt arguments**: Description updated to list only 3 scenarios

---

## Validation

After completing edits, verify:

```bash
# Total line count
wc -l packages/kodegen-mcp-schema/src/github/create_repository/prompts.rs
# Expected: 310-340 lines

# Scenario function count
grep "^fn prompt_" packages/kodegen-mcp-schema/src/github/create_repository/prompts.rs | wc -l
# Expected: 3

# Verify deleted scenarios
grep "fn prompt_workflows\|fn prompt_comprehensive" packages/kodegen-mcp-schema/src/github/create_repository/prompts.rs
# Expected: 0 matches

# Verify routing updated
grep -A 5 "fn generate_prompts" packages/kodegen-mcp-schema/src/github/create_repository/prompts.rs
# Expected: 3 Some("...") patterns, default returns prompt_basic()

# Verify no decorative headers
grep "^[=]\\{10,\\}" packages/kodegen-mcp-schema/src/github/create_repository/prompts.rs
# Expected: 0 matches

# Verify prompt arguments updated
grep "Scenario to show" packages/kodegen-mcp-schema/src/github/create_repository/prompts.rs
# Expected: mentions only (basic, options, organization)
```

---

## Alignment with Complexity 3 Standard

Per PRECURSOR_03_git_branch_create.md:
- ✓ **2-3 focused scenarios**: This will have exactly 3
- ✓ **Distinct use cases**: Basic creation, configuration options, organization usage
- ✓ **Brief workflow examples**: 1-2 per scenario, not exhaustive
- ✓ **Brief parameter documentation**: Cross-reference tool trait, not exhaustive lists
- ✓ **Total 250-400 lines**: Target 320 lines (52% reduction from 669)
- ✓ **5-minute comprehension**: Achieved by removing all duplication and decorative headers

**Result**: `github_create_repository` brought into full alignment with Complexity 3 standard for consistent, maintainable prompt libraries across all medium-complexity tools.
