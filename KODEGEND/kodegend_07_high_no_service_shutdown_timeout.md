# HIGH: No Timeout on Service Shutdown

## Severity
**HIGH** - Daemon can hang indefinitely during shutdown

## Location
`packages/kodegend/src/main.rs:148`

## Issue Description
The `service_manager.stop_all().await?` call has no timeout. If any service hangs during shutdown, the entire daemon shutdown process hangs indefinitely, preventing graceful daemon termination.

## Current Code
```rust
async fn run_service_manager(config: Config) -> Result<()> {
    // ... setup and run ...
    
    // Wait for shutdown signal
    let _ = shutdown_rx.recv().await;

    info!("Shutting down services...");
    service_manager.stop_all().await?;  // ← NO TIMEOUT, CAN HANG FOREVER

    // Clean up PID file
    if let Ok(config) = Config::load(&PathBuf::from("kodegend.toml")) {
        let _ = fs::remove_file(&config.daemon.pid_file);
    }

    info!("Daemon stopped");
    Ok(())
}
```

## Problem
If `stop_all()` never completes, the daemon:
1. Never removes PID file
2. Never exits cleanly
3. Blocks systemd/launchd service control
4. Requires manual SIGKILL to terminate

## Production Scenarios

### Scenario 1: Hung Database Connection
```
Service has open database connection that's blocked:
1. Shutdown signal received
2. Service tries to close DB connection
3. Database server is unresponsive (network issue)
4. Connection.close() hangs waiting for TCP timeout
5. stop_all() never completes
6. Daemon hangs until killed
```

### Scenario 2: Deadlock in Service Shutdown
```
Service A waiting for Service B to release lock
Service B waiting for Service A to signal completion
→ Deadlock
→ stop_all() hangs forever
```

### Scenario 3: Infinite Loop in Cleanup
```rust
// In some service's stop() implementation:
async fn stop(&mut self) {
    while !self.queue.is_empty() {
        // Process remaining items
        // But queue is constantly being fed by another thread
        // Never becomes empty
    }
}
```

### Scenario 4: External Process Won't Die
```
Service manages child process:
1. Service sends SIGTERM to child
2. Child ignores signal or is hung
3. Service waits for child to exit
4. Child never exits
5. Service stop() never completes
```

## Impact on Operations

### Systemd Service Management
```bash
$ systemctl stop kodegend
# Hangs... waits for DefaultTimeoutStopSec (usually 90s)
# After timeout, systemd sends SIGKILL
# Daemon killed forcefully
# Resources not cleaned up properly
# PID file remains
```

### Manual Stop Command
```bash
$ kodegend --stop
# Sends SIGTERM to daemon
# Daemon receives signal, starts shutdown
# stop_all() hangs
# User waits... nothing happens
# User hits Ctrl+C, but process already daemonized
# Must find PID and kill -9
```

### Restart Operations
```bash
$ kodegend --restart
# stop_daemon() sends SIGTERM
# Waits 1 second (see task file 04)
# Old daemon still shutting down (hung in stop_all)
# New daemon starts
# Both daemons now running → resource conflicts
```

### Container Orchestration
```yaml
# Kubernetes pod
terminationGracePeriodSeconds: 30

# Pod shutdown:
# 1. Kubelet sends SIGTERM
# 2. stop_all() hangs
# 3. After 30s, kubelet sends SIGKILL
# 4. Daemon killed, no cleanup
# 5. Resources leaked in cluster
```

## Recommended Fix

### Option 1: Timeout with tokio::time::timeout (Recommended)
```rust
use tokio::time::{timeout, Duration};

async fn run_service_manager(config: Config) -> Result<()> {
    // ... setup and run ...
    
    // Wait for shutdown signal
    let _ = shutdown_rx.recv().await;

    info!("Shutting down services...");
    
    let shutdown_timeout = Duration::from_secs(30);
    match timeout(shutdown_timeout, service_manager.stop_all()).await {
        Ok(Ok(())) => {
            info!("Services stopped successfully");
        }
        Ok(Err(e)) => {
            error!("Error stopping services: {}", e);
            return Err(e);
        }
        Err(_) => {
            error!("Service shutdown timed out after {:?}", shutdown_timeout);
            warn!("Some services may still be running");
            // Continue anyway to clean up what we can
        }
    }

    // Clean up PID file
    if let Err(e) = fs::remove_file(&config.daemon.pid_file) {
        warn!("Failed to remove PID file: {}", e);
    }

    info!("Daemon stopped");
    Ok(())
}
```

### Option 2: Configurable Timeout
```rust
// In DaemonConfig:
pub struct DaemonConfig {
    // ... existing fields ...
    #[serde(default = "default_shutdown_timeout")]
    pub shutdown_timeout_secs: u64,
}

fn default_shutdown_timeout() -> u64 {
    30
}

// In shutdown logic:
let shutdown_timeout = Duration::from_secs(config.daemon.shutdown_timeout_secs);
match timeout(shutdown_timeout, service_manager.stop_all()).await {
    // ...
}
```

