'use client';

import { useState, useEffect } from 'react';
import { getVideoData, saveVideo } from '@/lib/tauri-api';

interface VideoPlayerProps {
  jobId: string;
  cacheBuster?: number | null;
  onAdjust: () => void;
  onReset: () => void;
}

export default function VideoPlayer({ jobId, cacheBuster, onAdjust, onReset }: VideoPlayerProps) {
  const [videoUrl, setVideoUrl] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;

    const loadVideo = async () => {
      setIsLoading(true);
      setError(null);

      try {
        const dataUrl = await getVideoData(jobId);
        if (!cancelled) {
          setVideoUrl(dataUrl);
        }
      } catch (err) {
        console.error('Failed to load video:', err);
        if (!cancelled) {
          setError(err instanceof Error ? err.message : 'Failed to load video');
        }
      } finally {
        if (!cancelled) {
          setIsLoading(false);
        }
      }
    };

    loadVideo();

    return () => {
      cancelled = true;
    };
  }, [jobId, cacheBuster]);

  const handleDownload = async () => {
    setIsSaving(true);
    setError(null);

    try {
      const saved = await saveVideo(jobId);
      if (!saved) {
        // User cancelled save dialog
        return;
      }
    } catch (err) {
      console.error('Failed to save video:', err);
      setError(err instanceof Error ? err.message : 'Failed to save video');
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="w-full bg-cream-light border border-cream-dark rounded-2xl p-4 sm:p-6 md:p-8">
      <div className="space-y-4 sm:space-y-6">
        <div className="text-center">
          <h3 className="font-serif text-xl sm:text-2xl font-bold mb-1 sm:mb-2">Your Timelapse</h3>
          <p className="text-charcoal-muted text-xs sm:text-sm">Preview and download your video</p>
        </div>

        <div className="relative aspect-video bg-charcoal rounded-xl overflow-hidden">
          {isLoading ? (
            <div className="absolute inset-0 flex items-center justify-center">
              <div className="w-10 h-10 rounded-full border-2 border-cream/20 border-t-cream animate-spin" />
            </div>
          ) : error ? (
            <div className="absolute inset-0 flex items-center justify-center text-cream">
              <p className="text-sm">{error}</p>
            </div>
          ) : videoUrl ? (
            <video
              key={videoUrl}
              src={videoUrl}
              controls
              playsInline
              className="w-full h-full object-contain"
            >
              Your browser does not support the video tag.
            </video>
          ) : null}
        </div>

        <div className="space-y-3">
          <div className="flex flex-col sm:flex-row gap-2 sm:gap-3">
            <button
              onClick={onAdjust}
              className="flex-1 px-4 sm:px-6 py-2.5 sm:py-3 border border-charcoal/20 rounded-full hover:bg-cream-dark transition-colors text-sm sm:text-base"
            >
              Adjust & Regenerate
            </button>
            <button
              onClick={handleDownload}
              disabled={isSaving || isLoading}
              className="flex-1 inline-flex items-center justify-center gap-2 px-4 sm:px-6 py-2.5 sm:py-3 bg-charcoal text-cream rounded-full hover:bg-charcoal-light transition-colors font-medium text-sm sm:text-base disabled:opacity-50"
            >
              {isSaving ? (
                <>
                  <div className="w-4 h-4 rounded-full border-2 border-cream/20 border-t-cream animate-spin" />
                  Saving...
                </>
              ) : (
                <>
                  <svg
                    className="w-4 h-4 sm:w-5 sm:h-5"
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
                  Save Video
                </>
              )}
            </button>
          </div>

          <button
            onClick={onReset}
            className="w-full px-4 sm:px-6 py-2.5 sm:py-3 text-charcoal-muted hover:text-charcoal transition-colors text-sm sm:text-base"
          >
            Create Another Timelapse
          </button>
        </div>
      </div>
    </div>
  );
}
