# Builder Stage
FROM rust:latest AS builder

WORKDIR /opt/lilac/api

# Install the application dependencies
COPY ./backend/ ./
COPY ./backend/.sqlx .sqlx

RUN mkdir build/

# Set SQLX to offline mode for the build
ENV SQLX_OFFLINE=true
RUN cargo build --package controlplane-api --target-dir ./build/

# Final Stage
FROM rust:latest

WORKDIR /opt/lilac/api

# Install Helm
RUN apt-get update && \
    apt-get install -y curl && \
    curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 && \
    chmod 700 get_helm.sh && \
    ./get_helm.sh

COPY --from=builder /opt/lilac/api/build/debug/controlplane-api ./controlplane-api
RUN groupadd --system appgroup && \
    useradd --system --gid appgroup --no-create-home appuser

# Set user and home directory for helm
ENV XDG_CONFIG_HOME=/tmp/.config
ENV XDG_CACHE_HOME=/tmp/.cache
USER appuser

EXPOSE 8081

CMD [ "./controlplane-api" ]