#!/bin/bash

echo "Checking for hardcoded path separators..."
echo

# Find hardcoded forward slashes (Unix paths)
echo "=== Potential hardcoded Unix paths ==="
rg '"/"' packages/ --type rust \
  | grep -v "http://" \
  | grep -v "https://" \
  | grep -v "file://" \
  | grep -v "\\\\/" \
  | head -20

echo
echo "=== Potential hardcoded Windows paths ==="
# Find hardcoded backslashes
rg '"\\\\"' packages/ --type rust | head -20

echo
echo "=== Recommended pattern ==="
cat << 'EOF'
// GOOD - Platform-agnostic
let path = PathBuf::from(base).join("subdir").join("file.txt");

// BAD - Hardcoded separator
let path = format!("{}/subdir/file.txt", base);
let path = format!("{}\\subdir\\file.txt", base);
EOF
