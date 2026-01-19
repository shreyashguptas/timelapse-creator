FROM rust:1.83 as builder

WORKDIR /app

# Install FFmpeg
RUN apt-get update && apt-get install -y \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# Build the application
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

# Install FFmpeg runtime and curl for healthcheck
RUN apt-get update && apt-get install -y \
    ffmpeg \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/timelapse-backend /app/timelapse-backend

# Create temp directory
RUN mkdir -p /tmp/timelapse

EXPOSE 8080

CMD ["/app/timelapse-backend"]
