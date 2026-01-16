'use client';

import { useEffect, useState } from 'react';
import { getJobStatus, getDownloadUrl, type JobStatus } from '@/lib/api';

interface VideoStatusProps {
  jobId: string;
}

export default function VideoStatus({ jobId }: VideoStatusProps) {
  const [status, setStatus] = useState<JobStatus | null>(null);
  const [isPolling, setIsPolling] = useState(true);

  useEffect(() => {
    if (!jobId || !isPolling) return;

    const pollStatus = async () => {
      try {
        const currentStatus = await getJobStatus(jobId);
        setStatus(currentStatus);

        if (currentStatus.status === 'completed' || currentStatus.status === 'failed') {
          setIsPolling(false);
        }
      } catch (error) {
        console.error('Failed to poll status:', error);
        setIsPolling(false);
      }
    };

    // Poll immediately
    pollStatus();

    // Then poll every 2 seconds
    const interval = setInterval(pollStatus, 2000);

    return () => clearInterval(interval);
  }, [jobId, isPolling]);

  if (!status) {
    return (
      <div className="w-full max-w-2xl mx-auto p-6 bg-white rounded-lg shadow">
        <div className="flex items-center space-x-3">
          <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-gray-900"></div>
          <p className="text-gray-600">Checking status...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="w-full max-w-2xl mx-auto p-6 bg-white rounded-lg shadow">
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h3 className="text-lg font-semibold">Video Status</h3>
          <span
            className={`px-3 py-1 rounded-full text-sm font-medium ${
              status.status === 'completed'
                ? 'bg-green-100 text-green-800'
                : status.status === 'failed'
                ? 'bg-red-100 text-red-800'
                : status.status === 'processing'
                ? 'bg-blue-100 text-blue-800'
                : 'bg-gray-100 text-gray-800'
            }`}
          >
            {status.status}
          </span>
        </div>

        {status.status === 'processing' && (
          <div className="space-y-2">
            <div className="w-full bg-gray-200 rounded-full h-2">
              <div
                className="bg-blue-600 h-2 rounded-full transition-all duration-300"
                style={{ width: `${status.progress || 0}%` }}
              ></div>
            </div>
            <p className="text-sm text-gray-600">
              Processing... {status.progress || 0}%
            </p>
          </div>
        )}

        {status.status === 'completed' && (
          <div className="space-y-4">
            <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
              <p className="text-green-800 font-medium mb-2">
                Video created successfully!
              </p>
              <a
                href={getDownloadUrl(jobId)}
                download
                className="inline-block px-6 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
              >
                Download Video
              </a>
            </div>
          </div>
        )}

        {status.status === 'failed' && (
          <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
            <p className="text-red-800 font-medium">Error</p>
            <p className="text-red-600 text-sm mt-1">
              {status.error || 'Video creation failed'}
            </p>
          </div>
        )}

        {status.status === 'pending' && (
          <div className="flex items-center space-x-2">
            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-gray-900"></div>
            <p className="text-gray-600">Waiting to start processing...</p>
          </div>
        )}
      </div>
    </div>
  );
}
