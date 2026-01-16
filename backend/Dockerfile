FROM rust:1.75 as builder

WORKDIR /app

# Install FFmpeg
RUN apt-get update && apt-get install -y \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY backend/Cargo.toml backend/Cargo.lock* ./
COPY backend/src ./src

# Build the application
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

# Install FFmpeg runtime
RUN apt-get update && apt-get install -y \
    ffmpeg \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/timelapse-backend /app/timelapse-backend

# Create temp directory
RUN mkdir -p /tmp/timelapse

EXPOSE 8080

CMD ["/app/timelapse-backend"]
