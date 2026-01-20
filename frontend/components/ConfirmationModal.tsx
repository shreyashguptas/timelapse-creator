'use client';

interface ConfirmationModalProps {
  isOpen: boolean;
  title: string;
  message: string;
  confirmLabel?: string;
  cancelLabel?: string;
  onConfirm: () => void;
  onCancel: () => void;
}

export default function ConfirmationModal({
  isOpen,
  title,
  message,
  confirmLabel = 'OK',
  cancelLabel = 'Cancel',
  onConfirm,
  onCancel,
}: ConfirmationModalProps) {
  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-4"
      onClick={onCancel}
    >
      {/* Backdrop */}
      <div className="absolute inset-0 bg-charcoal/50" />

      {/* Modal */}
      <div
        className="relative bg-cream border border-cream-dark rounded-2xl p-6 sm:p-8 w-full max-w-sm sm:max-w-md shadow-xl"
        onClick={(e) => e.stopPropagation()}
      >
        <h2 className="font-serif text-xl sm:text-2xl font-bold mb-2 sm:mb-3">
          {title}
        </h2>
        <p className="text-charcoal-muted text-sm sm:text-base mb-6 sm:mb-8">
          {message}
        </p>

        <div className="flex flex-col sm:flex-row gap-3 sm:gap-4">
          <button
            onClick={onCancel}
            className="flex-1 px-4 sm:px-6 py-2.5 sm:py-3 border border-charcoal/20 rounded-full hover:bg-cream-dark transition-colors text-sm sm:text-base order-2 sm:order-1"
          >
            {cancelLabel}
          </button>
          <button
            onClick={onConfirm}
            className="flex-1 px-4 sm:px-6 py-2.5 sm:py-3 bg-charcoal text-cream rounded-full hover:bg-charcoal-light transition-colors font-medium text-sm sm:text-base order-1 sm:order-2"
          >
            {confirmLabel}
          </button>
        </div>
      </div>
    </div>
  );
}
