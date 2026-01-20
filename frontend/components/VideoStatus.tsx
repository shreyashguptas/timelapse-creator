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

    pollStatus();
    const interval = setInterval(pollStatus, 2000);

    return () => clearInterval(interval);
  }, [jobId, isPolling]);

  if (!status) {
    return (
      <div className="w-full bg-cream-light border border-cream-dark rounded-2xl p-8">
        <div className="flex items-center justify-center gap-3">
          <div className="w-5 h-5 rounded-full border-2 border-charcoal/20 border-t-charcoal animate-spin" />
          <p className="text-charcoal-muted">Checking status...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="w-full bg-cream-light border border-cream-dark rounded-2xl p-8">
      <div className="space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between">
          <h3 className="font-serif text-xl font-bold">Video Status</h3>
          <span
            className={`px-3 py-1 rounded-full text-sm font-medium ${
              status.status === 'completed'
                ? 'bg-success/10 text-success'
                : status.status === 'failed'
                ? 'bg-error/10 text-error'
                : status.status === 'processing'
                ? 'bg-charcoal/10 text-charcoal'
                : 'bg-charcoal-muted/10 text-charcoal-muted'
            }`}
          >
            {status.status}
          </span>
        </div>

        {/* Processing State */}
        {status.status === 'processing' && (
          <div className="space-y-3">
            <div className="w-full h-2 bg-cream-dark rounded-full overflow-hidden">
              <div
                className="h-full bg-charcoal rounded-full transition-all duration-300"
                style={{ width: `${status.progress || 0}%` }}
              />
            </div>
            <p className="text-sm text-charcoal-muted text-center">
              Processing... {status.progress || 0}%
            </p>
          </div>
        )}

        {/* Completed State */}
        {status.status === 'completed' && (
          <div className="space-y-4">
            <div className="p-6 bg-success/5 border border-success/20 rounded-xl text-center">
              <div className="w-12 h-12 mx-auto mb-4 rounded-full bg-success/10 flex items-center justify-center">
                <svg
                  className="w-6 h-6 text-success"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                  strokeWidth={2}
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M5 13l4 4L19 7"
                  />
                </svg>
              </div>
              <p className="text-success font-medium mb-4">
                Video created successfully
              </p>
              <a
                href={getDownloadUrl(jobId)}
                download
                className="inline-flex items-center gap-2 px-6 py-3 bg-charcoal text-cream rounded-full hover:bg-charcoal-light transition-colors font-medium"
              >
                <svg
                  className="w-5 h-5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                  strokeWidth={2}
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                  />
                </svg>
                Download Video
              </a>
            </div>
          </div>
        )}

        {/* Failed State */}
        {status.status === 'failed' && (
          <div className="p-6 bg-error/5 border border-error/20 rounded-xl">
            <p className="text-error font-medium mb-1">Error</p>
            <p className="text-error/80 text-sm">
              {status.error || 'Video creation failed'}
            </p>
          </div>
        )}

        {/* Pending State */}
        {status.status === 'pending' && (
          <div className="flex items-center justify-center gap-3 py-4">
            <div className="w-4 h-4 rounded-full border-2 border-charcoal/20 border-t-charcoal animate-spin" />
            <p className="text-charcoal-muted">Waiting to start processing...</p>
          </div>
        )}
      </div>
    </div>
  );
}
