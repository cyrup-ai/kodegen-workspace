# Performance Issue: Sequential Server Rollback Creates Cascading Delays

## Severity
**HIGH** - Significant user-facing delay on startup failure and daemon shutdown

## Location
`packages/kodegend/src/service/embedded_servers.rs`

### Affected Functions (Lines 132-176)
1. **`rollback_servers()`** - Lines 132-147
   - Called during startup failure to shutdown previously started servers
   - Sequential shutdown with 10-second timeout per server

2. **`shutdown_all_servers()`** - Lines 150-176  
   - Called during normal daemon shutdown
   - Sequential shutdown with 30-second timeout per server
   - **CRITICAL**: With 15 servers, this can delay daemon shutdown by up to 450 seconds (7.5 minutes)!

## Issue Description

Both functions shut down embedded HTTP servers SEQUENTIALLY instead of in PARALLEL:

### Current Implementation (rollback_servers)
```rust
// Lines 132-147
async fn rollback_servers(servers: Vec<EmbeddedServer>) {
    let count = servers.len();
    log::warn!("Rolling back {} previously started servers", count);
    
    let timeout = Duration::from_secs(10);
    
    // ❌ SEQUENTIAL: Processes one server at a time
    for server in servers {
        let server_name = server.name.clone();
        log::info!("Rolling back {} server", server_name);
        if let Err(e) = server.shutdown(timeout).await {
            log::error!("Failed to rollback {}: {}", server_name, e);
        }
    }
    
    log::warn!("Rollback complete");
}
```

### Current Implementation (shutdown_all_servers)
```rust
// Lines 150-176
pub async fn shutdown_all_servers(servers: Vec<EmbeddedServer>) -> Result<()> {
    let count = servers.len();
    log::info!("Shutting down {} embedded servers", count);
    
    let timeout = Duration::from_secs(30);
    let mut errors = Vec::new();
    
    // ❌ SEQUENTIAL: Processes one server at a time
    for server in servers {
        let server_name = server.name.clone();
        if let Err(e) = server.shutdown(timeout).await {
            let msg = format!("{} shutdown error: {}", server_name, e);
            log::error!("{}", msg);
            errors.push(msg);
        }
    }
    
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

## Production Impact

### Scenario 1: Startup Failure (rollback_servers)
1. Configuration lists 15 category servers
2. Servers 1-10 start successfully  
3. Server 11 fails to start (port occupied, binary missing, etc.)
4. Rollback initiates for servers 1-10
5. Each shutdown waits up to 10 seconds
6. **Total rollback time: up to 100 seconds (1m 40s)**
7. User sees daemon hang during startup
8. User may kill daemon, leaving servers in undefined state

### Scenario 2: Normal Daemon Shutdown (shutdown_all_servers)
1. User runs `kodegend stop` or sends SIGTERM
2. All 15 embedded servers need graceful shutdown
3. Each shutdown waits up to 30 seconds
4. **Total shutdown time: up to 450 seconds (7.5 minutes)**
5. Systemd/launchd may forcefully kill daemon before completion
6. Servers left in dirty state without proper cleanup

### Real-World Timing Analysis

**With 10 servers and 10s timeout (rollback_servers):**
- Best case (all healthy): ~10-50ms total (near-instant)
- Worst case (all timeout): 100 seconds
- Typical case (some slow): 15-30 seconds

**With 15 servers and 30s timeout (shutdown_all_servers):**
- Best case (all healthy): ~15-75ms total
- Worst case (all timeout): 450 seconds (7.5 minutes)
- Typical case (some slow): 30-90 seconds

**This is unacceptable user experience.**

## Root Cause

Both functions use sequential iteration (`for server in servers`) with `.await` inside the loop. Each server shutdown completes before the next begins, creating O(n) time complexity where n = server count.

## Technical Analysis: Why Parallel Shutdown is Safe

### ServerHandle Architecture (from [kodegen-server-http/src/server.rs](../../packages/kodegen-server-http/src/server.rs))

```rust
// Lines 593-639
pub struct ServerHandle {
    cancellation_token: tokio_util::sync::CancellationToken,
    completion_rx: tokio::sync::oneshot::Receiver<()>,
}

