# TASK 072: Trim process_kill

**Tool**: `process_kill`
**Complexity**: 2 (Simple)
**Current file**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/process/process_kill/prompts.rs`
**Current size**: 571 lines (4 scenarios)
**Target size**: 140-170 lines (1 scenario)
**Target scenarios**: Keep ONLY basic scenario, delete safety and workflows

---

## Reference

See **PRECURSOR_02_fs_read_file.md** for Complexity 2 template and deletion methodology.

---

## Current State Analysis

**File location**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/process/process_kill/prompts.rs`

**Current structure** (571 lines):
```
Lines 1-43:   Header, use statements, struct and impl
Lines 44-144: prompt_basic() ~101 lines
Lines 146-327: prompt_safety() ~182 lines (WILL DELETE)
Lines 329-570: prompt_workflows() ~242 lines (WILL DELETE)
Lines 571:    prompt_comprehensive() (WILL DELETE)
```

**Current match statement** (lines 20-25):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("safety") => prompt_safety(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**Scenario analysis**:

1. **`prompt_basic()`** (lines 44-144, ~101 lines) - KEEP, HEAVILY TRIM
   - Teaches core functionality: sending SIGKILL to a process by PID
   - Contains essential knowledge: what SIGKILL is, response structure, error codes
   - CONTAINS BLOAT: "COMMON SCENARIOS" section (use-case examples), "TYPICAL WORKFLOW" section, repeated warnings
   - Target: Reduce to ~90-100 lines by removing use-cases and consolidating repeated concepts

2. **`prompt_safety()`** (lines 146-327, ~182 lines) - DELETE
   - This is a use-case scenario, not a tool feature
   - Teaches workflow patterns (verify before kill, check system processes, etc.)
   - Not a parameter feature like large_files in fs_read_file
   - Process_kill has only ONE parameter: pid (no advanced parameters)
   - Safety practices belong in basic, not as separate scenario

3. **`prompt_workflows()`** (lines 329-570, ~242 lines) - DELETE
   - Pure use-case scenarios: stopping runaway processes, freeing ports, batch cleanup, etc.
   - Each workflow is a different APPLICATION pattern, not a tool feature
   - Not tied to any special parameter
   - Example: "WORKFLOW 1: STOPPING A RUNAWAY PROCESS" teaches application management, not tool usage

4. **`prompt_comprehensive()`** - DELETE
   - Combines all three scenarios (pure duplication)
   - No unique content
   - Only exists as default fallback

**Key insight**: process_kill is a SIMPLE 1-parameter tool. Unlike fs_read_file (which has offset/length parameters justifying multiple scenarios), process_kill has no advanced parameters. The "safety" scenario teaches workflow discipline, not tool features. The "workflows" scenario teaches application patterns, not tool knowledge.

---

## Trimming Instructions

### KEEP & HEAVILY TRIM: `prompt_basic()` (Target: ~90-100 lines)

**Current content structure** (lines 44-144):
- Lines 44-50: Function header and PromptMessage wrapper (7 lines) - KEEP
- Lines 51-59: "KILLING PROCESSES:" with basic example (9 lines) - KEEP, TRIM slightly
- Lines 60-65: "WARNING:" section (6 lines) - KEEP, consolidate with error messages later
- Lines 66-73: "BASIC USAGE:" explaining parameters (8 lines) - MERGE into examples section (DELETE this header, explain in-line)
- Lines 74-92: Extended explanation of parameters (19 lines) - TRIM to 8-10 lines
- Lines 93-114: "RESPONSE STRUCTURE:" with 3 example responses (22 lines) - KEEP all, essential reference
- Lines 115-124: "WHAT IS SIGKILL?" (10 lines) - KEEP, critical knowledge
- Lines 125-130: "WHEN TO USE BASIC KILL:" (6 lines) - TRIM to 4 lines (key points only)
- Lines 131-139: "COMMON SCENARIOS:" (9 lines) - DELETE (these are use-case examples: frozen app, runaway process, hung server)
- Lines 140-144: Rest of warnings/notes (5 lines) - CONSOLIDATE with earlier warnings

**Specific deletions from prompt_basic content**:
1. Remove section "COMMON SCENARIOS:" entirely (lines 131-139)
   ```
   COMMON SCENARIOS:
   1. Kill a frozen application:
      process_kill({"pid": 8765})
   ...
   ```
   These are use-case examples. The basic scenario teaches THE TOOL, not application scenarios.

2. Remove section "TYPICAL WORKFLOW:" entirely (if present in expanded output)
   - This teaches implementation patterns
   - Not tool knowledge
   - Belongs to applications, not process_kill

3. Merge "BASIC USAGE:" header content into the initial example
   - Current: separate section explaining the parameter
   - New: integrated into first example with inline explanation

4. Consolidate repetitive warnings
   - Current: "Always verify the PID before killing" appears multiple times
   - New: Mention ONCE in the main explanation, possibly once in error handling

5. Trim parameter explanation from ~19 lines to 8-10 lines
   - Current verbose explanation of what pid is and what SIGKILL does
   - New: One concise example with brief explanation

**Final structure of prompt_basic()** (~95-105 lines):
```
1. PromptMessage wrapper (2 lines)
2. Tool description and what SIGKILL does (3 lines)
3. Basic usage example - kill by PID (6 lines)
4. WARNING section - key cautions (4 lines)
5. Response structure - success and 2 failure cases (15 lines)
6. WHAT IS SIGKILL explanation (8 lines)
7. When to use and limitations (6 lines)
8. Error messages reference (6 lines)
9. Process ID important notes (6 lines)
```

**Keep in basic**:
- SIGKILL explanation (signal 9, forceful, kernel terminates, no cleanup)
- Single clear example showing process_kill({"pid": 12345})
- Response structure with success case: {"pid": 12345, "killed": true}
- Response structure with failure cases: killed: false with error message
- Limitations (can't kill PID 1, requires permissions)
- Error codes: "Process not found", "Permission denied"
- PID ranges and special PIDs (1, 2, system processes)
- Key fact: PIDs can be reused after process terminates

**Remove from basic**:
- Examples like "Kill a frozen application", "Stop a runaway process", etc. (use-cases)
- Workflow sections like "TYPICAL WORKFLOW: 1. Identify 2. Get PID 3. Verify 4. Kill 5. Confirm"
- Repeated "always verify before killing" statements (say once)
- Verbose explanations of obvious concepts
- Extended paragraphs about best practices (not a feature)

### DELETE ENTIRELY

1. **`prompt_safety()`** (182 lines):
   - Workflow pattern: "Find process first, then kill"
   - Safety checklist: Is it the right PID? Is it a system process? Will data be lost?
   - These are USER DISCIPLINE patterns, not TOOL FEATURES
   - No special parameter being taught
   - Classification: Use-case scenario

2. **`prompt_workflows()`** (242 lines):
   - 7 different workflow examples (runaway process, port freeing, frozen app, batch cleanup, daemon, dev server, resource recovery)
   - Each one is a different APPLICATION SCENARIO
   - Not tied to any tool parameter
   - Integration patterns with other tools
   - Error handling patterns specific to workflows
   - Classification: Pure use-case scenarios

3. **`prompt_comprehensive()`**:
   - Duplication only
   - Combines basic + safety + workflows
   - No unique content
   - Delete this function entirely

### Update routing in generate_prompts()

**Before** (lines 20-25):
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("basic") => prompt_basic(),
        Some("safety") => prompt_safety(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

**After**:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    let _ = args;  // Single scenario - ignore argument
    prompt_basic()
}
```

OR, even simpler if updating the function signature is acceptable:
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

The match statement is unnecessary since there's only one scenario.

### Verify prompt_arguments()

**Current state** (lines 27-35):
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show examples for (basic, safety, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

**Update**: Change the description to reflect only basic scenario:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show examples for (basic only)".to_string()),
            required: Some(false),
        }
    ]
}
```

Or remove the argument entirely if no scenarios are used:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![]
}
```

---

## Success Criteria

- ✓ File is 140-170 lines total
- ✓ ONE scenario function exists: basic
- ✓ No safety scenario function
- ✓ No workflows scenario function
- ✓ No comprehensive scenario function
- ✓ Match statement simplified or removed
- ✓ No decorative `═══` headers
- ✓ No repeated "always verify" statements (mention once maximum)
- ✓ SIGKILL knowledge explained (signal 9, forceful, no cleanup)
- ✓ Response structure shown once with 3 cases (success + 2 failures)
- ✓ Error codes documented (Process not found, Permission denied)
- ✓ Single usage example: process_kill({"pid": 12345})
- ✓ Limitations and safeguards mentioned (PID 1, permissions, etc.)

---

## Validation Checklist

After trimming, verify:

1. **Line count**: `wc -l prompts.rs` → 140-170 lines
2. **Scenario count**: `grep "^fn prompt_" prompts.rs` → exactly 1 function (prompt_basic)
3. **No comprehensive**: `grep "prompt_comprehensive" prompts.rs` → 0 results
4. **No redundancy**: `grep -c "SIGKILL"` → appears 1-2 times (concise, not repeated)
5. **No use-cases**: `grep "frozen\|runaway\|daemon"` → 0 results
6. **No workflow**: `grep "WORKFLOW"` → 0 results
7. **Clean routing**: `match args.scenario` either simplified or calls prompt_basic() directly
8. **Syntax valid**: File compiles with no errors: `cd packages/kodegen-mcp-schema && cargo check`

---

## Implementation Steps

### Step 1: Backup and plan (reference only)
- Original file: 571 lines with 4 scenario functions
- Target: 140-170 lines with 1 scenario function
- Deletion: ~390-410 lines of code

### Step 2: Trim prompt_basic() function
1. Identify and remove "COMMON SCENARIOS:" section (9 lines)
2. Remove "TYPICAL WORKFLOW:" section if present (7+ lines)
3. Merge "BASIC USAGE:" header into first example (reduce 8 lines to 2-3)
4. Consolidate repetitive warnings to single mention (save 3-4 lines)
5. Trim parameter explanation from 19 lines to 10 lines (save 9 lines)
6. Verify response structure stays complete (22 lines - keep all)
7. Verify SIGKILL explanation stays (10 lines - keep all)
8. Result: ~95-105 lines for basic scenario

### Step 3: Delete unused functions
1. Delete `fn prompt_safety()` entirely (lines 146-327)
2. Delete `fn prompt_workflows()` entirely (lines 329-570)
3. Delete `fn prompt_comprehensive()` entirely (last ~15 lines)

### Step 4: Update routing
1. Simplify `generate_prompts()` to remove match statement
2. Just call `prompt_basic()` directly
3. Update description in `prompt_arguments()` to reflect single scenario

### Step 5: Validation
1. Run `wc -l prompts.rs` → confirm 140-170 range
2. Run `grep "^fn prompt_"` → confirm exactly 1 function
3. Run `cargo check` in kodegen-mcp-schema → confirm compilation
4. Verify no decorative headers remain

---

## Code Pattern - Before & After

**Before**: 571 lines with 4 scenario functions
```rust
impl PromptProvider for ProcessKillPrompts {
    type PromptArgs = ProcessKillPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("safety") => prompt_safety(),           // DELETE
            Some("workflows") => prompt_workflows(),     // DELETE
            _ => prompt_comprehensive(),                 // DELETE
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                description: Some("Scenario to show examples for (basic, safety, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

fn prompt_basic() -> Vec<PromptMessage> { ... } // ~101 lines, TRIM to 95-105

fn prompt_safety() -> Vec<PromptMessage> { ... } // ~182 lines, DELETE

fn prompt_workflows() -> Vec<PromptMessage> { ... } // ~242 lines, DELETE

fn prompt_comprehensive() -> Vec<PromptMessage> { ... } // DELETE
```

**After**: 140-170 lines with 1 scenario function
```rust
impl PromptProvider for ProcessKillPrompts {
    type PromptArgs = ProcessKillPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        let _ = args;  // Unused: single scenario only
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]  // No arguments needed: single scenario
    }
}

fn prompt_basic() -> Vec<PromptMessage> { ... } // ~95-105 lines, trimmed
```

**Alternative simpler routing** (if updating signature):
```rust
fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    prompt_basic()
}
```

---

## Specific Content Cuts

### Remove from prompt_basic()

**Section 1: "COMMON SCENARIOS"** (9 lines, example):
```
COMMON SCENARIOS:
1. Kill a frozen application:
   process_kill({"pid": 8765})

