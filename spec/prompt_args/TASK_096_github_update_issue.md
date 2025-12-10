# TASK 096: Trim github_update_issue Prompts

**Tool**: `github_update_issue`
**Complexity**: 3 (Medium)
**Current size**: 1235 lines (5 scenarios)
**Target size**: 280-360 lines (2 scenarios)
**File**: `/Volumes/samsung_t9/kodegen-workspace/packages/kodegen-mcp-schema/src/github/update_issue/prompts.rs`

---

## Current State Analysis

### File Structure
The prompts.rs file is organized as follows:
- **Lines 1-40**: Module header, imports, struct definition, PromptProvider impl
- **Lines 43-204**: `prompt_state()` function (162 lines)
- **Lines 207-399**: `prompt_metadata()` function (193 lines)
- **Lines 402-591**: `prompt_content()` function (190 lines)
- **Lines 594-888**: `prompt_workflows()` function (295 lines)
- **Lines 891-1234**: `prompt_comprehensive()` function (344 lines)

### Current Routing Logic (Lines 16-24)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("state") => prompt_state(),
        Some("metadata") => prompt_metadata(),
        Some("content") => prompt_content(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}
```

### Scenario Descriptions

**prompt_state() (Lines 43-204)**
Covers state changes (open/closed):
- Simple close operations
- Close with comments and explanations
- Close and update body together
- Reopen patterns with reasoning
- Common state management patterns: bug fix workflow, wontfix closure, duplicate closure, stale cleanup
- Best practices for state management
- Permission requirements
- State change workflow steps
- Error handling (404, 403, 422)
- Response structure

**prompt_metadata() (Lines 207-399)**
Covers labels, assignees, milestones:
- Label management (single, multiple, clear, replace)
- Assignee management (single, multiple, unassign, reassign)
- Milestone management (set, change, remove)
- Combined metadata updates (4 examples: triage, mark ready, escalate, mark blocked)
- Best practices for labels, assignees, milestones
- Workflow tips for metadata
- Error scenarios
- Preserving existing metadata (important note about replacement behavior)
- Metadata automation patterns (auto-label by title, auto-assign by area, auto-milestone by priority)

**prompt_content() (Lines 402-591)**
Covers title and body updates:
- Title updates (fix typo, add prefix, add version info, make specific)
- Body updates (add missing info, update with findings, add code, format with template)
- Markdown formatting support
- Combined title + body updates
- Content update best practices
- Template examples (bug report, feature request)
- Error handling for long bodies and special characters
- Preserving history when updating

**prompt_workflows() (Lines 594-888)**
Covers 10 complex workflows:
1. Issue triage (get, analyze, update, comment)
2. Bug resolution (investigating, confirmed, PR created, resolved stages)
3. Feature request tracking (proposal, approved, in progress, completed)
4. Duplicate handling (identify, consolidate, cross-reference)
5. Stale issue cleanup (find stale, add comment, close)
6. Bulk label update (search, update each)
7. Milestone management (find, evaluate, move)
8. Issue escalation (identify, update priority, notify)
9. Community contribution setup (prepare, unassign)
10. Release preparation (list, complete, incomplete)

**prompt_comprehensive() (Lines 891-1234)**
Complete guide covering all aspects - essentially a superset of all other scenarios combined.

---

## Implementation Instructions

### Step 1: Delete Unused Scenarios
Delete these three functions entirely:
- `prompt_content()` (lines 402-591, 190 lines)
- `prompt_workflows()` (lines 594-888, 295 lines)
- `prompt_comprehensive()` (lines 891-1234, 344 lines)

### Step 2: Trim prompt_state() Function
**Current state**: 162 lines (lines 43-204)
**Target**: ~110 lines
**Trimming strategy**: Remove the COMMON STATE MANAGEMENT PATTERNS section while keeping essential examples

Remove these patterns (lines 113-170 approximate location):
- "Bug fix workflow" pattern (lines 114-127)
- "Won't fix closure" pattern (lines 129-141)
- "Duplicate closure" pattern (lines 143-155)
- "Stale issue cleanup" pattern (lines 157-169)

Keep:
- CHANGING ISSUE STATE section (state values)
- CLOSING AN ISSUE section (3 examples: simple, with explanation, with body update)
- REOPENING AN ISSUE section (2 examples)
- BEST PRACTICES section (trimmed to 5-6 key points)
- PERMISSIONS REQUIRED
- STATE CHANGE WORKFLOW
- ERROR HANDLING
- RESPONSE STRUCTURE

### Step 3: Trim prompt_metadata() Function
**Current state**: 193 lines (lines 207-399)
**Target**: ~150 lines
**Trimming strategy**: Remove automation patterns and condense best practices

Remove these sections:
- METADATA AUTOMATION PATTERNS (lines 384-395, approximately 12 lines)
- Condense COMBINED METADATA UPDATES from 4 examples to 2 examples (saves ~20 lines)
- Trim BEST PRACTICES section by removing less critical guidance

Keep:
- UPDATING LABELS section (4 examples, important behavior)
- COMMON LABEL PATTERNS
- UPDATING ASSIGNEES section (4 examples)
- UPDATING MILESTONES section (3 examples)
- COMBINED METADATA UPDATES (reduce to 2 most useful examples)
- BEST PRACTICES (condensed version)
- WORKFLOW TIPS (keep)
- ERROR SCENARIOS (keep)
- PRESERVING EXISTING METADATA (keep - critical)

### Step 4: Update PromptProvider Implementation
Update the `prompt_arguments()` function (lines 26-35) to only list valid scenarios:
```rust
fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (state, metadata)".to_string()),
            required: Some(false),
        }
    ]
}
```

Update the `generate_prompts()` function (lines 16-24) to only handle 2 scenarios:
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("metadata") => prompt_metadata(),
        _ => prompt_state(),  // Default to state
    }
}
```

