'use client';

import { useState } from 'react';
import { selectImages, uploadImages, type UploadResponse } from '@/lib/tauri-api';

interface FileUploaderProps {
  onUploadComplete: (response: UploadResponse) => void;
  disabled?: boolean;
}

export default function FileUploader({ onUploadComplete, disabled }: FileUploaderProps) {
  const [isBusy, setIsBusy] = useState(false);
  const [uploadProgress, setUploadProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const [fileCount, setFileCount] = useState(0);
  const [isProcessing, setIsProcessing] = useState(false);

  const handleSelectFiles = async () => {
    if (disabled || isBusy) return;

    setError(null);
    setUploadProgress(0);
    setIsBusy(true); // Prevent multiple clicks while file picker is open

    try {
      // Open native file picker
      const paths = await selectImages();

      if (paths.length === 0) {
        return; // User cancelled
      }

      setIsProcessing(true);
      setFileCount(paths.length);
      setUploadProgress(10); // Show initial progress

      console.log(`Starting upload of ${paths.length} files`);

      // Upload selected files
      const response = await uploadImages(paths);

      setUploadProgress(100);
      console.log('Upload completed successfully:', response);

      if (!response.jobId || !response.fileCount) {
        throw new Error('Invalid response from upload');
      }

      onUploadComplete(response);
    } catch (err) {
      console.error('Upload error:', err);
      setError(err instanceof Error ? err.message : 'Upload failed');
    } finally {
      setIsBusy(false);
      setIsProcessing(false);
    }
  };

  return (
    <div className="w-full">
      <button
        onClick={handleSelectFiles}
        disabled={isBusy || disabled}
        className={`w-full px-4 py-8 sm:px-8 sm:py-12 border-2 border-dashed rounded-2xl transition-all disabled:cursor-not-allowed ${
          disabled
            ? 'border-cream-dark bg-cream-light/50 opacity-60'
            : 'border-cream-dark bg-cream-light hover:border-charcoal-muted hover:bg-cream'
        }`}
      >
        <div className="flex flex-col items-center justify-center space-y-3 sm:space-y-4">
          {isProcessing ? (
            <>
              <div className="w-10 h-10 sm:w-12 sm:h-12 rounded-full border-2 border-charcoal/20 border-t-charcoal animate-spin" />
              <div className="text-center">
                <p className="text-charcoal font-medium text-sm sm:text-base">
                  Processing {fileCount} files...
                </p>
                <p className="text-charcoal-muted text-xs sm:text-sm mt-1">{uploadProgress}%</p>
              </div>
              <div className="w-full max-w-xs h-1.5 bg-cream-dark rounded-full overflow-hidden">
                <div
                  className="h-full bg-charcoal rounded-full transition-all duration-300"
                  style={{ width: `${uploadProgress}%` }}
                />
              </div>
            </>
          ) : (
            <>
              <div className="w-12 h-12 sm:w-16 sm:h-16 rounded-full bg-cream flex items-center justify-center">
                <svg
                  className="w-6 h-6 sm:w-8 sm:h-8 text-charcoal-muted"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                  strokeWidth={1.5}
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M12 16.5V9.75m0 0l3 3m-3-3l-3 3M6.75 19.5a4.5 4.5 0 01-1.41-8.775 5.25 5.25 0 0110.338-2.32 5.75 5.75 0 011.344 11.095"
                  />
                </svg>
              </div>
              <div className="text-center">
                <p className="text-charcoal font-medium text-sm sm:text-base">
                  Click to select images
                </p>
                <p className="text-charcoal-muted text-xs sm:text-sm mt-1">
                  PNG, JPEG, WebP supported
                </p>
              </div>
            </>
          )}
        </div>
      </button>

      {error && (
        <div className="mt-4 sm:mt-6 p-3 sm:p-4 bg-error/5 border border-error/20 rounded-xl">
          <p className="text-xs sm:text-sm font-medium text-error mb-1">Upload Error</p>
          <p className="text-xs sm:text-sm text-error/80 font-mono break-words">{error}</p>
        </div>
      )}
    </div>
  );
}
