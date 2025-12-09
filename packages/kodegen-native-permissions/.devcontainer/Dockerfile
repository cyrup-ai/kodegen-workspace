# Multi-stage Dockerfile for kodegen-native-permissions testing
# Simplified from kodegen-bundler-bundle - focuses on Linux testing only

# ============================================================================
# Stage 1: base-packages
# Quick test: package installation
# ============================================================================
FROM rust:latest AS base-packages

# Cache-busting argument - change this when updating system dependencies
ARG DEPS_VERSION=2025-12-08
RUN echo "Building with dependencies version: ${DEPS_VERSION}"

# Install system dependencies for Linux + Windows cross-compilation
RUN apt-get update && apt-get install -y \
    # Build essentials
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    curl \
    wget \
    # Windows cross-compilation toolchain
    gcc-mingw-w64-x86-64 \
    g++-mingw-w64-x86-64 \
    # D-Bus development libraries (required by ashpd via zbus)
    libdbus-1-dev \
    # D-Bus daemon for runtime testing
    dbus \
    # General utilities
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# ============================================================================
# Stage 2: user-setup
# Create build user with proper permissions
# ============================================================================
FROM base-packages AS user-setup

ARG USER_UID=1000
ARG USER_GID=1000

RUN groupadd --gid $USER_GID builder \
    && useradd --uid $USER_UID --gid $USER_GID -m builder \
    && apt-get update \
    && apt-get install -y sudo \
    && echo builder ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/builder \
    && chmod 0440 /etc/sudoers.d/builder \
    && rm -rf /var/lib/apt/lists/*

USER builder
ENV HOME=/home/builder
ENV CARGO_HOME=$HOME/.cargo
ENV PATH=$HOME/.cargo/bin:$PATH

# ============================================================================
# Stage 3: rust-toolchain
# Install Rust toolchain and components as builder user
# ============================================================================
FROM user-setup AS rust-toolchain

# Install Rust as builder user (no root needed)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly \
    && . $HOME/.cargo/env \
    && rustup component add rustfmt clippy \
    && rustup target add \
    x86_64-apple-darwin \
    wasm32-unknown-unknown \
    x86_64-unknown-linux-gnu \
    aarch64-unknown-linux-gnu \
    x86_64-pc-windows-msvc \
    x86_64-pc-windows-gnu \
    aarch64-pc-windows-msvc

# Verify Rust installation
RUN rustc --version && cargo --version

# ============================================================================
# Stage 4: permissions-tester (default)
# Complete build environment ready for kodegen-native-permissions testing
# ============================================================================
FROM rust-toolchain AS permissions-tester

ENV CARGO_HOME=$HOME/.cargo
ENV PATH=$HOME/.cargo/bin:$PATH

# Create cache directory for cargo target with builder ownership
USER root
RUN mkdir -p /cache/target && chown -R builder:builder /cache
USER builder

# Set working directory for runtime
WORKDIR /workspace

# Final verification
RUN rustc --version && \
    cargo --version && \
    dpkg -l | grep libdbus

# Build instructions:
# - Stage testing: docker build --target=<stage> -t kodegen-perms-test .
# - Full image: docker build -t kodegen-permissions-tester .
