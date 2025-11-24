# Performance Issue: Sequential Server Shutdown During Daemon Stop

## Severity
**CRITICAL** - Daemon shutdown can take 7+ minutes causing users to force-kill the process

## Core Objective
Convert sequential embedded server shutdown to parallel execution, reducing worst-case shutdown time from 450 seconds (7.5 minutes) to 5-10 seconds maximum.

## Location
- **Primary file**: [`packages/kodegend/src/service/embedded_servers.rs`](../packages/kodegend/src/service/embedded_servers.rs)
  - Lines 150-176: `shutdown_all_servers()` - **MUST FIX**
  - Lines 132-147: `rollback_servers()` - **MUST FIX**
- **Called from**: [`packages/kodegend/src/manager.rs`](../packages/kodegend/src/manager.rs)
  - Line 313: Critical daemon shutdown path (SIGTERM/SIGINT handler)

## Problem Analysis

### Current Sequential Implementation

The `shutdown_all_servers()` function iterates servers sequentially with a 30-second timeout per server:

```rust
// Current implementation - SEQUENTIAL (SLOW)
pub async fn shutdown_all_servers(servers: Vec<EmbeddedServer>) -> Result<()> {
    let count = servers.len();
    log::info!("Shutting down {} embedded servers", count);
    
    let timeout = Duration::from_secs(30);  // 30 seconds PER SERVER!
    let mut errors = Vec::new();
    
    for server in servers {  // SEQUENTIAL LOOP - BLOCKS ON EACH SERVER
        let server_name = server.name.clone();
        if let Err(e) = server.shutdown(timeout).await {
            let msg = format!("{} shutdown error: {}", server_name, e);
            log::error!("{}", msg);
            errors.push(msg);
        }
    }
    
    // Error handling...
}
```

**Impact**: With 15 configured servers (typical production setup):
- **Best case**: ~150ms (all servers shutdown instantly)
- **Worst case**: 15 servers × 30s = **450 seconds = 7.5 minutes**
- **Typical case**: 30-60 seconds (some slow responses)

### The Same Issue in Rollback

The `rollback_servers()` function has identical sequential pattern (lines 132-147):

```rust
async fn rollback_servers(servers: Vec<EmbeddedServer>) {
    let count = servers.len();
    log::warn!("Rolling back {} previously started servers", count);
    
    let timeout = Duration::from_secs(10);
    
    for server in servers {  // ALSO SEQUENTIAL
        let server_name = server.name.clone();
        log::info!("Rolling back {} server", server_name);
        if let Err(e) = server.shutdown(timeout).await {
            log::error!("Failed to rollback {}: {}", server_name, e);
        }
    }
}
```

### User Experience Impact

**Scenario**: User runs `systemctl stop kodegend` or `launchctl stop kodegend`

1. Manager receives SIGTERM signal ([`manager.rs:302`](../packages/kodegend/src/manager.rs))
2. Calls `shutdown_all_servers()` ([`manager.rs:313`](../packages/kodegend/src/manager.rs))
3. Terminal/systemd **hangs for 30+ seconds**
4. User gets impatient and sends SIGKILL
5. Servers are force-killed mid-shutdown → **potential data corruption**
6. Next startup finds stale ports/resources → **startup failures**

### Root Cause

Sequential iteration instead of parallel async execution. Each `server.shutdown(timeout).await` blocks the next server from starting its shutdown sequence.

## Understanding the ServerHandle API

The [`EmbeddedServer::shutdown()`](../packages/kodegend/src/service/embedded_servers.rs) method (lines 19-36) already supports async graceful shutdown:

```rust
pub async fn shutdown(self, timeout: Duration) -> Result<()> {
    log::info!("Shutting down {} server", self.name);
    
    // Trigger graceful shutdown
    self.server_handle.cancel();
    
    // Wait for completion with timeout
    match self.server_handle.wait_for_completion(timeout).await {
        Ok(()) => {
            log::info!("{} server shutdown successfully", self.name);
            Ok(())
        }
        Err(e) => {
            log::error!("{} server shutdown error: {}", self.name, e);
            Err(anyhow::anyhow!("{} shutdown failed: {}", self.name, e))
        }
    }
}
```

**Key insight**: This method is **already async** and **already handles timeouts**. It's perfectly designed for parallel execution - we just need to run multiple instances concurrently instead of sequentially.

