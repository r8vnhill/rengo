#!/bin/bash

# Create directories if they don't exist
[ ! -d "build" ] && mkdir -p "build"
[ ! -d "build/obj" ] && mkdir -p "build/obj"
[ ! -d "build/bin" ] && mkdir -p "build/bin"
[ ! -d "build/lib" ] && mkdir -p "build/lib"

# Determine the OS and select the appropriate makefile
OS=$(uname -s)
MAKEFILE=""

case "$OS" in
    Linux*)     MAKEFILE="linux/Makefile" ;;
    Darwin*)    MAKEFILE="mac/Makefile" ;;  # Assuming "mac/Makefile" is the makefile for macOS
    *)          echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Run make command with the selected makefile
make --makefile="$MAKEFILE" "$@"

# Remove empty directories
find build -type d -empty -delete