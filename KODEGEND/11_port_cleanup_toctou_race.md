# TOCTOU Race Condition: Port Availability Check

## Severity
**LOW** - Inherent to check-then-use pattern, low probability but worth fixing for robustness

## Location
Primary file: [`packages/kodegend/src/service/port_cleanup.rs`](../packages/kodegend/src/service/port_cleanup.rs)
- Lines 204-208: First availability check (early return if port free)
- Lines 218-221: Second availability check after process lookup failure
- Line 15-18: `check_port_available()` - tests binding but doesn't hold port

Caller: [`packages/kodegend/src/service/embedded_servers.rs`](../packages/kodegend/src/service/embedded_servers.rs)
- Line 68: Calls `cleanup_port_if_needed(config.port).await`
- Line 78: Calls `start_server(...)` which eventually binds to port
- **Race window exists between these two operations**

Server binding: [`packages/kodegen-server-http/src/server.rs`](../packages/kodegen-server-http/src/server.rs)
- Line 176-178: `serve_with_tls()` binds to socket internally

## Issue Description: Classic TOCTOU Race

The current implementation follows a **check-then-use** pattern with shared resources (network ports):

```rust
// Current flow in embedded_servers.rs (line 68-78)
cleanup_port_if_needed(config.port).await?;  // CHECK: port available?
                                              // <- RACE WINDOW HERE
match start_server(...).await {               // USE: bind to port
    Ok(handle) => { /* success */ }
    Err(e) => { /* may fail if port taken during race */ }
}
```

The `cleanup_port_if_needed` function performs two availability checks:

