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
- **Deployment**: Docker Compose with Nginx reverse proxy

## Quick Start with Docker (Recommended)

This is the easiest way to run the application.

### Prerequisites

- Docker and Docker Compose installed

### Starting the Application

```bash
docker-compose up -d --build
```

This builds and starts all services (backend, frontend, nginx) in the background.

### Accessing the App

Open http://localhost in your browser.

### Stopping the Application

```bash
docker-compose down
```

### Useful Docker Commands

**View logs:**
```bash
docker-compose logs -f
```

**Check status:**
```bash
docker-compose ps
```

## Updating After Code Changes

When you make changes to the code, you need to rebuild the containers.

### Standard Update

```bash
docker-compose down
docker-compose up -d --build
```

### Full Rebuild (Clean Slate)

If you encounter issues or want to start completely fresh:

```bash
docker-compose down -v --rmi local
docker-compose up -d --build
```

This removes volumes and locally-built images before rebuilding.

## Local Development (Alternative)

For development without Docker, you can run the services directly.

### Prerequisites

- Node.js 18+ and npm
- Rust 1.75+ and Cargo
- FFmpeg installed on your system

### Installing Rust

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
Download and run the installer from https://rustup.rs/

Verify installation:
```bash
rustc --version
cargo --version
```

### Installing FFmpeg

**macOS:**
```bash
brew install ffmpeg
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install ffmpeg
```

**Windows:**
Download from https://ffmpeg.org/download.html or use chocolatey:
```bash
choco install ffmpeg
```

### Running the Backend

```bash
cd backend
cargo run
```

The backend runs on http://localhost:8080

### Running the Frontend

In a new terminal:

```bash
cd frontend
npm install
npm run dev
```

The frontend runs on http://localhost:3000

### Environment Variables

**Frontend** - Create `frontend/.env.local`:
```
NEXT_PUBLIC_API_URL=http://localhost:8080
```

**Backend** - Create `backend/.env` (optional):
```
PORT=8080
TEMP_DIR=/tmp/timelapse
```

## Building for Production (without Docker)

### Frontend

```bash
cd frontend
npm run build
npm start
```

### Backend

```bash
cd backend
cargo build --release
./target/release/timelapse-backend
```

## Usage Workflow

1. Start the application (Docker or local development)
2. Open the app in your browser
3. Click to select image files (PNG, JPEG, or WebP)
4. Wait for upload to complete
5. Preview the middle frame
6. Adjust rotation if needed (0°, 90°, 180°, 270°)
7. Set frame rate (FPS) - default is 30
8. Click "Create Timelapse"
9. Wait for processing to complete
10. Download your video

## Troubleshooting

### Cargo command not found

Install Rust and Cargo:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
cargo --version
```

### FFmpeg not found

Ensure FFmpeg is installed and available in your PATH:
```bash
ffmpeg -version
```

### Port already in use

- For Docker: Change the port mapping in `docker-compose.yml`
- For local development: Change the port in `backend/src/main.rs` or set `PORT` environment variable

### Upload fails

- Check that the backend is running
- Verify `NEXT_PUBLIC_API_URL` is correct (for local development)
- Check browser console for errors
- Ensure sufficient disk space

### Video processing fails

- Verify FFmpeg is installed correctly
- Check that image files are valid
- Ensure sufficient disk space
- Check backend logs for FFmpeg error messages

### Docker issues

**Containers won't start:**
```bash
docker-compose logs
```

**Need a clean rebuild:**
```bash
docker-compose down -v --rmi local
docker-compose up -d --build
```

## Technical Details

- Video format: MP4 (H.264, CRF 18, yuv420p)
- Rotation: 90°, 180°, 270° via FFmpeg transpose filters
- File storage: Local filesystem (temporary, cleaned up after 24 hours)
- Maximum compatibility: MP4 format works with all major video editors and YouTube

## License

MIT
