# MEDIUM: Missing systemd Readiness Notification

## Severity
**MEDIUM** - Affects service orchestration and dependency management

## Location
`packages/kodegend/src/main.rs` (entire flow, no specific line)

## Issue Description
The daemon does not notify systemd when it's fully started and ready to accept requests. This causes systemd to consider the service "active" before it's actually ready, breaking service dependencies and health monitoring.

## Current Behavior
```rust
async fn run_service_manager(config: Config) -> Result<()> {
    // ... setup ...
    
    let mut service_manager = ServiceManager::new(config.services);
    service_manager.start_all().await?;

    info!("Service manager started, waiting for shutdown signal");
    // ← No sd_notify call here
    
    // Wait for shutdown signal
    let _ = shutdown_rx.recv().await;
    // ...
}
```

From systemd's perspective:
1. Fork happens (Daemonize::execute)
2. Parent exits
3. systemd marks service as "active" immediately
4. **But services may still be starting!**

## Context from CLAUDE.md
The project documentation states:
> "kodegend uses crossbeam-based service management with zero-allocation hot paths, double-fork daemonization with **systemd auto-detection**"

The code has systemd awareness but doesn't implement the readiness protocol.

## systemd Service Types

### Type=forking (Current Assumption)
```ini
[Service]
Type=forking
PIDFile=/var/run/kodegend.pid
```

systemd considers service "ready" when:
- Fork completes
- Parent process exits
- PID file created

**Problem**: Services might still be starting!

### Type=notify (Recommended)
```ini
[Service]
Type=notify
PIDFile=/var/run/kodegend.pid
NotifyAccess=main
```

systemd considers service "ready" when:
- Fork completes
- **AND** daemon sends `READY=1` notification

## Production Impact Scenarios

### Scenario 1: Dependent Service Starts Too Early
```ini
# web-proxy.service
[Unit]
After=kodegend.service
Requires=kodegend.service

[Service]
ExecStart=/usr/bin/web-proxy
```

Timeline:
```
0s:   systemd starts kodegend.service
0.1s: Fork completes, parent exits
0.1s: systemd marks kodegend as "active"
0.1s: systemd starts web-proxy.service
0.5s: web-proxy tries to connect to kodegend services
      Connection refused! Services not ready yet
1.0s: kodegend services actually finish starting
      Too late, web-proxy already failed
```

### Scenario 2: Health Checks Fail
```bash
# Monitoring system checks if service is up
$ systemctl is-active kodegend
active

# But actual health check fails
$ curl http://localhost:30438/health
Connection refused

# systemd says active, but service isn't ready
# Monitoring fires false alarms
```

### Scenario 3: Restart Cascades
```ini
[Service]
Restart=on-failure
RestartSec=5
```

Timeline:
```
0s:   kodegend restarts
0.1s: systemd sees it active
0.1s: dependent-service starts
0.2s: dependent-service fails (kodegend not ready)
0.2s: dependent-service triggers restart
5.2s: dependent-service restarts again
5.3s: Still fails (within startup window)
      ... restart loop ...
```

### Scenario 4: Load Balancer Registration
```
1. systemd starts kodegend
2. systemd marks active immediately
3. Service registration happens (consul, etc.)
4. Load balancer routes traffic
5. Requests fail - services not ready
6. Customers see errors during deployment
```

## systemd Readiness Protocol

### sd_notify API
```c
// C API
#include <systemd/sd-daemon.h>
sd_notify(0, "READY=1");
```

### Environment Variable Method
```bash
# systemd sets this environment variable
NOTIFY_SOCKET=/run/systemd/notify

# Daemon sends to this socket:
echo -n "READY=1" | socat - unix-sendto:$NOTIFY_SOCKET
```

### Rust Implementation
```rust
use std::env;
use std::os::unix::net::UnixDatagram;

fn notify_systemd_ready() -> Result<()> {
    if let Ok(notify_socket) = env::var("NOTIFY_SOCKET") {
        let socket = UnixDatagram::unbound()?;
        socket.send_to(b"READY=1", &notify_socket)?;
        info!("Notified systemd: service ready");
        Ok(())
    } else {
        debug!("NOTIFY_SOCKET not set, skipping systemd notification");
        Ok(())
    }
}
```

## Recommended Fix

### Option 1: Simple Integration
```rust
async fn run_service_manager(config: Config) -> Result<()> {
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
    
    // Setup signal handlers
    // ...
    
    // Create and start service manager
    let mut service_manager = ServiceManager::new(config.services);
    service_manager.start_all().await?;

    info!("Service manager started");
    
    // Notify systemd we're ready
    if let Err(e) = notify_systemd_ready() {
        warn!("Failed to notify systemd: {}", e);
        // Continue anyway - not critical
    }
    
    info!("Waiting for shutdown signal");
    
    // Wait for shutdown signal
    let _ = shutdown_rx.recv().await;
    // ...
}

fn notify_systemd_ready() -> Result<()> {
    if let Ok(notify_socket) = std::env::var("NOTIFY_SOCKET") {
        let socket = std::os::unix::net::UnixDatagram::unbound()?;
        socket.send_to(b"READY=1", &notify_socket)
            .context("Failed to send READY notification to systemd")?;
        info!("Sent READY=1 to systemd");
    }
    Ok(())
}
```

### Option 2: Use systemd Crate
```toml
# Cargo.toml
[dependencies]
systemd = "0.10"
```

```rust
use systemd::daemon;

async fn run_service_manager(config: Config) -> Result<()> {
    // ... start services ...
    
    service_manager.start_all().await?;
    
    // Notify systemd
    if daemon::notify(false, [(daemon::STATE_READY, "1")].iter())? {
        info!("Notified systemd: service ready");
    }
    
    // ...
}
```

