#!/bin/bash
set -e

echo "==> Creating build/ directory..."
mkdir -p build

echo "==> Building optimized Unix release binary..."
cargo build --release
cp target/release/tusk build/tusk-linux

echo "==> Building optimized Windows release binary..."
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/tusk.exe build/tusk-windows.exe

echo "==> Build complete! Your binaries are ready:"
ls -lh build/
