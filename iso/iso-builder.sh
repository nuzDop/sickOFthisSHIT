#!/bin/bash
set -e

# This is a STUB script for ISO creation.
# It will be replaced with a real implementation using xorriso in a later phase.

ISO_PATH=$1
BUILD_DIR=$2

echo "-> Creating dummy ISO at $ISO_PATH"

# Create a dummy file to represent the OS
mkdir -p "$BUILD_DIR/isofiles"
echo "LimitlessOS Boot Stub" > "$BUILD_DIR/isofiles/boot.txt"

# Create a dummy ISO file
# In a real scenario, this would use xorriso with bootloader info.
dd if=/dev/zero of="$ISO_PATH" bs=1M count=10
echo "-> Dummy ISO created."