### Option 3: Per-Service Timeouts
```rust
// In ServiceManager::stop_all()
pub async fn stop_all(&mut self) -> Result<()> {
    for service in &mut self.services {
        let timeout = Duration::from_secs(service.config.stop_timeout.unwrap_or(10));
        match timeout(timeout, service.stop()).await {
            Ok(Ok(())) => {
                info!("Service {} stopped", service.name);
            }
            Ok(Err(e)) => {
                error!("Service {} stop failed: {}", service.name, e);
            }
            Err(_) => {
                error!("Service {} stop timed out after {:?}", service.name, timeout);
                // Try to force kill if it has a PID
                if let Some(pid) = service.pid() {
                    warn!("Force killing service {} (PID {})", service.name, pid);
                    let _ = signal::kill(Pid::from_raw(pid), Signal::SIGKILL);
                }
            }
        }
    }
    Ok(())
}
```

## Escalation Strategy
For production robustness, implement escalation:

```rust
async fn shutdown_services_with_escalation(
    service_manager: &mut ServiceManager,
    config: &DaemonConfig,
) -> Result<()> {
    // Phase 1: Graceful shutdown (30s)
    info!("Attempting graceful service shutdown...");
    let graceful_timeout = Duration::from_secs(30);
    
    match timeout(graceful_timeout, service_manager.stop_all()).await {
        Ok(Ok(())) => {
            info!("All services stopped gracefully");
            return Ok(());
        }
        Ok(Err(e)) => {
            warn!("Service shutdown encountered errors: {}", e);
        }
        Err(_) => {
            warn!("Graceful shutdown timed out after {:?}", graceful_timeout);
        }
    }
    
    // Phase 2: Force stop remaining services (10s)
    info!("Force stopping remaining services...");
    let force_timeout = Duration::from_secs(10);
    
    match timeout(force_timeout, service_manager.force_stop_all()).await {
        Ok(Ok(())) => {
            info!("All services force stopped");
            return Ok(());
        }
        Ok(Err(e)) => {
            error!("Force stop failed: {}", e);
        }
        Err(_) => {
            error!("Force stop timed out after {:?}", force_timeout);
        }
    }
    
    // Phase 3: Nuclear option - SIGKILL everything
    warn!("Sending SIGKILL to all service processes...");
    service_manager.kill_all_processes();
    
    // Give a moment for kills to take effect
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    Ok(())
}
```

## Handling Timeout in Systemd
```toml
# kodegend.toml
[daemon]
shutdown_timeout_secs = 60  # Must be less than systemd TimeoutStopSec
```

```ini
# /etc/systemd/system/kodegend.service
[Service]
TimeoutStopSec=90  # 60s for graceful + 30s buffer
KillMode=mixed      # SIGTERM to main, SIGKILL to rest after timeout
```

## Logging During Timeout
```rust
// Before timeout:
info!("Shutting down services (timeout: {:?})...", shutdown_timeout);

// During shutdown (in ServiceManager::stop_all):
for (i, service) in self.services.iter().enumerate() {
    info!("Stopping service {}/{}: {}", i+1, self.services.len(), service.name);
    // ...
}

// On timeout:
error!("Shutdown timed out. Hung services:");
for service in self.services.iter().filter(|s| s.is_running()) {
    error!("  - {} (PID: {:?})", service.name, service.pid());
}
```

This helps debugging which service caused the hang.

## Testing Requirements
1. **Normal shutdown**: All services stop quickly → succeeds within timeout
2. **Slow service**: One service takes 15s → completes before timeout
3. **Hung service**: Mock service that never stops → timeout triggers
4. **Database timeout**: Mock DB connection hang → verify timeout
5. **Deadlock**: Create deadlock condition → verify timeout and logging
6. **Partial success**: Some services stop, others hang → verify cleanup

## Metrics and Monitoring
Consider adding metrics:

```rust
// Record shutdown duration
let start = Instant::now();
let result = timeout(shutdown_timeout, service_manager.stop_all()).await;
let duration = start.elapsed();

if duration > Duration::from_secs(10) {
    warn!("Slow shutdown detected: {:?}", duration);
}

// Log per-service metrics
service_manager.log_shutdown_metrics();
```

## Related Issues
- Restart verification (task file 04) needs to wait for shutdown timeout
- Signal handler (task file 08) should account for shutdown time
- Service implementation needs timeout awareness

## Configuration Example
```toml
[daemon]
pid_file = "/var/run/kodegend.pid"
shutdown_timeout_secs = 30  # Overall timeout
working_directory = "/var/lib/kodegend"

[[services]]
name = "web-server"
command = "/usr/bin/service-web"
stop_timeout_secs = 15  # Per-service override

[[services]]
name = "worker"
command = "/usr/bin/service-worker"
stop_timeout_secs = 60  # Longer for worker with batch jobs
```

## References
- systemd TimeoutStopSec directive
- Docker stop timeout (default 10s)
- Kubernetes terminationGracePeriodSeconds (default 30s)
- POSIX signal handling and timeouts
