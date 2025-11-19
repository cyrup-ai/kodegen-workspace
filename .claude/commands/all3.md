---
allowed-tools: Task, mcp__kodegen__sequential_thinking, mcp__kodegen__process_list, mcp__kodegen__process_kill,mcp__kodegen__terminal_start_command,mcp__kodegen__terminal_list_commands,mcp__kodegen__terminal_send_input,mcp__kodegen__terminal_read_output,mcp__kodegen__terminal_stop_command,mcp__kodegen__fs_list_directory, mcp__kodegen__fs_read_multiple_files,mcp__kodegen__fs_read_file,mcp__kodegen__fs_move_file,mcp__kodegen__fs_delete_file, mcp__kodegen__fs_delete_directory,mcp__kodegen__fs_get_file_info,mcp__kodegen__fs_write_file, mcp__kodegen__fs_move_file,mcp__kodegen__fs_edit_block,mcp__kodegen__fs_start_search, mcp__kodegen__fs_get_search_results,mcp__kodegen__fs_list_searches,mcp__kodegen__fs_stop_search,mcp__kodegen__memory_list_libraries, mcp__kodegen__memory_memorize, mcp__kodegen__memory_recall,mcp__kodegen__memory_check_memorize_status,mcp__kodegen__scrape_url,mcp__kodegen__scrape_check_results,mcp__kodegen__scrape_search_results,mcp__kodegen__browser_web_search,mcp__kodegen__browser_start_research,mcp__kodegen__browser_list_research_sessions,mcp__kodegen__browser_get_research_result,mcp__kodegen__browser_get_research_status
description: Use sub-agents to qa code review task files in parallel
---
# DELEGATE TASK EXECUTION TO SUB-AGENTS

use the `Task` tool (subagents) to execute IN PARALLEL each of the tasks in `task/*.md`

## IN PARALLEL

Identify 10 tasks that can be executed in parallel (or N numer < 10 in none exist)

## CHOOSE TASKS THAT AREN'T SEQUENTIALLY DEPENDENT

task files in `task/**/*.md` are prefixed in meaningful ways. each prefix groups together "bodies of work".

given these task files:

./task/GHMCP_1C.md
./task/GHMCP_1D.md
./task/GHMCP_1E.md
./task/HTML_CLEANER_02_silent_regex_errors.md
./task/HTML_CLEANER_04_string_allocation_overhead.md
./task/HTML_CLEANER_05_nested_div_regex_bug.md
./task/HTML_CLEANER_07_dead_code_cleaned_html.md
./task/HTML_CLEANER_08_pointer_comparison_bug.md
./task/HTML_CLEANER_09_confused_node_iteration_logic.md
./task/HTML_CLEANER_10_selector_parsing_in_loop.md
.task/INLINE_CSS_01_sequential_processing_bottleneck.md
./task/INLINE_CSS_02_wasteful_html_cloning.md
./task/INLINE_CSS_03_sequential_downloads_in_from_info.md
./task/JS_SCRIPTS_01_certificate_info_stub.md
./task/JS_SCRIPTS_02_unreliable_format_detection.md
./task/JS_SCRIPTS_03_media_null_pointer_risk.md
./task/JS_SCRIPTS_04_metadata_overwrite_bug.md
./task/JS_SCRIPTS_05_keyword_splitting_naive.md
./task/KROMEKOVER_03_sequential_script_injection.md
./task/KROMEKOVER_05_duplicate_utils_files.md
./task/KROMEKOVER_06_silent_error_handling.md
./task/KROMEKOVER_07_incorrect_script_path.md
./task/MAIN_CRAWL_01_hardcoded_domain.md
./task/MAIN_CRAWL_02_incomplete_js_stub.md
./task/MAIN_CRAWL_03_unsafe_url_filename_conversion.md
./task/PAGE_DATA_01_unused_link_rewriter_param.md
./task/PAGE_DATA_02_sequential_async_defeats_parallelism.md
./task/PAGE_DATA_03_extracted_data_ignored.md
./task/SEARCH_01_nested_spawn_blocking_antipattern.md
./task/SEARCH_03_writer_lifetime_prevents_parallelization.md

