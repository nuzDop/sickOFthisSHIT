#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::arch::x86_64::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

mod arch;
mod core;
mod drivers;
mod fs;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    arch::x86_64::vga_buffer::init();
    
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        drivers::gpu::framebuffer::init(framebuffer);
    }
    
    drivers::gpu::framebuffer::FRAMEBUFFER_WRITER.get().unwrap().lock().clear(0x001a1a2a);
    println!("LimitlessOS Kernel -- Booting...");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    
    let mut mapper = unsafe { crate::core::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        crate::core::allocator::BootInfoFrameAllocator::new(&boot_info.memory_map)
    };

    crate::core::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed");

    println!("[OK] Kernel Heap Initialized.");
    
    fs::init();
    println!("[OK] VFS Initialized (ramfs).");

    println!("\nTesting VFS...");
    if let Some(file_content) = fs::vfs::ROOT_FS.lock().as_ref().unwrap().read("/welcome.txt") {
        let text = core::str::from_utf8(&file_content).unwrap_or("Invalid UTF-8");
        println!("  Read from /welcome.txt: \"{}\"", text);
    } else {
        println!("  Failed to read /welcome.txt");
    }

    drivers::gpu::framebuffer::FRAMEBUFFER_WRITER
        .get().unwrap().lock().fill_rect(100, 100, 200, 150, 0x00e1e1e1);

    println!("\nInitialization complete. Halting CPU.");
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n*** KERNEL PANIC ***");
    println!("{}", info);
    if let Some(writer) = drivers::gpu::framebuffer::FRAMEBUFFER_WRITER.get() {
        writer.lock().clear(0x00ff0000);
    }
    hlt_loop();
}
