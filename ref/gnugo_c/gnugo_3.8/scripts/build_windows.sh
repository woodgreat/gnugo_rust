#!/bin/bash
# Windows build script for GNU Go using MSYS2 and MinGW-w64

echo "Setting up MSYS2 environment for Windows build..."

# Check if MSYS2 is available
if ! command -v pacman &> /dev/null; then
    echo "Error: MSYS2 not found. Please install MSYS2 from https://www.msys2.org/"
    exit 1
fi

# Update package database
echo "Updating package database..."
pacman -Sy --noconfirm

# Install required packages
echo "Installing required packages..."
pacman -S --noconfirm \
    base-devel \
    mingw-w64-x86_64-toolchain \
    mingw-w64-x86_64-curses \
    make \
    autoconf \
    automake \
    libtool \
    pkg-config

# Verify installation
echo "Verifying installations..."
which gcc || { echo "Error: gcc not found"; exit 1; }
which make || { echo "Error: make not found"; exit 1; }
which autoconf || { echo "Error: autoconf not found"; exit 1; }

# Configure the build
echo "Configuring GNU Go build..."
./configure \
    --host=x86_64-w64-mingw32 \
    --build=$(uname -m)-pc-linux-gnu \
    --enable-color \
    --with-curses \
    --enable-socket-support \
    --disable-debug

# Build the program
echo "Building GNU Go..."
make clean
make -j$(nproc)

# Check if build succeeded
if [ $? -eq 0 ]; then
    echo "Successfully built GNU Go for Windows!"
    echo "Binary located at: ./gnugo.exe"
else
    echo "Build failed. Please check the error messages above."
    exit 1
fi

# Create a simple test script
cat > test_gnugo.bat << 'EOF'
@echo off
echo Running GNU Go test...
./gnugo.exe -l 10 -d 5 -c 1000 -r 1000 -q < test.sgf
pause
EOF

echo "Build complete! You can run the Windows version with:"
echo "./test_gnugo.bat"