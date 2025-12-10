# TASK 095: Trim github_search_repositories

**Tool**: `github_search_repositories`
**Complexity**: 3 (Medium)
**Current size**: 766 lines (5 scenarios)
**Target size**: 320-360 lines (3 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/search_repositories/prompts.rs`

---

## Reference

See **PRECURSOR_03_git_branch_create.md** for Complexity 3 template pattern and trimming methodology.

---

## Current State Analysis

### File Statistics
- **Total lines**: 766 lines
- **Current scenarios**: 5 functions
- **Implementation structure**: Helper functions with PromptProvider trait

### Current Scenario Breakdown
The file contains 5 redundant/overlapping scenarios with the following line distributions:

1. **`prompt_basic()`** (lines 43-133, 91 lines) - Basic search fundamentals
   - Simple keyword search examples
   - Filtering results by language/stars/dates
   - Sorting and pagination
   - Parameter documentation
   - Response field explanation
   - Common use cases and tips

2. **`prompt_syntax()`** (lines 136-254, 119 lines) - Search qualifier reference
   - Language filters (language:rust, language:python, etc.)
   - Popularity filters (stars, forks ranges)
   - Date filters (created, pushed, updated with operators)
   - Owner filters (user, org)
   - Topic filters (topic:web, topic:async, etc.)
   - License filters (mit, apache-2.0, gpl-3.0, etc.)
   - Size filters
   - Status filters (archived, is:public, fork:false, etc.)
   - Activity filters (good-first-issues, help-wanted-issues)
   - Boolean logic explanation (AND, OR, NOT)
   - Date formats and range syntax

3. **`prompt_patterns()`** (lines 257-410, 154 lines) - Search pattern examples ← **CANDIDATE FOR CONSOLIDATION**
   - Pattern 1: Finding libraries (http client example)
   - Pattern 2: Finding examples and tutorials
   - Pattern 3: Finding project templates
   - Pattern 4: Finding active projects
   - Pattern 5: Finding trending new projects
   - Pattern 6: Finding similar projects (comparison)
   - Pattern 7: Finding organization projects
   - Pattern 8: Finding beginner-friendly projects
   - Pattern 9: Finding well-documented projects
   - Pattern 10: Finding small utility libraries
   - Pattern 11: Finding MIT-licensed projects
   - Pattern 12: Finding microservices examples

4. **`prompt_workflows()`** (lines 413-578, 166 lines) - Integrated discovery workflows ← **KEEP & TRIM**
   - Workflow 1: Researching libraries (3 steps)
   - Workflow 2: Finding alternatives (3 steps with comparison points)
   - Workflow 3: Technology exploration (4 steps)
   - Workflow 4: Competitive analysis (3 steps)
   - Workflow 5: Dependency selection (3 steps)
   - Workflow 6: Finding contribution opportunities (3 steps)

5. **`prompt_comprehensive()`** (lines 581-766, 186 lines) - ← **DELETE ENTIRELY**
   - Pure duplication: contains all content from basic + syntax + patterns + workflows
   - Headers: BASIC USAGE, SEARCH QUALIFIERS, RESPONSE STRUCTURE, COMMON PATTERNS, WORKFLOWS, BEST PRACTICES, QUICK REFERENCE

### Redundancy Analysis

The file has **severe redundancy**:
- `prompt_comprehensive()` duplicates ALL other scenarios (186 lines of pure duplication)
- `prompt_basic()` and sections of `prompt_comprehensive()` overlap completely
- `prompt_syntax()` and `prompt_comprehensive()` qualifier sections are identical
- `prompt_patterns()` contains abstract patterns not integrated with actual workflows
- `prompt_workflows()` and `prompt_patterns()` teach similar concepts (patterns vs. workflows)

**Total redundant content**: ~250+ lines (30% of file)

---

## Trimming Strategy

Follow the **PRECURSOR_03 Complexity 3 pattern**:
- Delete comprehensive scenario entirely
- Keep 3 focused scenarios (basic, syntax, workflows)
- Delete patterns scenario (merge pattern examples into workflows)
- Trim each remaining scenario to ~100-110 lines
- Update routing match statement

### Keep & Trim: `prompt_basic()` (Target: ~100 lines)

**Current**: 91 lines covering search fundamentals

**Keep intact** (this scenario is well-structured):
- Simple keyword search examples (5-8 lines)
- Filtering results section with 7 examples (30-35 lines):
  - Search by language (1 example)
  - Search by popularity (stars:>1000) (1 example)
  - Search recently active (pushed:>2024-01-01) (1 example)
  - Sort by stars with sort/order parameters (1 example)
  - Get more results per page (per_page) (1 example)
  - Pagination examples (1 example)
- Parameters documentation section (20-25 lines):
  - query, sort, order, page, per_page with descriptions
- Response fields documentation (15-20 lines):
  - full_name, description, language, stargazers_count, forks_count, topics, timestamps, license, archived, html_url
- Common use cases (15-20 lines):
  - Find popular libraries
  - Discover trending projects
  - Find active projects
  - Search by topic
- Tips section (10-15 lines)

**Trim minimally**:
- Already concise, minimal redundancy
- Keep all content as-is

**Target**: 95-105 lines

---

### Keep & Trim: `prompt_syntax()` (Target: ~110 lines)

**Current**: 119 lines covering all search qualifiers

**Keep all qualifiers** (they are non-redundant):
- Language filters (language:rust, language:python, language:javascript) - 8-10 lines
- Popularity filters (stars and forks with operators and ranges) - 10-12 lines
- Date filters (created, pushed, updated with operators) - 10-12 lines
- Owner filters (user:username, org:organization) - 8-10 lines
- Topic filters (topic:web, topic:machine-learning, etc.) - 8-10 lines
- License filters (license:mit, license:apache-2.0, etc.) - 8-10 lines
- Size filters (size:>1000, size:<100, etc.) - 8-10 lines
- Status filters (archived:false, is:public, fork:false, etc.) - 10-12 lines
- Activity filters (good-first-issues, help-wanted-issues) - 5-8 lines

**Consolidate and trim**:
- Combining qualifiers section - keep concise example (10-15 lines)
- Range syntax (.. operator) - keep brief explanation (3-5 lines)
- Comparison operators (>, >=, <, <=) - keep brief list (3-5 lines)
- Date formats section - keep example format (3-5 lines)
- Boolean logic (AND implicit, OR explicit, NOT) - keep brief (5-8 lines)
- Best practices - keep 5-6 items, not 10+ (8-10 lines)

**Remove/Consolidate**:
- Multiple examples per qualifier (reduce from 2-3 per qualifier to 1-2 per qualifier)
- Verbose explanations (e.g., lines 185-186 "Exclude archived repositories...")
- Extended filtering guidance

**Target**: 105-115 lines

---

### Delete Entirely: `prompt_patterns()` (154 lines)

**Rationale**:
- Abstract patterns are better demonstrated as concrete workflows
- Each pattern (Pattern 1-12) can be compressed into workflow steps
- Redundant with prompt_workflows() which already shows applied patterns

**Action**: Delete all 154 lines (lines 257-410)

**Note**: Essential pattern concepts (finding libraries, trending projects, beginner-friendly, etc.) will be represented in `prompt_workflows()` as actual workflow examples with step-by-step instructions.

---

### Keep & Trim: `prompt_workflows()` (Target: ~110 lines)

**Current**: 166 lines covering 6 workflows (3-4 steps each)

**Keep these 4 core workflows** (consolidate 6 into 4):
1. **Researching libraries** (steps: broad search → filter by quality → verify maintenance)
   - Lines: 30-35
2. **Finding alternatives** (steps: find popular options → check newer alternatives → compare)
   - Lines: 30-35
3. **Dependency selection** (steps: find stable popular options → verify active maintenance → check license)
   - Lines: 30-35
4. **Finding contribution opportunities** (steps: find beginner-friendly → find help-wanted → find active communities)
   - Lines: 20-25

**Delete/Consolidate**:
- **Technology exploration workflow** (currently steps 1-4) - This is a variant of "Researching libraries", consolidate into that workflow
- **Competitive analysis workflow** (currently steps 1-3) - Can be incorporated as comparative examples in "Finding alternatives"

**Integrate pattern examples**:
- Each workflow should include 2-3 concrete pattern examples integrated into the steps
- Example: In "Researching libraries", show patterns for: (a) finding by language/stars, (b) finding by activity, (c) finding examples/tutorials

**Structure each workflow**:
- Goal statement (1-2 lines)
- Step 1: [query example] (5 lines)
  - Analysis/rationale (2-3 lines)
- Step 2: [refined query] (5 lines)
  - Analysis/rationale (2-3 lines)
- Step 3: [final query or decision] (5 lines)
  - Analysis/rationale (2-3 lines)
- Decision criteria or metrics (5-8 lines)

**Target**: 110-120 lines

---

### Delete Entirely: `prompt_comprehensive()` (186 lines)

**Rationale**:
- Pure duplication of all other scenarios
- Becomes redundant once other scenarios are trimmed
- Adds no unique value beyond combining others

**Action**: Delete all 186 lines (lines 581-766)

The comprehensive content is better served by the three focused, non-redundant scenarios.

---

### Update Scenario Routing

**Current code** (lines 13-24):
```rust
impl PromptProvider for SearchRepositoriesPrompts {
    type PromptArgs = SearchRepositoriesPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("syntax") => prompt_syntax(),
            Some("patterns") => prompt_patterns(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

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
}
```

**Updated code** (new structure):
```rust
impl PromptProvider for SearchRepositoriesPrompts {
    type PromptArgs = SearchRepositoriesPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("syntax") => prompt_syntax(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_basic(),  // Make basic the default
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, syntax, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}
```

**Changes**:
- Line 18: Delete `Some("patterns") => prompt_patterns(),`
- Line 19: Change `_ => prompt_comprehensive(),` to `_ => prompt_basic(),`
- Line 31: Update description from "(basic, syntax, patterns, workflows)" to "(basic, syntax, workflows)"

---

## Implementation Steps

### Step 1: Delete Redundant Scenarios

1. Delete the entire `prompt_patterns()` function (lines 257-410)
   - This is 154 lines that will be consolidated into workflows
   - Lines to delete: 256-410 (actual function definition)

2. Delete the entire `prompt_comprehensive()` function (lines 581-766)
   - This is 186 lines of pure duplication
   - After deleting patterns, comprehensive will shift up
   - Estimated new position: lines 427-612

### Step 2: Trim `prompt_basic()`

No major trimming needed. The scenario is already concise (~91 lines).

**Verification**: Ensure it remains 90-110 lines

### Step 3: Trim `prompt_syntax()`

Consolidate examples and verbose explanations:

**For each qualifier section**, reduce from 2-3 examples per qualifier to 1-2:
- Language filters: Keep "language:rust" and one other example
- Popularity filters: Keep "stars:>1000" and one range example
- Date filters: Keep one representative example per date type
- Owner filters: Keep user and org examples
- Topic filters: Keep "topic:async" and one other
- License filters: Keep "license:mit" and maybe one other
- Size filters: Keep one size example
- Status filters: Keep archived and fork examples
- Activity filters: Keep both if they fit

**Consolidate sections**:
- "COMBINING QUALIFIERS" (currently lines 208-221): Keep 3 examples instead of 5 (reduce 10 lines)
- "RANGE SYNTAX" (lines 222-226): Already concise
- "BOOLEAN LOGIC" (lines 237-243): Already concise
- "BEST PRACTICES" (lines 244-250): Keep essential 5 items (reduce 5 lines)

**Target**: Reduce from 119 to 105-115 lines (remove ~5-10 lines total)

### Step 4: Trim `prompt_workflows()`

Consolidate 6 workflows into 4, reducing each from 3-4 steps to 2-3 steps:

**Keep these workflows**:
1. "Researching libraries" - Keep as primary workflow (target: 30-35 lines)
2. "Finding alternatives" - Essential for comparison use case (target: 30-35 lines)
3. "Dependency selection" - Critical for production decisions (target: 30-35 lines)
4. "Finding contribution opportunities" - Valuable for OSS community (target: 20-25 lines)

**Consolidate into others**:
- "Technology exploration" → Fold into "Researching libraries" (same pattern: broad search → educational → active)
- "Competitive analysis" → Fold into "Finding alternatives" (already covers comparison points)

**Structure each workflow with 2-3 steps**:
- Each step should include: a concrete query example + brief analysis
- Keep decision criteria/metrics at the end of each workflow
- Reduce from 3-4 paragraphs per workflow to 2-3 lines per step

**Target**: Reduce from 166 to 110-120 lines (remove ~45-55 lines)

### Step 5: Update Routing and Help Text

1. Update match statement (lines 17-23):
   - Remove `Some("patterns")` branch
   - Change default from `prompt_comprehensive()` to `prompt_basic()`

2. Update prompt_arguments() description (line 31):
   - Change from: "Scenario to show (basic, syntax, patterns, workflows)"
   - Change to: "Scenario to show (basic, syntax, workflows)"

### Step 6: Verify Total Line Count

After all deletions and trimming:
- Header/comments (lines 1-40): ~40 lines
- PromptProvider impl (lines 13-36): ~24 lines
- prompt_basic(): 95-105 lines
- prompt_syntax(): 105-115 lines
- prompt_workflows(): 110-120 lines
- **Total expected**: 340-365 lines (within 280-360 target range with buffer)

---

## Success Criteria (Definition of Done)

All of the following MUST be true after implementation:

- **Line count**: `wc -l prompts.rs` returns 340-365 lines
- **Scenario count**: `grep "^fn prompt_" prompts.rs` returns exactly 3 functions
- **No patterns scenario**: `grep "fn prompt_patterns" prompts.rs` returns 0 results
- **No comprehensive scenario**: `grep "fn prompt_comprehensive" prompts.rs` returns 0 results
- **Routing updated**: Match statement in `generate_prompts()` has exactly 3 branches (syntax, workflows, basic default)
- **Help text updated**: `prompt_arguments()` description includes only "(basic, syntax, workflows)"
- **Each scenario is readable**: Any developer can understand the tool in 5 minutes reading any single scenario
- **No redundancy**: No duplicate explanations across the three scenarios
- **All qualifiers present**: `prompt_syntax()` covers all GitHub search qualifiers
- **Real workflows included**: `prompt_workflows()` shows 4 realistic end-to-end discovery workflows

---

## Validation Checklist

After completing implementation, verify:

1. **Line count check**
   ```bash
   wc -l prompts.rs  # Should show 340-365
   ```

2. **Function count check**
   ```bash
   grep "^fn prompt_" prompts.rs  # Should show 3 results
   ```

3. **Scenario presence check**
   ```bash
   grep -c "Some(\"basic\")" prompts.rs     # Should show 1
   grep -c "Some(\"syntax\")" prompts.rs    # Should show 1
   grep -c "Some(\"workflows\")" prompts.rs # Should show 1
   grep -c "Some(\"patterns\")" prompts.rs  # Should show 0
   ```

4. **Comprehensive deletion check**
   ```bash
   grep "prompt_comprehensive" prompts.rs  # Should show 0 results
   ```

5. **Code structure check**
   ```bash
   cargo check -p kodegen-mcp-schema  # Must pass with no errors
   ```

6. **Manual review**:
   - Read `prompt_basic()` end-to-end (should take <3 minutes)
   - Read `prompt_syntax()` end-to-end (should take <3 minutes)
   - Read `prompt_workflows()` end-to-end (should take <4 minutes)
   - Verify no content is duplicated across scenarios
   - Verify all GitHub search qualifiers are in syntax scenario
   - Verify workflows show realistic use cases

---

## Reference Information

### Tool Purpose
The `github_search_repositories` tool searches GitHub repositories using powerful query syntax with these parameters:
- `query` (required): GitHub search query with qualifiers
- `sort` (optional): "stars", "forks", "updated"
- `order` (optional): "asc" or "desc"
- `page` (optional): Page number for pagination
- `per_page` (optional): Results per page (max 100)

### Key Scenarios After Trimming
1. **basic**: Fundamental search, filtering, sorting, pagination, response structure
2. **syntax**: Complete reference of all GitHub search qualifiers and operators
3. **workflows**: 4 realistic end-to-end discovery workflows (researching libraries, finding alternatives, dependency selection, contribution opportunities)

### Pattern Examples (Now in Workflows)
Rather than abstract patterns, workflows show:
- How to research libraries (with quality and activity filters)
- How to find and compare alternatives
- How to select production dependencies
- How to find open source contribution opportunities