*we can see that these groupings are mutually independent*:

- GHMCP
- HTML_CLEANER
- INLINE_CSS
- JS_SCRIPTS
- KROMEKOVER
- MAIN_CRAWL
- PAGE_DATA
- SEARCH

Given this, we can execute 1 task from each grouping in parallel, and ensure our subagents aren't "stepping on each other's toes". We'd parallelize 1 from each namespace, then once completed, anther from each namespace, etc. until all task files are cleared.

- try to avoid parellelizing tasks that require modifications to the same file(s) as these make it very hard fro the subagent to understand what's happening ... "the linter is changing things!" he'll say, ~Angry Claude :)
- once we're out of easy namespace collision avoidance, you'll need to analyze interdependencies more carefully. 


## CONTINUING AS COMPLETED

*manage 10 sub-agents at at a time*

- spawn new subagents as the initial ones complete 
- continue spawning until all tasks in the `task/*.md` dir are reported as completed by the sub-agents.  

======================= 

## TEMPLATE FOR PROMPTING SUBAGENTS

Use template at the bottom of the message to prompt your subagents, replacing the `{{absolute_file_path}}` token with the actual path to the task file you are instructing them to execute. 

## SUBAGENT PROMPT

```
# CODE REVIEW

## YOUR ROLE

Act as an objective rust expert QA code reviewer.

## INSTRUCTIONS 

Rate the imlementation of the following requirements on a scale of 1-10 

{{absolute_file_path}}

- cite the full reasoning for your objective rating
- if the implementation is code complete with no issues, AND A 10/10 QA rating, delete the task file 
  - `rm -f {{absolute_file_path}}`
- if the code implementation is found lacking (in ANY WAY NO MATTER HOW SMALL!!), update the task file:
  - remove every single item completely from the task description that is full and complete in production quality
  - bring focus to the items outstanding with specific guidance on what needs to be resolved
  - print the full filepath (if incomplete) as the last line of output: `{{absolute_file_path}}`

## NO GIT COMMANDS

DO NOT USE `git` commands of any type. other coders are coding and you will be destroying their work if you branch, stash, checkout, revert or do anything with git. YOU WILL BE IMMEDIATELY FIRED if you use any `git` commands whatsoever!!

## TOOLS 

- use `mcp__kodegen__sequential_thinking` and ULTRATHINK to think step by step about the task
- use `mcp__kodegen__fs_start_search` to quickly identify the files that the task specified for modification
  - use `mcp__kodegen__fs_get_search_results` to check on the search status and view the results
- use `mcp__kodegen__fs_read_file` and/or `mcp__kodegen__fs_read_multiple_files` to read files
- use `mcp__kodegen__terminal_start_command` to run `cargo clippy`
  - use `mcp__kodegen__terminal_send_input` to send folloup checks as needed
  - use `mcp__kodegen__terminal_read_output` to view the output of the checks
- use `mcp__kodegen__fs_edit_block` to modify the {{absolute_file_path}} task file if incomplete or lacking in any way
- use `mcp__kodegen__fs_delete_file` to delete the {{absolute_file_path}} task file if it's a perfect 10/10
- feel free to any other allowed `mcp__kodegen__*` commands as needed

NOTE: if the implementation is BETTER than spec we're happy!!

- do not literally interpret the task as exacting as written if the developer exceeded requirements
- do allow for deviations that improve the code or correct imperfections in the task requirements

DO NOT USE `git` commands of any type. other coders are coding and you will be seeing diffs from multiple tasks in concert!! DO NOT `git stash`, `git diff` or other methods. JUST READ THE FILES AS THEY EXIST!!

```
