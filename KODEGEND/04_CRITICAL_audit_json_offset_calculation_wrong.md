# CRITICAL: JSON Byte Offset Calculation Incorrect

## Priority: CRITICAL
## Category: Logic Error / Memory Safety
## File: `packages/kodegend/src/security/audit.rs`

## Issue Description

The `extract_vulnerability_at()` method has a critical bug in its byte offset calculation. After using `.skip(start)` on an iterator, the enumeration indices are reset to 0, but the code treats them as absolute positions. This causes incorrect string slicing and will likely panic or extract wrong data.

## Location

- **File**: `packages/kodegend/src/security/audit.rs`
- **Lines**: 445-486

## Problematic Code

```rust
fn extract_vulnerability_at(&self, json: &str, start: usize) -> Option<Vulnerability> {
    // Find JSON object boundaries
    let mut brace_count = 0;
    let mut in_string = false;
    let mut escape_next = false;
    let mut object_start = None;
    let mut object_end = None;

    // BUG: After skip(start), enumerate() starts at 0 again!
    for (i, byte) in json.bytes().enumerate().skip(start) {  // Line 453
        if escape_next {
            escape_next = false;
            continue;
        }

        match byte {
            b'\\' => escape_next = true,
            b'"' => in_string = !in_string,
            b'{' if !in_string => {
                if object_start.is_none() {
                    object_start = Some(i);  // i is RELATIVE to skip position!
                }
                brace_count += 1;
            }
            b'}' if !in_string => {
                brace_count -= 1;
                if brace_count == 0 {
                    object_end = Some(i + 1);  // i is RELATIVE!
                    break;
                }
            }
            _ => {}
        }
    }

    // Extract and parse vulnerability object
    if let (Some(start), Some(end)) = (object_start, object_end) {
        // BUG: start and end are relative indices, but we use them as absolute!
        let vuln_json = &json[start..end];  // Line 481 - WRONG!
        self.parse_vulnerability_json(vuln_json)
    } else {
        None
    }
}
```

## Why This is Critical

### The Bug Explained

When you do `iter.enumerate().skip(n)`:
1. `enumerate()` creates (index, value) pairs starting from 0
2. `skip(n)` skips the first n items
3. **BUT** the indices continue from 0, not from n!

Example:
```rust
let s = "ABCDEFGH";
for (i, c) in s.chars().enumerate().skip(5) {
    println!("{}: {}", i, c);
}
// Outputs:
// 0: F  <- index is 0, NOT 5!
// 1: G
// 2: H
```

### The Impact

Assume we find `"type":"vulnerability"` at position 1000 in the JSON:
1. `extract_vulnerability_at(json, 1000)` is called
2. Iterator starts at byte 1000 via `.skip(1000)`
3. First `{` is found at ACTUAL position 1020
4. But enumerate() returns `i=0` (relative to skip position)
5. `object_start = Some(0)` is stored
6. Later: `&json[0..end]` tries to slice from position 0, not 1020!
7. **Result**: Extracts wrong JSON, likely causes panic or invalid parse

### Consequences

1. **Wrong Data**: May extract JSON from wrong position
2. **Panics**: Slice indices may be out of bounds
3. **Silent Failures**: Invalid JSON causes None return, vulnerability is dropped
4. **Unpredictable**: Depends on JSON structure, may work sometimes and fail others

## Proof of Concept

```rust
#[test]
fn demonstrate_bug() {
    let json = r#"
    {
        "metadata": "...",
        "vulnerabilities": {
            "list": [
                {"type":"vulnerability", "id":"RUST-001"},
                {"type":"vulnerability", "id":"RUST-002"}
            ]
        }
    }
    "#;
    
    let scanner = VulnerabilityScanner::new(/* ... */);
    
    // Find second vulnerability at some offset > 0
    let start = json.find(r#""RUST-002""#).unwrap() - 20;
    
    // This will extract wrong data!
    let vuln = scanner.extract_vulnerability_at(json, start);
    
    // Will either panic or return None/wrong data
}
```

## Correct Implementation

### Option 1: Track Absolute Position

