const API_URL = process.env.NEXT_PUBLIC_API_URL ?? 'http://localhost:8080';

export interface UploadResponse {
  jobId: string;
  fileCount: number;
  filenames: string[];
}

export type Rotation = 0 | 90 | 180 | 270;

export interface CreateTimelapseRequest {
  jobId: string;
  rotation: Rotation;
  fps: number;
}

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

export async function uploadFiles(
  files: File[],
  onProgress?: (percent: number) => void
): Promise<UploadResponse> {
  const formData = new FormData();
  files.forEach((file) => {
    formData.append('files', file);
  });

  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest();

    xhr.upload.onprogress = (event) => {
      if (event.lengthComputable && onProgress) {
        const percent = Math.round((event.loaded / event.total) * 100);
        onProgress(percent);
      }
    };

    xhr.onload = () => {
      if (xhr.status >= 200 && xhr.status < 300) {
        try {
          const data = JSON.parse(xhr.responseText);
          console.log('Upload response:', data);
          resolve(data);
        } catch {
          reject(new Error(`Invalid JSON response from server: ${xhr.responseText.substring(0, 500)}`));
        }
      } else {
        console.error('Upload failed:', xhr.status, xhr.responseText);
        // Parse error response for more details
        let errorDetail = xhr.responseText;
        try {
          const errorJson = JSON.parse(xhr.responseText);
          errorDetail = errorJson.error || errorJson.message || JSON.stringify(errorJson);
        } catch {
          // Response is not JSON, use as-is (truncate if too long)
          errorDetail = xhr.responseText.substring(0, 500);
        }
        reject(new Error(`Upload failed (HTTP ${xhr.status}): ${errorDetail || xhr.statusText || 'Unknown error'}`));
      }
    };

    xhr.onerror = () => {
      reject(new Error(`Cannot connect to backend at ${API_URL}. Make sure the backend is running.`));
    };

    xhr.ontimeout = () => {
      reject(new Error('Upload timed out. For large uploads, this may take a while.'));
    };

    xhr.timeout = 0; // No timeout for large uploads

    xhr.open('POST', `${API_URL}/api/upload`);
    xhr.send(formData);
  });
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

export function getDownloadUrl(jobId: string, cacheBuster?: number): string {
  const url = `${API_URL}/api/download/${jobId}`;
  return cacheBuster ? `${url}?t=${cacheBuster}` : url;
}
