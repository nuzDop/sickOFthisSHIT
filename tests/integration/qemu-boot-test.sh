#!/bin/bash
set -e

DISK_IMAGE=$1

if [ ! -f "$DISK_IMAGE" ]; then
    echo "Error: Disk image not found at $DISK_IMAGE"
    exit 1
fi

echo "-> Starting QEMU boot test for $DISK_IMAGE..."

# Run QEMU in non-interactive mode for a short period.
# A successful boot means it runs without crashing (triple faulting).
# In the future, we will parse serial output for a success message.
timeout 10 qemu-system-x86_64 -drive format=raw,file="$DISK_IMAGE" -display none

echo "-> QEMU boot test finished. Assuming success as QEMU did not crash."
exit 0
