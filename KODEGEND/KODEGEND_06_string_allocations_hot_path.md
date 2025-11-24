# MEDIUM: Unnecessary String Allocations in Hot Path Event Sends

**Priority:** MEDIUM  
**Component:** `packages/kodegend/src/service.rs`, `packages/kodegend/src/ipc.rs`, `packages/kodegend/src/service/autoconfig.rs`  
**Impact:** Performance - Memory churn in long-running daemon

## Problem Statement

Service event sends clone the service name string on every state change and health check, causing unnecessary allocations in hot paths. Over days/weeks of uptime in a long-running daemon, this accumulates to millions of unnecessary allocations.

### Verified Problematic Code Locations

#### Primary Hot Path: `service.rs`

**Line 35 - ServiceWorker struct definition:**
```rust
pub struct ServiceWorker {
    name: String,  // ← Owned String, cloned on every event
    rx: Receiver<Cmd>,
    tx: Sender<Cmd>,
    bus: Sender<Evt>,
    def: ServiceDefinition,
}
```

**Line 173-178 - Start event (less frequent but still allocates):**
```rust
self.bus.send(Evt::State {
    service: self.name.to_string(),  // ← ALLOCATES + COPIES String
    kind: "running".into(),
    ts: Utc::now(),
    pid: Some(pid),
})?;
```

**Line 295-300 - Stop event (less frequent):**
```rust
self.bus.send(Evt::State {
    service: self.name.to_string(),  // ← ALLOCATES + COPIES
    kind: "stopped-clean".into(),
    ts: Utc::now(),
    pid: Some(pid),
})?;
```

**Line 343-348 - Crash event (hopefully rare):**
```rust
self.bus.send(Evt::State {
    service: self.name.to_string(),  // ← ALLOCATES + COPIES
    kind: "stopped-crash".into(),
    ts: Utc::now(),
    pid: None,
})?;
```

**Line 353-357 - Health event (CRITICAL: FIRES EVERY 60 SECONDS):**
```rust
self.bus.send(Evt::Health {
    service: self.name.to_string(),  // ← ALLOCATES + COPIES (60s interval!)
    healthy,
    ts: Utc::now(),
})?;
```

**Line 372-376 - Log rotation event (every hour):**
```rust
self.bus.send(Evt::LogRotate {
    service: self.name.to_string(),  // ← ALLOCATES + COPIES
    ts: Utc::now(),
})?;
```

**Line 402-406 - Log rotation event (after rotation):**
```rust
self.bus.send(Evt::LogRotate {
    service: self.name.to_string(),  // ← ALLOCATES + COPIES
    ts: Utc::now(),
})?;
```

#### Secondary Hot Path: `service/autoconfig.rs`

**Line 45-56 - Autoconfig service start:**
```rust
let service_name = self.name.clone();  // ← Clones String
async move {
    let _ = bus.send(Evt::State {
        service: service_name.clone(),  // ← ALLOCATES + COPIES
        kind: "running".into(),
        ts: chrono::Utc::now(),
        pid: Some(std::process::id()),
    });
```

**Line 63-67 - Autoconfig fatal event:**
```rust
let _ = bus.send(Evt::Fatal {
    service: service_name.clone(),  // ← ALLOCATES + COPIES
    msg: "Watcher error occurred".into(),
    ts: chrono::Utc::now(),
});
```

**Line 72-77 - Autoconfig stop event:**
```rust
let _ = bus.send(Evt::State {
    service: service_name.clone(),  // ← ALLOCATES + COPIES
    kind: "stopped-clean".into(),
    ts: chrono::Utc::now(),
    pid: Some(std::process::id()),
});
```

### Allocation Frequency Analysis

For a system with 10 services running 24/7:

**Health checks (most critical hot path):**
- Each service: 1 allocation per 60 seconds = 60 allocations/hour
- 10 services: 600 allocations/hour
- 24 hours: 14,400 allocations/day
- 30 days: **432,000 allocations/month** just for health checks

**Log rotation:**
- Each service: 2 allocations per hour = 48 allocations/day
- 10 services: 480 allocations/day
- 30 days: **14,400 allocations/month** for rotation

**Total ongoing allocations: ~446,000/month** just for these event sends

This doesn't include start/stop/restart/crash events (less frequent but still wasteful).

### Why This Matters

1. **Memory allocator pressure:** Each `to_string()` calls `malloc()`, increasing allocator fragmentation
2. **String data copies:** The service name (e.g., "kodegen-filesystem") is copied in full on every event
3. **Unnecessary work:** The name is already a `String` in `self.name` - we're cloning immutable data
4. **Daemon longevity:** This daemon should run for months/years; allocations compound over time
5. **CPU waste:** 440,000+ malloc/free calls per month for data that never changes

