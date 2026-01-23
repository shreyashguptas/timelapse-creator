'use client';

import { useState, useRef } from 'react';
import { uploadFiles, type UploadResponse } from '@/lib/api';

interface FileUploaderProps {
  onUploadComplete: (response: UploadResponse) => void;
  disabled?: boolean;
}

export default function FileUploader({ onUploadComplete, disabled }: FileUploaderProps) {
  const [isUploading, setIsUploading] = useState(false);
  const [uploadProgress, setUploadProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const [fileCount, setFileCount] = useState(0);
  const [isDragging, setIsDragging] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFiles = async (files: FileList | null) => {
    if (!files || files.length === 0 || disabled) return;

    setIsUploading(true);
    setError(null);
    setUploadProgress(0);

    try {
      const fileArray = Array.from(files);

      const imageFiles = fileArray.filter((file) => {
        const type = file.type.toLowerCase();
        return type.startsWith('image/') &&
               (type.includes('png') || type.includes('jpeg') || type.includes('jpg') || type.includes('webp'));
      });

      if (imageFiles.length === 0) {
        throw new Error('No valid image files selected. Please select PNG, JPEG, or WebP files.');
      }

      setFileCount(imageFiles.length);
      console.log(`Starting upload of ${imageFiles.length} files`);

      const response = await uploadFiles(imageFiles, (percent) => {
        setUploadProgress(percent);
      });

      console.log('Upload completed successfully:', response);

      if (!response.jobId || !response.fileCount) {
        throw new Error('Invalid response from server');
      }

      onUploadComplete(response);
    } catch (err) {
      console.error('Upload error:', err);
      setError(err instanceof Error ? err.message : 'Upload failed');
      setIsUploading(false);
      return;
    } finally {
      setIsUploading(false);
    }
  };

  const handleFileSelect = async (event: React.ChangeEvent<HTMLInputElement>) => {
    handleFiles(event.target.files);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    if (!disabled) setIsDragging(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
    if (!disabled) handleFiles(e.dataTransfer.files);
  };

  const handleClick = () => {
    if (!disabled) fileInputRef.current?.click();
  };

  return (
    <div className="w-full">
      <input
        ref={fileInputRef}
        type="file"
        multiple
        accept="image/png,image/jpeg,image/jpg,image/webp"
        onChange={handleFileSelect}
        className="hidden"
        disabled={isUploading || disabled}
      />

      <button
        onClick={handleClick}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        disabled={isUploading || disabled}
        className={`w-full px-4 py-8 sm:px-8 sm:py-12 border-2 border-dashed rounded-2xl transition-all disabled:cursor-not-allowed ${
          disabled
            ? 'border-cream-dark bg-cream-light/50 opacity-60'
            : isDragging
            ? 'border-charcoal bg-cream-dark'
            : 'border-cream-dark bg-cream-light hover:border-charcoal-muted hover:bg-cream'
        }`}
      >
        <div className="flex flex-col items-center justify-center space-y-3 sm:space-y-4">
          {isUploading ? (
            <>
              <div className="w-10 h-10 sm:w-12 sm:h-12 rounded-full border-2 border-charcoal/20 border-t-charcoal animate-spin" />
              <div className="text-center">
                <p className="text-charcoal font-medium text-sm sm:text-base">
                  Uploading {fileCount} files...
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
                  Drop images here or click to browse
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
