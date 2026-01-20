'use client';

import { useEffect, useState } from 'react';
import { getPreviewUrl, type Rotation } from '@/lib/api';

interface ImagePreviewProps {
  jobId: string;
  fileCount: number;
  rotation: Rotation;
}

export default function ImagePreview({ jobId, fileCount, rotation }: ImagePreviewProps) {
  const [loadedData, setLoadedData] = useState<{ url: string } | null>(null);
  const [error, setError] = useState<string | null>(null);

  // Compute the expected URL (null if no valid job)
  const expectedUrl = jobId && fileCount > 0
    ? getPreviewUrl(jobId, Math.floor(fileCount / 2))
    : null;

  // Derive loading state from comparison
  const loading = expectedUrl !== null && (loadedData === null || loadedData.url !== expectedUrl) && error === null;
  const imageUrl = loadedData?.url ?? null;

  useEffect(() => {
    if (!expectedUrl) {
      return;
    }

    // Check if already loaded
    if (loadedData?.url === expectedUrl) {
      return;
    }

    let cancelled = false;

    const img = new Image();
    img.onload = () => {
      if (!cancelled) {
        setLoadedData({ url: expectedUrl });
        setError(null);
      }
    };
    img.onerror = () => {
      if (!cancelled) {
        setError('Failed to load preview image');
      }
    };
    img.src = expectedUrl;

    return () => {
      cancelled = true;
    };
  }, [expectedUrl, loadedData?.url]);

  if (!jobId || fileCount === 0) {
    return (
      <div className="w-full aspect-video bg-cream rounded-xl border border-cream-dark flex items-center justify-center">
        <p className="text-charcoal-muted text-sm sm:text-base">No preview available</p>
      </div>
    );
  }

  return (
    <div className="w-full">
      <div className="relative aspect-video bg-cream rounded-xl border border-cream-dark overflow-hidden flex items-center justify-center">
        {loading && (
          <div className="absolute inset-0 flex items-center justify-center">
            <div className="w-8 h-8 sm:w-10 sm:h-10 rounded-full border-2 border-charcoal/20 border-t-charcoal animate-spin" />
          </div>
        )}

        {error && (
          <div className="absolute inset-0 flex items-center justify-center p-4">
            <p className="text-error text-sm sm:text-base text-center">{error}</p>
          </div>
        )}

        {imageUrl && !loading && (
          <img
            src={imageUrl}
            alt="Preview"
            className="max-w-full max-h-full object-contain"
            style={{
              transform: `rotate(${rotation}deg)`,
              transition: 'transform 0.3s ease',
            }}
          />
        )}
      </div>

      <p className="mt-2 sm:mt-3 text-xs sm:text-sm text-center text-charcoal-muted">
        Frame {Math.floor(fileCount / 2) + 1} of {fileCount}
      </p>
    </div>
  );
}
