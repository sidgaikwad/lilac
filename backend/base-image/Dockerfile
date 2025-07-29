# --- Builder Stage ---
# This stage compiles the lilac-agent binary
FROM rust:1-bullseye AS builder

# Docker buildx provides this argument automatically
ARG TARGETARCH

# Install cross-compilation tools for ARM64
RUN apt-get update && apt-get install -y gcc-aarch64-linux-gnu

# Add both target toolchains
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add aarch64-unknown-linux-gnu

# Copy the agent source code
COPY agent /usr/src/lilac-agent

# Build the agent in release mode
WORKDIR /usr/src/lilac-agent

# Select the correct target and linker based on the build architecture,
# then copy the final binary to a standard location.
RUN case ${TARGETARCH} in \
        "amd64") \
            cargo build --release --target=x86_64-unknown-linux-gnu && \
            cp target/x86_64-unknown-linux-gnu/release/lilac-agent /usr/src/lilac-agent/release-binary \
            ;; \
        "arm64") \
            CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
            cargo build --release --target=aarch64-unknown-linux-gnu && \
            cp target/aarch64-unknown-linux-gnu/release/lilac-agent /usr/src/lilac-agent/release-binary \
            ;; \
    esac

# --- Final Stage ---
# This is the final, clean base image
# We use a Python slim image as a base to provide a good starting point for ML jobs
FROM python:3.10-slim-bullseye

# Copy the compiled agent binary from the builder stage
# We copy the statically-linked musl build to ensure it runs on any x86_64 Linux kernel
# without external dependencies.
COPY --from=builder /usr/src/lilac-agent/release-binary /usr/local/bin/lilac-agent

# This image has no entrypoint. The entrypoint will be defined by the
# final user-facing image.