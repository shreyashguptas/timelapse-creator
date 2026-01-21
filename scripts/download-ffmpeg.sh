#!/bin/bash
set -e
mkdir -p src-tauri/binaries

ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
    TARGET="aarch64-apple-darwin"
else
    TARGET="x86_64-apple-darwin"
fi

if [ ! -f "src-tauri/binaries/ffmpeg-$TARGET" ]; then
    echo "Downloading FFmpeg..."
    curl -L "https://evermeet.cx/ffmpeg/ffmpeg-7.1.1.zip" -o /tmp/ffmpeg.zip
    unzip -o /tmp/ffmpeg.zip -d /tmp
    mv /tmp/ffmpeg "src-tauri/binaries/ffmpeg-$TARGET"
    chmod +x "src-tauri/binaries/ffmpeg-$TARGET"
    rm /tmp/ffmpeg.zip
fi
echo "FFmpeg ready: src-tauri/binaries/ffmpeg-$TARGET"
