# --- Builder Stage ---
# This stage compiles the lilac_cli binary for amd64.
FROM rust:1.88 AS builder

# Install the C build toolchain and the x86_64-linux-gnu cross-compiler
RUN apt-get update && apt-get install -y build-essential gcc-x86-64-linux-gnu

# Add the x86_64 target
RUN rustup target add x86_64-unknown-linux-gnu

# Set the linker for the target architecture
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc

# Copy the agent source code
WORKDIR /usr/src/lilac
COPY ../../agent .

# Build the agent in release mode
RUN cargo build --release --target=x86_64-unknown-linux-gnu

# --- Final Stage ---
# This is the final, "batteries-included" image.
FROM nvidia/cuda:12.4.0-runtime-ubuntu22.04

# Set environment variables to avoid interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies: Docker, NVIDIA Container Toolkit, Python, etc.
RUN apt-get update && apt-get install -y \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg \
    lsb-release \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Add Docker's official GPG key
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

# Set up the stable Docker repository
RUN echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null

# Install Docker Engine
RUN apt-get update && apt-get install -y docker-ce docker-ce-cli containerd.io

# Install the NVIDIA Container Toolkit
RUN curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg \
  && curl -s -L https://nvidia.github.io/libnvidia-container/stable/ubuntu22.04/nvidia-container-toolkit.list | \
    sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' | \
    tee /etc/apt/sources.list.d/nvidia-container-toolkit.list
RUN apt-get update && apt-get install -y nvidia-container-toolkit

# Configure the Docker daemon to use the NVIDIA runtime
RUN nvidia-ctk runtime configure --runtime=docker

# Copy the compiled lilac_cli binary from the builder stage
COPY --from=builder /usr/src/lilac/target/x86_64-unknown-linux-gnu/release/lilac_cli /usr/local/bin/lilac_cli

# Copy the entrypoint script
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh


# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]