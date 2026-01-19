# MePassa Bootstrap Node Dockerfile

FROM rust:1.75-slim-bookworm as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY core ./core
COPY server/bootstrap ./server/bootstrap

# Build bootstrap node
RUN cargo build --release --package mepassa-bootstrap

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/mepassa-bootstrap /usr/local/bin/mepassa-bootstrap

# Create data directory
RUN mkdir -p /app/data

EXPOSE 4001

ENV RUST_LOG=info
ENV BOOTSTRAP_PORT=4001
ENV DATA_DIR=/app/data

CMD ["mepassa-bootstrap"]