The underlying [`ServerHandle`](../packages/kodegen-server-http/src/server.rs) API (lines 593-638):
- `cancel()`: Signals shutdown via `CancellationToken` (non-blocking, lock-free)
- `wait_for_completion(timeout)`: Waits for shutdown with timeout
- Zero-allocation design using atomic operations

## Dependencies Already Available

From [`packages/kodegend/Cargo.toml`](../packages/kodegend/Cargo.toml):
- Line 118: `futures = "0.3"` ✓ **Already present**
- Line 115: `futures-util = { version = "0.3" }` ✓ **Already present**
- Line 90-99: `tokio` with full features ✓ **Already present**

**No new dependencies required!** We can use `futures::future::join_all` immediately.

## Solution: Parallel Shutdown with Error Collection

### Implementation Pattern

Replace sequential loops with `futures::future::join_all()`:

```rust
use futures::future::join_all;

pub async fn shutdown_all_servers(servers: Vec<EmbeddedServer>) -> Result<()> {
    let count = servers.len();
    log::info!("Shutting down {} embedded servers in parallel", count);
    
    // Reduce timeout - healthy servers should shutdown in <1s
    let timeout = Duration::from_secs(5);
    
    // Create shutdown futures for ALL servers upfront
    let shutdown_futures = servers.into_iter().map(|server| {
        let server_name = server.name.clone();
        async move {
            match server.shutdown(timeout).await {
                Ok(_) => {
                    log::debug!("{} shutdown successful", server_name);
                    Ok(())
                }
                Err(e) => {
                    let msg = format!("{} shutdown error: {}", server_name, e);
                    log::error!("{}", msg);
                    Err(msg)
                }
            }
        }
    });
    
    // Execute ALL shutdowns CONCURRENTLY
    let results = join_all(shutdown_futures).await;
    
    // Collect errors from all results
    let errors: Vec<_> = results.into_iter()
        .filter_map(|r| r.err())
        .collect();
    
    if !errors.is_empty() {
        return Err(anyhow::anyhow!(
            "Shutdown completed with {} errors: {}",
            errors.len(),
            errors.join("; ")
        ));
    }
    
    log::info!("All {} servers stopped successfully", count);
    Ok(())
}
```

**Key changes**:
1. Import `futures::future::join_all` at top of file
2. Reduce timeout from 30s → 5s (healthy servers should be instant)
3. Create all shutdown futures upfront (map over servers)
4. Execute all futures concurrently with `join_all()`
5. Collect and aggregate errors after all complete
6. Same error handling semantics, but parallel execution

### Apply Same Pattern to Rollback

```rust
async fn rollback_servers(servers: Vec<EmbeddedServer>) {
    let count = servers.len();
    log::warn!("Rolling back {} previously started servers in parallel", count);
    
    let timeout = Duration::from_secs(10);
    
    // Create rollback futures for all servers
    let rollback_futures = servers.into_iter().map(|server| {
        let server_name = server.name.clone();
        async move {
            log::info!("Rolling back {} server", server_name);
            if let Err(e) = server.shutdown(timeout).await {
                log::error!("Failed to rollback {}: {}", server_name, e);
            }
        }
    });
    
    // Execute all rollbacks concurrently
    join_all(rollback_futures).await;
    
    log::warn!("Rollback complete");
}
```

### Optional: Overall Timeout Wrapper

For additional safety, wrap the parallel shutdown with an overall timeout:

```rust
pub async fn shutdown_all_servers(servers: Vec<EmbeddedServer>) -> Result<()> {
    let overall_timeout = Duration::from_secs(10);
    
    match tokio::time::timeout(overall_timeout, shutdown_all_servers_impl(servers)).await {
        Ok(result) => result,
        Err(_) => {
            log::error!("Overall shutdown timeout exceeded - some servers may not have stopped cleanly");
            Err(anyhow::anyhow!("Shutdown timeout after 10 seconds"))
        }
    }
}

async fn shutdown_all_servers_impl(servers: Vec<EmbeddedServer>) -> Result<()> {
    // ... parallel implementation from above ...
}
```

This ensures that even if individual timeouts fail, the daemon won't hang for more than 10 seconds total.

## Implementation Checklist

### Required Changes in `packages/kodegend/src/service/embedded_servers.rs`

1. **Add import** at top of file (after line 5):
   ```rust
   use futures::future::join_all;
   ```

