#!/bin/bash
# Docker build script for GNU Go Windows version

echo "Building GNU Go for Windows using Docker..."

# Pull the MSYS2 image with MinGW-w64
docker pull msys2/msys2:base-x86_64

# Build the GNU Go executable inside a container
docker run --rm -v $(pwd):/src -w /src msys2/msys2:base-x86_64 bash << 'EOF'
set -e

echo "Installing dependencies..."
pacman -Sy --noconfirm \
    base-devel \
    mingw-w64-x86_64-toolchain \
    mingw-w64-x86_64-curses \
    make \
    autoconf \
    automake \
    libtool \
    pkg-config

echo "Configuring build..."
./configure \
    --host=x86_64-w64-mingw32 \
    --build=$(uname -m)-pc-linux-gnu \
    --enable-color \
    --with-curses \
    --enable-socket-support \
    --disable-debug

echo "Building GNU Go..."
make clean
make -j$(nproc)

echo "Build completed successfully!"
EOF

echo "Windows executable created in ./gnugo.exe"