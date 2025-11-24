# MEDIUM: Download Timeout Can Cause Installation Hang

## Severity: MEDIUM

## Location
- `packages/kodegend/src/install/download/core.rs:20-21` - Timeout constants
- `packages/kodegend/src/install/download/core.rs:114` - Inactivity timeout wrapper

## Issue Description

While the download logic implements inactivity timeout (300 seconds with no data), there are edge cases where installation can hang or fail unexpectedly:

1. **Total download time unbounded** - Only checks inactivity, not total time
2. **Large binary downloads** - 5 minute inactivity may trigger on slow connections
3. **No progress indication during stall** - User sees frozen progress bar

## Vulnerable Code

```rust
// Lines 20-21: Timeout constants
const DOWNLOAD_CONNECT_TIMEOUT: Duration = Duration::from_secs(30);  // OK
const DOWNLOAD_INACTIVITY_TIMEOUT: Duration = Duration::from_secs(300); // 5 min

// Line 114: Timeout wrapper
let chunk_result = match timeout(DOWNLOAD_INACTIVITY_TIMEOUT, stream.next()).await {
    Ok(Some(Ok(chunk))) => chunk,
    Ok(Some(Err(e))) => return Err(e.into()),
    Ok(None) => break,
    Err(_) => {
        // Timeout triggered - no data for 5 minutes
        return Err(anyhow!(
            "Download timeout: No data received for {} seconds...",
            DOWNLOAD_INACTIVITY_TIMEOUT.as_secs()
        ));
    }
};
```

## Issues

### 1. No Total Download Time Limit
A download could take hours if bandwidth is extremely low but data trickles in:
- 100MB binary at 1KB/s = 27 hours
- No total timeout, installer hangs indefinitely
- User can't tell if download is progressing or stalled

### 2. Inactivity Timeout Too Aggressive for Slow Connections
5 minutes seems long, but consider:
- Mobile hotspot with poor signal
- Corporate proxies with aggressive caching/scanning
- VPN with high latency
- Binary hitting chunk boundary during network hiccup

### 3. No Retry Logic
Single transient network error aborts entire installation:
```rust
Ok(Some(Err(e))) => return Err(e.into()),  // No retry!
```

### 4. Progress Stops During Timeout Wait
If network stalls, progress bar freezes at last update:
```rust
// Last progress update at 50MB
pb_download_clone.set_position(50 * 1024 * 1024);

// Network stalls for 4 minutes 59 seconds
// Progress bar shows 50MB for entire duration
// User thinks installer is frozen, kills it
```

## Real-World Impact Scenarios

### Scenario 1: Slow Connection Timeout
```
User on slow connection (500 KB/s)
Binary size: 150 MB
Expected download time: 5 minutes
Inactivity timeout: 5 minutes

If ANY 5-second period has <256KB transferred (chunk threshold):
- Inactivity timeout may trigger
- Installation fails
- User retries, same issue
- Installation impossible on slow connections
```

### Scenario 2: Proxy Interference
```
Corporate proxy scans large downloads
- Proxy buffers entire binary before forwarding
- First byte arrives immediately (connection established)
- Remaining data delayed 6 minutes (virus scan)
- Inactivity timeout (5 min) kills download
- Installation fails even with fast connection
```

### Scenario 3: GitHub CDN Issue
```
GitHub CDN experiencing issues
- Downloads start normally
- CDN node fails mid-transfer
- GitHub redirects to different CDN node
- Redirect takes 6 minutes
- Timeout kills download
```

## Remediation

### 1. Add Total Download Time Limit

```rust
const DOWNLOAD_TOTAL_TIMEOUT: Duration = Duration::from_secs(30 * 60); // 30 min max

let download_start = tokio::time::Instant::now();

loop {
    // Check total time elapsed
    if download_start.elapsed() > DOWNLOAD_TOTAL_TIMEOUT {
        return Err(anyhow!(
            "Download exceeded maximum time limit of {} minutes. \
             Downloaded {}/{} bytes. Please check your internet connection.",
            DOWNLOAD_TOTAL_TIMEOUT.as_secs() / 60,
            downloaded,
            total_bytes
        ));
    }

    // Existing inactivity timeout logic...
}
```

