#!/bin/bash

# Script to build TurboDownload for Linux using Docker
# This is required because of cross-compilation dependencies

echo "Building TurboDownload for Linux x86_64..."
echo "Current directory: $(pwd)"

# Change to the project root directory (one level up from src-tauri)
cd "$(dirname "$0")"

echo "Executing Docker build command..."

# Execute the Docker build command
docker run --rm -v "$(pwd)":/home/rust/src -w /home/rust/src \
  ghcr.io/tauri-apps/tauri:debian \
  cargo tauri build --target x86_64-unknown-linux-gnu

BUILD_STATUS=$?

if [ $BUILD_STATUS -eq 0 ]; then
    echo "Build completed successfully!"
    echo "Linux binary should be located at: ./src-tauri/target/x86_64-unknown-linux-gnu/release/"
else
    echo "Build failed with status: $BUILD_STATUS"
    exit $BUILD_STATUS
fi