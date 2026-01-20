'use client';

import { useEffect, useState } from 'react';
import { getPreview, type Rotation } from '@/lib/tauri-api';

interface ImagePreviewProps {
  jobId: string;
  fileCount: number;
  rotation: Rotation;
}

export default function ImagePreview({ jobId, fileCount, rotation }: ImagePreviewProps) {
  const [imageUrl, setImageUrl] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const previewIndex = Math.floor(fileCount / 2);

  useEffect(() => {
    if (!jobId || fileCount === 0) {
      return;
    }

    let cancelled = false;

    const loadPreview = async () => {
      setLoading(true);
      setError(null);
      setImageUrl(null);

      try {
        const dataUrl = await getPreview(jobId, previewIndex);
        if (!cancelled) {
          setImageUrl(dataUrl);
        }
      } catch (err) {
        console.error('Failed to load preview:', err);
        if (!cancelled) {
          setError(err instanceof Error ? err.message : 'Failed to load preview image');
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    };

    loadPreview();

    return () => {
      cancelled = true;
    };
  }, [jobId, fileCount, previewIndex]);

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
        Frame {previewIndex + 1} of {fileCount}
      </p>
    </div>
  );
}