### Option 3: Enhanced with Status Updates
```rust
fn notify_systemd(message: &str) -> Result<()> {
    if let Ok(notify_socket) = std::env::var("NOTIFY_SOCKET") {
        let socket = std::os::unix::net::UnixDatagram::unbound()?;
        socket.send_to(message.as_bytes(), &notify_socket)?;
    }
    Ok(())
}

async fn run_service_manager(config: Config) -> Result<()> {
    // ... setup ...
    
    notify_systemd("STATUS=Starting service manager...")?;
    
    let mut service_manager = ServiceManager::new(config.services);
    
    notify_systemd("STATUS=Starting services...")?;
    service_manager.start_all().await?;
    
    notify_systemd("READY=1\nSTATUS=All services started")?;
    
    info!("Waiting for shutdown signal");
    let _ = shutdown_rx.recv().await;
    
    notify_systemd("STOPPING=1\nSTATUS=Shutting down services...")?;
    service_manager.stop_all().await?;
    
    Ok(())
}
```

## systemd Unit File Updates

### Current (Type=forking)
```ini
[Unit]
Description=KODEGEN Daemon Service Manager
After=network.target

[Service]
Type=forking
PIDFile=/var/run/kodegend.pid
ExecStart=/usr/bin/kodegend --config /etc/kodegend/config.toml
ExecStop=/usr/bin/kodegend --stop
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
```

### Updated (Type=notify)
```ini
[Unit]
Description=KODEGEN Daemon Service Manager
After=network.target

[Service]
Type=notify
NotifyAccess=main
PIDFile=/var/run/kodegend.pid
ExecStart=/usr/bin/kodegend --config /etc/kodegend/config.toml
ExecStop=/usr/bin/kodegend --stop
Restart=on-failure
RestartSec=5s
TimeoutStartSec=60s

[Install]
WantedBy=multi-user.target
```

Key changes:
- `Type=notify`: Wait for READY=1
- `NotifyAccess=main`: Only main process can notify
- `TimeoutStartSec=60s`: Fail if no READY within 60s

## Watchdog Support (Bonus)
systemd can monitor daemon health:

```ini
[Service]
Type=notify
WatchdogSec=30s
```

```rust
use std::time::Duration;

// In main loop
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(15));
    loop {
        interval.tick().await;
        
        // Health check
        if service_manager.is_healthy() {
            notify_systemd("WATCHDOG=1")?;
        }
    }
});
```

If daemon doesn't send WATCHDOG=1 within 30s, systemd restarts it.

## Platform Detection
Only notify if running under systemd:

```rust
fn is_systemd_service() -> bool {
    // Check if NOTIFY_SOCKET is set
    std::env::var("NOTIFY_SOCKET").is_ok()
}

fn notify_systemd_ready() -> Result<()> {
    if !is_systemd_service() {
        debug!("Not running under systemd, skipping notification");
        return Ok(());
    }
    
    // ... send notification ...
}
```

## Error Handling
Don't fail daemon startup if notification fails:

```rust
match notify_systemd_ready() {
    Ok(()) => info!("systemd notified successfully"),
    Err(e) => {
        warn!("Failed to notify systemd: {}", e);
        warn!("Continuing anyway - daemon may appear not ready to systemd");
        // Don't return error - notification is optional
    }
}
```

## Testing

### Test with systemd
```bash
# Install unit file
sudo systemctl daemon-reload
sudo systemctl start kodegend

# Check if service is active AND ready
systemctl is-active kodegend
# Should show "active" only after READY=1 sent

# Check status
systemctl status kodegend
# Should show "active (running)" not "activating (start)"
```

### Test without systemd
```bash
# Run directly (no NOTIFY_SOCKET set)
/usr/bin/kodegend

# Should start normally, skip notification
# No errors about missing NOTIFY_SOCKET
```

### Monitor notifications
```bash
# As root, monitor systemd notification socket
sudo socat -u UNIX-RECV:/run/systemd/notify -

# In another terminal, start kodegend
sudo systemctl start kodegend

# Should see:
# READY=1
# STATUS=All services started
```

## Documentation Updates

### Installation Guide
```markdown
## systemd Integration

kodegend supports systemd Type=notify for accurate service state tracking:

1. Install unit file with `Type=notify`
2. Daemon will send READY=1 when all services started
3. Dependent services will wait for actual readiness

Benefits:
- Accurate service dependency management
- Better health monitoring
- Prevents premature dependent service starts
```

### Troubleshooting
```markdown
### Service Shows "activating" Forever

If `systemctl status kodegend` shows "activating (start)" and never becomes "active":

1. Check logs: `journalctl -u kodegend -n 50`
2. Verify READY=1 notification sent
3. Increase `TimeoutStartSec` if startup is legitimately slow
4. Check if service startup is actually hanging
```

## Performance Impact
- Sending notification: ~100 microseconds (negligible)
- No ongoing overhead (one-time notification)
- Watchdog keepalive: ~50 microseconds every 15s

## Related Issues
- Service shutdown timeout (task file 07) - affects restart timing
- Restart verification (task file 04) - systemd handles this better

## Migration Path
1. **Phase 1**: Add notification code (backward compatible)
2. **Phase 2**: Update unit files to Type=notify
3. **Phase 3**: Add watchdog support (optional)

Old unit files continue working with Type=forking even after code changes.

## Alternative: systemd Socket Activation
For advanced use, consider socket activation:

```ini
[Service]
Type=notify
Sockets=kodegend.socket
```

systemd creates socket, passes to daemon. Instant "readiness".

## References
- systemd.service(5) - Type=notify documentation
- sd_notify(3) - Notification protocol
- systemd.exec(5) - NOTIFY_SOCKET environment
- freedesktop.org: systemd Developer Documentation
