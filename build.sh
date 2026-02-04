#!/bin/bash
set -e

cd "$(dirname "$0")"

if command -v cross &> /dev/null; then
    echo "Using cross for cross-compilation..."

    echo "Building for armv7 (reMarkable 1/2)..."
    cross build --release --target armv7-unknown-linux-gnueabihf

    echo "Building for aarch64 (reMarkable Paper Pro)..."
    cross build --release --target aarch64-unknown-linux-gnu
else
    echo "Using cargo (requires ARM toolchains installed)..."

    rustup target add armv7-unknown-linux-gnueabihf
    rustup target add aarch64-unknown-linux-gnu

    echo "Building for armv7 (reMarkable 1/2)..."
    cargo build --release --target armv7-unknown-linux-gnueabihf

    echo "Building for aarch64 (reMarkable Paper Pro)..."
    cargo build --release --target aarch64-unknown-linux-gnu
fi

echo "Done! Binaries at:"
echo "  target/armv7-unknown-linux-gnueabihf/release/remarkable-pokemon"
echo "  target/aarch64-unknown-linux-gnu/release/remarkable-pokemon"
