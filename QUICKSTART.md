# Quick Start Guide

## Prerequisites

- Node.js 18+ and npm
- Rust 1.75+ and Cargo (see installation instructions below)
- FFmpeg installed on your system

### Installing Rust and Cargo

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
Download and run the installer from https://rustup.rs/

After installation, verify:
```bash
rustc --version
cargo --version
```

### Installing Rust and Cargo

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
Download and run the installer from https://rustup.rs/

After installation, verify:
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

## Running Locally

### 1. Start the Backend

```bash
cd backend
cargo run
```

The backend will start on `http://localhost:8080`

### 2. Start the Frontend

In a new terminal:

```bash
cd frontend
npm install
npm run dev
```

The frontend will start on `http://localhost:3000`

### 3. Use the Application

1. Open `http://localhost:3000` in your browser
2. Click to select image files (PNG, JPEG, or WebP)
3. Wait for upload to complete
4. Preview the middle frame
5. Adjust rotation if needed (0°, 90°, 180°, 270°)
6. Set frame rate (FPS) - default is 30
7. Click "Create Timelapse"
8. Wait for processing (this may take a while for large image sets)
9. Download your video when ready

## Environment Variables

### Frontend

Create `frontend/.env.local`:
```
NEXT_PUBLIC_API_URL=http://localhost:8080
```

For production, set this to your backend URL.

### Backend

Create `backend/.env` (optional):
```
PORT=8080
TEMP_DIR=/tmp/timelapse
```

## Building for Production

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

## Docker Deployment

### Backend

```bash
cd backend
docker build -t timelapse-backend .
docker run -p 8080:8080 timelapse-backend
```

## Troubleshooting

### Cargo command not found

If you see `zsh: command not found: cargo build`, you need to install Rust:
```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Then verify installation
cargo --version
```

### FFmpeg not found

Make sure FFmpeg is installed and available in your PATH:
```bash
ffmpeg -version
```

### Port already in use

Change the port in `backend/src/main.rs` or set `PORT` environment variable.

### Upload fails

- Check that the backend is running
- Verify `NEXT_PUBLIC_API_URL` is correct
- Check browser console for errors
- Ensure sufficient disk space in `/tmp/timelapse`

### Video processing fails

- Verify FFmpeg is installed correctly
- Check that image files are valid
- Ensure sufficient disk space
- Check backend logs for FFmpeg error messages
