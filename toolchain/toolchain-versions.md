# LimitlessOS Toolchain Specification

This document specifies the exact versions of the tools required to build LimitlessOS from source.

## Primary Toolchain

* **Rust**: `stable 1.79.0` (or newer)
    * Targets: `x86_64-unknown-none` (for kernel), `x86_64-unknown-limitless` (custom userspace target)
    * Components: `rustc`, `cargo`, `rustfmt`, `clippy`
* **LLVM / Clang**: `18.1.0` (or newer)
    * Purpose: C/ASM compilation, linking (via `lld`), and cross-ABI object file generation (PE, ELF, Mach-O).
* **Build System**: `GNU Make 4.3` (or newer)
* **Source Control**: `Git 2.40` (or newer)

## Host System Prerequisites

Developers should install the following packages on their host machine (e.g., via `apt`, `brew`, `pacman`):
* `build-essential` or equivalent C/C++ build tools
* `qemu-system-x86`
* `mtools`
* `xorriso`

## ISO & Testing

* **QEMU**: `8.2.0` (or newer)
    * Purpose: Emulation for automated boot, integration, and kernel tests.
* **ISO Toolkit**: `xorriso`
    * Purpose: Assembling the final bootable ISO image.
