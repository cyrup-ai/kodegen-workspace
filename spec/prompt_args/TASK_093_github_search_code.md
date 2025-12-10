# TASK 093: Trim github_search_code

**Tool**: `github_search_code`
**Complexity**: 3 (Medium)
**Current size**: 896 lines (5 scenarios)
**Target size**: 320 lines (3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/search_code/prompts.rs`

---

## Reference

See **PRECURSOR_03_git_branch_create.md** for Complexity 3 template.

---

## Current State Analysis

### Overview

The github_search_code tool searches code across GitHub repositories with parameters:
- `query` (required): Search query with GitHub code search syntax
- `sort` (optional): "indexed" sorts by last indexed time
- `order` (optional): "asc" or "desc" (default: "desc")
- `page` (optional): Page number for pagination (default: 1)
- `per_page` (optional): Results per page, max 100 (default: 30)
- `enrich_stars` (optional): Include repository star counts (default: false)

### Current Scenario Analysis

**Current scenarios** (896 lines total):

1. `prompt_basic()` - 81 lines (lines 43-123)
   - Covers: Basic search operations, parameters, response format, authentication, rate limiting
   - Quality: Good foundation but lacks depth on auth and rate limits
   - Action: KEEP and TRIM TO ~100 lines (ADD auth/rate limit details)

2. `prompt_syntax()` - 125 lines (lines 127-252)
   - Covers: Search qualifiers (repo, user, org, language, path, filename, extension, size)
   - Boolean operators: AND, OR, NOT, exact phrases
   - Combining qualifiers with practical examples
   - Quality: Well-balanced, good reference for syntax
   - Action: KEEP and TRIM TO ~110 lines (consolidate similar examples)

3. `prompt_patterns()` - 179 lines (lines 255-434)
   - Covers: 20 common search patterns across learning, API research, testing, architecture, security
   - Problem: Redundant with basic scenarios
   - These patterns are extensions of syntax qualifiers already documented
   - Action: DELETE ENTIRELY (179 lines removed, patterns integrated into workflows)

4. `prompt_workflows()` - 199 lines (lines 437-636)
   - Covers: 6 workflows (learning library, discovering patterns, finding similar code, security audit, API migration, performance research)
   - Quality: Comprehensive but too long, contains overlapping workflows
   - Action: KEEP and TRIM TO ~110 lines (reduce 6 workflows to 3 most essential)

5. `prompt_comprehensive()` - 257 lines (lines 639-896)
   - Pure duplication of basic + syntax + patterns + workflows reorganized
   - Contains all content from 4 other scenarios without new information
   - Action: DELETE ENTIRELY (257 lines removed)

---

## Trimming Instructions

### STEP 1: Keep & Trim `prompt_basic()` (Target: ~100 lines)

**Current**: 81 lines covering basic repository searching

**What to Keep:**
- User question (2 lines)
- Basic searching overview (8 lines):
  - Simple keyword search example
  - Search in repository example
  - Search by language example
  - Search in path example
- Parameters documentation (18 lines):
  - query, sort, order, page, per_page, enrich_stars with brief descriptions
- Response structure (12 lines):
  - JSON response example with field descriptions
- Authentication section (6 lines):
  - GITHUB_TOKEN requirement
  - Token scope requirements
- Rate limiting section (12 lines):
  - 30 requests per minute limit (strict)
  - Comparison to general GitHub API
  - Early warning about rate limit strictness
- Common patterns (20 lines):
  - 4-5 real-world examples (find function, API usage, config, organization search)
- Best practices (14 lines):
  - Start with specific queries
  - Use language filters
  - Add repo or org qualifiers
  - Set per_page to 100 for batch
  - Check total_count before paginating
  - Use enrich_stars for popular repos
  - Respect rate limits

**What to Remove/Consolidate:**
- Lines 54-60: Reduce basic examples from 4 to 2 (keep HTTPS and SSH only)
- Lines 67-73: Parameters section - condense descriptions
- Lines 100-119: Common patterns - keep 4 examples only, remove longest explanations

**Implementation Approach:**
- Compress "BASIC SEARCHING" section: 4 examples reduced to 2 inline examples
- Keep PARAMETERS section intact (essential reference)
- Expand RATE LIMITING from brief mention to full section (add urgency)
- Expand AUTHENTICATION with token scope requirements
- Keep COMMON PATTERNS but reduce examples from 4 to 4 with shorter explanations
- Keep BEST PRACTICES condensed list

**Target output**: 95-105 lines

---

### STEP 2: Keep & Trim `prompt_syntax()` (Target: ~110 lines)

**Current**: 125 lines covering search qualifiers and syntax

**What to Keep:**
- User question (2 lines)
- SEARCH QUALIFIERS section (70 lines):
  - Repository qualifiers (repo, user, org) - 12 lines with examples
  - Language qualifiers (language) - 8 lines with examples
  - Path qualifiers (path, filename, extension) - 12 lines with examples
  - Size qualifiers (size:>n, size:<n, size:n..m) - 10 lines with examples
  - Boolean operators (AND, OR, NOT, exact phrases) - 15 lines with examples
  - Combining qualifiers section - 13 lines with 3 examples
- PRACTICAL EXAMPLES section (15 lines):
  - 3-4 real-world searches combining qualifiers
- SPECIAL CHARACTERS section (3 lines)
- SYNTAX TIPS section (10 lines)
- LIMITATIONS section (8 lines):
  - File size limit (384 KB)
  - Default branch only
  - Rate limit (30 requests/minute)
  - Maximum results (1000)
  - Complex regex not supported

**What to Remove/Consolidate:**
- Lines 189-210: "Combining Qualifiers" - reduce from 5 examples to 3 examples
- Lines 232-241: "SPECIAL CHARACTERS" and "SYNTAX TIPS" - keep but condense (reduce by 25%)
- Lines 243-248: "LIMITATIONS" - keep all but condense descriptions

**Implementation Approach:**
- Keep all major qualifier categories (essential reference material)
- Reduce combining qualifiers examples from 5 to 3 (most representative)
- Condense special characters section (4 lines instead of 6)
- Consolidate syntax tips into 8 lines instead of 11
- Keep limitations section (important for understanding tool behavior)
- This scenario should be a focused reference card for search syntax

**Target output**: 105-115 lines

---

### STEP 3: Keep & Trim `prompt_workflows()` (Target: ~110 lines)

**Current**: 199 lines covering 6 research workflows

**Which Workflows to Keep:**
1. WORKFLOW 1: LEARNING A NEW LIBRARY (lines 449-480) - 31 lines ← KEEP (essential for most agents)
2. WORKFLOW 2: DISCOVERING PATTERNS (lines 481-502) - 21 lines ← KEEP (complements syntax scenario)
3. WORKFLOW 3: FINDING SIMILAR CODE (lines 504-524) - 20 lines ← DELETE (redundant with basic + syntax)
4. WORKFLOW 4: SECURITY AUDIT (lines 526-546) - 20 lines ← KEEP (important security use case)
5. WORKFLOW 5: API MIGRATION (lines 548-570) - 22 lines ← DELETE (too specialized/niche)
6. WORKFLOW 6: PERFORMANCE RESEARCH (lines 572-632) - 60 lines ← DELETE (can be learned from other workflows)

**What to Keep:**
- User question (2 lines)
- WORKFLOW 1: LEARNING A NEW LIBRARY - KEEP ENTIRE (31 lines):
  - Step 1: Find basic usage examples (4 lines)
  - Step 2: Find specific API usage (4 lines)
  - Step 3: Study production code (4 lines)
  - Step 4: Find error patterns (4 lines)
  - This is the most common workflow for agents

- WORKFLOW 2: DISCOVERING PATTERNS - KEEP ENTIRE (21 lines):
  - Step 1: Broad pattern search (3 lines)
  - Step 2: Narrow to specific runtime (3 lines)
  - Step 3: Find configuration patterns (3 lines)
  - This teaches the iterative refinement approach

- WORKFLOW 4: SECURITY AUDIT - KEEP ENTIRE (20 lines):
  - Step 1: Find authentication code (3 lines)
  - Step 2: Find token usage (3 lines)
  - Step 3: Find SQL queries (3 lines)
  - This is a critical workflow for security-focused agents

- ADVANCED TECHNIQUES section (lines 594-632) - CONDENSE TO 7 lines:
  - Keep only "Incremental refinement" and "Popularity-based learning" (5 lines)
  - Remove "Cross-language learning" and "Historical research" (too specialized)

- BEST PRACTICES section (lines 615-632) - CONDENSE TO 8 lines:
  - Start broad, narrow with qualifiers
  - Use enrich_stars for quality signals
  - Respect rate limits (30/minute)
  - Save queries for common searches
  - Review code context on GitHub
  - Verify pattern applicability

- RATE LIMIT MANAGEMENT section (lines 626-632) - MOVE TO BASIC SCENARIO

**What to Delete:**
- Lines 504-524: WORKFLOW 3 (Finding Similar Code) - 20 lines
- Lines 548-570: WORKFLOW 5 (API Migration) - 22 lines
- Lines 572-632: WORKFLOW 6 (Performance Research) - 60 lines
- Lines 600-610: "Cross-language learning" advanced technique
- Lines 612-613: "Historical research" advanced technique

**Implementation Approach:**
- Keep 3 core workflows (learning, discovering, security)
- Each workflow flows through 3-4 steps with concrete examples
- Consolidate advanced techniques to 2 key techniques (5 lines)
- Consolidate best practices to 6 key bullets (8 lines)
- Result: focused set of real-world workflows agents will use

**Target output**: 105-115 lines

---

### STEP 4: Delete `prompt_patterns()` ENTIRELY

**Reason for Deletion:**
This scenario (179 lines, lines 255-434) teaches the same search patterns in a different format than what's already in prompt_syntax() and prompt_workflows().

**Content Analysis:**
- Lines 265-433: 20 numbered patterns across different categories
- Each pattern is essentially: "Here's a search goal + the query to use"
- All of this is synthesizable from:
  - prompt_syntax(): teaches what qualifiers exist
  - prompt_workflows(): teaches how to combine qualifiers in workflows

**Why It's Redundant:**
- Agents learning syntax can create their own patterns
- Agents learning workflows see multiple patterns in context
- The 20 patterns are not teaching anything new beyond syntax + combining qualifiers
- Having both "patterns" and "workflows" duplicates the teaching method

**Action:**
Delete entire `prompt_patterns()` function and all 179 lines.

---

### STEP 5: Delete `prompt_comprehensive()` ENTIRELY

**Reason for Deletion:**
This scenario (257 lines, lines 639-896) is pure duplication of the other 4 scenarios reorganized.

**Content Breakdown:**
- Lines 650-684: "OVERVIEW" section (duplicates prompt_basic overview)
- Lines 685-750: "BASIC USAGE" and "PARAMETERS" (duplicates prompt_basic)
- Lines 752-799: "SEARCH SYNTAX" (duplicates prompt_syntax)
- Lines 801-845: "COMMON USE CASES" (duplicates patterns from prompt_patterns)
- Lines 847-900: "AUTHENTICATION" through "LIMITATIONS" (duplicates all sections)

**Pattern:**
This follows the classic problematic "comprehensive scenario" pattern identified in PRECURSOR_03. It maintains all information from specialized scenarios in one place, making the file:
- Harder to maintain (changes need updating in 5 places)
- Harder to use (agents must read 900 lines instead of 3 focused scenarios)
- Violates Complexity 3 standard (no comprehensive scenario)

**Action:**
Delete entire `prompt_comprehensive()` function and all 257 lines.

---

### STEP 6: Update Scenario Routing

**Current routing** (lines 17-24):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("syntax") => prompt_syntax(),
        Some("patterns") => prompt_patterns(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**New routing** (keep first 3, delete patterns and comprehensive, change default):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("syntax") => prompt_syntax(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_basic(),
    }
}
```

**Changes:**
- Remove `Some("patterns") => prompt_patterns(),` arm
- Change default case from `prompt_comprehensive()` to `prompt_basic()`
- `prompt_basic()` becomes the default when no scenario specified
- Keep `syntax` and `workflows` as alternatives

---

### STEP 7: Update `prompt_arguments()` Documentation

**Current** (lines 26-35):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, syntax, patterns, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Change to**:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show: basic (default), syntax, workflows".to_string()),
            required: Some(false),
        }
    ]
}
```

**Changes:**
- Remove "patterns" from description (scenario deleted)
- Change "basic, syntax, patterns, workflows" to "basic (default), syntax, workflows"
- Clarify that "basic" is the default scenario
- Update format from list to include "(default)" marker

---

## Exact Line Ranges for Trimming

### Phase 1: Identify Current Scenario Boundaries

```
Lines 1-40:     Imports and struct definitions (KEEP ALL)
Lines 42-124:   prompt_basic() [81 lines] → TRIM TO ~100
Lines 127-252:  prompt_syntax() [125 lines] → TRIM TO ~110
Lines 255-434:  prompt_patterns() [179 lines] → DELETE ENTIRE
Lines 437-636:  prompt_workflows() [199 lines] → TRIM TO ~110
Lines 639-896:  prompt_comprehensive() [257 lines] → DELETE ENTIRE
```

### Phase 2: Trim prompt_basic()

**Within lines 43-123:**
- Keep lines 43-50 (user question and opening)
- Lines 52-108: Basic examples - REDUCE from 4 examples to 2 examples (remove lines for clone in specific repo, search by language)
- Lines 109-133: PARAMETERS - condense descriptions but keep all 6 parameters
- Lines 134-150: RESPONSE STRUCTURE - consolidate JSON example
- Lines 151-180: Add AUTHENTICATION + RATE LIMITING (expand from 5 to 17 lines)
- Lines 94-119: COMMON PATTERNS - keep 4 examples but shorten descriptions
- Lines 113-120: BEST PRACTICES - keep list but condense

**Target**: Result should be 95-105 lines

### Phase 3: Trim prompt_syntax()

**Within lines 127-252:**
- Keep all content but consolidate:
- Lines 139-189: REPOSITORY QUALIFIERS + LANGUAGE QUALIFIERS + PATH QUALIFIERS - keep intact (45 lines)
- Lines 175-210: SIZE QUALIFIERS + BOOLEAN OPERATORS - keep intact (35 lines)
- Lines 189-210: COMBINING QUALIFIERS - reduce from 5 examples to 3 examples (save 6 lines)
- Lines 211-231: PRACTICAL EXAMPLES - keep 3 examples, shorten to 12 lines
- Lines 232-241: SPECIAL CHARACTERS - condense from 6 to 4 lines
- Lines 236-241: SYNTAX TIPS - reduce from 11 to 8 lines
- Lines 243-248: LIMITATIONS - condense from 6 to 5 lines

**Target**: Result should be 105-115 lines

### Phase 4: Delete prompt_patterns() Entirely

**Action**: Remove lines 255-434 completely (179 lines)

This includes:
- Function definition
- User question
- 20 numbered search patterns
- All section headers
- All examples and explanations

### Phase 5: Trim prompt_workflows()

**Within lines 437-636:**
- Keep lines 437-445 (function header and user question)
- KEEP lines 449-480: WORKFLOW 1 (Learning a New Library) [31 lines]
- KEEP lines 481-502: WORKFLOW 2 (Discovering Patterns) [21 lines]
- DELETE lines 504-524: WORKFLOW 3 (Finding Similar Code) [20 lines]
- KEEP lines 526-546: WORKFLOW 4 (Security Audit) [20 lines]
- DELETE lines 548-570: WORKFLOW 5 (API Migration) [22 lines]
- DELETE lines 572-632: WORKFLOW 6 (Performance Research) [60 lines]
- CONDENSE lines 594-610: ADVANCED TECHNIQUES (from 17 lines to 7 lines)
  - Keep only "Incremental refinement" and "Popularity-based learning"
  - Remove "Cross-language learning" and "Historical research"
- CONDENSE lines 615-632: BEST PRACTICES (from 17 lines to 8 lines)
  - Keep: Start broad, enrich_stars, rate limits, save queries, review context, verify applicability

**Kept Content:**
- 3 workflows with 4 steps each
- 3 best practices bullets
- 2 advanced techniques bullets
- 1 rate limit management bullet

**Target**: Result should be 105-115 lines

### Phase 6: Delete prompt_comprehensive() Entirely

**Action**: Remove lines 639-896 completely (257 lines)

This includes:
- Function definition
- User question
- All 8 major sections (OVERVIEW, BASIC USAGE, PARAMETERS, SEARCH SYNTAX, RESPONSE STRUCTURE, COMMON USE CASES, AUTHENTICATION, RATE LIMITING, PAGINATION, ERROR HANDLING, BEST PRACTICES, LIMITATIONS, EXAMPLES BY SCENARIO)
- All subsections and examples

---

## Success Criteria

After trimming, file MUST satisfy ALL criteria:

**Line Count:**
- ✓ Total file: 280-360 lines (down from 896)
- ✓ Header/imports/impl/match: ~40 lines
- ✓ Three scenarios: ~100-120 lines each
- ✓ Total code: 320-370 lines

**Scenario Functions:**
- ✓ `prompt_basic()`: ~100 lines covering basic searching, parameters, auth, rate limits, patterns, best practices
- ✓ `prompt_syntax()`: ~110 lines covering all search qualifiers, boolean operators, combining qualifiers, practical examples, limitations
- ✓ `prompt_workflows()`: ~110 lines covering 3 workflows (learning, discovering, security) with advanced techniques and best practices
- ✓ No `prompt_patterns()` function exists (grep returns 0 results)
- ✓ No `prompt_comprehensive()` function exists (grep returns 0 results)

**Routing:**
- ✓ Match statement has exactly 3 arms: syntax, workflows, basic (default)
- ✓ Default case (`_`) returns `prompt_basic()`
- ✓ No references to deleted scenarios
- ✓ `prompt_arguments()` describes only "basic (default), syntax, workflows"

**Content Quality:**
- ✓ All 4 required tool parameters documented (query, sort, order, page, per_page, enrich_stars)
- ✓ Authentication section includes GITHUB_TOKEN requirement
- ✓ Rate limiting section prominently mentions 30 requests/minute limit
- ✓ Each scenario covers distinct use case/aspect:
  - basic: fundamental operations, parameters, auth, rate limits
  - syntax: all available search qualifiers and operators
  - workflows: real-world multi-step research processes
- ✓ One workflow example per workflow (not multiple)
- ✓ Practical examples in each scenario (real repositories, real queries)
- ✓ No false claims about tool features

**No Redundancy:**
- ✓ No duplicate content between scenarios
- ✓ No comprehensive scenario
- ✓ Each scenario focused on one primary concept
- ✓ grep "comprehensive\|patterns" returns 0 non-comment results

---

## Validation Checklist

```bash
# 1. Line count - should be 280-360
wc -l prompts.rs

