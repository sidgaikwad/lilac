#!/bin/sh
set -e

# Clean up any stale PID files
rm -f /var/run/docker.pid

# Start the Docker daemon in the background
# The --storage-driver is recommended for Docker-in-Docker
dockerd --storage-driver=vfs &

# Wait for the Docker daemon to be ready
echo "Waiting for Docker daemon to start..."
timeout=20
while ! docker info > /dev/null 2>&1; do
    if [ "$timeout" -le 0 ]; then
        echo "Docker daemon failed to start in time."
        exit 1
    fi
    echo "Waiting for the Docker daemon to be available... ($timeout seconds left)"
    sleep 1
    timeout=$((timeout - 1))
done
echo "Docker daemon started successfully."

# Wait for the NVIDIA runtime to be ready
echo "Waiting for NVIDIA runtime to be available..."
timeout=20
while ! nvidia-smi > /dev/null 2>&1; do
    if [ "$timeout" -le 0 ]; then
        echo "NVIDIA runtime failed to become available in time."
        # We can still continue without GPU support, the agent will just report no GPUs.
        break
    fi
    echo "Waiting for NVIDIA runtime... ($timeout seconds left)"
    sleep 1
    timeout=$((timeout - 1))
done
echo "NVIDIA runtime is available."


# Run the Lilac agent
exec /usr/local/bin/lilac agent start