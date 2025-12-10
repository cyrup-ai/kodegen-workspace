# TASK 104: Trim browser_agent prompts.rs

**Tool**: `browser_agent`
**Complexity**: 5 (Very Complex)
**Current size**: 379 lines (5 scenarios)
**Target size**: 560-640 lines (5 focused scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/agent/prompts.rs`

---

## Reference Standard

**terminal** at 591 lines is the GOLD STANDARD for Complexity 5.

File location: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/terminal/prompts.rs`

Terminal structure (follow this pattern):
- impl PromptProvider with generate_prompts() routing to 5 functions
- 5 scenario functions: basic, parallel, background, monitoring, comprehensive
- Each scenario is tightly focused on one use case/action
- comprehensive ties everything together with clear section dividers (=== headers)
- Total: 591 lines with clear, prescriptive examples

---

## Current State Analysis

### File Structure
- Location: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/agent/prompts.rs`
- Current size: 379 lines
- Current scenarios: 5 functions

### Current Scenario Breakdown
1. **prompt_basic()** (~80 lines, lines 45-126)
   - Covers PROMPT, READ, KILL actions
   - Shows basic task spawning and progress checking
   - Shows all three actions but not deeply

2. **prompt_navigation()** (~100 lines, lines 128-232)
   - Covers PROMPT action only
   - Shows navigation patterns (search, multi-page, form interaction)
   - Examples of start_url and max_steps parameters

3. **prompt_extraction()** (~120 lines, lines 234-356)
   - Covers PROMPT action only
   - Shows data extraction patterns
   - Examples of structured output, multi-page extraction

4. **prompt_management()** (~120 lines, lines 358-475)
   - Covers PROMPT, READ, KILL actions
   - Shows background tasks, multiple agents, concurrency
   - Shows timeout control and agent lifecycle

5. **prompt_comprehensive()** (~340 lines, lines 477-379)
   - Covers all PROMPT/READ/KILL with full guide
   - Currently too verbose (should be ~200-250 lines)
   - Has decorative headers and redundant information

### Issues to Address
1. Comprehensive section is 340 lines - needs trimming to 200-250 lines
2. Navigation and extraction scenarios are both PROMPT-specific and have overlap
3. Scenarios don't map cleanly to action coverage (terminal pattern is cleaner)
4. Redundant examples across scenarios (e.g., background tasks explained in both management and comprehensive)

---

## Implementation Instructions

### Step 1: Consolidate Scenario Structure

Rename and refocus the 5 scenario functions to match terminal.rs pattern:

**Current → New mapping:**
```
prompt_basic()        → prompt_basic() (REFOCUS on PROMPT action)
prompt_navigation()   → MERGE into prompt_autonomous()
prompt_extraction()   → MERGE into prompt_autonomous()
prompt_management()   → KEEP as prompt_management() (RENAME focus to KILL action)
(NEW)                 → prompt_monitoring() (EXTRACT from basic/management, focus on READ)
prompt_comprehensive()→ prompt_comprehensive() (REWRITE more tightly)
```

### Step 2: Rewrite prompt_basic() (140-160 lines)

**Focus**: PROMPT action - spawning and executing browser agent tasks

**Structure**:
- User question: "How do I spawn and execute browser agent tasks?"
- Assistant answer covering:
  - Basic PROMPT syntax (spawning a task)
  - Task description requirements
  - start_url parameter usage
  - max_steps and max_actions_per_step configuration
  - Response structure (agent number, status, result)
  - 4-5 practical examples showing different task types:
    * Documentation lookup
    * Search and extraction
    * Form interaction
    * Multi-page research
  - Key parameters table: task, start_url, max_steps, max_actions_per_step, max_tokens, temperature, additional_info
  - Output interpretation (status: pending/running/completed, result field content)

**Target lines**: 150-160
**Key action**: PROMPT (spawning tasks)
**Include**: Practical examples with expected responses

### Step 3: Create prompt_autonomous() (130-140 lines)

**Focus**: Autonomous multi-step navigation and complex workflows

**Structure**:
- User question: "How do I use browser_agent for autonomous multi-step operations?"
- Assistant answer covering:
  - Multi-step navigation concepts
  - When to use multi-step vs single-step
  - Setting max_steps appropriately for complexity
  - Agent reasoning and decision-making in autonomous mode
  - Vision-based element detection and interaction
  - Handling dynamic content and pagination
  - 4-5 examples showing:
    * Multi-page form workflows
    * Navigation chains (breadcrumb-style)
    * Research workflows (search → navigate → extract)
    * Fallback handling (retry on failure)
  - Best practices for task clarity and constraints
  - Timeout and step budgeting

**Target lines**: 130-140
**Key action**: PROMPT (with multi-step focus)
**Include**: Examples showing reasoning and complexity

### Step 4: Create prompt_monitoring() (110-120 lines)

**Focus**: READ action - checking agent progress and status

**Structure**:
- User question: "How do I monitor browser agent progress?"
- Assistant answer covering:
  - READ action syntax and when to use it
  - Response fields: agent number, status, current_url, steps_completed, progress, exit_code
  - Fire-and-forget pattern (await_completion_ms: 0)
  - Polling strategy for long-running tasks
  - When to check (after timeout, periodic checks, completion detection)
  - Status interpretation:
    * null/pending = not started
    * running = actively executing steps
    * completed = finished (check exit_code)
  - 4-5 examples showing:
    * Fire-and-forget + polling
    * Multiple agents monitored in parallel
    * Timeout detection and recovery
    * Progress interpretation from output
  - Tail parameter for output limiting
  - Best practices for monitoring without spamming

**Target lines**: 110-120
**Key action**: READ (progress monitoring)
**Include**: Polling patterns and status interpretation

### Step 5: Refocus prompt_management() (90-100 lines)

**Focus**: KILL action - agent lifecycle and cleanup

**Structure**:
- User question: "How do I manage agent lifecycle and cleanup?"
- Assistant answer covering:
  - KILL action syntax and when to use
  - Lifecycle: PROMPT → monitor with READ → KILL when done
  - Graceful shutdown behavior
  - Cleanup patterns:
    * Single agent cleanup
    * Multiple agent cleanup
    * Background task termination
  - 3-4 examples showing:
    * Simple cleanup after completion
    * Multiple agent cleanup workflow
    * Error recovery cleanup
    * Background service teardown
  - Concurrency patterns (multiple agents running, managing each)
  - Resource management
  - When NOT to KILL (monitoring still needed)

**Target lines**: 90-100
**Key action**: KILL (lifecycle management)
**Include**: Cleanup workflows and concurrency

### Step 6: Rewrite prompt_comprehensive() (200-250 lines)

**Focus**: Complete reference guide with clear organization

**Structure** (following terminal.rs pattern):
```
Intro: "The browser_agent provides autonomous web automation with AI-driven navigation..."

THREE ACTIONS:
1. PROMPT: Spawn new browser automation task
2. READ: Check current agent progress/status
3. KILL: Gracefully shutdown agent and cleanup

==== SECTION: ACTION 1 - PROMPT (Spawn Tasks) ====
- Syntax and basic usage
- Configuration parameters (start_url, max_steps, etc.)
- Response structure
- Timeout control (await_completion_ms)
- Example workflow

==== SECTION: ACTION 2 - READ (Monitor Progress) ====
- Syntax and basic usage
- When to use READ
- Response fields and interpretation
- Polling patterns
- Example workflow

==== SECTION: ACTION 3 - KILL (Cleanup) ====
- Syntax and basic usage
- Lifecycle management
- When to KILL
- Example workflow

==== SECTION: DECISION TREE - Which Action? ====
- Need to RUN task → PROMPT
- Need to CHECK progress → READ
- Need to STOP agent → KILL

==== SECTION: COMMON WORKFLOWS ====
- Documentation research
- Data collection and extraction
- Multi-page form submission
- Competitive analysis
- Background research (fire-and-forget + polling)

==== SECTION: CONCURRENCY PATTERNS ====
- Multiple agents for parallel tasks
- Agent numbering (0, 1, 2...)
- Independent state management
- Monitoring multiple agents
- Cleanup strategies

==== SECTION: BEST PRACTICES ====
- Clear, specific task descriptions
- Setting appropriate max_steps
- Using start_url when known
- Output format specification
- Error handling strategies
```

**Target lines**: 200-250
**Key content**: All three ACTIONS with practical patterns
**Include**: Decision tree and workflows from terminal pattern

---

## Scenario Routing Update

The `generate_prompts()` function in AgentPrompts impl must route to these 5 functions:

```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),           // PROMPT action focus
        Some("autonomous") => prompt_autonomous(), // Multi-step operations
        Some("monitoring") => prompt_monitoring(), // READ action focus
        Some("management") => prompt_management(), // KILL action focus
        _ => prompt_comprehensive(),                // All actions reference
    }
}
```

**Update prompt_arguments()** to describe the 5 scenarios:

```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some(
                "Scenario: basic (PROMPT action), autonomous (multi-step), monitoring (READ action), management (KILL action)".to_string()
            ),
            required: Some(false),
        }
    ]
}
```

---

## Line Count Targets (Total: 560-640 lines)

- Header/imports/impl: ~45 lines (unchanged)
- prompt_basic(): 150-160 lines
- prompt_autonomous(): 130-140 lines
- prompt_monitoring(): 110-120 lines
- prompt_management(): 90-100 lines
- prompt_comprehensive(): 200-250 lines
- **Total: ~725-795 lines**

⚠️ **NOTE**: This exceeds target. Terminal is 591 lines total. Need to be more aggressive with trimming:
- Reduce comprehensive to 150-180 lines (consolidate examples)
- Make scenarios more concise (reduce examples from 4-5 to 3-4)
- Use tighter formatting in terminal.rs style

**Revised realistic target: 580-640 lines** (following terminal more closely)

---

## Line-by-Line Checklist

### Phase 1: Setup
- [ ] Open `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/agent/prompts.rs`
- [ ] Keep header comment and imports (lines 1-7)
- [ ] Keep AgentPrompts struct definition (lines 9-11)
- [ ] Update PromptProvider impl and prompt_arguments() for new scenarios

### Phase 2: Rewrite Scenario Functions
- [ ] Rewrite prompt_basic() with PROMPT focus (~150 lines)
- [ ] Create prompt_autonomous() from navigation+extraction merge (~130 lines)
- [ ] Create prompt_monitoring() from READ patterns (~110 lines)
- [ ] Refocus prompt_management() on KILL action (~90 lines)
- [ ] Rewrite prompt_comprehensive() tightly organized (~180 lines)

### Phase 3: Verification
- [ ] Total line count: 560-640 lines
- [ ] All 5 scenarios present and routable
- [ ] PROMPT action documented in basic
- [ ] READ action documented in monitoring
- [ ] KILL action documented in management
- [ ] Autonomous action shown in autonomous scenario
- [ ] Concurrency patterns included in monitoring/management
- [ ] No decorative headers except section dividers in comprehensive
- [ ] Examples are prescriptive (this IS how to do it) not optional

---

## Key Requirements (MUST FOLLOW)

1. **Line count**: 560-640 lines total (not 725)
2. **Scenarios**: Exactly 5 (basic, autonomous, monitoring, management, comprehensive)
3. **ACTION documentation**: All 3 must be clearly documented
   - PROMPT: In basic and comprehensive
   - READ: In monitoring and comprehensive
   - KILL: In management and comprehensive
4. **Routing**: generate_prompts() must match the 5 scenarios
5. **Format**: Follow terminal.rs style (prescriptive, organized, clear)
6. **Concurrency**: Must include patterns for parallel agents
7. **No tests/benchmarks**: Remove if present
8. **No optional language**: Use prescriptive tone ("IS how", not "can be")

---

## Definition of Done

The implementation is complete when:

1. File has exactly 560-640 lines (verified with line count)
2. All 5 scenarios are present and routable
3. Each scenario teaches one primary action/pattern
4. All 3 ACTIONS are documented clearly
5. Concurrency patterns are included
6. Examples are practical and prescriptive
7. No decorative headers (only section dividers in comprehensive)
8. File compiles with no warnings
9. Organization matches terminal.rs pattern
10. Comments make it clear what each scenario teaches

---

## Common Patterns to Include

All scenarios must show:
- Clear syntax with JSON examples
- Response structure and fields
- When to use (real-world scenarios)
- Parameter explanations
- Best practices for that action

---

## Reference Files

- **Gold standard**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/terminal/prompts.rs` (591 lines)
- **Current file**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/agent/prompts.rs` (379 lines)
- **Related types**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/browser/agent/prompt_args.rs`
