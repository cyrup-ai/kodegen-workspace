# CRITICAL: Shell Executor Must Kill Processes on Timeout

## Priority: CRITICAL
## Category: Process Management / Resource Leak
## File: `packages/kodegend/src/security/shell_executor.rs`

---

## Problem Statement

The timeout mechanism in `ShellExecutor::execute()` **does not kill processes** when timeout occurs. The implementation correctly uses `tokio::process::Command` for async I/O, but uses `wait_with_output()` which **consumes the child**, making `child.kill()` impossible.

**Result:** Zombie processes accumulate, consuming system resources indefinitely.

---

## Current Broken Implementation

**File:** [`packages/kodegend/src/security/shell_executor.rs`](../packages/kodegend/src/security/shell_executor.rs)  
**Lines:** 121-142

```rust
// ❌ BROKEN: wait_with_output() consumes child
let output_future = child.wait_with_output();
let output = match timeout(self.timeout_duration, output_future).await {
    Ok(Ok(output)) => output,
    Ok(Err(e)) => { /* ... */ }
    Err(_) => {
        // ❌ Child was consumed - cannot call child.kill()!
        // Comment admits: "We can't kill it anymore, but the OS will clean it up"
        return ShellExecuteResponse {
            stderr: "Command execution timeout (30s)".to_string(),
            exit_code: Some(124),
            is_error: true,
            stdout: String::new(),
        };
    }
};
```

### Evidence of Failure

```bash
$ grep -n "kill" packages/kodegend/src/security/shell_executor.rs
136:    // We can't kill it anymore, but the OS will clean it up
```

**Finding:** NO `child.kill().await` call exists. Only a comment admitting it's impossible.

```rust
#[allow(unused_mut)]  // Line 110
let mut child = match child {
```

**Finding:** `#[allow(unused_mut)]` proves `mut child` is never mutated (no `kill()` call).

---

## Root Cause Analysis

### Why wait_with_output() Fails

```rust
pub async fn wait_with_output(self) -> io::Result<Output>
//                              ^^^^ 
//                              Takes ownership (consumes self)
```

**Problem:**
1. `wait_with_output()` takes `self` (moves Child out of scope)
2. Child is moved into the future
3. When timeout occurs, child no longer exists in scope
4. Cannot call `child.kill()` on a consumed value

**Impact:**
- Processes continue running after timeout
- Resources never freed
- Zombie process accumulation
- Daemon instability under load

---

## Solution: Use Established Pattern from Codebase

### Reference Implementation

