# Architecture Documentation

## Overview

The Timelapse Creator is a web application consisting of a Next.js frontend and a Rust backend service.

## Components

### Frontend (Next.js 16)

Located in `frontend/`, the frontend provides:
- File upload interface
- Image preview with rotation controls
- Video processing status
- Download functionality

**Key Files:**
- `app/page.tsx` - Main application page
- `components/FileUploader.tsx` - File upload component
- `components/ImagePreview.tsx` - Preview image display
- `components/RotationControls.tsx` - Rotation UI controls
- `components/VideoStatus.tsx` - Processing status display
- `lib/api.ts` - Backend API client

### Backend (Rust/Actix Web)

Located in `backend/`, the backend provides:
- Multipart file upload handling
- Local file storage
- FFmpeg video processing
- Job status tracking

**Key Files:**
- `src/main.rs` - HTTP server setup
- `src/handlers/` - API endpoint handlers
- `src/video/` - Video processing logic
- `src/storage/` - Local filesystem operations
- `src/models.rs` - Data structures

## Data Flow

1. User selects image files in frontend
2. Files uploaded via multipart/form-data to `/api/upload`
3. Backend stores files in `/tmp/timelapse/{job_id}/frames/`
4. Backend returns job ID and file metadata
5. Frontend displays preview image from `/api/preview/{job_id}/{index}`
6. User adjusts rotation and FPS settings
7. Frontend calls `/api/create-timelapse` with job ID and settings
8. Backend processes video asynchronously using FFmpeg
9. Frontend polls `/api/job-status/{job_id}` for progress
10. When complete, user downloads from `/api/download/{job_id}`

## API Endpoints

- `POST /api/upload` - Upload image files
- `GET /api/preview/{job_id}/{index}` - Get preview image
- `POST /api/create-timelapse` - Start video processing
- `GET /api/job-status/{job_id}` - Get processing status
- `GET /api/download/{job_id}` - Download completed video
- `GET /health` - Health check

## Storage

Files are stored temporarily on the backend server's filesystem:
- Upload location: `/tmp/timelapse/{job_id}/frames/`
- Output location: `/tmp/timelapse/{job_id}/output.mp4`

Files are cleaned up after download or after 24 hours (to be implemented).

## Video Processing

FFmpeg is used to create the timelapse video with:
- High quality settings (CRF 18)
- H.264 codec for maximum compatibility
- yuv420p pixel format for browser/editor support
- Rotation via transpose filters

## Security Considerations

- File type validation (images only)
- Filename sanitization
- CORS configuration for frontend domain
- Rate limiting (to be implemented)
- Size limits (to be implemented)
