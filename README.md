# Timelapse Creator

A desktop application for creating high-quality timelapse videos from image frames. Upload your images, preview and rotate them, then generate a professional MP4 video suitable for YouTube and video editing.

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
- **Desktop App**: Tauri v2 for native macOS/Windows builds
- **Video Processing**: FFmpeg for timelapse generation

## Desktop App (Tauri)

The application is built as a native desktop app using Tauri. This creates standalone installers that work completely offline.

### Prerequisites

- Node.js 18+ and npm
- Rust 1.75+ and Cargo
- FFmpeg installed on your system (the app uses system FFmpeg)
- Tauri CLI: `cargo install tauri-cli`

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

### Development Mode

Run the desktop app in development mode with hot-reload:

```bash
# Terminal 1: Start frontend dev server
cd frontend && npm run dev

# Terminal 2: Start Tauri dev mode
cargo tauri dev
```

### Building for Release

Build production installers for your current platform:

```bash
# From project root
cargo tauri build
```

**Output locations:**
- macOS: `src-tauri/target/release/bundle/macos/Timelapse Creator.app`
- macOS DMG: `src-tauri/target/release/bundle/dmg/Timelapse Creator_<version>_aarch64.dmg`
- Windows: `src-tauri/target/release/bundle/msi/Timelapse Creator_<version>_x64.msi`

### Changing Version Numbers

Update the version in `src-tauri/tauri.conf.json`:

```json
{
  "version": "1.0.0"
}
```

### GitHub Releases Setup

To automate builds and releases via GitHub Actions:

#### 1. Generate Signing Keys (for auto-updates)

```bash
cargo tauri signer generate -w ~/.tauri/timelapse-creator.key
```

Save the public key displayed in the terminal.

#### 2. Add GitHub Secrets

Go to your repository Settings > Secrets and variables > Actions, and add:

- `TAURI_SIGNING_PRIVATE_KEY`: Contents of `~/.tauri/timelapse-creator.key`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`: Password used when generating the key

#### 3. Create GitHub Actions Workflow

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      fail-fast: false
      matrix:
        include:
          - platform: 'macos-latest'
            args: '--target aarch64-apple-darwin'
          - platform: 'windows-latest'
            args: ''

    runs-on: ${{ matrix.platform }}

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install Rust stable
        uses: dtolnay/rust-action@stable
        with:
          targets: ${{ matrix.platform == 'macos-latest' && 'aarch64-apple-darwin' || '' }}

      - name: Install frontend dependencies
        run: cd frontend && npm ci

      - name: Install Tauri CLI
        run: cargo install tauri-cli

      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}
          TAURI_SIGNING_PRIVATE_KEY_PASSWORD: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY_PASSWORD }}
        with:
          tagName: ${{ github.ref_name }}
          releaseName: 'Timelapse Creator ${{ github.ref_name }}'
          releaseBody: 'See the assets to download for your platform.'
          releaseDraft: true
          prerelease: false
          args: ${{ matrix.args }}
```

#### 4. Creating a Release

```bash
# 1. Update version in src-tauri/tauri.conf.json
# 2. Commit changes
git add -A
git commit -m "Bump version to 1.1.0"

# 3. Create and push a version tag
git tag v1.1.0
git push origin v1.1.0

# 4. GitHub Actions will automatically build and create a draft release
# 5. Go to GitHub Releases, review the draft, and publish it
```

### Manual Release (Without GitHub Actions)

If you prefer to build and upload manually:

```bash
# 1. Build the app
cargo tauri build

# 2. Go to GitHub > Releases > Create new release
# 3. Create a new tag (e.g., v1.0.0)
# 4. Upload the files from:
#    - src-tauri/target/release/bundle/dmg/*.dmg (macOS)
#    - src-tauri/target/release/bundle/msi/*.msi (Windows)
# 5. Publish the release
```

### Installing the Desktop App

**macOS:**
1. Download the `.dmg` file from GitHub Releases
2. Open the DMG and drag "Timelapse Creator" to Applications
3. First launch: Right-click > Open (to bypass Gatekeeper)

**Windows:**
1. Download the `.msi` file from GitHub Releases
2. Double-click to install
3. If Windows Defender SmartScreen appears, click "More info" > "Run anyway"

## Local Development

For development, you can run the services directly without Tauri.

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

## Usage Workflow

1. Start the application
2. Open the app
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

Change the port in `backend/src/main.rs` or set `PORT` environment variable.

### Upload fails

- Check that the backend is running
- Verify `NEXT_PUBLIC_API_URL` is correct
- Check browser console for errors
- Ensure sufficient disk space

### Video processing fails

- Verify FFmpeg is installed correctly
- Check that image files are valid
- Ensure sufficient disk space
- Check backend logs for FFmpeg error messages

### App won't open on macOS

- Right-click the app and select "Open" instead of double-clicking
- Or go to System Preferences > Security & Privacy and click "Open Anyway"

### Upload fails in desktop app

- Ensure FFmpeg is installed on your system: `ffmpeg -version`
- Check that you have read/write permissions for the selected files

### Build fails with "icon not found"

- Ensure icon files exist in `src-tauri/icons/`
- Required: `32x32.png`, `128x128.png`, `128x128@2x.png`

## Technical Details

- Video format: MP4 (H.264, CRF 18, yuv420p)
- Rotation: 90°, 180°, 270° via FFmpeg transpose filters
- File storage: Local filesystem (temporary, cleaned up after 24 hours)
- Maximum compatibility: MP4 format works with all major video editors and YouTube

## License

MIT