**Source:** [`packages/kodegen-tools-git/src/operations/push/core.rs:114-145`](../packages/kodegen-tools-git/src/operations/push/core.rs#L114-L145)

The git push operation uses the **exact pattern needed** for timeout with process control:

```rust
// Spawn child process with handle for proper cancellation
let mut child = cmd.spawn().map_err(GitError::Io)?;

// Wait with timeout and cancellation support using select!
let status = tokio::select! {
    result = child.wait() => {
        result.map_err(GitError::Io)?
    }
    () = tokio::time::sleep(timeout_duration) => {
        // Timeout - kill the child process
        let _ = child.kill().await;
        return Err(GitError::InvalidInput(format!(
            "Push operation timed out after {} seconds", 
            timeout_secs.unwrap_or(300)
        )));
    }
};

// Read stdout and stderr after process completes
use tokio::io::AsyncReadExt;
let mut stdout_data = Vec::new();
let mut stderr_data = Vec::new();

if let Some(mut stdout) = child.stdout.take() {
    let _ = stdout.read_to_end(&mut stdout_data).await;
}
if let Some(mut stderr) = child.stderr.take() {
    let _ = stderr.read_to_end(&mut stderr_data).await;
}

let output = std::process::Output {
    status,
    stdout: stdout_data,
    stderr: stderr_data,
};
```

### Why tokio::select! Works

**Key difference from `timeout()`:**
- `timeout(duration, future)` → Wraps future, may consume its inputs
- `select! { fut1 => {}, fut2 => {} }` → **Races futures without moving their owners**

**Result:**
- `child` remains in scope in both branches
- Timeout branch can call `child.kill().await`
- Process is guaranteed to be killed on timeout

---

## Prescriptive Implementation

### Complete Fixed Code for execute() Method

Replace **lines 103-148** with:

```rust
pub async fn execute(&self, command: &str) -> ShellExecuteResponse {
    // Validate first
    if let Err(e) = self.validate_command(command) {
        return ShellExecuteResponse {
            stdout: String::new(),
            stderr: e,
            exit_code: Some(1),
            is_error: true,
        };
    }

    // Execute with timeout
    let child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut child = match child {
        Ok(c) => c,
        Err(e) => {
            return ShellExecuteResponse {
                stdout: String::new(),
                stderr: format!("Failed to spawn process: {e}"),
                exit_code: Some(1),
                is_error: true,
            };
        }
    };

    // Wait with timeout using select! (allows killing on timeout)
    let status = tokio::select! {
        result = child.wait() => {
            match result {
                Ok(status) => status,
                Err(e) => {
                    return ShellExecuteResponse {
                        stdout: String::new(),
                        stderr: format!("Process execution failed: {e}"),
                        exit_code: Some(1),
                        is_error: true,
                    };
                }
            }
        }
        _ = tokio::time::sleep(self.timeout_duration) => {
            // Timeout - kill the child process
            let _ = child.kill().await;
            return ShellExecuteResponse {
                stdout: String::new(),
                stderr: "Command execution timeout (30s)".to_string(),
                exit_code: Some(124),
                is_error: true,
            };
        }
    };

    // Read stdout and stderr after process completes
    use tokio::io::AsyncReadExt;
    let mut stdout_data = Vec::new();
    let mut stderr_data = Vec::new();

    if let Some(mut stdout) = child.stdout.take() {
        let _ = stdout.read_to_end(&mut stdout_data).await;
    }
    if let Some(mut stderr) = child.stderr.take() {
        let _ = stderr.read_to_end(&mut stderr_data).await;
    }

    ShellExecuteResponse {
        stdout: String::from_utf8_lossy(&stdout_data).to_string(),
        stderr: String::from_utf8_lossy(&stderr_data).to_string(),
        exit_code: status.code(),
        is_error: !status.success(),
    }
}
```

### Key Changes Explained

| Change | Why | Impact |
|--------|-----|--------|
| Remove `#[allow(unused_mut)]` | `mut` is now actually used by `kill()` | No more warning suppression |
| Use `child.wait()` instead of `wait_with_output()` | Doesn't consume child | Can call `kill()` on timeout |
| Use `tokio::select!` instead of `timeout()` | Keeps child in scope | Timeout branch has access to child |
| Add `child.kill().await` in timeout branch | Actually kills process | No zombie processes |
| Manually read stdout/stderr | Wait doesn't return output | Same functionality as before |
| Add `use tokio::io::AsyncReadExt` | For `read_to_end()` | Imports the async read trait |

---

## Line-by-Line Changes

### 1. Remove Unused Mut Suppression

**Line 110:** Delete this entire line:
```rust
#[allow(unused_mut)]  // ❌ DELETE
```

### 2. Remove Incorrect Variable and Comment

**Lines 121-137:** Replace with select! block:

**DELETE:**
```rust
let output_future = child.wait_with_output();
let output = match timeout(self.timeout_duration, output_future).await {
    Ok(Ok(output)) => output,
    Ok(Err(e)) => {
        return ShellExecuteResponse {
            stdout: String::new(),
            stderr: format!("Process execution failed: {e}"),
            exit_code: Some(1),
            is_error: true,
        };
    }
    Err(_) => {
        // Timeout occurred - child was consumed by wait_with_output() future
        // We can't kill it anymore, but the OS will clean it up
        return ShellExecuteResponse { /* ... */ };
    }
};
```

**REPLACE WITH:**
```rust
// Wait with timeout using select! (allows killing on timeout)
let status = tokio::select! {
    result = child.wait() => {
        match result {
            Ok(status) => status,
            Err(e) => {
                return ShellExecuteResponse {
                    stdout: String::new(),
                    stderr: format!("Process execution failed: {e}"),
                    exit_code: Some(1),
                    is_error: true,
                };
            }
        }
    }
    _ = tokio::time::sleep(self.timeout_duration) => {
        // Timeout - kill the child process
        let _ = child.kill().await;
        return ShellExecuteResponse {
            stdout: String::new(),
            stderr: "Command execution timeout (30s)".to_string(),
            exit_code: Some(124),
            is_error: true,
        };
    }
};

// Read stdout and stderr after process completes
use tokio::io::AsyncReadExt;
let mut stdout_data = Vec::new();
let mut stderr_data = Vec::new();

if let Some(mut stdout) = child.stdout.take() {
    let _ = stdout.read_to_end(&mut stdout_data).await;
}
if let Some(mut stderr) = child.stderr.take() {
    let _ = stderr.read_to_end(&mut stderr_data).await;
}
```

### 3. Update Response Construction

**Lines 140-145:** Change from using `output` to using separate data:

**CHANGE FROM:**
```rust
ShellExecuteResponse {
    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    exit_code: output.status.code(),
    is_error: !output.status.success(),
}
```

**CHANGE TO:**
```rust
ShellExecuteResponse {
    stdout: String::from_utf8_lossy(&stdout_data).to_string(),
    stderr: String::from_utf8_lossy(&stderr_data).to_string(),
    exit_code: status.code(),
    is_error: !status.success(),
}
```

---

## Import Requirements

**Current imports (lines 1-9):** No changes needed - all required imports already present:

```rust
use tokio::process::Command;      // ✅ Already present
use std::process::Stdio;          // ✅ Already present  
use tokio::time::timeout;         // ⚠️  No longer used, but harmless to keep
```

**New import needed (add inside execute() method):**
```rust
use tokio::io::AsyncReadExt;  // For read_to_end() on stdout/stderr
```

Note: The `use tokio::time::timeout` import is no longer used but can remain (doesn't hurt).

---

## Verification Commands

After implementation:

```bash
# 1. Verify child.kill().await is present
cd packages/kodegend
grep -n "child.kill().await" src/security/shell_executor.rs
# Expected: Line number with the kill call (around line 132)

# 2. Verify no unused_mut warning suppression
grep "#\[allow(unused_mut)\]" src/security/shell_executor.rs
# Expected: (empty - no matches)

# 3. Verify tokio::select! is used
grep -n "tokio::select!" src/security/shell_executor.rs
# Expected: Line number with select! (around line 124)

# 4. Verify no wait_with_output
grep "wait_with_output" src/security/shell_executor.rs
# Expected: (empty - no matches)

# 5. Compile check
cargo check 2>&1 | grep -i "shell_executor"
# Expected: No errors

# 6. Clippy check
cargo clippy --all-targets 2>&1 | grep -A 2 "shell_executor"
# Expected: No warnings
```

---

## Definition of Done

- [ ] Line 110: `#[allow(unused_mut)]` removed
- [ ] Lines 121-137: Replaced with `tokio::select!` implementation
- [ ] `child.kill().await` present in timeout branch (around line 132)
- [ ] `child.wait()` used instead of `wait_with_output()`
- [ ] Manual stdout/stderr reading implemented
- [ ] `use tokio::io::AsyncReadExt` added in method
- [ ] Lines 140-145: Updated to use `stdout_data`/`stderr_data`/`status`
- [ ] `cargo check` passes
- [ ] `cargo clippy` shows no warnings
- [ ] `grep "child.kill().await"` finds the call
- [ ] `grep "wait_with_output"` returns empty
- [ ] No zombie processes accumulate during execution

---

## Codebase References

### Established Patterns (Already in Production)

1. **Git Push - Timeout with Process Kill**  
   [`packages/kodegen-tools-git/src/operations/push/core.rs:114-145`](../packages/kodegen-tools-git/src/operations/push/core.rs#L114-L145)  
   ✅ Shows exact pattern: `select!` + `child.wait()` + `child.kill().await`

2. **Git Push Delete - Select with Timeout**  
   [`packages/kodegen-tools-git/src/operations/push/delete.rs:78`](../packages/kodegen-tools-git/src/operations/push/delete.rs#L78)  
   ✅ Another example of `select!` for timeout handling

3. **Git Push Check - Process Control**  
   [`packages/kodegen-tools-git/src/operations/push/check.rs:83`](../packages/kodegen-tools-git/src/operations/push/check.rs#L83)  
   ✅ Third example in git operations

### Pattern Statistics

```bash
$ grep -r "tokio::select!" packages/ | wc -l
33
```

**Finding:** `tokio::select!` is used **33 times** across the codebase - this is a well-established pattern.

---

## Technical Deep Dive

### tokio::select! Macro Internals

```rust
tokio::select! {
    <pattern> = <async expression> => <handler>,
    <pattern> = <async expression> => <handler>,
    ...
}
```

**How it works:**
1. Polls all futures concurrently
2. When first future completes, executes its handler
3. **Drops remaining futures** (important for cleanup)
4. **Does not move the polled values** (unlike `wait_with_output()`)

**Why this solves our problem:**
```rust
let mut child = spawn_child();

tokio::select! {
    result = child.wait() => {
        // child is still in scope here ✅
    }
    _ = sleep(timeout) => {
        // child is still in scope here ✅
        child.kill().await;  // This works!
    }
}
```

### Comparison with timeout()

**Using timeout() (broken):**
```rust
let future = child.wait_with_output();  // Moves child into future
timeout(duration, future).await;        // child consumed
// Cannot access child here ❌
```

**Using select! (works):**
```rust
let mut child = spawn();
select! {
    _ = child.wait() => { /* child still available */ }
    _ = sleep() => { child.kill().await; }  // Works! ✅
}
```

---

## Performance Considerations

### Memory Impact

**Before (wait_with_output):**
- Allocates buffers inside wait_with_output()
- No control over buffer allocation

**After (manual reading):**
- Explicit buffer allocation (Vec::new())
- Same memory usage
- No performance impact

### CPU Impact

**Before:**
- Single async operation (wait_with_output)

**After:**
- Three async operations (wait + 2x read_to_end)
- Negligible overhead (~microseconds)
- All operations are I/O-bound, not CPU-bound

### Latency Impact

**Before:** 
- Blocking wait for all I/O in single call

**After:**
- Separate wait then reads
- Total latency: **identical** (same syscalls)

**Conclusion:** No measurable performance impact. Code correctness improved significantly.

---

## Error Handling Improvements

### Zombie Process Prevention

**Before:**
```rust
Err(_) => {
    // ❌ Process keeps running!
    return timeout_error();
}
```

**After:**
```rust
_ = sleep(timeout) => {
    let _ = child.kill().await;  // ✅ Process killed
    return timeout_error();
}
```

### Graceful Degradation

```rust
if let Some(mut stdout) = child.stdout.take() {
    let _ = stdout.read_to_end(&mut stdout_data).await;
}
```

**Why use `let _`:**
- If stdout read fails, continue anyway
- stderr may still have useful error info
- Exit code is the source of truth
- Matches behavior of `wait_with_output()`

---

## Security Considerations

### Process Isolation

The security validation layer **remains unchanged**:

```rust
fn validate_command(&self, cmd: &str) -> Result<(), String> {
    // ✅ All validation still applies
    // - Blocked patterns (rm -rf, fork bombs)
    // - Command injection prevention  
    // - Whitelist enforcement
}
```

**Key point:** This fix only changes the **execution layer**, not the **security layer**.

### Resource Limits

**Before:** 
- 30s timeout returns error, but **process keeps running**
- No actual resource limit enforcement

**After:**
- 30s timeout **kills process**
- Actual resource limit enforcement
- DOS prevention improved

---

## Alternative Approaches Considered (and Rejected)

### ❌ Option 1: Save PID and use nix::kill

```rust
let pid = child.id();
// ... timeout ...
kill(Pid::from_raw(pid as i32), Signal::SIGKILL);
```

**Why rejected:**
- Platform-specific (requires nix on Unix)
- More complex error handling
- tokio::Child::kill() is the canonical approach

### ❌ Option 2: Use spawn_blocking with std::process

```rust
tokio::task::spawn_blocking(move || {
    std::process::Command::new("sh").output()
})
```

**Why rejected:**
- Defeats purpose of using tokio::process
- No access to tokio runtime from blocking thread
- Blocks thread pool worker

### ✅ Option 3: tokio::select! with child.wait()

**Why selected:**
- Established pattern in codebase (33 uses)
- Clean, idiomatic Rust async code
- Maintains access to Child for killing
- Platform-independent (tokio abstracts OS differences)

---

## File Structure

**Single file to modify:** `packages/kodegend/src/security/shell_executor.rs`

**No other changes needed:**
- No Cargo.toml changes (tokio already has process feature)
- No import changes at file level (add one use inside method)
- No other files modified

---

## Summary

**Core Issue:** `wait_with_output()` consumes child → cannot kill on timeout

**Solution:** Use `tokio::select!` with `child.wait()` → child remains in scope

**Pattern Source:** Already used in `kodegen-tools-git/src/operations/push/core.rs`

**Impact:** Fixes zombie process accumulation (critical daemon stability issue)

**Complexity:** Low - well-established pattern with 33 existing uses in codebase

**Risk:** Minimal - same syscalls, same memory usage, same behavior (except now processes actually get killed)
