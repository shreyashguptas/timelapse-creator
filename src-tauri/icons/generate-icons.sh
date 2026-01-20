#!/bin/bash
# Generate .icns (macOS) and .ico (Windows) from source PNG
# Requires: ImageMagick (brew install imagemagick) or uses sips on macOS

set -e
cd "$(dirname "$0")"

SOURCE="icon-512.png"  # 512x512 source image

if [ ! -f "$SOURCE" ]; then
    echo "Error: Source image $SOURCE not found"
    exit 1
fi

echo "Generating icons from $SOURCE..."

# Generate macOS .icns using iconutil (native macOS)
generate_icns() {
    echo "Creating macOS .icns..."

    ICONSET="icon.iconset"
    rm -rf "$ICONSET"
    mkdir -p "$ICONSET"

    # Generate all required sizes using sips (native macOS)
    sips -z 16 16     "$SOURCE" --out "$ICONSET/icon_16x16.png" >/dev/null
    sips -z 32 32     "$SOURCE" --out "$ICONSET/icon_16x16@2x.png" >/dev/null
    sips -z 32 32     "$SOURCE" --out "$ICONSET/icon_32x32.png" >/dev/null
    sips -z 64 64     "$SOURCE" --out "$ICONSET/icon_32x32@2x.png" >/dev/null
    sips -z 128 128   "$SOURCE" --out "$ICONSET/icon_128x128.png" >/dev/null
    sips -z 256 256   "$SOURCE" --out "$ICONSET/icon_128x128@2x.png" >/dev/null
    sips -z 256 256   "$SOURCE" --out "$ICONSET/icon_256x256.png" >/dev/null
    sips -z 512 512   "$SOURCE" --out "$ICONSET/icon_256x256@2x.png" >/dev/null
    sips -z 512 512   "$SOURCE" --out "$ICONSET/icon_512x512.png" >/dev/null
    cp "$SOURCE" "$ICONSET/icon_512x512@2x.png"  # Use source for largest (upscaling if needed)

    # Convert iconset to icns
    iconutil -c icns "$ICONSET" -o icon.icns
    rm -rf "$ICONSET"

    echo "Created icon.icns"
}

# Generate Windows .ico using ImageMagick
generate_ico() {
    echo "Creating Windows .ico..."

    if command -v magick &> /dev/null; then
        # ImageMagick 7
        magick "$SOURCE" -define icon:auto-resize=256,128,64,48,32,16 icon.ico
    elif command -v convert &> /dev/null; then
        # ImageMagick 6
        convert "$SOURCE" -define icon:auto-resize=256,128,64,48,32,16 icon.ico
    else
        echo "Warning: ImageMagick not found. Install with: brew install imagemagick"
        echo "Skipping .ico generation"
        return 1
    fi

    echo "Created icon.ico"
}

# Run generation
if [[ "$OSTYPE" == "darwin"* ]]; then
    generate_icns
fi

generate_ico || echo "Note: .ico generation requires ImageMagick"

echo "Icon generation complete!"
