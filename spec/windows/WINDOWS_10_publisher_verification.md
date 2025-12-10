# Task: Implement Authenticode Publisher Verification

## Priority: P0 (Security Critical)

## Related Error
- `signing/windows.rs:20` - constant `EXPECTED_PUBLISHER` is never used

## Problem Statement

The Windows signing module has a stub for publisher verification:
```rust
const EXPECTED_PUBLISHER: &str = "Kodegen";

fn verify_publisher(_exe_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement publisher verification using CryptQueryObject
    Ok(())  // STUB - Always returns Ok!
}
```

This means ANY validly signed binary is accepted, not just Kodegen-signed binaries.

## Security Risk

Without publisher verification:
1. Attacker obtains any valid code signing certificate
2. Signs malicious kodegen binary
3. Downloads malicious binary passes `verify_signature()`
4. User installs compromised software

## Required Implementation

### 1. Extract Certificate from Signed Binary

Use `CryptQueryObject` to get certificate context:

```rust
use windows::Win32::Security::Cryptography::{
    CryptQueryObject, CERT_QUERY_OBJECT_FILE, CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED,
    CERT_QUERY_FORMAT_BINARY, CERT_CONTEXT,
};

fn get_signer_certificate(exe_path: &Path) -> Result<*const CERT_CONTEXT, ...> {
    let path_wide: Vec<u16> = exe_path.encode_utf16().chain(Some(0)).collect();

    let mut cert_store = std::ptr::null_mut();
    let mut msg = std::ptr::null_mut();
    let mut context = std::ptr::null();

    unsafe {
        CryptQueryObject(
            CERT_QUERY_OBJECT_FILE,
            path_wide.as_ptr() as *const _,
            CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED,
            CERT_QUERY_FORMAT_BINARY,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut cert_store,
            &mut msg,
            &mut context,
        )?;
    }

    // Extract signer certificate from message
    // ...
}
```

### 2. Extract Subject CN from Certificate

Use `CertGetNameStringW` to get the Common Name:

```rust
use windows::Win32::Security::Cryptography::{
    CertGetNameStringW, CERT_NAME_SIMPLE_DISPLAY_TYPE,
};

fn get_certificate_subject_cn(cert: *const CERT_CONTEXT) -> Result<String, ...> {
    let mut buffer = [0u16; 256];

    unsafe {
        let len = CertGetNameStringW(
            cert,
            CERT_NAME_SIMPLE_DISPLAY_TYPE,
            0,
            std::ptr::null_mut(),
            Some(&mut buffer),
        );

        if len == 0 {
            return Err("Failed to get certificate name");
        }

        Ok(String::from_utf16_lossy(&buffer[..len as usize - 1]))
    }
}
```

### 3. Compare Against Expected Publisher

```rust
fn verify_publisher(exe_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let cert = get_signer_certificate(exe_path)?;
    let subject_cn = get_certificate_subject_cn(cert)?;

    // Free certificate after extracting name
    unsafe { CertFreeCertificateContext(cert); }

    if subject_cn != EXPECTED_PUBLISHER {
        return Err(format!(
            "Binary signed by '{}', expected '{}'",
            subject_cn, EXPECTED_PUBLISHER
        ).into());
    }

    log::info!("Publisher verified: {}", subject_cn);
    Ok(())
}
```

### 4. Handle Edge Cases

- **Multiple signers**: Some binaries have multiple signatures (e.g., SHA-1 + SHA-256)
- **Timestamp signatures**: Counter-signatures for timestamping
- **Certificate chain**: May want to verify intermediate/root CA too

## Windows API Reference

Required APIs from `windows` crate:
```rust
use windows::Win32::Security::Cryptography::{
    // Query signed object
    CryptQueryObject,
    CERT_QUERY_OBJECT_FILE,
    CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED,
    CERT_QUERY_FORMAT_BINARY,

    // Certificate context
    CERT_CONTEXT,
    CertFreeCertificateContext,

    // Get certificate name
    CertGetNameStringW,
    CERT_NAME_SIMPLE_DISPLAY_TYPE,

    // Message handling (for extracting signer)
    CryptMsgGetParam,
    CMSG_SIGNER_CERT_INFO_PARAM,
    CertFindCertificateInStore,
};
```

## Files to Modify

- `src/signing/windows.rs`
  - Implement `verify_publisher()`
  - Add helper functions for certificate extraction
  - Use `EXPECTED_PUBLISHER` constant

## Testing

1. Test with Kodegen-signed binary - should pass
2. Test with other vendor's signed binary - should fail
3. Test with unsigned binary - `verify_signature()` should fail before reaching publisher check
4. Test with self-signed binary - should fail (not trusted + wrong publisher)

## Acceptance Criteria

- [ ] `EXPECTED_PUBLISHER` constant is used
- [ ] Publisher verification extracts certificate CN
- [ ] Non-Kodegen signed binaries are rejected
- [ ] Error messages clearly indicate publisher mismatch
- [ ] No dead code warnings
