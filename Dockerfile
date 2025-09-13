# ---- Build Stage ----
FROM rust:1.80 AS builder

WORKDIR /usr/src/app

# Copy only Cargo.toml and Cargo.lock first to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Pre-fetch dependencies
RUN cargo fetch --locked
RUN cargo build --release --locked || true

# Now copy the full source and build
COPY . .
RUN cargo build --release --locked

# ---- Runtime Stage ----
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      libpq5 \
      ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy binary only
COPY --from=builder /usr/src/app/target/release/wallet_service /usr/local/bin/

# Run as non-root for security
RUN useradd -m wallet
USER wallet

CMD ["wallet_service"]
