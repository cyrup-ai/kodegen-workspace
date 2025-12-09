# Justfile for kodegen-native-permissions
# Multi-platform testing for Linux permission APIs

# Docker cache volumes for faster builds
CARGO_CACHE_VOLUME := "kodegen-permissions-cargo-cache"
TARGET_CACHE_VOLUME := "kodegen-permissions-target-cache"

# Create cache volumes (one-time setup)
create-cache-volumes:
    @echo "Creating Docker cache volumes..."
    @docker volume create {{CARGO_CACHE_VOLUME}} || true
    @docker volume create {{TARGET_CACHE_VOLUME}} || true
    @echo "✓ Cache volumes ready"

# Build and test on current platform (macOS)
test-macos:
    @echo "🍎 Testing on macOS..."
    cargo check
    cargo clippy -- -D warnings
    cargo test

# Build and test in Linux container
test-linux:
    #!/usr/bin/env bash
    set -e
    echo "🐧 Testing on Linux in Docker..."

    # Build Docker image if it doesn't exist or Dockerfile changed
    IMAGE_NAME="kodegen-permissions-tester"
    if ! docker images | grep -q "$IMAGE_NAME"; then
        echo "Building Docker image..."
        docker build -t $IMAGE_NAME .devcontainer/
    else
        echo "Using existing Docker image (run 'just rebuild-image' to force rebuild)"
    fi

    # Run tests in container
    docker run --rm \
        -v kodegen-permissions-cargo-cache:/home/builder/.cargo \
        -v kodegen-permissions-target-cache:/cache/target \
        -v "$(pwd)/../..:/workspace" \
        -w /workspace/packages/kodegen-native-permissions \
        -e CARGO_TARGET_DIR=/cache/target \
        $IMAGE_NAME \
        bash -c "cargo check && cargo clippy -- -D warnings && cargo test"

# Rebuild Docker image (force rebuild)
rebuild-image:
    @echo "🔨 Rebuilding Docker image..."
    docker build --no-cache -t kodegen-permissions-tester .devcontainer/

# Test compilation only (faster)
check-linux:
    #!/usr/bin/env bash
    set -e
    echo "🔍 Checking compilation on Linux..."

    IMAGE_NAME="kodegen-permissions-tester"
    if ! docker images | grep -q "$IMAGE_NAME"; then
        echo "Building Docker image..."
        docker build -t $IMAGE_NAME .devcontainer/
    fi

    docker run --rm \
        -v kodegen-permissions-cargo-cache:/home/builder/.cargo \
        -v kodegen-permissions-target-cache:/cache/target \
        -v "$(pwd)/../..:/workspace" \
        -w /workspace/packages/kodegen-native-permissions \
        -e CARGO_TARGET_DIR=/cache/target \
        $IMAGE_NAME \
        bash -c "cargo check"

# Run clippy on Linux
clippy-linux:
    #!/usr/bin/env bash
    set -e
    echo "📎 Running clippy on Linux..."

    IMAGE_NAME="kodegen-permissions-tester"
    if ! docker images | grep -q "$IMAGE_NAME"; then
        echo "Building Docker image..."
        docker build -t $IMAGE_NAME .devcontainer/
    fi

    docker run --rm \
        -v kodegen-permissions-cargo-cache:/home/builder/.cargo \
        -v kodegen-permissions-target-cache:/cache/target \
        -v "$(pwd)/../..:/workspace" \
        -w /workspace/packages/kodegen-native-permissions \
        -e CARGO_TARGET_DIR=/cache/target \
        $IMAGE_NAME \
        bash -c "cargo clippy -- -D warnings"

# Build and test Windows cross-compilation
test-windows:
    #!/usr/bin/env bash
    set -e
    echo "🪟 Testing Windows cross-compilation in Docker..."

    IMAGE_NAME="kodegen-permissions-tester"
    if ! docker images | grep -q "$IMAGE_NAME"; then
        echo "Building Docker image..."
        docker build -t $IMAGE_NAME .devcontainer/
    fi

    # Cross-compile for Windows
    docker run --rm \
        -v kodegen-permissions-cargo-cache:/home/builder/.cargo \
        -v kodegen-permissions-target-cache:/cache/target \
        -v "$(pwd)/../..:/workspace" \
        -w /workspace/packages/kodegen-native-permissions \
        -e CARGO_TARGET_DIR=/cache/target \
        $IMAGE_NAME \
        bash -c "cargo check --target x86_64-pc-windows-gnu && cargo clippy --target x86_64-pc-windows-gnu -- -D warnings"

# Check Windows compilation only
check-windows:
    #!/usr/bin/env bash
    set -e
    echo "🔍 Checking Windows cross-compilation..."

    IMAGE_NAME="kodegen-permissions-tester"
    if ! docker images | grep -q "$IMAGE_NAME"; then
        echo "Building Docker image..."
        docker build -t $IMAGE_NAME .devcontainer/
    fi

    docker run --rm \
        -v kodegen-permissions-cargo-cache:/home/builder/.cargo \
        -v kodegen-permissions-target-cache:/cache/target \
        -v "$(pwd)/../..:/workspace" \
        -w /workspace/packages/kodegen-native-permissions \
        -e CARGO_TARGET_DIR=/cache/target \
        $IMAGE_NAME \
        bash -c "cargo check --target x86_64-pc-windows-gnu"

# Test all platforms
test-all:
    @echo "🌍 Testing on all platforms..."
    just test-macos
    just test-linux
    just test-windows

# Clean up Docker resources
clean-docker:
    @echo "🧹 Cleaning up Docker resources..."
    docker rmi -f kodegen-permissions-tester || true
    docker system prune -f

# Interactive Linux shell (for debugging)
shell-linux:
    #!/usr/bin/env bash
    set -e
    echo "🐚 Opening interactive Linux shell..."

    IMAGE_NAME="kodegen-permissions-tester"
    if ! docker images | grep -q "$IMAGE_NAME"; then
        echo "Building Docker image..."
        docker build -t $IMAGE_NAME .devcontainer/
    fi

    docker run --rm -it \
        -v kodegen-permissions-cargo-cache:/home/builder/.cargo \
        -v kodegen-permissions-target-cache:/cache/target \
        -v "$(pwd)/../..:/workspace" \
        -w /workspace/packages/kodegen-native-permissions \
        -e CARGO_TARGET_DIR=/cache/target \
        $IMAGE_NAME \
        bash

# Show help
help:
    @echo "kodegen-native-permissions testing commands:"
    @echo ""
    @echo "  just test-macos            - Test on macOS (current platform)"
    @echo "  just test-linux            - Test on Linux (Docker)"
    @echo "  just test-windows          - Test Windows cross-compilation (Docker)"
    @echo "  just test-all              - Test on all platforms"
    @echo ""
    @echo "  just check-linux           - Quick compilation check on Linux"
    @echo "  just check-windows         - Quick compilation check on Windows"
    @echo "  just clippy-linux          - Run clippy on Linux"
    @echo ""
    @echo "  just create-cache-volumes  - Create Docker cache volumes (one-time)"
    @echo "  just rebuild-image         - Force rebuild Docker image"
    @echo "  just clean-docker          - Remove Docker image and clean up"
    @echo "  just shell-linux           - Open interactive Linux shell"