### Step 5: Verify Structure
After edits, the file structure should be:
- Lines 1-40: Module definition (unchanged)
- Lines 41-~150: `prompt_state()` function (trimmed)
- Lines ~150-~300: `prompt_metadata()` function (trimmed)

---

## Specific Code Changes

### Before: Lines 16-35 (routing and arguments)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("state") => prompt_state(),
        Some("metadata") => prompt_metadata(),
        Some("content") => prompt_content(),
        Some("workflows") => prompt_workflows(),
        _ => prompt_comprehensive(),
    }
}

fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (state, metadata, content, workflows)".to_string()),
            required: Some(false),
        }
    ]
}
```

### After: Lines 16-35 (routing and arguments)
```rust
fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
    match args.scenario.as_deref() {
        Some("metadata") => prompt_metadata(),
        _ => prompt_state(),
    }
}

fn prompt_arguments() -> Vec<PromptArgument> {
    vec![
        PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (state, metadata)".to_string()),
            required: Some(false),
        }
    ]
}
```

### Before: Lines 113-170 (to delete from prompt_state)
Remove these 4 workflow pattern examples entirely.

### Before: Lines 384-395 (to delete from prompt_metadata)
Remove the METADATA AUTOMATION PATTERNS section.

---

## Success Criteria

- **✓ Final line count**: 280-360 lines total (target: ~300)
- **✓ Scenario count**: 2 scenarios only (state, metadata)
- **✓ Deleted items**:
  - `prompt_content()` function completely removed
  - `prompt_workflows()` function completely removed
  - `prompt_comprehensive()` function completely removed
  - All decorative headers and patterns removed
  - Automation patterns section removed
- **✓ Routing logic**: Match statement handles only "state" and "metadata" with proper default
- **✓ Arguments**: Argument description updated to only list valid scenarios
- **✓ File compiles**: No syntax errors, proper closing braces
- **✓ Content quality**: Both remaining scenarios are comprehensive yet concise

---

## Detailed Trimming Calculations

### prompt_state() Trimming
- Original: 162 lines
- Remove: COMMON STATE MANAGEMENT PATTERNS (4 examples × ~14 lines each = ~56 lines)
- Result: ~106 lines
- Target met: YES (goal was 100-120)

### prompt_metadata() Trimming
- Original: 193 lines
- Remove: METADATA AUTOMATION PATTERNS (~12 lines)
- Reduce: COMBINED METADATA UPDATES from 4 to 2 examples (~20 lines saved)
- Condense: BEST PRACTICES section (~10 lines saved)
- Result: ~151 lines
- Target met: YES (goal was 100-120, but metadata is naturally more content-rich)

### Total Calculation
- Module structure + impl: 40 lines
- prompt_state(): 106 lines
- prompt_metadata(): 151 lines
- **Total: 297 lines** (within 280-360 target)

---

## Execution Checklist

1. **[ ] Delete prompt_content() function entirely**
   - Remove lines 402-591 (190 lines)

2. **[ ] Delete prompt_workflows() function entirely**
   - Remove lines 594-888 (295 lines)

3. **[ ] Delete prompt_comprehensive() function entirely**
   - Remove lines 891-1234 (344 lines)

4. **[ ] Trim prompt_state() COMMON STATE MANAGEMENT PATTERNS**
   - Remove the "Bug fix workflow" example
   - Remove the "Won't fix closure" example
   - Remove the "Duplicate closure" example
   - Remove the "Stale issue cleanup" example
   - Keep all other sections (CHANGING ISSUE STATE, CLOSING, REOPENING, BEST PRACTICES, etc.)

5. **[ ] Trim prompt_metadata() automation section**
   - Remove METADATA AUTOMATION PATTERNS section completely
   - Reduce COMBINED METADATA UPDATES from 4 examples to 2 (keep triage and escalate)

6. **[ ] Update generate_prompts() match statement**
   - Change to: `match args.scenario.as_deref() { Some("metadata") => prompt_metadata(), _ => prompt_state(), }`
   - Remove branches for "content", "workflows", and comprehensive default

7. **[ ] Update prompt_arguments() function**
   - Change description from "Scenario to show (state, metadata, content, workflows)" to "Scenario to show (state, metadata)"
   - Keep structure identical, only update the string

8. **[ ] Verify compilation**
   - Run `cargo check` in packages/kodegen-mcp-schema/
   - Verify no syntax errors
   - Verify all braces match

9. **[ ] Count final lines**
   - Should be approximately 297 lines total
   - Within target range of 280-360 lines

---

## Notes

- The goal is NOT to remove important content from the remaining scenarios, but to eliminate duplication and redundant pattern examples
- Both `prompt_state()` and `prompt_metadata()` are comprehensive enough to guide AI agents in using the tool effectively
- The deletion of `prompt_content()` is acceptable because title/body updates are covered in the state and metadata scenarios
- The deletion of `prompt_workflows()` is acceptable because common workflows are now omitted in favor of core functionality
- Default behavior should show `prompt_state()` when no scenario is specified
