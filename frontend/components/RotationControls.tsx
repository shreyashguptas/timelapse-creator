'use client';

interface RotationControlsProps {
  rotation: number;
  onRotate: (rotation: number) => void;
}

export default function RotationControls({ rotation, onRotate }: RotationControlsProps) {
  const handleRotate = () => {
    const newRotation = (rotation + 90) % 360;
    onRotate(newRotation);
  };

  return (
    <div className="flex flex-col items-center space-y-4">
      <button
        onClick={handleRotate}
        className="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors font-medium"
      >
        Rotate 90°
      </button>
      
      <div className="text-sm text-gray-600">
        Current rotation: {rotation}°
      </div>
      
      <div className="flex space-x-2">
        <button
          onClick={() => onRotate(0)}
          className={`px-4 py-2 text-sm rounded ${
            rotation === 0
              ? 'bg-blue-600 text-white'
              : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
          }`}
        >
          0°
        </button>
        <button
          onClick={() => onRotate(90)}
          className={`px-4 py-2 text-sm rounded ${
            rotation === 90
              ? 'bg-blue-600 text-white'
              : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
          }`}
        >
          90°
        </button>
        <button
          onClick={() => onRotate(180)}
          className={`px-4 py-2 text-sm rounded ${
            rotation === 180
              ? 'bg-blue-600 text-white'
              : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
          }`}
        >
          180°
        </button>
        <button
          onClick={() => onRotate(270)}
          className={`px-4 py-2 text-sm rounded ${
            rotation === 270
              ? 'bg-blue-600 text-white'
              : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
          }`}
        >
          270°
        </button>
      </div>
    </div>
  );
}
