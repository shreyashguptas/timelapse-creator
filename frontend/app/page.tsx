'use client';

import { useState } from 'react';
import FileUploader from '@/components/FileUploader';
import ImagePreview from '@/components/ImagePreview';
import RotationControls from '@/components/RotationControls';
import VideoStatus from '@/components/VideoStatus';
import { createTimelapse, type UploadResponse } from '@/lib/api';

type Step = 'upload' | 'preview' | 'processing';

export default function Home() {
  const [step, setStep] = useState<Step>('upload');
  const [jobId, setJobId] = useState<string | null>(null);
  const [fileCount, setFileCount] = useState(0);
  const [rotation, setRotation] = useState(0);
  const [fps, setFps] = useState(30);
  const [error, setError] = useState<string | null>(null);

  const handleUploadComplete = (response: UploadResponse) => {
    setJobId(response.jobId);
    setFileCount(response.fileCount);
    setStep('preview');
  };

  const handleCreateTimelapse = async () => {
    if (!jobId) return;

    setError(null);
    setStep('processing');

    try {
      await createTimelapse({
        jobId,
        rotation,
        fps,
      });
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create timelapse');
      setStep('preview');
    }
  };

  return (
    <main className="min-h-screen py-12 px-4">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-4xl font-bold text-center mb-2">
          Timelapse Creator
        </h1>
        <p className="text-center text-gray-600 mb-8">
          Upload your image frames and create a high-quality timelapse video
        </p>

        {step === 'upload' && (
          <div className="space-y-6">
            <FileUploader onUploadComplete={handleUploadComplete} />
          </div>
        )}

        {step === 'preview' && jobId && (
          <div className="space-y-6">
            <div className="bg-white p-6 rounded-lg shadow">
              <h2 className="text-2xl font-semibold mb-4">Preview & Settings</h2>
              
              <div className="space-y-6">
                <ImagePreview jobId={jobId} fileCount={fileCount} rotation={rotation} />
                
                <div className="border-t pt-6">
                  <RotationControls rotation={rotation} onRotate={setRotation} />
                </div>

                <div className="border-t pt-6">
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Frame Rate (FPS)
                  </label>
                  <input
                    type="number"
                    min="1"
                    max="60"
                    value={fps}
                    onChange={(e) => setFps(parseInt(e.target.value) || 30)}
                    className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  />
                </div>

                {error && (
                  <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
                    <p className="text-sm text-red-800">{error}</p>
                  </div>
                )}

                <div className="flex space-x-4">
                  <button
                    onClick={() => setStep('upload')}
                    className="px-6 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
                  >
                    Back
                  </button>
                  <button
                    onClick={handleCreateTimelapse}
                    className="flex-1 px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors font-medium"
                  >
                    Create Timelapse
                  </button>
                </div>
              </div>
            </div>
          </div>
        )}

        {step === 'processing' && jobId && (
          <div className="space-y-6">
            <VideoStatus jobId={jobId} />
            <button
              onClick={() => {
                setStep('upload');
                setJobId(null);
                setFileCount(0);
                setRotation(0);
                setFps(30);
              }}
              className="w-full px-6 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
            >
              Create Another Timelapse
            </button>
          </div>
        )}
      </div>
    </main>
  );
}