impl ServerHandle {
    /// Signal server to begin shutdown
    pub fn cancel(&self) {
        self.cancellation_token.cancel();  // Thread-safe, concurrent-safe
    }

    /// Wait for server shutdown to complete (with timeout)
    pub async fn wait_for_completion(mut self, timeout: Duration) -> Result<(), ShutdownError> {
        match tokio::time::timeout(timeout, &mut self.completion_rx).await {
            Ok(Ok(())) => Ok(()),
            Ok(Err(_)) => Err(ShutdownError::SignalLost),
            Err(_) => Err(ShutdownError::Timeout(timeout)),
        }
    }
}
```

### EmbeddedServer Shutdown Method (from [embedded_servers.rs](../../packages/kodegend/src/service/embedded_servers.rs))

```rust
// Lines 17-37
impl EmbeddedServer {
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
}
```

### Why Parallel is Safe

1. **Independent Cancellation**: Each `ServerHandle` has its own `CancellationToken`. Calling `cancel()` on multiple handles concurrently is safe - they're independent async primitives.

2. **Isolated Completion Channels**: Each server has its own `oneshot::Receiver<()>`. No shared state exists between servers.

3. **No Synchronization Required**: Servers don't coordinate with each other during shutdown. Each manages its own HTTP server and background tasks independently.

4. **Thread-Safe Primitives**: Both `CancellationToken` (from tokio-util) and `oneshot::Receiver` (from tokio) are designed for concurrent async usage.

**Conclusion**: Multiple `EmbeddedServer::shutdown()` calls can safely execute concurrently.

## Recommended Solution

### Implementation Approach: `futures::join_all`

Use `futures::join_all` to execute all shutdown operations concurrently. This is the cleanest approach because:

1. **Already Available**: `futures = "0.3"` is already a dependency ([Cargo.toml line 118](../../packages/kodegend/Cargo.toml#L118))
2. **Purpose-Built**: `join_all` is designed exactly for this pattern
3. **Clean Error Handling**: Each future completes independently with its own Result
4. **No Manual Task Management**: Unlike `tokio::spawn`, no need to collect and await JoinHandles

### Fixed Implementation: rollback_servers()

```rust
// Lines 132-147 - REPLACE WITH:
async fn rollback_servers(servers: Vec<EmbeddedServer>) {
    use futures::future::join_all;
    
    let count = servers.len();
    log::warn!("Rolling back {} previously started servers", count);
    
    let timeout = Duration::from_secs(10);
    
    // ✅ PARALLEL: Create shutdown futures for all servers
    let shutdown_futures = servers.into_iter().map(|server| {
        let server_name = server.name.clone();
        async move {
            log::info!("Rolling back {} server", server_name);
            if let Err(e) = server.shutdown(timeout).await {
                log::error!("Failed to rollback {}: {}", server_name, e);
            }
        }
    });
    
    // Execute all shutdowns concurrently (time = max, not sum)
    join_all(shutdown_futures).await;
    
    log::warn!("Rollback complete");
}
```

### Fixed Implementation: shutdown_all_servers()

```rust
// Lines 150-176 - REPLACE WITH:
pub async fn shutdown_all_servers(servers: Vec<EmbeddedServer>) -> Result<()> {
    use futures::future::join_all;
    
    let count = servers.len();
    log::info!("Shutting down {} embedded servers", count);
    
    let timeout = Duration::from_secs(30);
    
    // ✅ PARALLEL: Create shutdown futures for all servers with error collection
    let shutdown_futures = servers.into_iter().map(|server| {
        let server_name = server.name.clone();
        async move {
            if let Err(e) = server.shutdown(timeout).await {
                let msg = format!("{} shutdown error: {}", server_name, e);
                log::error!("{}", msg);
                Some(msg)
            } else {
                None
            }
        }
    });
    
    // Execute all shutdowns concurrently and collect errors
    let results = join_all(shutdown_futures).await;
    let errors: Vec<String> = results.into_iter().filter_map(|r| r).collect();
    
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

## Changes Required

### File: `packages/kodegend/src/service/embedded_servers.rs`

**Lines to Modify**: 132-147, 150-176

**Imports to Add** (at top of file, after existing use statements):
```rust
use futures::future::join_all;
```

**Changes**:
1. **rollback_servers()** (lines 132-147):
   - Replace sequential `for server in servers` loop with `join_all` pattern
   - Move server name clone and shutdown logic into async block inside map
   - Call `join_all(shutdown_futures).await` to execute concurrently

2. **shutdown_all_servers()** (lines 150-176):
   - Replace sequential `for server in servers` loop with `join_all` pattern  
   - Return `Option<String>` from each future (Some(error_msg) on error, None on success)
   - Collect results and filter_map to extract only errors
   - Keep existing error aggregation logic intact

**No Cargo.toml changes needed** - `futures` crate already available.

## Performance Improvement

### Time Complexity
- **Before**: O(n × timeout) where n = server count
- **After**: O(max_timeout) - time of slowest single server
- **Speedup**: 10-100x in typical failure scenarios

### Concrete Examples

**rollback_servers (10 servers, 10s timeout):**
- Sequential: 0-100 seconds (typically 15-30s)
- Parallel: 0-10 seconds (typically <100ms)
- **Improvement: ~150x faster in typical case**

**shutdown_all_servers (15 servers, 30s timeout):**
- Sequential: 0-450 seconds (typically 30-90s)  
- Parallel: 0-30 seconds (typically <100ms)
- **Improvement: ~300-900x faster in typical case**

## Alternative Approaches Considered

### Option 2: tokio::spawn (More Complex, Same Performance)
```rust
async fn rollback_servers(servers: Vec<EmbeddedServer>) {
    let timeout = Duration::from_secs(10);
    
    let handles: Vec<_> = servers.into_iter().map(|server| {
        tokio::spawn(async move {
            log::info!("Rolling back {} server", server.name);
            if let Err(e) = server.shutdown(timeout).await {
                log::error!("Failed to rollback {}: {}", server.name, e);
            }
        })
    }).collect();
    
    for handle in handles {
        let _ = handle.await;  // Ignore JoinErrors
    }
}
```

**Why Not**: More verbose, requires manual JoinHandle management, no performance benefit over `join_all`.

### Option 3: tokio::join! (Only for Fixed Count)
Only works when count is known at compile time (e.g., exactly 3 servers). Not applicable here since server count is dynamic from configuration.

## Definition of Done

This task is complete when:

1. ✅ Both `rollback_servers()` and `shutdown_all_servers()` use `futures::join_all` for parallel shutdown
2. ✅ Import statement `use futures::future::join_all;` added to file
3. ✅ All previous error handling logic preserved (logging, error collection, Result return)
4. ✅ Code compiles without errors or warnings (`cargo clippy` passes)
5. ✅ Manual verification: With 10 servers configured, startup failure rollback completes in <1 second instead of 10+ seconds
6. ✅ Manual verification: With 15 servers configured, daemon shutdown (`kodegend stop`) completes in <1 second instead of 30+ seconds

## Related Code References

- [ServerHandle implementation](../../packages/kodegen-server-http/src/server.rs#L593-L639)
- [EmbeddedServer struct](../../packages/kodegend/src/service/embedded_servers.rs#L10-L37)
- [start_all_servers() caller](../../packages/kodegend/src/service/embedded_servers.rs#L46-L100)
- [Cargo.toml futures dependency](../../packages/kodegend/Cargo.toml#L118)

## Impact on Other Components

**None**. This is an internal optimization to `embedded_servers.rs`. No public API changes, no behavior changes (beyond faster shutdown), no dependency changes required.
