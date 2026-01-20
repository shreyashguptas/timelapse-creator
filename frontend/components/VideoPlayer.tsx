'use client';

import { getDownloadUrl } from '@/lib/api';

interface VideoPlayerProps {
  jobId: string;
  cacheBuster?: number | null;
  onAdjust: () => void;
  onReset: () => void;
}

export default function VideoPlayer({ jobId, cacheBuster, onAdjust, onReset }: VideoPlayerProps) {
  const videoUrl = getDownloadUrl(jobId, cacheBuster ?? undefined);

  return (
    <div className="w-full bg-cream-light border border-cream-dark rounded-2xl p-4 sm:p-6 md:p-8">
      <div className="space-y-4 sm:space-y-6">
        <div className="text-center">
          <h3 className="font-serif text-xl sm:text-2xl font-bold mb-1 sm:mb-2">Your Timelapse</h3>
          <p className="text-charcoal-muted text-xs sm:text-sm">Preview and download your video</p>
        </div>

        <div className="relative aspect-video bg-charcoal rounded-xl overflow-hidden">
          <video
            key={videoUrl}
            src={videoUrl}
            controls
            playsInline
            preload="metadata"
            className="w-full h-full object-contain"
          >
            Your browser does not support the video tag.
          </video>
        </div>

        <div className="space-y-3">
          <div className="flex flex-col sm:flex-row gap-2 sm:gap-3">
            <button
              onClick={onAdjust}
              className="flex-1 px-4 sm:px-6 py-2.5 sm:py-3 border border-charcoal/20 rounded-full hover:bg-cream-dark transition-colors text-sm sm:text-base"
            >
              Adjust & Regenerate
            </button>
            <a
              href={videoUrl}
              download={`timelapse_${jobId}.mp4`}
              className="flex-1 inline-flex items-center justify-center gap-2 px-4 sm:px-6 py-2.5 sm:py-3 bg-charcoal text-cream rounded-full hover:bg-charcoal-light transition-colors font-medium text-sm sm:text-base"
            >
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
              Download Video
            </a>
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
