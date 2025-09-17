#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

// The alloc crate is now enabled via .cargo/config.toml
extern crate alloc;

// NOTE: We are temporarily removing the VFS/allocator code to get a clean boot.
// We will re-add it in the next step.

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

mod arch;
// mod core;
// mod fs;
// mod panic; // Panic handler is now in this file

// New entry point signature for bootloader v0.9
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    x86_64::instructions::interrupts::disable();
    arch::x86_64::vga_buffer::init();
    
    println!("LimitlessOS Kernel -- Booting...");
    println!("[OK] Boot successful with new bootloader.");
    println!("\nHalting CPU.");

    hlt_loop();
}

/// A simple loop that repeatedly halts the CPU.
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n*** KERNEL PANIC ***");
    println!("{}", info);
    hlt_loop();
}