**Check 1: Early return optimization** ([`port_cleanup.rs:204-208`](../packages/kodegend/src/service/port_cleanup.rs#L204-L208))
```rust
// Quick check: is port already free?
if check_port_available(port).await {  // Bind attempt, immediate release
    log::debug!("Port {} is already available", port);
    return Ok(());  // Caller will try to bind later <- RACE!
}
```

**Check 2: After process lookup** ([`port_cleanup.rs:218-221`](../packages/kodegend/src/service/port_cleanup.rs#L218-L221))
```rust
if check_port_available(port).await {  // Bind attempt, immediate release
    log::info!("Port {} became available during lookup", port);
    return Ok(());  // Caller will try to bind later <- RACE!
}
```

Both checks use `check_port_available()` which **binds then immediately releases**:
```rust
pub async fn check_port_available(port: u16) -> bool {
    tokio::net::TcpListener::bind(("127.0.0.1", port))
        .await
        .is_ok()  // Port released here - another process can grab it!
}
```

## Race Condition Scenarios

### Scenario 1: Port Free, Then Stolen
1. `check_port_available(30438)` → binds → **releases** → returns `true`
2. **RACE WINDOW** (microseconds to milliseconds): Another process binds port 30438
3. `cleanup_port_if_needed` returns Ok (thinks port is free)
4. `start_server` attempts bind → **FAILS** with "Address already in use"

### Scenario 2: Port Released During Cleanup, Then Stolen
1. Port occupied by PID 12345
2. `cleanup_port_if_needed` kills process 12345
3. `check_port_available` → binds → **releases** → returns `true`
4. **RACE WINDOW**: Another process binds port
5. `cleanup_port_if_needed` returns Ok
6. `start_server` attempts bind → **FAILS**

## Root Cause Analysis

The fundamental issue: **there is no OS-level atomic "check-and-reserve" operation**. Any check-then-use pattern with shared resources has an inherent race window between:
- **Time of Check (TOC)**: Testing if port is available
- **Time of Use (TOU)**: Actually binding to the port

The OS does not provide a "reserve port if available" syscall - you must bind to claim it.

## Production Impact Assessment

**Probability**: Very low (race window is microseconds on typical systems)
**Severity**: Low (bind failure is handled gracefully with clear error messages)
**Confusion Factor**: Medium (error says "port in use" but cleanup logged success)

## Recommended Solution: Eliminate the Race Window

### Implementation Strategy

The solution is to **hold the port** from cleanup through to server startup by returning the bound `TcpListener` directly. This eliminates the race window entirely.

### Files to Modify

1. **[`packages/kodegend/src/service/port_cleanup.rs`](../packages/kodegend/src/service/port_cleanup.rs)** - Add new function
2. **[`packages/kodegend/src/service/embedded_servers.rs`](../packages/kodegend/src/service/embedded_servers.rs)** - Update caller
3. **[`packages/kodegen-server-http/src/server.rs`](../packages/kodegen-server-http/src/server.rs)** - Add listener-accepting variant

---

## Detailed Implementation Plan

### Step 1: Add `cleanup_and_reserve_port()` to port_cleanup.rs

Add this new function after the existing `cleanup_port_if_needed()` function (around line 243):

```rust
/// Clean up port if needed and immediately bind to reserve it
///
/// This eliminates the TOCTOU race by returning a bound TcpListener,
/// ensuring no other process can claim the port between cleanup and use.
///
/// Steps:
/// 1. Try to bind immediately (port may already be free)
/// 2. If binding fails, perform cleanup (find process, kill, wait)
/// 3. After cleanup, bind immediately and return listener
///
/// Returns bound TcpListener that the caller must pass to server startup
pub async fn cleanup_and_reserve_port(port: u16) -> Result<tokio::net::TcpListener> {
    let addr = ("127.0.0.1", port);
    
    // Try to bind first - port might already be free
    match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            log::debug!("Port {} was already available, reserved successfully", port);
            return Ok(listener);
        }
        Err(_) => {
            log::warn!("Port {} is in use, attempting cleanup", port);
        }
    }
    
    // Port is occupied - find and kill the process
    let pid = match find_process_by_port(port).await? {
        Some(pid) => pid,
        None => {
            return Err(anyhow::anyhow!(
                "Port {} in use but no process found (system command may have failed)",
                port
            ));
        }
    };
    
    log::warn!("Terminating process {} using port {}", pid, port);
    
    // Kill the process gracefully
    kill_process_graceful(pid)
        .await
        .context(format!("Failed to kill process {}", pid))?;
    
    // Wait for port to be released (polls with 100ms interval)
    wait_for_port_release(port, Duration::from_secs(3))
        .await
        .context(format!("Port {} still in use after killing process {}", port, pid))?;
    
    // Immediately bind to reserve the port - this is CRITICAL
    // Minimizes race window to nanoseconds (single syscall)
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context(format!(
            "Port {} still occupied after cleanup (another process may have claimed it)",
            port
        ))?;
    
    log::info!("Successfully freed and reserved port {} (terminated PID {})", port, pid);
    Ok(listener)
}
```

**Keep the existing `cleanup_port_if_needed()` function** for backward compatibility with other callers (if any).

### Step 2: Add `serve_with_listener()` to kodegen-server-http

In [`packages/kodegen-server-http/src/server.rs`](../packages/kodegen-server-http/src/server.rs), add this method to the `HttpServer` impl block (after `serve_with_tls`, around line 448):

```rust
/// Create and serve HTTP server using a pre-bound listener
///
/// This variant accepts a TcpListener that's already bound to an address.
/// Use this to eliminate TOCTOU races when port cleanup is required.
///
/// Returns ServerHandle for graceful shutdown coordination.
/// Spawns background tasks for HTTP/HTTPS server and shutdown monitoring.
///
/// # Arguments
/// * `listener` - Pre-bound TcpListener (port already reserved)
/// * `tls_config` - Optional (cert_path, key_path) for HTTPS
/// * `shutdown_timeout` - Graceful shutdown timeout
pub async fn serve_with_listener(
    self,
    listener: tokio::net::TcpListener,
    tls_config: Option<(PathBuf, PathBuf)>,
    shutdown_timeout: Duration,
) -> Result<ServerHandle>
where
    SM: std::any::Any + 'static,
{
    use tokio::sync::oneshot;
    use tokio_util::sync::CancellationToken;

    let managers = self.managers.clone();
    let protocol = if tls_config.is_some() { "https" } else { "http" };
    
    // Get the address the listener is bound to
    let addr = listener.local_addr()
        .map_err(|e| anyhow::anyhow!("Failed to get listener address: {}", e))?;

    log::info!("Starting HTTP server on {protocol}://{addr} (using pre-bound listener)");

    // Allocate timeout budget (70% HTTP drain, 30% cleanup)
    let http_drain_timeout = shutdown_timeout.mul_f32(0.7);
    let manager_buffer = shutdown_timeout.mul_f32(0.3);
    
    log::info!(
        "Shutdown timeout budget: total={:?}, HTTP drain={:?}, cleanup buffer={:?}",
        shutdown_timeout,
        http_drain_timeout,
        manager_buffer
    );

    // Create completion channel for graceful shutdown signaling
    let (completion_tx, completion_rx) = oneshot::channel();
    let ct = CancellationToken::new();

    // Register session manager for graceful shutdown (LocalSessionManager only)
    let session_manager = self.session_manager.clone();
    let session_manager_any: &dyn std::any::Any = &*session_manager;
    if session_manager_any.downcast_ref::<LocalSessionManager>().is_some() {
        let local_sm: Arc<LocalSessionManager> = unsafe {
            std::mem::transmute(session_manager.clone())
        };
        managers.register(LocalSessionManagerHook {
            session_manager: local_sm,
        }).await;
    }

    // Create service factory closure
    let service_factory = {
        let server = self.clone();
        move || Ok::<_, std::io::Error>(server.clone())
    };

    // Create StreamableHttpService
    let http_service = StreamableHttpService::new(
        service_factory,
        session_manager,
        StreamableHttpServerConfig {
            stateful_mode: true,
            sse_keep_alive: Some(Duration::from_secs(15)),
        },
    );

    // Build Axum router with CORS
    let router = Router::new()
        .nest_service("/mcp", http_service)
        .layer(CorsLayer::permissive());

    // Spawn server with or without TLS
    let server_task = if let Some((cert_path, key_path)) = tls_config {
        log::info!("Loading TLS certificate from: {cert_path:?}");
        
        let rustls_config = build_rustls_config(cert_path, key_path)?;
        let tls_acceptor = TlsAcceptor::from(rustls_config);
        let ct_for_tls = ct.clone();
        let active_requests = self.active_requests.clone();
        
        tokio::spawn(async move {
            loop {
                // Accept TCP connection
                let (tcp_stream, remote_addr) = tokio::select! {
                    _ = ct_for_tls.cancelled() => break,
                    result = listener.accept() => {
                        match result {
                            Ok(conn) => conn,
                            Err(e) => {
                                log::error!("Failed to accept connection: {e}");
                                continue;
                            }
                        }
                    }
                };
                
                // Clone for task
                let tls_acceptor = tls_acceptor.clone();
                let router = router.clone();
                let active_requests = active_requests.clone();
                
                // Spawn connection handler
                tokio::spawn(async move {
                    // TLS handshake
                    let tls_stream = match tls_acceptor.accept(tcp_stream).await {
                        Ok(stream) => stream,
                        Err(e) => {
                            log::error!("TLS handshake failed from {remote_addr}: {e}");
                            return;
                        }
                    };
                    
                    // Convert to hyper-compatible IO
                    let io = TokioIo::new(tls_stream);
                    
                    // Create hyper service from router
                    let tower_service = router.clone();
                    let hyper_service = hyper::service::service_fn(move |request| {
                        tower_service.clone().call(request)
                    });
                    
                    // Track active request
                    let _guard = RequestGuard::new(active_requests.clone());
                    
                    // Serve connection
                    if let Err(e) = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
                        .serve_connection_with_upgrades(io, hyper_service)
                        .await
                    {
                        log::debug!("Connection error from {remote_addr}: {e}");
                    }
                });
            }
        })
    } else {
        // HTTP (no TLS) - use axum::serve directly
        let ct_for_http = ct.clone();
        tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, router)
                .with_graceful_shutdown(async move {
                    ct_for_http.cancelled().await;
                })
                .await
            {
                log::error!("HTTP server error: {e}");
            }
        })
    };

    let ct_clone = ct.clone();
    let active_requests = self.active_requests.clone();

    // Spawn monitor task for graceful shutdown (identical to serve_with_tls)
    // [Copy the monitor task code from serve_with_tls lines 316-444]
    tokio::spawn(async move {
        tokio::pin!(server_task);
        
        let early_exit = tokio::select! {
            _ = ct_clone.cancelled() => {
                log::debug!("Cancellation triggered, initiating graceful shutdown");
                
                let server_shutdown_timeout = http_drain_timeout + Duration::from_secs(5);
                match tokio::time::timeout(server_shutdown_timeout, &mut server_task).await {
                    Ok(Ok(_)) => {
                        log::debug!("HTTP server shutdown complete");
                    }
                    Ok(Err(e)) => {
                        log::error!("HTTP server task panicked during shutdown");
                        log::error!("  JoinError: {:?}", e);
                        if e.is_panic()
                            && let Ok(panic_payload) = e.try_into_panic() {
                            if let Some(msg) = panic_payload.downcast_ref::<&str>() {
                                log::error!("  Panic message: {}", msg);
                            } else if let Some(msg) = panic_payload.downcast_ref::<String>() {
                                log::error!("  Panic message: {}", msg);
                            } else {
                                log::error!("  Panic payload: {:?}", panic_payload);
                            }
                        }
                    }
                    Err(_) => {
                        log::error!(
                            "HTTP server shutdown timeout ({:?}) - server task did not complete. Proceeding with manager shutdown.",
                            server_shutdown_timeout
                        );
                    }
                }
                
                false
            }
            
            result = &mut server_task => {
                log::error!("╔═══════════════════════════════════════════════════════╗");
                log::error!("║  HTTP SERVER TASK EXITED UNEXPECTEDLY                ║");
                log::error!("║  Server terminated before shutdown signal received   ║");
                log::error!("╚═══════════════════════════════════════════════════════╝");
                
                match result {
                    Ok(_) => {
                        log::error!("Server exited normally without cancellation signal");
                        log::error!("This indicates a bug in the server implementation or misconfiguration");
                    }
                    Err(e) => {
                        log::error!("Server task PANICKED");
                        log::error!("  JoinError: {:?}", e);
                        
                        if e.is_panic() {
                            if let Ok(panic_payload) = e.try_into_panic() {
                                if let Some(msg) = panic_payload.downcast_ref::<&str>() {
                                    log::error!("  Panic message: {}", msg);
                                } else if let Some(msg) = panic_payload.downcast_ref::<String>() {
                                    log::error!("  Panic message: {}", msg);
                                } else {
                                    log::error!("  Panic payload type: {:?}", panic_payload.type_id());
                                }
                            }
                        } else if e.is_cancelled() {
                            log::error!("Server task was cancelled (unexpected)");
                        }
                    }
                }
                
                log::error!("Proceeding with emergency cleanup (server already dead)");
                true
            }
        };

        // Wait for all in-flight request handlers to complete
        if early_exit {
            log::warn!("Server panicked - draining in-flight requests before manager cleanup");
        } else {
            log::info!("Draining in-flight request handlers before manager shutdown");
        }
        
        let drain_timeout = Duration::from_secs(30);
        let drain_start = std::time::Instant::now();
        
        loop {
            let active = active_requests.load(Ordering::SeqCst);
            
            if active == 0 {
                log::info!("All request handlers completed successfully");
                break;
            }
            
            if drain_start.elapsed() > drain_timeout {
                log::warn!(
                    "Request drain timeout after {:?}, {} requests still active - proceeding with shutdown",
                    drain_timeout,
                    active
                );
                break;
            }
            
            log::debug!("Waiting for {} active request handlers to complete...", active);
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        // Shut down managers
        log::debug!("Starting manager shutdown");
        if let Err(e) = managers.shutdown().await {
            log::error!("Failed to shutdown managers: {e}");
        }
        log::debug!("Manager shutdown complete");

        // Signal shutdown complete
        if completion_tx.send(()).is_err() {
            log::debug!(
                "Shutdown completion signal not delivered (receiver dropped). \
                 This is expected if wait_for_completion() timed out or was cancelled."
            );
        }
    });

    Ok(ServerHandle::new(ct, completion_rx))
}
```

**Note**: The monitor task code (lines 316-444 in the original `serve_with_tls`) should be copied into this new function. The only difference is the listener is already bound.

### Step 3: Update `embedded_servers.rs` caller

Modify [`packages/kodegend/src/service/embedded_servers.rs`](../packages/kodegend/src/service/embedded_servers.rs) lines 60-95:

**Before:**
```rust
for config in configs {
    if !config.enabled {
        log::info!("Skipping disabled server: {}", config.name);
        continue;
    }
    
    let addr: SocketAddr = format!("127.0.0.1:{}", config.port)
        .parse()
        .context("Invalid socket address")?;
    
    log::info!("Starting {} server on {}", config.name, addr);
    
    // Pre-startup port cleanup (best-effort)
    if let Err(e) = super::port_cleanup::cleanup_port_if_needed(config.port).await {
        log::warn!(
            "Port cleanup for {} (port {}) failed: {}. Will attempt startup anyway.",
            config.name,
            config.port,
            e
        );
    }
    
    // Start server (non-blocking - returns ServerHandle immediately)
    match start_server(&config.name, addr, tls_cert.clone(), tls_key.clone()).await {
        // ... error handling ...
    }
}
```

**After:**
```rust
for config in configs {
    if !config.enabled {
        log::info!("Skipping disabled server: {}", config.name);
        continue;
    }
    
    log::info!("Starting {} server on port {}", config.name, config.port);
    
    // Clean up port and immediately reserve it (eliminates TOCTOU race)
    let listener = match super::port_cleanup::cleanup_and_reserve_port(config.port).await {
        Ok(listener) => {
            log::debug!("Port {} reserved for {} server", config.port, config.name);
            listener
        }
        Err(e) => {
            log::error!("Failed to reserve port {} for {} server: {}", 
                config.port, config.name, e);
            
            // Rollback: shutdown all previously started servers
            rollback_servers(servers).await;
            
            return Err(e).context(format!(
                "Failed to reserve port {} for {} server", 
                config.port, config.name
            ));
        }
    };
    
    // Extract address from pre-bound listener
    let addr = listener.local_addr()
        .context("Failed to get listener address")?;
    
    // Start server with pre-bound listener (non-blocking - returns ServerHandle immediately)
    match start_server_with_listener(&config.name, listener, tls_cert.clone(), tls_key.clone()).await {
        Ok(server_handle) => {
            log::info!("✓ Started {} server on port {}", config.name, config.port);
            servers.push(EmbeddedServer {
                name: config.name.clone(),
                port: config.port,
                server_handle,
            });
        }
        Err(e) => {
            log::error!("✗ Failed to start {} server: {}", config.name, e);
            
            // Rollback: shutdown all previously started servers
            rollback_servers(servers).await;
            
            return Err(e).context(format!("Failed to start {} server", config.name));
        }
    }
}
```

### Step 4: Add `start_server_with_listener()` helper

Add this new function in `embedded_servers.rs` after the existing `start_server()` function (around line 130):

```rust
/// Route to appropriate tool package's HTTP server startup using pre-bound listener
///
/// This variant accepts a pre-bound TcpListener to eliminate TOCTOU races
/// with port availability checks during cleanup.
async fn start_server_with_listener(
    category: &str,
    listener: tokio::net::TcpListener,
    tls_cert: Option<PathBuf>,
    tls_key: Option<PathBuf>,
) -> Result<ServerHandle> {
    let addr = listener.local_addr()
        .map_err(|e| anyhow::anyhow!("Failed to get listener address: {}", e))?;
    
    log::debug!("Starting embedded {} server on {} with pre-bound listener", category, addr);
    
    // Build TLS config tuple
    let tls_config = match (tls_cert, tls_key) {
        (Some(cert), Some(key)) => Some((cert, key)),
        _ => None,
    };
    
    // Create HTTP server directly using kodegen-server-http
    // Since we can't modify all tool packages, we use the http server library directly
    use kodegen_server_http::{HttpServer, Managers, RouterSet, register_tool};
    use rmcp::handler::server::router::{prompt::PromptRouter, tool::ToolRouter};
    use kodegen_config_manager::ConfigManager;
    use kodegen_utils::usage_tracker::UsageTracker;
    use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
    use std::sync::Arc;
    use std::time::Duration;
    
    // Initialize config and tracker
    let config_manager = ConfigManager::new();
    config_manager.init().await?;
    
    let timestamp = chrono::Utc::now();
    let pid = std::process::id();
    let instance_id = format!("{}-{}", timestamp.format("%Y%m%d-%H%M%S-%9f"), pid);
    let usage_tracker = UsageTracker::new(format!("{}-{}", category, instance_id));
    
    // Initialize global tool history
    kodegen_mcp_tool::tool_history::init_global_history(instance_id.clone()).await;
    
    // Build routers based on category
    let routers = match category {
        "filesystem" => {
            let mut tool_router = ToolRouter::new();
            let mut prompt_router = PromptRouter::new();
            let managers = Managers::new();
            
            // Register filesystem tools
            use kodegen_tools_filesystem::*;
            (tool_router, prompt_router) = register_tool(tool_router, prompt_router, 
                ReadFileTool::new(config_manager.clone()));
            // ... register other tools ...
            
            RouterSet::new(tool_router, prompt_router, managers)
        }
        // ... other categories would follow same pattern ...
        _ => return Err(anyhow::anyhow!("Category {} not yet updated for listener-based startup", category)),
    };
    
    // Create session manager
    let session_manager = Arc::new(LocalSessionManager::default());
    
    // Create HTTP server
    let server = HttpServer::new(
        routers.tool_router,
        routers.prompt_router,
        usage_tracker,
        config_manager,
        routers.managers,
        session_manager,
    );
    
    // Start server with pre-bound listener
    let shutdown_timeout = Duration::from_secs(30);
    server.serve_with_listener(listener, tls_config, shutdown_timeout).await
}
```

**IMPORTANT**: The above implementation shows the pattern. In practice, you should call the existing tool package functions where possible, but since they don't accept listeners yet, you may need to inline the server creation logic temporarily, or modify each tool package's `start_server` to accept an optional pre-bound listener.

**Simpler Alternative for Step 4**: Keep existing `start_server()` and just drop the listener before calling it. This still has a tiny race window but it's much smaller than the current implementation:

```rust
async fn start_server_with_listener(
    category: &str,
    listener: tokio::net::TcpListener,
    tls_cert: Option<PathBuf>,
    tls_key: Option<PathBuf>,
) -> Result<ServerHandle> {
    let addr = listener.local_addr()
        .map_err(|e| anyhow::anyhow!("Failed to get listener address: {}", e))?;
    
    // Drop the listener immediately before calling start_server
    // This creates a tiny race window (nanoseconds) but is much better than before
    drop(listener);
    
    // Call existing start_server
    start_server(category, addr, tls_cert, tls_key).await
}
```

**Best Alternative**: If you want to avoid modifying all tool packages immediately, use the simpler alternative above. The race window is reduced from milliseconds to nanoseconds.

---

## Definition of Done

The fix is complete when:

1. ✅ New function `cleanup_and_reserve_port()` added to `port_cleanup.rs` that returns a bound `TcpListener`
2. ✅ New method `serve_with_listener()` added to `HttpServer` in `kodegen-server-http/src/server.rs` 
3. ✅ `embedded_servers.rs` updated to:
   - Call `cleanup_and_reserve_port()` first
   - Pass the bound listener to server startup
   - Handle errors appropriately with rollback
4. ✅ Code compiles without errors: `cd packages/kodegend && cargo check`
5. ✅ The server starts successfully with pre-bound listeners
6. ✅ Logs show "using pre-bound listener" message during startup

**Verification**: Start kodegend and observe logs showing successful port reservation followed by server startup with no "Address already in use" errors.

## Implementation Notes

### Why This Fixes the TOCTOU Race

The fix works because the `TcpListener` **holds the port** from the moment it's created until it's used by the server. The OS kernel maintains the socket binding, preventing any other process from claiming the port during the handoff period.

**Before** (race window = milliseconds):
```
check_port_available() -> bind -> RELEASE -> return true
                                    ↓
                              RACE WINDOW (another process can bind)
                                    ↓
start_server() -> bind() -> may fail
```

**After** (race window eliminated):
```
cleanup_and_reserve_port() -> bind -> KEEP BOUND -> return listener
                                                           ↓
start_server_with_listener(listener) -> use listener directly
```

### Existing Helper Functions (No Changes Needed)

These functions in `port_cleanup.rs` are already correct and will be reused:
- `find_process_by_port()` - Unix: uses `lsof`, Windows: uses `netstat`
- `kill_process_graceful()` - Tries SIGTERM, then SIGKILL after 2s
- `wait_for_port_release()` - Polls availability every 100ms up to timeout

### Why Keep `cleanup_port_if_needed()`?

The old function remains for backward compatibility and for cases where the TOCTOU race is acceptable (non-critical servers, single-threaded environments, etc.). Only kodegend's embedded servers use the new race-free variant.

### Alternative: Retry Loop (Not Recommended)

The task file originally mentioned a retry loop approach. This is **not recommended** because:
- Still has race window (just retries on failure)
- Adds complexity and latency
- Doesn't solve the root cause

The listener-passing approach eliminates the race entirely with minimal code changes.

---

## Source Code References

All source file links use relative paths from the task file location:

- Port cleanup implementation: [`../packages/kodegend/src/service/port_cleanup.rs`](../packages/kodegend/src/service/port_cleanup.rs)
- Embedded servers caller: [`../packages/kodegend/src/service/embedded_servers.rs`](../packages/kodegend/src/service/embedded_servers.rs)  
- HTTP server implementation: [`../packages/kodegen-server-http/src/server.rs`](../packages/kodegen-server-http/src/server.rs)
- HTTP server library API: [`../packages/kodegen-server-http/src/lib.rs`](../packages/kodegen-server-http/src/lib.rs)

---

## Summary

This task eliminates a classic TOCTOU race condition by changing from a **check-then-use** pattern to a **reserve-and-use** pattern. The fix involves:
1. Creating a new cleanup function that returns a bound listener
2. Adding server startup support for pre-bound listeners
3. Updating the embedded server launcher to use the new pattern

The race window is reduced from **milliseconds** (vulnerable) to **zero** (eliminated), improving robustness without breaking existing functionality.
