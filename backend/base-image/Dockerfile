# --- Builder Stage ---
# This stage compiles the lilac-agent binary
FROM rust:latest AS builder

# Copy the agent source code
COPY agent /usr/src/lilac-agent

# Build the agent in release mode
WORKDIR /usr/src/lilac-agent
RUN cargo build --release

# --- Final Stage ---
# This is the final, clean base image
# We use a Python slim image as a base to provide a good starting point for ML jobs
FROM python:3.10-slim-bullseye

# Install necessary certificates for HTTPS communication
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled agent binary from the builder stage
COPY --from=builder /usr/src/lilac-agent/target/release/lilac-agent /usr/local/bin/lilac-agent

# Set the entrypoint for the container. When a user's image is built on top of this,
# their CMD will be passed as arguments to our agent's `run` command.
ENTRYPOINT ["/usr/local/bin/lilac-agent", "run", "--"]