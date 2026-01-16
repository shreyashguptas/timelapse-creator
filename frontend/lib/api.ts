const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

export interface UploadResponse {
  jobId: string;
  fileCount: number;
  filenames: string[];
}

export interface CreateTimelapseRequest {
  jobId: string;
  rotation: 0 | 90 | 180 | 270;
  fps: number;
}

export interface CreateTimelapseResponse {
  jobId: string;
  status: 'pending' | 'processing';
}

export interface JobStatus {
  status: 'pending' | 'processing' | 'completed' | 'failed';
  progress?: number;
  error?: string;
}

export async function uploadFiles(files: File[]): Promise<UploadResponse> {
  const formData = new FormData();
  files.forEach((file) => {
    formData.append('files', file);
  });

  try {
    const response = await fetch(`${API_URL}/api/upload`, {
      method: 'POST',
      body: formData,
    });

    if (!response.ok) {
      const errorText = await response.text();
      console.error('Upload failed:', response.status, errorText);
      throw new Error(`Upload failed: ${response.statusText}`);
    }

    const data = await response.json();
    console.log('Upload response:', data);
    return data;
  } catch (error) {
    if (error instanceof TypeError && error.message.includes('fetch')) {
      throw new Error(`Cannot connect to backend at ${API_URL}. Make sure the backend is running.`);
    }
    throw error;
  }
}

export function getPreviewUrl(jobId: string, index: number): string {
  return `${API_URL}/api/preview/${jobId}/${index}`;
}

export async function createTimelapse(
  request: CreateTimelapseRequest
): Promise<CreateTimelapseResponse> {
  const response = await fetch(`${API_URL}/api/create-timelapse`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    throw new Error(`Failed to create timelapse: ${response.statusText}`);
  }

  return response.json();
}

export async function getJobStatus(jobId: string): Promise<JobStatus> {
  const response = await fetch(`${API_URL}/api/job-status/${jobId}`);

  if (!response.ok) {
    throw new Error(`Failed to get job status: ${response.statusText}`);
  }

  return response.json();
}

export function getDownloadUrl(jobId: string): string {
  return `${API_URL}/api/download/${jobId}`;
}
