# INFO: Self-Signed Certificate Has 100-Year Validity (Security Best Practice)

## Severity: INFO / BEST PRACTICE

## Location
- `packages/kodegend/src/install/installer/config/certificates.rs:188-192` - Certificate validity period

## Issue Description

The generated self-signed TLS certificate has a 100-year validity period. While this works functionally, it violates security best practices for certificate lifecycle management.

## Code

```rust
// Lines 188-192
use time::OffsetDateTime;
let now = OffsetDateTime::now_utc();
params.not_before = now;
params.not_after = now + time::Duration::seconds(100 * 365 * 24 * 60 * 60);  // 100 years!
```

## Why This Is Not Ideal

### 1. No Key Rotation
Certificate used for 100 years means:
- Same private key for entire period
- No opportunity for key rotation
- If key compromised, impact lasts a century

### 2. Certificate Lifecycle Management
Industry best practices:
- Let's Encrypt: 90 days
- Commercial CAs: 1-2 years maximum
- Code signing: 1-3 years
- Internal/self-signed: 1-5 years recommended

### 3. Cryptographic Aging
100 years from now:
- Current crypto algorithms may be broken
- 2048-bit RSA may be insufficient
- SHA-256 may be deprecated
- No way to force upgrade

### 4. Trust Store Policies
Some systems/browsers:
- Reject certificates with excessive validity
- Flag >5 year certs as suspicious
- May break compatibility in future

## Real-World Impact

### Currently: LOW
- This is a self-signed certificate for localhost
- Only used for local MCP server (mcp.kodegen.ai → 127.0.0.1)
- Not public-facing
- User explicitly imports to trust store

### Future Considerations
If kodegen ever:
- Uses certificates for public services
- Needs to comply with regulations (HIPAA, PCI-DSS)
- Distributes certificates to customers
- Integrates with enterprise PKI

Then 100-year validity becomes a compliance issue.

## Recommended Changes

### Option 1: Reduce to 10 Years (Balanced)

```rust
// 10 years: Long enough to avoid renewal pain, short enough to force rotation
params.not_after = now + time::Duration::days(10 * 365);
```

### Option 2: 5 Years with Auto-Renewal (Best Practice)

```rust
// 5-year validity
params.not_after = now + time::Duration::days(5 * 365);

// Add renewal check on daemon startup
pub async fn ensure_certificate_valid() -> Result<()> {
    let cert_path = get_cert_dir().join("wildcard.pem");

    if !cert_path.exists() {
        // Generate new certificate
        return generate_wildcard_certificate_only().await;
    }

    // Check if certificate expires within 1 year
    let content = tokio::fs::read_to_string(&cert_path).await?;
    if needs_renewal(&content)? {
        log::info!("Certificate expiring soon, regenerating...");
        return generate_wildcard_certificate_only().await;
    }

    Ok(())
}

fn needs_renewal(cert_pem: &str) -> Result<bool> {
    let cert_der = pem::parse(cert_pem)?;
    let cert = x509_parser::parse_x509_certificate(cert_der.contents())?.1;

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let not_after = cert.validity().not_after.timestamp() as u64;

    // Renew if expires within 1 year
    const ONE_YEAR: u64 = 365 * 24 * 60 * 60;
    Ok(not_after - now < ONE_YEAR)
}
```

### Option 3: 90 Days with Automatic Rotation (Enterprise Grade)

```rust
// 90-day validity (Let's Encrypt style)
params.not_after = now + time::Duration::days(90);

// Auto-rotate every 60 days (30 days before expiry)
#[tokio::main]
async fn main() -> Result<()> {
    // Schedule certificate rotation task
    tokio::spawn(async {
        let mut interval = tokio::time::interval(Duration::from_secs(60 * 24 * 60 * 60));  // 60 days

        loop {
            interval.tick().await;

            if let Err(e) = rotate_certificate().await {
                log::error!("Certificate rotation failed: {}", e);
            }
        }
    });

    // ... rest of daemon ...
}

async fn rotate_certificate() -> Result<()> {
    log::info!("Rotating TLS certificate...");

    // Generate new certificate
    let new_cert = generate_wildcard_certificate_only().await?;

    // Import to system trust store
    let cert_path = get_cert_dir().join("wildcard.pem");
    import_certificate_to_system(&cert_path).await?;

    // Reload MCP server with new certificate
    reload_mcp_server().await?;

    log::info!("Certificate rotation complete");
    Ok(())
}
```

## Certificate Lifecycle Events to Log

```rust
// Add audit logging for certificate operations
log::info!(
    "Generated self-signed certificate for mcp.kodegen.ai. \
     Valid from: {} to: {}. \
     Validity period: {} days.",
    params.not_before,
    params.not_after,
    (params.not_after - params.not_before).whole_days()
);

// Log when certificate is nearing expiration
if days_until_expiry < 30 {
    log::warn!(
        "Certificate expires in {} days. Auto-renewal scheduled.",
        days_until_expiry
    );
}
```

## Compliance Considerations

### Current Code Comments Mention "Non-expiring"
```rust
// Line 188: Set non-expiring validity period (100 years)
```

This comment is misleading:
- Certificate DOES expire (in 100 years)
- "Non-expiring" suggests infinite validity
- Should say "Long-lived" or specify actual duration

### Better Documentation

```rust
/// Generate self-signed certificate with extended validity period.
///
/// # Validity Period
///
/// Default: 5 years (renewable automatically)
///
/// The certificate is valid for 5 years from generation. The daemon
/// automatically checks certificate validity on startup and rotates
/// certificates within 1 year of expiration.
///
/// This approach balances:
/// - User convenience (infrequent manual intervention)
/// - Security best practices (regular key rotation)
/// - System compatibility (avoids excessive validity periods)
///
/// For production deployments requiring shorter validity periods
/// (e.g., 90 days), set KODEGEN_CERT_VALIDITY_DAYS environment variable.
```

## Recommendation

**Change to 5-year validity with auto-renewal check on startup.**

This provides:
- ✅ Reasonable validity period (industry standard)
- ✅ Automatic renewal before expiry
- ✅ No user intervention needed
- ✅ Compliance with best practices
- ✅ Future-proof for enterprise adoption

## References
- CA/Browser Forum Baseline Requirements (825-day maximum for public certs)
- RFC 5280: Internet X.509 Public Key Infrastructure
- NIST SP 800-57: Key Management Guidelines
- Let's Encrypt Certificate Lifecycle
