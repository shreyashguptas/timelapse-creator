'use client';

import { useState, useRef } from 'react';
import { uploadFiles, type UploadResponse } from '@/lib/api';

interface FileUploaderProps {
  onUploadComplete: (response: UploadResponse) => void;
}

export default function FileUploader({ onUploadComplete }: FileUploaderProps) {
  const [isUploading, setIsUploading] = useState(false);
  const [uploadProgress, setUploadProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileSelect = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = event.target.files;
    if (!files || files.length === 0) return;

    setIsUploading(true);
    setError(null);
    setUploadProgress(0);

    try {
      const fileArray = Array.from(files);
      
      // Filter for image files only
      const imageFiles = fileArray.filter((file) => {
        const type = file.type.toLowerCase();
        return type.startsWith('image/') && 
               (type.includes('png') || type.includes('jpeg') || type.includes('jpg') || type.includes('webp'));
      });

      if (imageFiles.length === 0) {
        throw new Error('No valid image files selected. Please select PNG, JPEG, or WebP files.');
      }

      // Upload files
      const response = await uploadFiles(imageFiles);
      setUploadProgress(100);
      
      // Validate response has required fields
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

  const handleClick = () => {
    fileInputRef.current?.click();
  };

  return (
    <div className="w-full max-w-2xl mx-auto">
      <input
        ref={fileInputRef}
        type="file"
        multiple
        accept="image/png,image/jpeg,image/jpg,image/webp"
        onChange={handleFileSelect}
        className="hidden"
        disabled={isUploading}
      />
      
      <button
        onClick={handleClick}
        disabled={isUploading}
        className="w-full px-6 py-4 border-2 border-dashed border-gray-300 rounded-lg hover:border-gray-400 transition-colors disabled:opacity-50 disabled:cursor-not-allowed bg-white"
      >
        <div className="flex flex-col items-center justify-center space-y-2">
          {isUploading ? (
            <>
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
              <p className="text-sm text-gray-600">Uploading... {uploadProgress}%</p>
            </>
          ) : (
            <>
              <svg
                className="w-12 h-12 text-gray-400"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                />
              </svg>
              <p className="text-sm font-medium text-gray-700">
                Click to select image files
              </p>
              <p className="text-xs text-gray-500">
                PNG, JPEG, or WebP files supported
              </p>
            </>
          )}
        </div>
      </button>

      {error && (
        <div className="mt-4 p-4 bg-red-50 border border-red-200 rounded-lg">
          <p className="text-sm text-red-800">{error}</p>
        </div>
      )}
    </div>
  );
}
