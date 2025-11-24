# MEDIUM: Regex Patterns Recompiled on Every Executor Instantiation

## Priority: MEDIUM
## Category: Performance
## File: `packages/kodegend/src/security/shell_executor.rs`

## Issue Description

Every call to `ShellExecutor::new()` recompiles all regex patterns from scratch. This is wasteful when patterns are constant and should be compiled once globally.

## Location

- **File**: `packages/kodegend/src/security/shell_executor.rs`
- **Lines**: 38-68

## Problematic Code

```rust
impl ShellExecutor {
    pub fn new() -> Self {
        let mut blocked_patterns = Vec::new();

        // These patterns are CONSTANT but compiled every time!
        if let Ok(pattern) = Regex::new(r"rm\s+(-[rfRF]*\s+)*/*\s*$") {
            blocked_patterns.push(pattern);
        }
        if let Ok(pattern) = Regex::new(r"rm\s+(-[rfRF]*\s+)*/\s*$") {
            blocked_patterns.push(pattern);
        }
        // ... 6 more regex compilations

        Self {
            timeout_duration: Duration::from_secs(30),
            blocked_patterns,
            allowed_commands: None,
        }
    }
}
```

## Performance Impact

Regex compilation is expensive:
- Parse pattern syntax
- Build NFA/DFA automata
- Optimize state machines
- Allocate internal buffers

Typical cost: **10-100μs per pattern**

For 8 patterns: **~80-800μs per ShellExecutor::new()**

If executor is created per-request: significant overhead.

## Correct Implementation

### Option 1: Use lazy_static

```rust
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BLOCKED_PATTERNS: Vec<Regex> = {
        vec![
            Regex::new(r"rm\s+(-[rfRF]*\s+)*/*\s*$").unwrap(),
            Regex::new(r"rm\s+(-[rfRF]*\s+)*/\s*$").unwrap(),
            Regex::new(r":\(\)\s*\{").unwrap(),
            Regex::new(r"\|\s*:\s*&").unwrap(),
            Regex::new(r"`.*`").unwrap(),
            Regex::new(r"\$\(.*\)").unwrap(),
        ]
    };
}

impl ShellExecutor {
    pub fn new() -> Self {
        Self {
            timeout_duration: Duration::from_secs(30),
            blocked_patterns: BLOCKED_PATTERNS.clone(), // Cheap Arc clone
            allowed_commands: None,
        }
    }
}
```

Wait, `Regex` doesn't implement Clone cheaply. Better:

```rust
use std::sync::Arc;

lazy_static! {
    static ref BLOCKED_PATTERNS: Arc<Vec<Regex>> = {
        Arc::new(vec![
            Regex::new(r"rm\s+(-[rfRF]*\s+)*/*\s*$").unwrap(),
            // ... rest
        ])
    };
}

pub struct ShellExecutor {
    timeout_duration: Duration,
    blocked_patterns: Arc<Vec<Regex>>,
    allowed_commands: Option<Vec<String>>,
}

impl ShellExecutor {
    pub fn new() -> Self {
        Self {
            timeout_duration: Duration::from_secs(30),
            blocked_patterns: Arc::clone(&BLOCKED_PATTERNS), // Cheap!
            allowed_commands: None,
        }
    }
    
    fn validate_command(&self, cmd: &str) -> Result<(), String> {
        for pattern in self.blocked_patterns.iter() {
            if pattern.is_match(cmd) {
                return Err(format!("Command blocked by security policy: {}", cmd));
            }
        }
        
        if let Some(allowed) = &self.allowed_commands {
            let cmd_base = cmd.split_whitespace().next().unwrap_or("");
            if !allowed.contains(&cmd_base.to_string()) {
                return Err(format!("Command not in whitelist: {}", cmd_base));
            }
        }
        
        Ok(())
    }
}
```

### Option 2: Use OnceCell (More Modern)

```rust
use std::sync::OnceLock;

static BLOCKED_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();

fn get_blocked_patterns() -> &'static Vec<Regex> {
    BLOCKED_PATTERNS.get_or_init(|| {
        vec![
            Regex::new(r"rm\s+(-[rfRF]*\s+)*/*\s*$").unwrap(),
            Regex::new(r"rm\s+(-[rfRF]*\s+)*/\s*$").unwrap(),
            Regex::new(r":\(\)\s*\{").unwrap(),
            Regex::new(r"\|\s*:\s*&").unwrap(),
            Regex::new(r"`.*`").unwrap(),
            Regex::new(r"\$\(.*\)").unwrap(),
        ]
    })
}

pub struct ShellExecutor {
    timeout_duration: Duration,
    allowed_commands: Option<Vec<String>>,
}

impl ShellExecutor {
    pub fn new() -> Self {
        Self {
            timeout_duration: Duration::from_secs(30),
            allowed_commands: None,
        }
    }
    
    fn validate_command(&self, cmd: &str) -> Result<(), String> {
        let patterns = get_blocked_patterns();
        for pattern in patterns {
            if pattern.is_match(cmd) {
                return Err(format!("Command blocked by security policy: {}", cmd));
            }
        }
        // ... rest
    }
}
```

### Option 3: Const Patterns (Requires regex-automata)

For even better performance, use DFA-based matching:

```rust
use regex_automata::dfa::regex::Regex as DfaRegex;

static COMMAND_VALIDATOR: OnceLock<CommandValidator> = OnceLock::new();

struct CommandValidator {
    dfa: DfaRegex,
}

impl CommandValidator {
    fn new() -> Self {
        let pattern = r"(?:rm\s+(?:-[rfRF]*\s+)*/*\s*$)|(?::\(\)\s*\{)|(?:`.*`)|(?:\$\(.*\))";
        Self {
            dfa: DfaRegex::new(pattern).unwrap(),
        }
    }
    
    fn is_blocked(&self, cmd: &str) -> bool {
        self.dfa.is_match(cmd.as_bytes())
    }
}
```

## Recommendation

Use **Option 2 (OnceLock)** as it's:
- Stdlib (no extra dependencies for lazy_static)
- Modern Rust idiom
- Zero runtime overhead after first init
- Thread-safe

## Performance Gain

- **Before**: 80-800μs per new()
- **After**: <1μs per new() (just copies a Duration)
- **Speedup**: 80-800x for executor creation

## Testing

```rust
#[test]
fn bench_executor_creation() {
    use std::time::Instant;
    
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = ShellExecutor::new();
    }
    let duration = start.elapsed();
    
    println!("1000 executors created in {:?}", duration);
    // Should be <1ms with caching, ~100ms without
}
```

## Related Issues

- Issue #2: Command injection (these regex patterns need expansion anyway)

## Dependencies

None - OnceLock is in std::sync (Rust 1.70+)