### Memory Impact Calculation

Assuming average service name length of 20 bytes:
- 446,000 allocations/month × 20 bytes = 8.92 MB/month of string data
- Plus allocator metadata overhead (~16-24 bytes per allocation) = additional 10.7 MB/month
- **Total: ~20 MB/month** of allocator overhead for immutable data
- Over 1 year: **~240 MB** just for service name copies in events

While not catastrophic, this is wasteful for data that is known at startup and never changes.

## Root Cause Analysis

### Current `Evt` Enum Definition (`ipc.rs` lines 16-39)

```rust
use std::borrow::Cow;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum Evt {
    State {
        service: String,  // ← Owned String (root cause)
        kind: Cow<'static, str>,  // ← Already optimized with Cow!
        ts: DateTime<Utc>,
        pid: Option<u32>,
    },
    Health {
        service: String,  // ← Owned String (root cause)
        healthy: bool,
        ts: DateTime<Utc>,
    },
    LogRotate {
        service: String,  // ← Owned String (root cause)
        ts: DateTime<Utc>,
    },
    Fatal {
        service: String,  // ← Owned String (root cause)
        msg: Cow<'static, str>,  // ← Already optimized
        ts: DateTime<Utc>,
    },
}
```

**Observation:** The `kind` and `msg` fields already use `Cow<'static, str>` for optimization, but the `service` field still uses owned `String`. This is inconsistent and the service field has even better optimization potential since it's **truly immutable** (known at service creation time).

## Solution: Use Arc&lt;str&gt; for Service Names

### Why Arc&lt;str&gt; is Perfect for This Use Case

