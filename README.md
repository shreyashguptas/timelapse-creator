# Timelapse Creator

A web application for creating high-quality timelapse videos from image frames. Upload your images, preview and rotate them, then generate a professional MP4 video suitable for YouTube and video editing.

## Features

- Upload multiple image files (PNG, JPEG, WebP)
- Preview middle frame with rotation controls
- Adjustable frame rate (FPS)
- High-quality MP4 output (H.264, CRF 18)
- Real-time processing status
- Direct download of generated videos

## Architecture

- **Frontend**: Next.js 16 with React and Tailwind CSS
- **Backend**: Rust with Actix Web for high-performance video processing
- **Video Processing**: FFmpeg for timelapse generation

## Prerequisites

- Node.js 18+ and npm
- Rust 1.75+ and Cargo (install from https://rustup.rs/)
- FFmpeg installed on your system

**Quick Install Commands:**

```bash
# Install Rust (macOS/Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install FFmpeg (macOS)
brew install ffmpeg

# Verify installations
cargo --version
ffmpeg -version
```

## Setup

### Frontend

```bash
cd frontend
npm install
npm run dev
```

The frontend will run on `http://localhost:3000`

### Backend

```bash
cd backend
cargo build --release
cargo run
```

The backend will run on `http://localhost:8080`

Make sure to set the `NEXT_PUBLIC_API_URL` environment variable in `frontend/.env.local` to point to your backend URL.

## Usage

1. Start both frontend and backend servers
2. Open the frontend in your browser
3. Click to select image files (or select a folder)
4. Wait for upload to complete
5. Preview the middle frame and adjust rotation if needed
6. Set your desired frame rate (FPS)
7. Click "Create Timelapse"
8. Wait for processing to complete
9. Download your video

## Deployment

### Frontend (Vercel)

1. Connect your repository to Vercel
2. Set `NEXT_PUBLIC_API_URL` environment variable to your backend URL
3. Deploy

### Backend

The backend requires:
- FFmpeg installed
- Sufficient disk space for temporary file storage
- Port 8080 accessible (or configure as needed)

You can deploy using:
- Docker (see `backend/Dockerfile`)
- Railway, Fly.io, or any VPS with Rust support

## Technical Details

- Video format: MP4 (H.264, CRF 18, yuv420p)
- Rotation: 90°, 180°, 270° via FFmpeg transpose filters
- File storage: Local filesystem (temporary, cleaned up after 24 hours)
- Maximum compatibility: MP4 format works with all major video editors and YouTube

## License

MIT
