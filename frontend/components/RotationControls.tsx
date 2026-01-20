'use client';

import { type Rotation } from '@/lib/api';

interface RotationControlsProps {
  rotation: Rotation;
  onRotate: (rotation: Rotation) => void;
}

const rotationOptions: Rotation[] = [0, 90, 180, 270];

export default function RotationControls({ rotation, onRotate }: RotationControlsProps) {
  return (
    <div className="grid grid-cols-2 gap-2 sm:flex sm:items-center sm:gap-2">
      {rotationOptions.map((value) => (
        <button
          key={value}
          onClick={() => onRotate(value)}
          className={`px-3 py-2 sm:px-4 text-xs sm:text-sm rounded-full transition-all ${
            rotation === value
              ? 'bg-charcoal text-cream'
              : 'bg-cream border border-cream-dark text-charcoal hover:bg-cream-dark'
          }`}
        >
          {value}°
        </button>
      ))}
    </div>
  );
}