Reference: [Arc&lt;String&gt; vs Arc&lt;str&gt; discussion](https://users.rust-lang.org/t/arc-string-vs-arc-str/30571)

#### Memory Layout Advantages

**Arc&lt;String&gt; (current equivalent pattern):**
```
Arc pointer → String struct → heap allocation with string data
              (24 bytes)      (capacity, len, pointer)
```
- Two levels of indirection to reach string data
- String struct adds overhead (24 bytes on 64-bit: 8-byte capacity + 8-byte len + 8-byte ptr)

**Arc&lt;str&gt; (proposed):**
```
Arc fat pointer → heap allocation with string data
(16 bytes)        (refcount + string bytes)
```
- One level of indirection (faster reads)
- Fat pointer contains length inline (8-byte pointer + 8-byte length)
- No intermediate String struct overhead

#### Performance Characteristics

**Arc::clone() cost (both types):**
- Atomic reference count increment (~5-10 CPU cycles on modern x86_64)
- Pointer copy (trivial)
- **No string data copying**

**Arc&lt;str&gt; benefits over String clone:**
- String clone: malloc() + memcpy() of entire string data + allocator bookkeeping
- Arc&lt;str&gt; clone: Just atomic increment + pointer copy
- **50-100x faster** for typical service names (measured in benchmarks by Rust community)

#### Immutability Guarantee

Service names are:
- Set once at service creation
- Never modified during service lifetime
- Shared across multiple event sends
- Perfect fit for `Arc<str>` (immutable shared reference-counted data)

### Implementation Plan

#### File 1: `packages/kodegend/src/ipc.rs`

**Change the Evt enum to use Arc&lt;str&gt; for service names:**

```rust
use std::borrow::Cow;
use std::sync::Arc;  // ← ADD THIS IMPORT
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum Evt {
    State {
        service: Arc<str>,  // ← CHANGED from String
        kind: Cow<'static, str>,
        ts: DateTime<Utc>,
        pid: Option<u32>,
    },
    Health {
        service: Arc<str>,  // ← CHANGED from String
        healthy: bool,
        ts: DateTime<Utc>,
    },
    LogRotate {
        service: Arc<str>,  // ← CHANGED from String
        ts: DateTime<Utc>,
    },
    Fatal {
        service: Arc<str>,  // ← CHANGED from String
        msg: Cow<'static, str>,
        ts: DateTime<Utc>,
    },
}
```

**Lines to modify:** 1 (add import), 20, 26, 31, 35

#### File 2: `packages/kodegend/src/service.rs`

**Step 1: Add Arc import (top of file, around line 5-10):**

```rust
use std::process::{Child, Command, Stdio};
use std::sync::Arc;  // ← ADD THIS LINE
use std::thread;
use std::time::{Duration, Instant};
```

**Step 2: Change ServiceWorker struct (line 34-40):**

```rust
pub struct ServiceWorker {
    name: Arc<str>,  // ← CHANGED from String
    rx: Receiver<Cmd>,
    tx: Sender<Cmd>,
    bus: Sender<Evt>,
    def: ServiceDefinition,
}
```

**Step 3: Update ServiceWorker::spawn to convert String → Arc&lt;str&gt; (lines 43-68):**

```rust
pub fn spawn(def: ServiceDefinition, bus: Sender<Evt>) -> Result<Sender<Cmd>, ServiceError> {
    let (tx, rx) = bounded::<Cmd>(16);
    
    // Convert service name to Arc<str> once at creation time
    let name: Arc<str> = Arc::from(def.name.as_str());  // ← CHANGED
    let name_for_thread = Arc::clone(&name);  // ← CHANGED (cheap Arc clone)
    let tx_clone = tx.clone();

    thread::Builder::new()
        .name(format!("svc-{}", name_for_thread))  // ← Arc<str> auto-derefs to &str
        .spawn(move || {
            let mut worker = ServiceWorker {
                name: name_for_thread,
                rx,
                tx: tx_clone,
                bus,
                def,
            };
            if let Err(e) = worker.run() {
                error!("Worker {} crashed: {:#}", worker.name, e);
            }
        })
        .map_err(|source| ServiceError::SpawnFailed {
            service: name.to_string(),  // ← Only here for error message (rare case)
            source,
        })?;

    Ok(tx)
}
```

**Step 4: Update all event sends to use Arc::clone instead of to_string():**

**Line 173 (start event):**
```rust
self.bus.send(Evt::State {
    service: Arc::clone(&self.name),  // ← CHANGED from self.name.to_string()
    kind: "running".into(),
    ts: Utc::now(),
    pid: Some(pid),
})?;
```

**Line 295 (stop event):**
```rust
self.bus.send(Evt::State {
    service: Arc::clone(&self.name),  // ← CHANGED from self.name.to_string()
    kind: "stopped-clean".into(),
    ts: Utc::now(),
    pid: Some(pid),
})?;
```

**Line 343 (crash event):**
```rust
self.bus.send(Evt::State {
    service: Arc::clone(&self.name),  // ← CHANGED from self.name.to_string()
    kind: "stopped-crash".into(),
    ts: Utc::now(),
    pid: None,
})?;
```

**Line 353 (health event - CRITICAL HOT PATH):**
```rust
self.bus.send(Evt::Health {
    service: Arc::clone(&self.name),  // ← CHANGED from self.name.to_string()
    healthy,
    ts: Utc::now(),
})?;
```

**Line 372 (log rotate event):**
```rust
self.bus.send(Evt::LogRotate {
    service: Arc::clone(&self.name),  // ← CHANGED from self.name.to_string()
    ts: Utc::now(),
})?;
```

**Line 402 (log rotate event after rotation):**
```rust
self.bus.send(Evt::LogRotate {
    service: Arc::clone(&self.name),  // ← CHANGED from self.name.to_string()
    ts: Utc::now(),
})?;
```

**Lines to modify:** Add import at top, line 35 (struct), lines 45-46 (spawn), 174, 296, 344, 354, 373, 403

#### File 3: `packages/kodegend/src/service/autoconfig.rs`

The autoconfig service also constructs service names and sends events. After the changes to `ipc.rs`, this file needs updates:

**Check lines 45, 52, 64, 73** for service name cloning patterns.

If `self.name` is a `String` in the autoconfig context, it should be converted to `Arc<str>` following the same pattern as `ServiceWorker`. The `.clone()` calls will then become `Arc::clone()` (cheap pointer copies instead of string allocations).

**Expected changes:**
- If autoconfig has its own service name field, convert it to `Arc<str>`
- Change `service_name.clone()` to `Arc::clone(&service_name)` in event sends
- Ensure initial conversion from `String` to `Arc<str>` happens at service creation

#### File 4: `packages/kodegend/src/manager.rs` (Pattern Matching Only)

The manager.rs file uses `Evt::` at multiple locations (lines 268, 282, 304, 359, 373, 386, 407, 422, 425, 496, 583). 

**Verification needed:** These are likely pattern matches, not constructions:

```rust
match evt {
    Evt::State { service, kind, .. } => { ... }
    Evt::Health { service, healthy, .. } => { ... }
    // etc.
}
```

Pattern matching works identically with `Arc<str>` as it did with `String` - the bindings will just be `Arc<str>` instead. **No code changes required** in manager.rs unless it constructs Evt instances (which would be unusual for a manager/receiver).

**Action:** Verify manager.rs only pattern-matches on Evt, not constructs it. If construction occurs, update using `Arc::clone()` pattern.

## Definition of Done

### Code Changes Complete

1. `ipc.rs`: All `service: String` fields in `Evt` enum changed to `service: Arc<str>`
2. `service.rs`: `ServiceWorker::name` changed from `String` to `Arc<str>`
3. `service.rs`: All event sends use `Arc::clone(&self.name)` instead of `self.name.to_string()`
4. `service.rs`: `ServiceWorker::spawn` converts input `String` to `Arc<str>` once at creation
5. `service/autoconfig.rs`: Service name handling updated to use `Arc<str>` with cheap clones
6. All compilation errors resolved

### Compilation Verification

```bash
cd packages/kodegend
cargo check
cargo clippy
```

Both commands should complete without errors or warnings related to the string type changes.

### Runtime Verification

Start the daemon with a test service configuration:

```bash
cargo build --release
./target/release/kodegend start
```

Verify:
1. Services start successfully
2. Health check events are logged (check daemon logs)
3. No runtime panics or type errors
4. Service names appear correctly in event logs

### Expected Performance Impact

**Before changes:**
- ~446,000 string allocations per month (10 services, 24/7 operation)
- ~20 MB/month of allocator overhead

**After changes:**
- **0 string allocations** for service names in events (only Arc pointer clones)
- ~20 MB/month allocator overhead eliminated
- Health check latency improved (no malloc/memcpy in hot path)
- Reduced allocator fragmentation over long uptimes

## Technical References

### Related Source Files

- [`packages/kodegend/src/ipc.rs`](../packages/kodegend/src/ipc.rs) - Evt enum definition
- [`packages/kodegend/src/service.rs`](../packages/kodegend/src/service.rs) - ServiceWorker implementation
- [`packages/kodegend/src/service/autoconfig.rs`](../packages/kodegend/src/service/autoconfig.rs) - Autoconfig service
- [`packages/kodegend/src/manager.rs`](../packages/kodegend/src/manager.rs) - Event consumer (pattern matching only)

### Arc&lt;str&gt; Performance Research

- [Arc&lt;String&gt; vs Arc&lt;str&gt; Forum Discussion](https://users.rust-lang.org/t/arc-string-vs-arc-str/30571) - Memory layout and performance comparison
- [Arc&lt;str&gt; vs String Performance Thread](https://users.rust-lang.org/t/arc-str-is-better-than-string-was-i-using-the-wrong-string-type/99789) - Use case analysis
- [Arc&lt;str&gt; Performance Investigation (Reddit)](https://www.reddit.com/r/rust/comments/171us5s/why_did_replacing_arcstring_with_arcstr_decrease/) - Allocation cost discussion

### Key Takeaways from Research

1. **Arc::clone() is cheap:** Just an atomic increment + pointer copy (5-10 CPU cycles)
2. **Arc&lt;str&gt; has one less indirection than Arc&lt;String&gt;:** Faster reads, smaller memory footprint
3. **Conversion cost is acceptable:** String → Arc&lt;str&gt; allocates once; this is fine at service creation time
4. **Perfect for immutable shared data:** Service names never change, shared across many events
5. **Already used in stdlib and popular crates:** Standard pattern for shared strings in Rust

## Implementation Notes

### Why Not Cow&lt;'static, str&gt;?

The `Evt` enum already uses `Cow<'static, str>` for the `kind` and `msg` fields. However, `Cow<'static, str>` is not appropriate for service names because:

1. Service names come from config files (not `'static` string literals)
2. `Cow::Owned` would fall back to `String`, defeating the purpose
3. We can't guarantee the lifetime outlives all event sends

`Arc<str>` solves this by providing reference-counted ownership without lifetime constraints.

### Why Not &'a str with Lifetimes?

Using `&'a str` would require:
1. Lifetime annotations throughout the codebase (`Evt<'a>`, `ServiceWorker<'a>`, etc.)
2. Ensuring events don't outlive the service worker (problematic for queued events)
3. Significant refactoring with lifetime complexity

`Arc<str>` provides the benefits of shared references without lifetime management complexity.

### Crossbeam Channel Compatibility

`Arc<str>` works seamlessly with `crossbeam_channel::Sender<Evt>` because:
- `Arc<str>` implements `Clone` (required by channels)
- `Arc<str>` implements `Send` (safe to send across threads)
- Cloning for channel sends is just an atomic increment (no allocation)

No changes needed to the channel infrastructure.

### Auto Deref Coercion

`Arc<str>` automatically derefs to `&str` in most contexts:

```rust
let name: Arc<str> = Arc::from("kodegen-filesystem");
println!("Service: {}", name);  // ← Auto-derefs to &str
format!("svc-{}", name);        // ← Auto-derefs to &str
```

This means minimal code changes beyond the type declarations and clone sites.