2. **Replace `shutdown_all_servers()` function** (lines 150-176):
   - Remove sequential `for` loop
   - Create futures with `servers.into_iter().map(|server| async move { ... })`
   - Execute with `join_all(shutdown_futures).await`
   - Reduce timeout from 30s to 5s
   - Keep same error collection and return semantics

3. **Replace `rollback_servers()` function** (lines 132-147):
   - Apply same parallel pattern
   - Keep 10s timeout (rollback is less time-critical)
   - Execute with `join_all(rollback_futures).await`

4. **Optional but recommended**: Add overall timeout wrapper
   - Create `shutdown_all_servers_impl()` with parallel logic
   - Wrap in `tokio::time::timeout(Duration::from_secs(10), ...)`

### Why This Solution Works

1. **No API changes**: `EmbeddedServer::shutdown()` already async and returns `Result`
2. **No new dependencies**: `futures` crate already in `Cargo.toml`
3. **Same error semantics**: Still collects all errors and returns aggregated error
4. **Lock-free**: ServerHandle uses `CancellationToken` (atomic, no locks)
5. **Proven pattern**: `join_all` is standard Rust async pattern for parallel execution

## Performance Improvement

| Scenario | Before (Sequential) | After (Parallel) | Speedup |
|----------|---------------------|------------------|---------|
| **Best case** (all instant) | ~150ms | ~150ms | 1x |
| **Typical case** (some slow) | 30-60s | ~5s | 6-12x |
| **Worst case** (all timeout) | 450s (7.5 min) | 5-10s | 45-90x |

**Production impact**: Users can stop the daemon in seconds instead of minutes, eliminating force-kill scenarios and data corruption risk.

## Definition of Done

The task is complete when:

1. `shutdown_all_servers()` executes all server shutdowns in parallel using `join_all`
2. `rollback_servers()` executes all server shutdowns in parallel using `join_all`
3. Individual server timeout reduced to 5 seconds (from 30 seconds)
4. Overall shutdown completes in ~5-10 seconds with 15 servers (not 450 seconds)
5. Error collection still works - all shutdown errors are aggregated and returned
6. No new dependencies added - uses existing `futures` crate
7. Daemon shutdown responds quickly to SIGTERM/SIGINT signals

## References

### Source Files
- [`packages/kodegend/src/service/embedded_servers.rs`](../packages/kodegend/src/service/embedded_servers.rs) - Main implementation file
- [`packages/kodegend/src/manager.rs`](../packages/kodegend/src/manager.rs) - Daemon manager (calls shutdown)
- [`packages/kodegen-server-http/src/server.rs`](../packages/kodegen-server-http/src/server.rs) - ServerHandle implementation

### Dependencies
- [`packages/kodegend/Cargo.toml`](../packages/kodegend/Cargo.toml) - Dependency declarations

### External Documentation
- [futures::future::join_all](https://docs.rs/futures/latest/futures/future/fn.join_all.html) - Rust async parallel execution
- [tokio::time::timeout](https://docs.rs/tokio/latest/tokio/time/fn.timeout.html) - Overall timeout wrapper
- [tokio_util::sync::CancellationToken](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html) - Lock-free cancellation used by ServerHandle

## Implementation Notes

### Why 30 Seconds Was Wrong

The original 30-second timeout suggests expecting servers to hang regularly. **This is wrong**:
- Healthy HTTP servers shutdown in <100ms (close connections, flush buffers)
- 30s timeout **masks** underlying problems instead of fixing them
- If servers regularly hang, **fix the hang** - don't work around it with long timeouts

### Why 5 Seconds Is Right

- Gives ample time for graceful shutdown (50x longer than needed)
- Fails fast if server is truly hung
- Prevents user frustration and force-kill scenarios
- With parallel execution, multiple 5s timeouts run concurrently (not sequentially)

### Error Handling Preserved

The parallel implementation maintains identical error semantics:
- Individual errors are logged immediately
- All errors are collected and aggregated
- Returns `Err` with error summary if any server fails
- Successful if all servers shutdown cleanly

### Why join_all Not FuturesUnordered

- `join_all`: Waits for ALL futures to complete (exactly what we need)
- `FuturesUnordered`: Returns results as they complete (harder to collect errors)
- `select_all`: Returns when FIRST completes (wrong semantics for shutdown)

The correct choice is `join_all` for shutdown - we must wait for all servers, not just the first one.
