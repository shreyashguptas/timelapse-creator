'use client';

import { useState, useEffect } from 'react';
import FileUploader from '@/components/FileUploader';
import ImagePreview from '@/components/ImagePreview';
import RotationControls from '@/components/RotationControls';
import VideoPlayer from '@/components/VideoPlayer';
import ConfirmationModal from '@/components/ConfirmationModal';
import { createTimelapse, getJobStatus, type UploadResponse, type Rotation } from '@/lib/api';

export default function Home() {
  const [jobId, setJobId] = useState<string | null>(null);
  const [fileCount, setFileCount] = useState(0);
  const [rotation, setRotation] = useState<Rotation>(0);
  const [fps, setFps] = useState(30);
  const [fpsInput, setFpsInput] = useState('30');
  const [error, setError] = useState<string | null>(null);

  const [isUploaded, setIsUploaded] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const [isCompleted, setIsCompleted] = useState(false);
  const [progress, setProgress] = useState(0);
  const [stage, setStage] = useState<string | null>(null);
  const [currentFrame, setCurrentFrame] = useState<number | null>(null);
  const [totalFrames, setTotalFrames] = useState<number | null>(null);
  const [showResetConfirm, setShowResetConfirm] = useState(false);

  useEffect(() => {
    if (!jobId || !isProcessing) return;

    const pollStatus = async () => {
      try {
        const status = await getJobStatus(jobId);
        setProgress(status.progress || 0);
        setStage(status.stage || null);
        setCurrentFrame(status.currentFrame || null);
        setTotalFrames(status.totalFrames || null);

        if (status.status === 'completed') {
          setIsProcessing(false);
          setIsCompleted(true);
        } else if (status.status === 'failed') {
          setError(status.error || 'Video creation failed');
          setIsProcessing(false);
        }
      } catch (err) {
        console.error('Failed to poll status:', err);
        setError('Failed to check processing status');
        setIsProcessing(false);
      }
    };

    pollStatus();
    const interval = setInterval(pollStatus, 2000);

    return () => clearInterval(interval);
  }, [jobId, isProcessing]);

  const handleUploadComplete = (response: UploadResponse) => {
    console.log('Upload complete:', response);
    if (response.jobId && response.fileCount > 0) {
      setJobId(response.jobId);
      setFileCount(response.fileCount);
      setIsUploaded(true);
      setError(null);
    } else {
      setError('Invalid upload response. Please try again.');
    }
  };

  const handleCreateTimelapse = async () => {
    if (!jobId) return;

    setError(null);
    setIsProcessing(true);
    setProgress(0);

    try {
      await createTimelapse({
        jobId,
        rotation,
        fps,
      });
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create timelapse');
      setIsProcessing(false);
    }
  };

  const handleAdjust = () => {
    setIsCompleted(false);
    setProgress(0);
    setStage(null);
    setCurrentFrame(null);
    setTotalFrames(null);
  };

  const handleReset = () => {
    setJobId(null);
    setFileCount(0);
    setRotation(0);
    setFps(30);
    setFpsInput('30');
    setError(null);
    setIsUploaded(false);
    setIsProcessing(false);
    setIsCompleted(false);
    setProgress(0);
    setStage(null);
    setCurrentFrame(null);
    setTotalFrames(null);
  };

  const handleResetConfirm = () => {
    handleReset();
    setShowResetConfirm(false);
  };

  const handleResetCancel = () => {
    setShowResetConfirm(false);
  };

  return (
    <main className="min-h-screen py-8 px-4 sm:py-12 md:py-16 lg:py-24">
      <div className="max-w-sm sm:max-w-xl md:max-w-2xl lg:max-w-3xl mx-auto">
        {/* Header */}
        <header className="text-center mb-8 sm:mb-12 md:mb-16">
          <h1 className="font-serif text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold tracking-tight mb-3 sm:mb-4">
            Create Beautiful
            <br />
            Timelapses
          </h1>
          <p className="text-charcoal-muted text-sm sm:text-base md:text-lg">
            Transform your images into stunning high-quality videos
          </p>
        </header>

        <div className="space-y-6 sm:space-y-8">
          {/* Upload Section */}
          {!isUploaded && (
            <section>
              <FileUploader onUploadComplete={handleUploadComplete} />
            </section>
          )}

          {/* Upload Error */}
          {error && !isUploaded && (
            <div className="p-3 sm:p-4 bg-error/5 border border-error/20 rounded-xl">
              <p className="text-xs sm:text-sm text-error">{error}</p>
            </div>
          )}

          {/* Settings Section - visible after upload, before completion */}
          {isUploaded && !isCompleted && jobId && (
            <section className="bg-cream-light border border-cream-dark rounded-2xl p-4 sm:p-6 md:p-8">
              <div className="space-y-6 sm:space-y-8">
                {/* Preview Section */}
                <div>
                  <h2 className="font-serif text-xl sm:text-2xl font-bold mb-4 sm:mb-6">Preview</h2>
                  <ImagePreview jobId={jobId} fileCount={fileCount} rotation={rotation} />
                </div>

                {/* Rotation Section */}
                <div className="pt-4 sm:pt-6 border-t border-cream-dark">
                  <h3 className="text-xs sm:text-sm font-medium text-charcoal-muted uppercase tracking-wide mb-3 sm:mb-4">
                    Rotation
                  </h3>
                  <RotationControls rotation={rotation} onRotate={setRotation} />
                </div>

                {/* FPS Section */}
                <div className="pt-4 sm:pt-6 border-t border-cream-dark">
                  <h3 className="text-xs sm:text-sm font-medium text-charcoal-muted uppercase tracking-wide mb-3 sm:mb-4">
                    Frame Rate
                  </h3>
                  <div className="space-y-2">
                    <div className="flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-4">
                      <input
                        type="number"
                        min="1"
                        max="60"
                        value={fpsInput}
                        onChange={(e) => setFpsInput(e.target.value)}
                        onBlur={(e) => {
                          const val = parseInt(e.target.value);
                          const clamped = isNaN(val) ? 30 : Math.max(1, Math.min(60, val));
                          setFps(clamped);
                          setFpsInput(String(clamped));
                        }}
                        disabled={isProcessing}
                        className="w-full sm:w-24 px-4 py-2.5 bg-cream border border-cream-dark rounded-full text-center focus:outline-none focus:ring-2 focus:ring-charcoal/20 focus:border-charcoal transition-all disabled:opacity-50"
                      />
                      <span className="text-charcoal-muted text-sm">FPS (1-60)</span>
                    </div>
                    <p className="text-xs sm:text-sm text-charcoal-muted/80">
                      Increasing the frame rate means the timelapse will be faster, decreasing means slower.
                    </p>
                  </div>
                </div>

                {/* Error Display */}
                {error && (
                  <div className="p-3 sm:p-4 bg-error/5 border border-error/20 rounded-xl">
                    <p className="text-xs sm:text-sm text-error">{error}</p>
                  </div>
                )}

                {/* Processing Progress */}
                {isProcessing && (
                  <div className="space-y-3">
                    <div className="flex items-center justify-between text-sm">
                      <span className="text-charcoal-muted">
                        {stage === 'preparing' && 'Preparing frames...'}
                        {stage === 'encoding' && currentFrame && totalFrames
                          ? `Encoding frame ${currentFrame} of ${totalFrames}...`
                          : stage === 'encoding' ? 'Encoding...' : null}
                        {stage === 'finalizing' && 'Finalizing video...'}
                        {!stage && 'Processing...'}
                      </span>
                      <span className="font-medium">{progress}%</span>
                    </div>
                    <div className="w-full h-2 bg-cream-dark rounded-full overflow-hidden">
                      <div
                        className="h-full bg-charcoal rounded-full transition-all duration-300"
                        style={{ width: `${progress}%` }}
                      />
                    </div>
                  </div>
                )}

                {/* Actions */}
                {!isProcessing && (
                  <div className="flex flex-col sm:flex-row gap-3 sm:gap-4 pt-2 sm:pt-4">
                    <button
                      onClick={() => setShowResetConfirm(true)}
                      className="px-4 sm:px-6 py-2.5 sm:py-3 border border-charcoal/20 rounded-full hover:bg-cream-dark transition-colors text-sm sm:text-base order-2 sm:order-1"
                    >
                      Start Over
                    </button>
                    <button
                      onClick={handleCreateTimelapse}
                      className="flex-1 px-4 sm:px-6 py-2.5 sm:py-3 bg-charcoal text-cream rounded-full hover:bg-charcoal-light transition-colors font-medium text-sm sm:text-base order-1 sm:order-2"
                    >
                      Generate Timelapse
                    </button>
                  </div>
                )}
              </div>
            </section>
          )}

          {/* Video Player Section - visible after completion */}
          {isCompleted && jobId && (
            <section>
              <VideoPlayer
                jobId={jobId}
                onAdjust={handleAdjust}
                onReset={() => setShowResetConfirm(true)}
              />
            </section>
          )}
        </div>
      </div>

      <ConfirmationModal
        isOpen={showResetConfirm}
        title="Start Over?"
        message="Your uploaded images will be cleared. Are you sure you want to start over?"
        confirmLabel="Yes, Start Over"
        cancelLabel="Cancel"
        onConfirm={handleResetConfirm}
        onCancel={handleResetCancel}
      />
    </main>
  );
}