# 2. Scenario count - should show exactly 3 functions
grep "^fn prompt_" prompts.rs

# 3. Deleted scenarios - should show 0 results
grep "fn prompt_patterns\|fn prompt_comprehensive" prompts.rs

# 4. Match arms - should show exactly 3 arms
grep -A5 "fn generate_prompts" prompts.rs

# 5. Scenario references - should show 0 results
grep '"patterns"\|"comprehensive"' prompts.rs | grep Some

# 6. Default scenario - should be basic
grep "_ =>" prompts.rs | grep "prompt_basic"

# 7. Parameters documentation - should all be present
grep -i "github_token\|rate limit\|30 requests" prompts.rs

# 8. Syntax checker
cargo check -p kodegen-mcp-schema
```

---

## Scope & Definition of Done

**This task ONLY edits one file:**
- `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/search_code/prompts.rs`

**This task DOES NOT:**
- Modify tests
- Change tool implementation or API
- Update documentation outside this file
- Modify prompt_args.rs or schema.rs
- Create new test cases

**File is complete when:**
1. Trimming instructions fully applied
2. All success criteria met
3. No orphaned scenario references
4. Match statement correctly updated
5. File compiles without errors (cargo check succeeds)
6. Three scenarios are present and correct
7. File is 280-360 lines total

---

## Trimming Workflow

Execute these steps in order:

1. Read entire current prompts.rs
2. Copy/backup the file
3. Update prompt_arguments() description (lines 26-35)
4. Trim prompt_basic() from 81 to 100 lines
5. Trim prompt_syntax() from 125 to 110 lines
6. Trim prompt_workflows() from 199 to 110 lines
7. Delete prompt_patterns() function entirely (lines 255-434)
8. Delete prompt_comprehensive() function entirely (lines 639-896)
9. Update generate_prompts() match statement (lines 17-24)
10. Verify line count: 280-360 total
11. Run validation checklist
12. cargo check -p kodegen-mcp-schema must succeed

---

## Why This Structure

This trimming follows the **Complexity 3 standard** from PRECURSOR_03:

- **3 focused scenarios**, not 5, not 1 comprehensive
- Each scenario covers one key aspect:
  - Basic: Parameters, auth, rate limits, essential operations
  - Syntax: Complete reference for search qualifiers
  - Workflows: Multi-step real-world research processes
- Total file matches Complexity 3 size (280-360 lines)
- No advanced features that aren't in the API
- Maintainable: Changes in one place, not duplicated 5 ways
- Readable: Agents understand tool in 5 minutes
- Actionable: Real examples with concrete parameters
- Focused: Each scenario teaches one concept well
