# TASK 105: Trim claude_agent

**Tool**: `claude_agent`
**Complexity**: 5 (Very Complex)
**Current size**: 1297 lines (71.9 KB)
**Target size**: 560-640 lines (5 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/claude_agent/agent/prompts.rs`

---

## Reference

**terminal** at 591 lines is the GOLD STANDARD for Complexity 5.

---

## Current State Analysis

### File Structure
Location: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/claude_agent/agent/prompts.rs`
- Total lines: 1297
- File size: 71.9 KB
- Last modified: 6 hours ago

### Current Scenarios (6 total)
1. **prompt_basic_delegation** (~200 lines)
   - Covers SPAWN action, when to delegate vs handle directly
   - Contains 3 examples (simple delegation, fire-and-forget, sequential refinement)
   - Teaches basic delegation patterns
   - Starting around line 50

2. **prompt_specialized_agents** (~200 lines)
   - Covers allowed_tools (allowlist) and disallowed_tools (blocklist)
   - Covers tool constraints for security/focus
   - Contains 5 examples (read-only, analysis, database, testing, documentation)
   - Has common tool categories and security patterns

3. **prompt_parallel_coordination** (~200 lines)
   - Covers SPAWN multiple agents (0, 1, 2...)
   - Covers LIST action (show all agents)
   - Covers coordination patterns
   - Contains 4 examples (parallel code analysis, research, testing, divide-and-conquer)

4. **prompt_research_agents** (~200 lines)
   - Covers add_dirs parameter (context loading)
   - Covers when to use add_dirs
   - Contains 6 examples (auth research, API docs, database schema, code review, migration analysis, parallel research)
   - Has research workflow patterns

5. **prompt_progress_monitoring** (~150 lines)
   - Covers READ action (check agent progress)
   - Covers timeout handling and execution modes
   - Contains 4 examples (fire-and-forget, timeout handling, parallel monitoring, progressive refinement)
   - Has monitoring patterns and response fields

6. **prompt_comprehensive** (~97 lines) - REDUNDANT
   - Covers all 5 ACTIONS in one scenario
   - Duplicates information from other scenarios
   - Can be safely deleted as other scenarios cover all actions

### Action Coverage
- SPAWN: prompt_basic_delegation (primary)
- SEND: Mentioned in examples but not featured (need to verify still documented)
- READ: prompt_progress_monitoring (primary)
- LIST: prompt_parallel_coordination (primary)
- KILL: prompt_progress_monitoring (mentioned with READ)

### Routing Logic (lines 15-28)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic_delegation(),
        Some("specialized") => prompt_specialized_agents(),
        Some("parallel") => prompt_parallel_coordination(),
        Some("research") => prompt_research_agents(),
        Some("monitoring") => prompt_progress_monitoring(),
        _ => prompt_comprehensive(),  // DEFAULT - DELETE AFTER REMOVING prompt_comprehensive
    }
}
```

After trimming, update default case to one of the kept scenarios (e.g., `prompt_basic_delegation()`).

---

## Implementation Instructions

### Step 1: Delete prompt_comprehensive Function
- Find and delete the entire prompt_comprehensive function (lines ~1201-1297)
- This saves ~97 lines
- Update routing default case (line 26) from `_ => prompt_comprehensive()` to `_ => prompt_basic_delegation()`

### Step 2: Trim prompt_basic_delegation (Target: 140 lines)
**Keep**:
- "When should I delegate?" question
- WHEN TO DELEGATE section (4-5 bullet points)
- WHEN NOT TO DELEGATE section (4-5 bullet points)
- BASIC DELEGATION PATTERN (3 steps)
- 2 focused examples:
  - Example 1: Simple Delegation (basic SPAWN with defaults)
  - Example 2: Fire-and-Forget (SPAWN with await_completion_ms: 0)
- KEY PARAMETERS section (core parameters only)
- BEST PRACTICES (3-4 essentials)

**Delete**:
- Example 3: Sequential Refinement (covered in monitoring scenario)
- DECISION TREE (covered elsewhere)
- RESPONSE FIELDS (covered in monitoring scenario)
- Verbose explanations beyond prescriptive guidance
- Decorative section headers (====)

**Before** (lines ~50-250): ~200 lines with verbose explanations, 3+ examples, multiple decision trees
**After** (target): ~140 lines, 2 focused examples, prescriptive language only

### Step 3: Trim prompt_specialized_agents (Target: 130 lines)
**Keep**:
- "How do I create specialized sub-agents?" question
- WHY CONSTRAIN AGENTS section (4-5 bullet points)
- TWO APPROACHES section (allowlist vs blocklist)
- 3 focused examples:
  - Example 1: Read-Only Research Agent (allowed_tools with fs_read_file, fs_search)
  - Example 2: Code Analysis (disallowed_tools blocking execution)
  - Example 3: Database-Focused (allowed_tools specific to database)
- SECURITY PATTERNS (3-4 key patterns)
- BEST PRACTICES (3-4 essentials)

**Delete**:
- Examples 4, 5, 6 (Testing, Documentation, Parallel - too many)
- COMMON TOOL CATEGORIES (reference not needed)
- TOOL CONSTRAINT SYNTAX (shown in examples)
- COMBINING WITH OTHER PARAMETERS (covered in SPAWN scenario)
- Verbose "why" explanations

**Before**: ~200 lines with 5+ examples, decorative headers, verbose categories
**After**: ~130 lines with 3 focused examples, prescriptive only

### Step 4: Trim prompt_parallel_coordination (Target: 120 lines)
**Keep**:
- "How do I spawn multiple sub-agents?" question
- PARALLEL AGENT PATTERN (5 steps)
- 2 focused examples:
  - Example 1: Parallel Code Analysis (3 agents, different tasks)
  - Example 2: Divide-and-Conquer File Processing (split work across agents)
- MONITORING PARALLEL AGENTS section (LIST and READ usage)
- COORDINATION PATTERNS (2-3 key patterns)
- BEST PRACTICES (3-4 essentials)

**Delete**:
- Examples 2, 3 (Research tasks, Testing - reduce redundancy)
- AGENT NUMBERING section (basic)
- RESULT SYNTHESIS section (covered in examples)
- WHEN TO USE / WHEN NOT TO USE (already covered)
- Decorative headers

**Before**: ~200 lines with 4+ examples
**After**: ~120 lines with 2 focused examples

### Step 5: Trim prompt_progress_monitoring (Target: 100 lines)
**Keep**:
- "How do I monitor progress?" question
- THREE EXECUTION MODES (BLOCKING, FIRE-AND-FORGET, CUSTOM)
- MONITORING WITH READ ACTION (what READ returns)
- 2 focused examples:
  - Example 1: Fire-and-Forget with Periodic Monitoring
  - Example 2: Timeout Handling
- UNDERSTANDING RESPONSE FIELDS (key fields: completed, working, exit_code, output)
- Timeout recommendations table
- BEST PRACTICES (3-4 essentials)

**Delete**:
- Examples 3, 4 (Parallel monitoring, Progressive refinement - too many)
- MONITORING PATTERNS section (detailed pseudocode)
- COMBINING READ WITH SEND (covered in basic scenario)
- Verbose monitoring explanations

**Before**: ~150 lines with 4+ examples
**After**: ~100 lines with 2 focused examples

### Step 6: Trim prompt_research_agents (Target: 110 lines)
**Keep**:
- "How do I create research-focused agents?" question
- WHAT IS add_dirs section
- WHEN TO USE add_dirs (4-5 bullet points)
- WHEN NOT TO USE add_dirs (3-4 bullet points)
- 2 focused examples:
  - Example 1: Deep Authentication Research (load auth directories)
  - Example 2: Database Schema Analysis (load db directories)
- add_dirs SYNTAX (single vs multiple directories)
- RESEARCH WORKFLOW PATTERN (with SEND for iteration)
- BEST PRACTICES (3-4 essentials)

**Delete**:
- Examples 3, 4, 5, 6 (too many examples - keep only 2)
- CONTEXT LIMITS detailed explanation
- COMBINING WITH OTHER PARAMETERS (covered in SPAWN scenario)
- Example 6: Parallel Research (too complex for this scenario)
- Verbose explanations

**Before**: ~200 lines with 6 examples
**After**: ~110 lines with 2 focused examples

### Step 7: Verify All 5 ACTIONS Are Documented
After trimming, verify documentation:
- **SPAWN**: prompt_basic_delegation (examples show `"action": "SPAWN"`)
- **SEND**: prompt_research_agents (research workflow shows `"action": "SEND"` for follow-ups)
- **READ**: prompt_progress_monitoring (primary focus, multiple examples)
- **LIST**: prompt_parallel_coordination (example shows `"action": "LIST"`)
- **KILL**: prompt_progress_monitoring (mentioned with cleanup, add example if missing)

If SEND or KILL need more visibility, add brief mention in basic_delegation or ensure examples show them clearly.

### Step 8: Update Routing Default Case
Change line 26 from:
```rust
_ => prompt_comprehensive(),
```
to:
```rust
_ => prompt_basic_delegation(),
```

---

## Code Patterns: Before vs After

### Reducing Examples Pattern
**Before** (5 examples in specialized_agents):
```text
EXAMPLE 1: Read-Only Research Agent
EXAMPLE 2: Code Analysis Agent
EXAMPLE 3: Database-Focused Agent
EXAMPLE 4: Testing Agent
EXAMPLE 5: Documentation Agent
```

**After** (3 examples):
```text
EXAMPLE 1: Read-Only Research Agent
EXAMPLE 2: Code Analysis Agent
EXAMPLE 3: Database-Focused Agent
```
Delete Examples 4-5 that add redundant patterns.

### Removing Decorative Headers
**Before**:
```text
============================================================================
HELPER FUNCTIONS - TEACHING AI AGENTS META-COGNITION
============================================================================
```

**After**: Just use section headers with `///` comments or remove entirely