2. Stop a runaway process:
   process_kill({"pid": 4321})

3. Terminate hung server:
   process_kill({"pid": 9999})
```
Reason: These are use-case examples (frozen app, runaway process, hung server are APPLICATION-LEVEL patterns, not tool features).

**Section 2: "TYPICAL WORKFLOW"** (7 lines, example):
```
TYPICAL WORKFLOW:
1. Identify the process (by name or behavior)
2. Get its PID from process_list
3. Verify it's the correct process
4. Call process_kill with the PID
5. Confirm termination succeeded
```
Reason: This is an implementation PATTERN that applies to workflow scenarios, not tool knowledge.

**Section 3: Trim "BASIC USAGE" explanation** (reduce from 8 lines to 2-3):
```
BEFORE:
BASIC USAGE:
The process_kill tool requires only one parameter:
- pid (required): The Process ID to terminate

Example:
process_kill({"pid": 12345})

AFTER:
The process_kill tool takes one parameter - pid (required) - and sends SIGKILL to that process:
process_kill({"pid": 12345})
```

### Keep in final basic scenario

All essential knowledge:
- What SIGKILL is (signal 9, forceful, kernel handles, no cleanup)
- What a PID is and how to provide it
- Response structure for success and failures
- Error messages: "Process not found", "Permission denied"
- Limitations: Can't kill PID 1, requires permissions
- Warning: This is forceful, no undo
- PID notes: ranges, reuse, special PIDs to never kill

---

## Notes for Executor

1. This tool is Complexity 2 because it has 1 simple parameter (pid) with no variations or special features
2. Unlike fs_read_file (which justifies 2 scenarios via offset/length parameters), process_kill has no feature variations
3. Safety practices and workflows are USE CASES, not TOOL FEATURES, so they belong in applications that use process_kill, not in the tool prompt
4. The template (PRECURSOR_02) is the standard for all Complexity 2 tools: keep 1-2 feature-based scenarios, delete use-cases and comprehensive
5. After this trimming, process_kill becomes the standard reference for other 1-parameter tools

