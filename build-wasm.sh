#!/bin/bash

echo "Building Rust Steam Game for WebAssembly..."
echo

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed!"
    echo
    echo "Please install wasm-pack first:"
    echo "  cargo install wasm-pack"
    echo
    echo "Or download from: https://rustwasm.github.io/wasm-pack/installer/"
    exit 1
fi

# Clean previous builds
rm -rf pkg dist

echo "Step 1: Building WASM package..."
wasm-pack build --target web --out-dir pkg --dev

if [ $? -ne 0 ]; then
    echo
    echo "Error: WASM build failed!"
    exit 1
fi

echo
echo "Step 2: Creating distribution directory..."
mkdir -p dist

echo "Step 3: Copying files..."
cp index.html dist/
cp -r pkg dist/

echo
echo "✅ Build complete!"
echo
echo "To run the game:"
echo "  1. Start a local web server in the 'dist' directory"
echo "  2. For example: python -m http.server 8000"
echo "  3. Open http://localhost:8000 in your browser"
echo
echo "Files are in: dist/"
echo

# Check if Python is available for quick server
if command -v python3 &> /dev/null; then
    echo "Quick start option:"
    echo "  cd dist"
    echo "  python3 -m http.server 8000"
    echo
elif command -v python &> /dev/null; then
    echo "Quick start option:"
    echo "  cd dist"
    echo "  python -m http.server 8000"
    echo
fi