### Condensed Scenario Structure
Each scenario becomes:
1. User question (2 lines)
2. Assistant intro (2 lines)
3. Core concept 1 (5-10 lines)
4. Example 1 (15-20 lines)
5. Core concept 2 (5-10 lines)
6. Example 2 (15-20 lines)
7. Best practices (10-15 lines)
Total: ~70-140 lines per scenario (vs current 150-200)

---

## Success Criteria

All criteria must be met:
- Final line count: 560-640 lines total
- 5 scenarios included:
  - prompt_basic_delegation (140 lines)
  - prompt_specialized_agents (130 lines)
  - prompt_parallel_coordination (120 lines)
  - prompt_progress_monitoring (100 lines)
  - prompt_research_agents (110 lines)
- All 5 ACTIONS explicitly documented:
  - SPAWN with examples in basic_delegation
  - SEND with examples in research_agents workflow
  - READ with examples in progress_monitoring
  - LIST with examples in parallel_coordination
  - KILL with examples in progress_monitoring
- No prompt_comprehensive function
- Routing default case updated
- All guidance is prescriptive (THIS IS how to do it, not "you could")
- No language about testing, benchmarking, or documentation generation
- Each scenario focused on distinct use case/ACTION
- Code examples show exact JSON/function calls
- Specific parameter names and values mentioned
