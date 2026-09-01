# Multi-stage Dockerfile for Trading Lab Backend service
# Reference: rust:1-bookworm builder -> debian:bookworm-slim runtime

FROM rust:1-bookworm AS builder
WORKDIR /build

# Copy manifest and source files needed for compilation
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Copy migrations so sqlx::migrate!() can embed SQL migration files at compile-time
COPY migrations ./migrations

# Build the release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime

# Install CA certificates for secure external API calls (e.g. Sectors.app)
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled backend binary from the builder stage
COPY --from=builder /build/target/release/trading_lab_backend /usr/local/bin/trading_lab_backend

# Expose default API server port
EXPOSE 8000

# Run the backend service
ENTRYPOINT ["/usr/local/bin/trading_lab_backend"]
