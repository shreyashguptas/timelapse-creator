import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';

export interface UploadResponse {
  jobId: string;
  fileCount: number;
  filenames: string[];
}

export type Rotation = 0 | 90 | 180 | 270;

export interface CreateTimelapseResponse {
  jobId: string;
  status: 'pending' | 'processing';
}

export interface JobStatus {
  status: 'pending' | 'processing' | 'completed' | 'failed';
  progress?: number;
  stage?: 'preparing' | 'encoding' | 'finalizing' | 'complete';
  currentFrame?: number;
  totalFrames?: number;
  error?: string;
}

export async function selectImages(): Promise<string[]> {
  const files = await open({
    multiple: true,
    filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'webp'] }],
  });

  if (files === null) {
    return [];
  }

  // Handle both single file and multiple files
  if (Array.isArray(files)) {
    return files;
  }
  return [files];
}

export async function uploadImages(paths: string[]): Promise<UploadResponse> {
  const result = await invoke<{
    job_id: string;
    file_count: number;
    filenames: string[];
  }>('upload_images', { paths });

  return {
    jobId: result.job_id,
    fileCount: result.file_count,
    filenames: result.filenames,
  };
}

export async function getPreview(jobId: string, index: number): Promise<string> {
  return invoke<string>('get_preview', { jobId, index });
}

export async function createTimelapse(
  jobId: string,
  fps: number,
  rotation: Rotation
): Promise<CreateTimelapseResponse> {
  const result = await invoke<{
    job_id: string;
    status: string;
  }>('create_timelapse', { jobId, fps, rotation });

  return {
    jobId: result.job_id,
    status: result.status as 'pending' | 'processing',
  };
}

export async function getJobStatus(jobId: string): Promise<JobStatus> {
  const result = await invoke<{
    status: string;
    progress?: number;
    stage?: string;
    current_frame?: number;
    total_frames?: number;
    error?: string;
  }>('get_job_status', { jobId });

  return {
    status: result.status as JobStatus['status'],
    progress: result.progress,
    stage: result.stage as JobStatus['stage'],
    currentFrame: result.current_frame,
    totalFrames: result.total_frames,
    error: result.error,
  };
}

export async function getVideoData(jobId: string): Promise<string> {
  return invoke<string>('get_video_data', { jobId });
}

export async function saveVideo(jobId: string): Promise<boolean> {
  const savePath = await save({
    filters: [{ name: 'Video', extensions: ['mp4'] }],
    defaultPath: `timelapse_${jobId}.mp4`,
  });

  if (!savePath) {
    return false;
  }

  return invoke<boolean>('save_video', { jobId, savePath });
}

export async function cleanupJob(jobId: string): Promise<boolean> {
  return invoke<boolean>('cleanup_job', { jobId });
}
