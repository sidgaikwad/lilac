# --- Builder Stage ---
# This stage compiles the lilac binary for multiple architectures.
FROM rust:1.82-bookworm AS builder

# Docker buildx provides this argument automatically.
# It will be "amd64" or "arm64".
ARG TARGETARCH

# Install cross-compilation tools for ARM64 if needed.
RUN apt-get update && apt-get install -y gcc-aarch64-linux-gnu

# Add both target toolchains to rustup.
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add aarch64-unknown-linux-gnu

# Copy the agent source code.
WORKDIR /usr/src/lilac
COPY . .

# Build the agent in release mode, selecting the correct target
# and linker based on the build architecture.
RUN case ${TARGETARCH} in \
        "amd64") \
            cargo build --release --target=x86_64-unknown-linux-gnu && \
            cp target/x86_64-unknown-linux-gnu/release/lilac /release-binary \
            ;; \
        "arm64") \
            CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
            cargo build --release --target=aarch64-unknown-linux-gnu && \
            cp target/aarch64-unknown-linux-gnu/release/lilac /release-binary \
            ;; \
    esac

# --- Final Stage ---
# This is the final, minimal image.
FROM debian:bookworm-slim

# Copy the compiled binary from the builder stage.
COPY --from=builder /release-binary /usr/local/bin/lilac

# Set a default command. This can be overridden when running the container.
CMD ["/usr/local/bin/lilac", "agent", "start"]