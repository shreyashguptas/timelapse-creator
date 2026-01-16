'use client';

import { useEffect, useState } from 'react';
import { getPreviewUrl } from '@/lib/api';

interface ImagePreviewProps {
  jobId: string;
  fileCount: number;
  rotation: number;
}

export default function ImagePreview({ jobId, fileCount, rotation }: ImagePreviewProps) {
  const [imageUrl, setImageUrl] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!jobId || fileCount === 0) return;

    const middleIndex = Math.floor(fileCount / 2);
    const url = getPreviewUrl(jobId, middleIndex);
    
    setLoading(true);
    setError(null);
    
    // Add cache busting
    const img = new Image();
    img.onload = () => {
      setImageUrl(url);
      setLoading(false);
    };
    img.onerror = () => {
      setError('Failed to load preview image');
      setLoading(false);
    };
    img.src = url;
  }, [jobId, fileCount]);

  if (!jobId || fileCount === 0) {
    return (
      <div className="w-full max-w-2xl mx-auto aspect-video bg-gray-100 rounded-lg flex items-center justify-center">
        <p className="text-gray-400">No preview available</p>
      </div>
    );
  }

  return (
    <div className="w-full max-w-2xl mx-auto">
      <div className="relative aspect-video bg-gray-100 rounded-lg overflow-hidden flex items-center justify-center">
        {loading && (
          <div className="absolute inset-0 flex items-center justify-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900"></div>
          </div>
        )}
        
        {error && (
          <div className="absolute inset-0 flex items-center justify-center">
            <p className="text-red-600">{error}</p>
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
      
      <p className="mt-2 text-sm text-center text-gray-600">
        Preview: Frame {Math.floor(fileCount / 2) + 1} of {fileCount}
      </p>
    </div>
  );
}
