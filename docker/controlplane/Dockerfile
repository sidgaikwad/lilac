# Builder Stage
FROM rust:latest AS builder

WORKDIR /opt/lilac/api

# Install the application dependencies
COPY ./backend/Cargo.toml ./backend/Cargo.lock ./
COPY ./backend/controlplane-api/ ./controlplane-api/
COPY ./backend/common/ ./common/
COPY ./backend/.sqlx .sqlx

RUN mkdir build/

# Set SQLX to offline mode for the build
ENV SQLX_OFFLINE=true
RUN cargo build --package controlplane-api --target-dir ./build/

# Final Stage
FROM rust:latest

WORKDIR /opt/lilac/api
COPY --from=builder /opt/lilac/api/build/debug/controlplane-api ./controlplane-api
RUN groupadd --system appgroup && \
    useradd --system --gid appgroup --no-create-home appuser
USER appuser

EXPOSE 8081

CMD [ "./controlplane-api" ]