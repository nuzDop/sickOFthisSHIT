# LimitlessOS

LimitlessOS is a modern, secure, and high-performance operating system designed from the ground up for native multi-ABI execution. It aims to run unmodified Windows (PE), Linux (ELF), and macOS (Mach-O) binaries on x86_64 hardware with first-class performance and security.

## Core Architecture

* **Micro-hybrid Kernel**: Written primarily in Rust for safety and performance.
* **Native Multi-ABI Subsystem**: No emulation or virtualization. PE, ELF, and Mach-O binaries are loaded and executed natively via dedicated system loaders and clean-room ABI runtime libraries.
* **Axiom AI**: A system-level AI service for intelligent task automation and system management.
* **Security First**: Full-disk encryption, secure boot, signed binaries, and mandatory access control are default.

## Build Prerequisites

Ensure you have the tools specified in `toolchain/toolchain-versions.md` installed on your system. This includes Rust, LLVM/Clang, QEMU, and standard build utilities.

## How to Build

A full build can be orchestrated using the main build script:

```sh
# This will build the toolchain, kernel, userspace, and assemble the ISO
./build.sh

# How to Run

After a successful build, you can run LimitlessOS in QEMU:

```Bash

# The build script will output the path to the final ISO
qemu-system-x86_64 -cdrom build/limitless-os.iso -m 2G -boot d