```rust
fn extract_vulnerability_at(&self, json: &str, start: usize) -> Option<Vulnerability> {
    let mut brace_count = 0;
    let mut in_string = false;
    let mut escape_next = false;
    let mut object_start = None;
    let mut object_end = None;

    // Enumerate without skip, then check position
    for (absolute_i, byte) in json.bytes().enumerate() {
        // Skip until we reach start position
        if absolute_i < start {
            continue;
        }
        
        if escape_next {
            escape_next = false;
            continue;
        }

        match byte {
            b'\\' => escape_next = true,
            b'"' => in_string = !in_string,
            b'{' if !in_string => {
                if object_start.is_none() {
                    object_start = Some(absolute_i);  // Absolute position
                }
                brace_count += 1;
            }
            b'}' if !in_string => {
                brace_count -= 1;
                if brace_count == 0 {
                    object_end = Some(absolute_i + 1);  // Absolute position
                    break;
                }
            }
            _ => {}
        }
    }

    if let (Some(start_idx), Some(end_idx)) = (object_start, object_end) {
        let vuln_json = &json[start_idx..end_idx];  // Now correct!
        self.parse_vulnerability_json(vuln_json)
    } else {
        None
    }
}
```

### Option 2: Adjust Indices (More Efficient)

```rust
fn extract_vulnerability_at(&self, json: &str, start: usize) -> Option<Vulnerability> {
    let mut brace_count = 0;
    let mut in_string = false;
    let mut escape_next = false;
    let mut object_start = None;
    let mut object_end = None;

    // Use skip but adjust indices
    for (relative_i, byte) in json.bytes().enumerate().skip(start) {
        if escape_next {
            escape_next = false;
            continue;
        }

        match byte {
            b'\\' => escape_next = true,
            b'"' => in_string = !in_string,
            b'{' if !in_string => {
                if object_start.is_none() {
                    // Convert relative to absolute: relative_i + start
                    object_start = Some(relative_i + start);
                }
                brace_count += 1;
            }
            b'}' if !in_string => {
                brace_count -= 1;
                if brace_count == 0 {
                    object_end = Some(relative_i + start + 1);
                    break;
                }
            }
            _ => {}
        }
    }

    if let (Some(start_idx), Some(end_idx)) = (object_start, object_end) {
        let vuln_json = &json[start_idx..end_idx];
        self.parse_vulnerability_json(vuln_json)
    } else {
        None
    }
}
```

### Option 3: Use Slicing (Clearest)

```rust
fn extract_vulnerability_at(&self, json: &str, start: usize) -> Option<Vulnerability> {
    // Work with a slice from start position
    let remaining = &json[start..];
    
    let mut brace_count = 0;
    let mut in_string = false;
    let mut escape_next = false;
    let mut object_start = None;
    let mut object_end = None;

    for (i, byte) in remaining.bytes().enumerate() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match byte {
            b'\\' => escape_next = true,
            b'"' => in_string = !in_string,
            b'{' if !in_string => {
                if object_start.is_none() {
                    object_start = Some(i);
                }
                brace_count += 1;
            }
            b'}' if !in_string => {
                brace_count -= 1;
                if brace_count == 0 {
                    object_end = Some(i + 1);
                    break;
                }
            }
            _ => {}
        }
    }

    if let (Some(start_idx), Some(end_idx)) = (object_start, object_end) {
        // Extract from the remaining slice
        let vuln_json = &remaining[start_idx..end_idx];
        self.parse_vulnerability_json(vuln_json)
    } else {
        None
    }
}
```

## Testing

```rust
#[test]
fn test_extract_at_various_positions() {
    let json = r#"{"meta":"data"}{"type":"vulnerability","id":"TEST-001","package":"test"}"#;
    let scanner = VulnerabilityScanner::new(/* ... */);
    
    // Try extracting at position 0
    let vuln1 = scanner.extract_vulnerability_at(json, 0);
    
    // Try extracting at middle position
    let pos = json.find(r#""type""#).unwrap();
    let vuln2 = scanner.extract_vulnerability_at(json, pos - 5);
    
    assert!(vuln2.is_some(), "Should extract vulnerability at offset position");
}
```

## Related Issues

- Issue #3: JSON buffer unused
- Issue #15: Silent failures in parsing

## Priority Justification

This bug will cause:
1. Immediate panics in production
2. Incorrect vulnerability detection
3. Silent data loss

**Must be fixed before any use of this code.**
