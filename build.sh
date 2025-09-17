#!/bin/bash
set -e

echo "========================================"
echo "    LIMITLESS OS - FULL SYSTEM BUILD    "
echo "========================================"

# The build process is now much simpler.
echo "[1/2] Building bootable kernel..."
make build
echo "    -> Build complete."

echo "[2/2] Success!"
echo ""
echo "The bootable kernel is located at: target/x86_64-limitless/debug/kernel"
echo "To run, execute: make run"
echo ""
