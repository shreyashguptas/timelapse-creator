# Stage 1: Builder with Alpine
FROM rust:1.83-alpine AS builder

WORKDIR /app

# Install musl build dependencies
RUN apk add --no-cache musl-dev

# Copy and build dependencies first (for caching)
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source and rebuild
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Stage 2: Minimal Alpine runtime
FROM alpine:3.19

WORKDIR /app

# Install ONLY runtime dependencies
RUN apk add --no-cache \
    ffmpeg \
    ca-certificates \
    curl

# Copy the optimized binary (already stripped via Cargo profile)
COPY --from=builder /app/target/release/timelapse-backend /app/timelapse-backend

RUN mkdir -p /tmp/timelapse

EXPOSE 8080

CMD ["/app/timelapse-backend"]