### 2. Implement Retry Logic with Exponential Backoff

```rust
const MAX_DOWNLOAD_RETRIES: usize = 3;

for attempt in 1..=MAX_DOWNLOAD_RETRIES {
    match download_with_timeout(url, &mut file).await {
        Ok(_) => break,
        Err(e) if attempt < MAX_DOWNLOAD_RETRIES => {
            log::warn!("Download attempt {} failed: {}. Retrying...", attempt, e);

            // Exponential backoff: 2s, 4s, 8s
            let delay = Duration::from_secs(2u64.pow(attempt as u32));
            tokio::time::sleep(delay).await;

            // Reset file position for retry
            file.seek(SeekFrom::Start(0)).await?;
        }
        Err(e) => return Err(e),
    }
}
```

### 3. Resume Partial Downloads

```rust
// Send Range header to resume from last byte
let resume_from = if package_path.exists() {
    tokio::fs::metadata(&package_path).await?.len()
} else {
    0
};

let client = reqwest::Client::builder()
    .connect_timeout(DOWNLOAD_CONNECT_TIMEOUT)
    .user_agent("kodegen-installer/0.1")
    .build()?;

let response = client
    .get(&asset.browser_download_url)
    .header("Range", format!("bytes={}-", resume_from))
    .send()
    .await?;

// Open file in append mode if resuming
let mut file = if resume_from > 0 {
    tokio::fs::OpenOptions::new()
        .append(true)
        .open(&package_path)
        .await?
} else {
    tokio::fs::File::create(&package_path).await?
};

let mut downloaded = resume_from;
```

### 4. Add Heartbeat Progress Updates

```rust
let mut last_heartbeat = tokio::time::Instant::now();
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

loop {
    // Even if no new data, send heartbeat to show download is alive
    if last_heartbeat.elapsed() > HEARTBEAT_INTERVAL {
        send_best_effort(InstallProgress::download(
            binary_name.to_string(),
            binary_index,
            BINARY_COUNT,
            downloaded,
            total_bytes,
            DownloadPhase::Downloading,
            version.clone(),
        ));
        last_heartbeat = tokio::time::Instant::now();
    }

    // Existing download logic...
}
```

### 5. Adaptive Timeout Based on Connection Speed

```rust
// Calculate average speed
let elapsed = download_start.elapsed().as_secs_f64();
let avg_speed = downloaded as f64 / elapsed;  // bytes/sec

// Adjust timeout based on speed
let adaptive_timeout = if avg_speed < 10_000.0 {
    // Very slow connection (<10KB/s): allow 10 min inactivity
    Duration::from_secs(600)
} else if avg_speed < 100_000.0 {
    // Slow connection (<100KB/s): allow 5 min inactivity
    Duration::from_secs(300)
} else {
    // Normal connection: 2 min inactivity
    Duration::from_secs(120)
};
```

## Testing

```rust
#[tokio::test]
async fn test_download_timeout_enforcement() {
    // Mock slow server that sends 1 byte every 6 minutes
    let slow_server = MockServer::start().await;
    slow_server.mock_slow_response(Duration::from_secs(360));

    let result = download_binary(&slow_server.url(), "test", 1, Platform::detect()?).await;

    // Should timeout and fail
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("timeout"));
}

#[tokio::test]
async fn test_download_retry_on_network_error() {
    let flaky_server = MockServer::start().await;
    flaky_server.fail_first_n_requests(2);  // Fail first 2 attempts

    let result = download_binary(&flaky_server.url(), "test", 1, Platform::detect()?).await;

    // Should succeed after retries
    assert!(result.is_ok());
    assert_eq!(flaky_server.request_count(), 3);  // 2 failures + 1 success
}
```

## References
- Network Programming Best Practices
- HTTP Range Requests (RFC 7233)
- Exponential Backoff Algorithms
- Resilient HTTP Client Design
